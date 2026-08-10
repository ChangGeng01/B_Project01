//! 文档与打印端口。
//!
//! 阶段 1 只建空文件。SheetSpec、ColumnSpec、CellValue、PrintLayout 与
//! SpreadsheetPort、DocTemplatePort、PdfRenderPort 由阶段 5 按 A-08 补齐，
//! 实现落在 ep-adapter-doc，其后各阶段只在这三个 trait 上增量。
