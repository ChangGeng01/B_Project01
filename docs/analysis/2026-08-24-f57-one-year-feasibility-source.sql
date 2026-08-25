-- F-57 one-year feasibility report source, SQLite-compatible.
-- These planning rows reproduce the executed notebook's reviewed report datasets.
-- They are assumptions and derived results, not production measurements.

DROP TABLE IF EXISTS constants;
CREATE TABLE constants (
    capacity_gib REAL NOT NULL,
    red_free_gib REAL NOT NULL
);
INSERT INTO constants VALUES (931.3225746154785, 46.5661287307739);

DROP TABLE IF EXISTS scenarios;
CREATE TABLE scenarios (
    scenario TEXT PRIMARY KEY,
    start_gib REAL NOT NULL,
    workspace_gib REAL NOT NULL,
    annual_growth_gib REAL NOT NULL,
    p95_daily_growth_gib REAL NOT NULL,
    yellow_free_gib REAL NOT NULL,
    attachment_count INTEGER NOT NULL,
    avg_attachment_mib REAL NOT NULL,
    offline_media_floor_gib REAL NOT NULL,
    first_yellow_month INTEGER,
    first_red_month INTEGER,
    end_status TEXT NOT NULL
);
INSERT INTO scenarios VALUES
    ('轻载', 35, 45, 114, 0.963993, 93.132, 30000, 2.048, 207.720, NULL, NULL, 'GREEN'),
    ('基准', 40, 60, 264, 2.187504, 93.132, 60000, 2.560, 430.425, NULL, NULL, 'GREEN'),
    ('附件重载压力', 40, 80, 810, 7.200667, 216.020, 120000, 5.120, 1236.020, 9, 12, 'RED');

DROP TABLE IF EXISTS month_weights;
CREATE TABLE month_weights (
    month TEXT PRIMARY KEY,
    month_num INTEGER UNIQUE NOT NULL,
    weight REAL NOT NULL
);
INSERT INTO month_weights VALUES
    ('M1', 1, 0.07), ('M2', 2, 0.07), ('M3', 3, 0.08),
    ('M4', 4, 0.08), ('M5', 5, 0.08), ('M6', 6, 0.08),
    ('M7', 7, 0.08), ('M8', 8, 0.08), ('M9', 9, 0.12),
    ('M10', 10, 0.08), ('M11', 11, 0.09), ('M12', 12, 0.09);

DROP VIEW IF EXISTS monthly_storage;
CREATE VIEW monthly_storage AS
WITH cumulative AS (
    SELECT
        m.month,
        m.month_num,
        SUM(prior.weight) AS cumulative_weight
    FROM month_weights AS m
    JOIN month_weights AS prior ON prior.month_num <= m.month_num
    GROUP BY m.month, m.month_num
), calculated AS (
    SELECT
        c.month,
        c.month_num,
        s.scenario,
        s.start_gib + s.workspace_gib + s.annual_growth_gib * c.cumulative_weight AS used_gib,
        k.capacity_gib - (s.start_gib + s.workspace_gib + s.annual_growth_gib * c.cumulative_weight) AS free_gib,
        s.yellow_free_gib,
        k.red_free_gib,
        s.annual_growth_gib,
        s.attachment_count,
        s.avg_attachment_mib,
        k.capacity_gib
    FROM cumulative AS c
    CROSS JOIN scenarios AS s
    CROSS JOIN constants AS k
)
SELECT
    month,
    month_num,
    scenario,
    ROUND(used_gib, 3) AS used_gib,
    ROUND(free_gib, 3) AS free_gib,
    used_gib / capacity_gib AS occupancy_rate,
    CASE
        WHEN free_gib < red_free_gib THEN 'RED'
        WHEN free_gib < yellow_free_gib THEN 'YELLOW'
        ELSE 'GREEN'
    END AS status,
    yellow_free_gib,
    ROUND(red_free_gib, 3) AS red_free_gib,
    annual_growth_gib,
    attachment_count,
    avg_attachment_mib
FROM calculated;

DROP VIEW IF EXISTS scenario_summary;
CREATE VIEW scenario_summary AS
SELECT
    s.scenario,
    s.start_gib + s.workspace_gib + s.annual_growth_gib AS end_used_gib,
    ROUND(k.capacity_gib - (s.start_gib + s.workspace_gib + s.annual_growth_gib), 3) AS end_free_gib,
    (s.start_gib + s.workspace_gib + s.annual_growth_gib) / k.capacity_gib AS occupancy_rate,
    s.p95_daily_growth_gib,
    s.yellow_free_gib,
    ROUND(k.red_free_gib, 3) AS red_free_gib,
    s.first_yellow_month,
    s.first_red_month,
    s.end_status,
    s.offline_media_floor_gib,
    s.attachment_count,
    s.avg_attachment_mib,
    s.annual_growth_gib
FROM scenarios AS s
CROSS JOIN constants AS k;

DROP VIEW IF EXISTS storage_composition;
CREATE VIEW storage_composition AS
SELECT
    scenario,
    '已占用' AS component,
    end_used_gib AS gib,
    end_status,
    occupancy_rate,
    p95_daily_growth_gib,
    yellow_free_gib,
    attachment_count,
    avg_attachment_mib
FROM scenario_summary
UNION ALL
SELECT
    scenario,
    '剩余' AS component,
    end_free_gib AS gib,
    end_status,
    occupancy_rate,
    p95_daily_growth_gib,
    yellow_free_gib,
    attachment_count,
    avg_attachment_mib
FROM scenario_summary;

DROP VIEW IF EXISTS headline;
CREATE VIEW headline AS
SELECT
    (SELECT end_used_gib FROM scenario_summary WHERE scenario = '基准') AS baseline_end_used_gib,
    (SELECT end_free_gib FROM scenario_summary WHERE scenario = '基准') AS baseline_end_free_gib,
    (SELECT first_yellow_month FROM scenario_summary WHERE scenario = '附件重载压力') AS pressure_first_yellow_month,
    52.0 / 25.0 AS average_weeks_per_task;

DROP TABLE IF EXISTS production_gates;
CREATE TABLE production_gates (
    gate_order INTEGER PRIMARY KEY,
    gate TEXT NOT NULL,
    current_evidence TEXT NOT NULL,
    result TEXT NOT NULL
);
INSERT INTO production_gates VALUES
    (1, '完成 F57-01…F57-25 与最终签名聚合', 'IMPLEMENTATION_NOT_STARTED', 'BLOCKING'),
    (2, '数据盘 CMR、SMART、BitLocker、flush 与卷身份取证', 'UNKNOWN', 'BLOCKING'),
    (3, '可通信 UPS 与安全关机/恢复启动演练', 'UNKNOWN', 'BLOCKING'),
    (4, '服务器外、独立身份、只追加连续目标', 'UNKNOWN', 'BLOCKING'),
    (5, '至少两块加密且平时断开的离线轮换 HDD', 'UNKNOWN', 'BLOCKING'),
    (6, '应用与备份两套分域恢复材料（共六枚独立 PIV）', 'UNKNOWN', 'BLOCKING'),
    (7, '洁净 Windows 恢复主机及足够工作空间', 'UNKNOWN', 'BLOCKING'),
    (8, '20 人 + Control Center + 备份/报表的 72 小时实机负载', 'NOT_RUN', 'BLOCKING'),
    (9, '完整恢复、PITR、附件一致 cut 与投毒备份演练', 'NOT_RUN', 'BLOCKING'),
    (10, '生产、备份、恢复、provider、支持链路中国大陆驻留证据', 'UNKNOWN', 'BLOCKING');

DROP TABLE IF EXISTS year_events;
CREATE TABLE year_events (
    month INTEGER PRIMARY KEY,
    event TEXT NOT NULL,
    expected_result TEXT NOT NULL,
    runtime_evidence TEXT NOT NULL
);
INSERT INTO year_events VALUES
    (1, '上线、导入与权限初始化', '法人/密钥域/权限重验；客户字节只落 HDD；导入有检查点', 'NOT_IMPLEMENTED'),
    (2, '报价→合同→订单', '转换幂等，版本、来源、审批和审计可追溯', 'NOT_IMPLEMENTED'),
    (3, '采购与分批到货', '数量不超上游；部分收货、交付、退货可继续闭环', 'NOT_IMPLEMENTED'),
    (4, '锁期后迟到发票', '不重开已锁期间；顺延开放期间并保留更正链', 'NOT_IMPLEMENTED'),
    (5, '模块热升级时存在长流程', '新请求见新代；在途流程钉住旧兼容代；失败回滚', 'NOT_IMPLEMENTED'),
    (6, '第一次洁净恢复演练', '从服务器外、离线介质和分域材料恢复并对账', 'NOT_IMPLEMENTED'),
    (7, '离职、设备撤销与临时委派', '权限实时收回；委派有范围/到期；离线意图重验', 'NOT_IMPLEMENTED'),
    (8, 'MCP/外部系统超时但可能成功', '进入 Unknown；先对账，禁止盲重做不可逆动作', 'NOT_IMPLEMENTED'),
    (9, '订单与附件双倍峰值', '交易优先；报表/导入排队；到黄色线自动降级', 'NOT_IMPLEMENTED'),
    (10, '扫描定义过期或恶意附件', '继续隔离；UNKNOWN/SKIPPED/超时一律不发布', 'NOT_IMPLEMENTED'),
    (11, '勒索演练且最新备份投毒', 'fence 权威端；跳过污染代；洁净恢复 known-clean 代', 'NOT_IMPLEMENTED'),
    (12, '年结、完整导出与保留', '只追加更正；可验证导出；legal hold/retention 不受许可证破坏', 'NOT_IMPLEMENTED');

DROP TABLE IF EXISTS build_plan;
CREATE TABLE build_plan (
    quarter_order INTEGER PRIMARY KEY,
    quarter TEXT NOT NULL,
    scope TEXT NOT NULL,
    target TEXT NOT NULL,
    schedule_assessment TEXT NOT NULL
);
INSERT INTO build_plan VALUES
    (1, 'Q1', 'Tasks 1–6', '再基线、签名存储、核心业务持久化、统一命令权威', '激进但可管理'),
    (2, 'Q2', 'Tasks 7–15', '容量、动态权限、generation、耐久消息/自动化、包与 MCP provider', '关键依赖密集'),
    (3, 'Q3', 'Tasks 16–23', '四端 Workbench、离线安全、完整业务闭环、门户、Excel 与生命周期', '最高延期风险'),
    (4, 'Q4', 'Tasks 24–25', 'Windows/P340、72小时、恢复/勒索、最终聚合与受控试点', '仅在前序零重大返工时成立');

DROP TABLE IF EXISTS commercial_scale;
CREATE TABLE commercial_scale (
    client_count INTEGER PRIMARY KEY,
    assumed_annual_fee_cny INTEGER NOT NULL,
    annual_license_revenue_m_cny REAL NOT NULL,
    separate_activation_count_assumption INTEGER NOT NULL,
    assumed_activation_fee_cny INTEGER NOT NULL,
    activation_revenue_m_cny REAL NOT NULL,
    combined_target_m_cny REAL
);
INSERT INTO commercial_scale VALUES
    (10, 320000, 3.2, 50, 80000, 4, NULL),
    (50, 320000, 16, 50, 80000, 4, NULL),
    (100, 320000, 32, 50, 80000, 4, NULL),
    (300, 320000, 96, 50, 80000, 4, 100);

-- Reviewed report datasets.
SELECT * FROM headline;
SELECT * FROM monthly_storage ORDER BY scenario, month_num;
SELECT * FROM storage_composition ORDER BY scenario, component;
SELECT * FROM scenario_summary ORDER BY end_used_gib DESC;
SELECT gate_order AS "order", gate, current_evidence, result FROM production_gates ORDER BY gate_order;
SELECT * FROM year_events ORDER BY month;
SELECT * FROM build_plan ORDER BY quarter_order;
SELECT * FROM commercial_scale ORDER BY client_count;
