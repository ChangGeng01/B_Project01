//! 模块码枚举，按技术基线第 1.2 节的 15 个模块码冻结。

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, serde::Serialize, serde::Deserialize)]
pub enum ModuleCode {
    Mdm,
    Crm,
    Cpq,
    Clm,
    Sales,
    Procure,
    Inventory,
    Costing,
    Project,
    Service,
    Finance,
    Ledger,
    Invoice,
    Portal,
    Reporting,
}

impl ModuleCode {
    /// 全部 15 项，顺序即第 1.2 节表格顺序。
    pub const ALL: [ModuleCode; 15] = [
        ModuleCode::Mdm,
        ModuleCode::Crm,
        ModuleCode::Cpq,
        ModuleCode::Clm,
        ModuleCode::Sales,
        ModuleCode::Procure,
        ModuleCode::Inventory,
        ModuleCode::Costing,
        ModuleCode::Project,
        ModuleCode::Service,
        ModuleCode::Finance,
        ModuleCode::Ledger,
        ModuleCode::Invoice,
        ModuleCode::Portal,
        ModuleCode::Reporting,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            ModuleCode::Mdm => "mdm",
            ModuleCode::Crm => "crm",
            ModuleCode::Cpq => "cpq",
            ModuleCode::Clm => "clm",
            ModuleCode::Sales => "sales",
            ModuleCode::Procure => "procure",
            ModuleCode::Inventory => "inventory",
            ModuleCode::Costing => "costing",
            ModuleCode::Project => "project",
            ModuleCode::Service => "service",
            ModuleCode::Finance => "finance",
            ModuleCode::Ledger => "ledger",
            ModuleCode::Invoice => "invoice",
            ModuleCode::Portal => "portal",
            ModuleCode::Reporting => "reporting",
        }
    }
}

impl core::fmt::Display for ModuleCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}
