//! 用法与版本文本。单列一个模块，是为了让退出码约定只有一处出处：用法里印
//! 的六个码直接由 `MigrateExit::ALL` 生成，改了枚举而忘了改文档这件事不会发生。

use crate::cli::{
    Subcommand, DB_URL_ENV, DEFAULT_HISTORY_SCHEMA, DEFAULT_HISTORY_TABLE, DEFAULT_MIGRATIONS_DIR,
    DEFAULT_TTL_MINUTES, MAX_TTL_MINUTES, SUBCOMMANDS,
};
use crate::exit::MigrateExit;
use crate::preflight::TOOL_VERSION;

fn exit_code_table() -> String {
    MigrateExit::ALL
        .iter()
        .map(|e| format!("  {:>3}  {}", e.code(), e.label()))
        .collect::<Vec<_>>()
        .join("\n")
}

fn subcommand_options(sub: Subcommand) -> String {
    sub.options()
        .iter()
        .map(|o| format!("--{o}"))
        .collect::<Vec<_>>()
        .join(" ")
}

/// 版本文本。连带印出被冻结的迁移历史表参数，方便运维核对制品。
pub fn version() -> String {
    format!(
        "ep-migrate {TOOL_VERSION}\n\
         迁移历史表：{DEFAULT_HISTORY_SCHEMA}.{DEFAULT_HISTORY_TABLE}（全库唯一，结构任何阶段不改）\n\
         迁移目录默认：{DEFAULT_MIGRATIONS_DIR}\n\
         子命令实现体由阶段 2 交付，本制品只含参数解析与退出码约定。"
    )
}

/// 用法文本。给了子命令就只印该子命令那一段。
pub fn usage(sub: Option<Subcommand>) -> String {
    let mut out = String::new();
    out.push_str("ep-migrate — 迁移执行 CLI。阶段 1 只交付参数解析与退出码约定。\n\n");

    match sub {
        Some(s) => {
            out.push_str(&format!("用法：ep-migrate {} [选项]\n", s.name()));
            out.push_str(&format!("可用选项：{}\n", subcommand_options(s)));
            if s == Subcommand::Apply {
                out.push_str(
                    "  --window-id 必填：apply 必须出示一个由 open-window 开启的迁移窗口，\
                     缺它即退出码 3。\n",
                );
            }
            if s == Subcommand::OpenWindow {
                out.push_str(&format!(
                    "  --ttl-minutes 默认 {DEFAULT_TTL_MINUTES}，上限 {MAX_TTL_MINUTES}。\n"
                ));
            }
        }
        None => {
            out.push_str(&format!("用法：ep-migrate <{}> [选项]\n\n", SUBCOMMANDS.join("|")));
            out.push_str("子命令：\n");
            out.push_str("  apply         按文件版本号全序执行迁移\n");
            out.push_str("  status        输出迁移历史表的单一版本，--format=manifest 输出制品清单\n");
            out.push_str("  check         执行 db/checks/ 的编号合规断言\n");
            out.push_str("  gen-rls       按行级安全模板生成策略语句\n");
            out.push_str("  open-window   开启迁移窗口\n");
        }
    }

    out.push_str(&format!(
        "\n连接串来源：--db-url，缺省时读环境变量 {DB_URL_ENV}。gen-rls 不连库。\n"
    ));
    out.push_str("\n退出码：\n");
    out.push_str(&exit_code_table());
    out.push_str(
        "\n\n本阶段的实现状态：五个子命令的实现体按裁定 C-01 与 C-02 归阶段 2。\
         本制品跑完前置阶梯后一律以退出码 78 报出环境自检项 subcommand-implemented \
         不通过，不会静默返回 0，也不会执行任何迁移动作。\n",
    );
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usage_lists_all_six_exit_codes() {
        let text = usage(None);
        for e in MigrateExit::ALL {
            assert!(
                text.contains(&format!("{}  {}", e.code(), e.label())),
                "用法文本必须列出退出码 {}：{text}",
                e.code()
            );
        }
    }

    #[test]
    fn usage_lists_all_five_subcommands() {
        let text = usage(None);
        for name in SUBCOMMANDS {
            assert!(text.contains(name), "用法文本必须列出子命令 {name}");
        }
    }

    #[test]
    fn version_pins_the_history_table() {
        let text = version();
        assert!(text.contains("platform_core.schema_history"));
    }
}
