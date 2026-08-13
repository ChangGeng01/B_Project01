-- rollback: 无法安全逆向。六个约定函数、三个触发器函数与两个姊妹函数被后续全部
-- rollback: 建表迁移引用，撤销将使已挂接触发器与已建策略失去实现；
-- rollback: 回退只能用升级前备份或影子表回退。
-- db/migrations/platform_core/V20260901090500__platform_core_conventions.sql
-- 阶段 2 计划第 3.6、3.7、3.8 节的机制交付：
--   六个约定函数：business_day、business_today、current_legal_entity、effective_level、
--                 apply_le_rls、attach_table_guards；
--   三个触发器函数：assert_row_version_bump、assert_append_only、assert_immutable_columns；
--   姊妹函数：assert_baseline_indexes。
-- 行级安全策略全库只经 apply_le_rls 一份模板生成，迁移中不得手写策略（第 3.6 节）。
set role ep_mod_platform_core;
set lock_timeout = '5s';
set statement_timeout = '30min';

-- 营业日：按 Asia/Shanghai 取自然日。IMMUTABLE 便于单元测试直接喂固定时刻。
create or replace function platform_core.business_day(ts timestamptz)
returns date
language sql immutable as $$
  select (ts at time zone 'Asia/Shanghai')::date;
$$;

-- 服务器当前营业日。全库禁止直接取服务器自然日（sqlcheck SQL-004 与 db/checks/09 双重拦截），
-- 一律取本函数。STABLE：同一事务内同一时刻取值不变。
create or replace function platform_core.business_today()
returns date
language sql stable as $$
  select platform_core.business_day(now());
$$;

-- 会话变量中的当前法人。变量缺失时返回 NULL，比较结果为 NULL 即默认拒绝。
create or replace function platform_core.current_legal_entity()
returns uuid
language sql stable as $$
  select nullif(current_setting('app.legal_entity_id', true), '')::uuid;
$$;

-- 字段级密级未赋值（NULL）时取所属对象密级。
create or replace function platform_core.effective_level(obj smallint, fld smallint)
returns smallint
language sql immutable as $$
  select coalesce(fld, obj);
$$;

-- 行级安全唯一模板。enable + force 之后按固定策略名 rls_<table>_le 建策略，
-- using 与 with check 均为法人等值判定；会话变量缺失时比较结果为 NULL，默认拒绝。
-- 先按名 drop 再 create 使本函数可重复调用（幂等）。
create or replace function platform_core.apply_le_rls(p_schema text, p_table text)
returns void language plpgsql as $$
begin
  execute format('alter table %I.%I enable row level security', p_schema, p_table);
  execute format('alter table %I.%I force row level security', p_schema, p_table);
  execute format('drop policy if exists %I on %I.%I',
                 'rls_' || p_table || '_le', p_schema, p_table);
  execute format(
    'create policy %I on %I.%I using (legal_entity_id = nullif(current_setting(''app.legal_entity_id'', true), '''')::uuid) '
    'with check (legal_entity_id = nullif(current_setting(''app.legal_entity_id'', true), '''')::uuid)',
    'rls_' || p_table || '_le', p_schema, p_table);
end $$;

-- 乐观锁守卫：BEFORE UPDATE，NEW.row_version 必须恰为 OLD.row_version + 1。
-- 把基线第 3.7 节的乐观锁写法从代码纪律变成数据库约束，代码路径遗漏时立即失败。
create or replace function platform_core.assert_row_version_bump()
returns trigger language plpgsql as $$
begin
  if new.row_version is distinct from old.row_version + 1 then
    raise exception '%.%: row_version must be old.row_version + 1 on update (got %, expected %)',
      tg_table_schema, tg_table_name, new.row_version, old.row_version + 1;
  end if;
  return new;
end $$;

-- 仅追加守卫：BEFORE UPDATE OR DELETE，一律拒绝。
-- 用于裁定 B-02 终表中 mode 取 APPEND_ONLY 的表，逐表清单以 B-02 为唯一出处。
create or replace function platform_core.assert_append_only()
returns trigger language plpgsql as $$
begin
  raise exception '%.%: table is APPEND_ONLY; updates and physical removal are forbidden',
    tg_table_schema, tg_table_name;
  return null;
end $$;

-- 列级不可变守卫：BEFORE UPDATE，mutable_columns 白名单之外的列有任何变化即拒绝。
-- 用于 Outbox 与死信表：Outbox 白名单为 status、attempts、available_at、locked_by、
-- locked_until、last_error；死信白名单为 state、repaired_by、repaired_at、approval_ref、
-- discard_reason。白名单以 platform_core.append_only_registry.mutable_columns 登记为唯一出处，
-- 少登记一列即在上线后拒绝对应路径的写入。
create or replace function platform_core.assert_immutable_columns()
returns trigger language plpgsql as $$
declare
  v_mutable text[];
  v_old jsonb := to_jsonb(old);
  v_new jsonb := to_jsonb(new);
  v_changed text[] := '{}';
  v_col text;
begin
  select r.mutable_columns into v_mutable
  from platform_core.append_only_registry r
  where r.schema_name = tg_table_schema and r.table_name = tg_table_name;
  if v_mutable is null then
    raise exception '%.%: not registered in platform_core.append_only_registry, immutable-columns guard refuses write',
      tg_table_schema, tg_table_name;
  end if;
  for v_col in
    select a.attname
    from pg_attribute a
    where a.attrelid = tg_relid and a.attnum > 0 and not a.attisdropped
      and not (a.attname = any (v_mutable))
  loop
    if (v_old -> v_col) is distinct from (v_new -> v_col) then
      v_changed := v_changed || v_col;
    end if;
  end loop;
  if cardinality(v_changed) > 0 then
    raise exception '%.%: IMMUTABLE_COLUMNS violation, columns changed outside mutable_columns whitelist: %',
      tg_table_schema, tg_table_name, array_to_string(v_changed, ', ');
  end if;
  return new;
end $$;

-- 基线索引断言：迁移末尾调用，缺失即迁移失败（第 3.8 节）。
-- 基线第 3.10 节三条：pk_<table>；带法人列的表须有 ix_<table>_legal_entity_id_created_at；
-- 单据类另加 ux_<table>_legal_entity_id_doc_no。
-- security invoker（默认）：函数只读 pg 目录，但 %I.%I::regclass 的标识符解析
-- 按生效角色权限走；若用 security definer，生效角色变为 ep_mod_platform_core，
-- 解析他 schema 的表会 permission denied。pg 目录对任意角色可读，调用方
-- 权限下无需额外授权。
create or replace function platform_core.assert_baseline_indexes(
  p_schema text, p_table text, p_is_document boolean)
returns void language plpgsql as $$
declare
  v_rel regclass;
  v_has_le boolean;
  v_missing text[] := '{}';
begin
  v_rel := format('%I.%I', p_schema, p_table)::regclass;
  select count(*) > 0 into v_has_le
  from pg_attribute a
  where a.attrelid = v_rel and a.attname = 'legal_entity_id'
    and a.attnum > 0 and not a.attisdropped;
  if not exists (select 1 from pg_constraint con
                 where con.conrelid = v_rel and con.contype = 'p') then
    v_missing := v_missing || ('pk_' || p_table);
  end if;
  if v_has_le and not exists (
      select 1 from pg_index i join pg_class ic on ic.oid = i.indexrelid
      where i.indrelid = v_rel
        and ic.relname = 'ix_' || p_table || '_legal_entity_id_created_at') then
    v_missing := v_missing || ('ix_' || p_table || '_legal_entity_id_created_at');
  end if;
  if p_is_document and v_has_le and not exists (
      select 1 from pg_index i join pg_class ic on ic.oid = i.indexrelid
      where i.indrelid = v_rel
        and ic.relname = 'ux_' || p_table || '_legal_entity_id_doc_no') then
    v_missing := v_missing || ('ux_' || p_table || '_legal_entity_id_doc_no');
  end if;
  if cardinality(v_missing) > 0 then
    raise exception '%.%: baseline indexes missing: %',
      p_schema, p_table, array_to_string(v_missing, ', ');
  end if;
end $$;

-- 守卫挂接器：各阶段建表迁移在末尾调用。按 append_only_registry 登记自动挂接触发器，
-- 对带法人列的表调用 apply_le_rls，使「登记即生效」不需要各迁移重复手写。
-- security invoker（默认）：本函数对目标表做 ALTER/CREATE TRIGGER，必须以表属主
-- 身份执行（PostgreSQL 要求 ALTER TABLE 的执行者是表属主），调用方恰是各表属主
-- 角色 ep_mod_<schema>。函数内对 platform_core 登记表的 SELECT 所需权限，由
-- 登记表创建迁移（V20260901093000）与授权收口迁移（V20260901100500）对全部
-- 24 个 ep_mod_* 显式授予。若改为 security definer，生效角色变为函数属主
-- ep_mod_platform_core，对他 schema 的表做 ALTER 会 permission denied。
create or replace function platform_core.attach_table_guards(p_schema text, p_table text)
returns void language plpgsql as $$
declare
  v_mode text;
  v_has_row_version boolean;
  v_has_le boolean;
begin
  -- 登记表自身由后续迁移（V20260901093000）创建；空库全序下本函数先于登记表被调用，
  -- 表不存在时尚无任何登记行，按未登记处理，不得因缺表报错。
  if to_regclass('platform_core.append_only_registry') is not null then
    select r.mode into v_mode
    from platform_core.append_only_registry r
    where r.schema_name = p_schema and r.table_name = p_table;
  end if;

  select count(*) > 0 into v_has_row_version
  from pg_class c join pg_namespace n on n.oid = c.relnamespace
    join pg_attribute a on a.attrelid = c.oid
  where n.nspname = p_schema and c.relname = p_table
    and a.attname = 'row_version' and a.attnum > 0 and not a.attisdropped;

  select count(*) > 0 into v_has_le
  from pg_class c join pg_namespace n on n.oid = c.relnamespace
    join pg_attribute a on a.attrelid = c.oid
  where n.nspname = p_schema and c.relname = p_table
    and a.attname = 'legal_entity_id' and a.attnum > 0 and not a.attisdropped;

  if v_mode = 'APPEND_ONLY' then
    execute format('drop trigger if exists %I on %I.%I',
                   'trg_' || p_table || '_append_only', p_schema, p_table);
    execute format('create trigger %I before update or delete on %I.%I for each row execute function platform_core.assert_append_only()',
                   'trg_' || p_table || '_append_only', p_schema, p_table);
  elsif v_mode = 'IMMUTABLE_COLUMNS' then
    execute format('drop trigger if exists %I on %I.%I',
                   'trg_' || p_table || '_immutable_columns', p_schema, p_table);
    execute format('create trigger %I before update on %I.%I for each row execute function platform_core.assert_immutable_columns()',
                   'trg_' || p_table || '_immutable_columns', p_schema, p_table);
  elsif v_has_row_version then
    execute format('drop trigger if exists %I on %I.%I',
                   'trg_' || p_table || '_row_version', p_schema, p_table);
    execute format('create trigger %I before update on %I.%I for each row execute function platform_core.assert_row_version_bump()',
                   'trg_' || p_table || '_row_version', p_schema, p_table);
  end if;

  if v_has_le then
    perform platform_core.apply_le_rls(p_schema, p_table);
  end if;
end $$;

reset role;
