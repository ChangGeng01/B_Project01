//! 样本档（`--scale`）的登记与解析。
//!
//! 档位名与取数口径的权威在技术基线第 625 行：生成器接受 `--seed` 与 `--scale`，
//! 默认档对应规格附录 A.3 的规模。阶段 2 按 D-09 追加 `t0`（T0 最小样本的平台部分：
//! 1 个法人及其组织架构最小行）与 `small`（2 个法人）两档；`default` 已登记但未实现——
//! 它必须以「未交付」的独立退出码报出来，不得静默产出一个规模不足的数据集冒充默认档。

/// 已登记的全部档位名。未登记的档位名属参数错误，不是未交付。
pub const REGISTERED: [&str; 4] = ["t0-min", "t0", "small", "default"];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Scale {
    /// 阶段 1 的贯通线最小样本：一个法人、一个客户、一个产品。
    T0Min,
    /// T0 最小样本的平台部分：一个法人及其组织架构最小行（D-09）。
    T0,
    /// 两个法人的小型样本（D-09）：每个法人各带一套组织架构最小行。
    Small,
}

/// 解析失败的两种成因，对应两个不同的退出码，不得合并。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ScaleError {
    /// 档位名未登记，属调用方参数错误。
    Unknown(String),
    /// 档位名已登记但本阶段未实现。
    NotDelivered {
        name: &'static str,
        owner: &'static str,
    },
}

impl std::fmt::Display for ScaleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScaleError::Unknown(name) => {
                write!(f, "未知档位 {name}；已登记档位：{}", REGISTERED.join("、"))
            }
            ScaleError::NotDelivered { name, owner } => {
                write!(f, "档位 {name} 已登记但本阶段未交付（{owner}）")
            }
        }
    }
}

impl Scale {
    pub fn parse(name: &str) -> Result<Scale, ScaleError> {
        match name {
            "t0-min" => Ok(Scale::T0Min),
            "t0" => Ok(Scale::T0),
            "small" => Ok(Scale::Small),
            "default" => Err(ScaleError::NotDelivered {
                name: "default",
                owner: "规格附录 A.3 的规模取值，由后续阶段的性能基线一并交付",
            }),
            other => Err(ScaleError::Unknown(other.to_string())),
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Scale::T0Min => "t0-min",
            Scale::T0 => "t0",
            Scale::Small => "small",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t0_min_parses() {
        assert_eq!(Scale::parse("t0-min"), Ok(Scale::T0Min));
        assert_eq!(Scale::T0Min.as_str(), "t0-min");
    }

    /// D-09：`t0` 与 `small` 两档必须可解析且档位名来回一致。
    #[test]
    fn t0_and_small_parse() {
        assert_eq!(Scale::parse("t0"), Ok(Scale::T0));
        assert_eq!(Scale::T0.as_str(), "t0");
        assert_eq!(Scale::parse("small"), Ok(Scale::Small));
        assert_eq!(Scale::Small.as_str(), "small");
    }

    /// 负样例：已登记但未交付的档位必须报「未交付」，绝不落到某个能跑的分支上。
    #[test]
    fn default_scale_is_reported_as_not_delivered() {
        assert!(matches!(
            Scale::parse("default"),
            Err(ScaleError::NotDelivered {
                name: "default",
                ..
            })
        ));
    }

    /// 负样例：未登记的档位是参数错误，与「未交付」区分开。
    #[test]
    fn unknown_scale_is_a_parameter_error() {
        assert_eq!(
            Scale::parse("t0-mega"),
            Err(ScaleError::Unknown("t0-mega".to_string()))
        );
        assert_eq!(Scale::parse(""), Err(ScaleError::Unknown(String::new())));
    }

    /// 登记表必须覆盖每个可解析的档位，新增档位忘了登记会被这条拦下。
    #[test]
    fn every_parsable_scale_is_registered() {
        for scale in [Scale::T0Min, Scale::T0, Scale::Small] {
            assert!(REGISTERED.contains(&scale.as_str()));
        }
        for name in REGISTERED {
            assert!(
                !matches!(Scale::parse(name), Err(ScaleError::Unknown(_))),
                "已登记档位 {name} 却被判为未知"
            );
        }
    }
}
