use crate::spreadsheet::{Cell, DataTable};

/// Excel-compatible SpreadsheetML 2003 XML export.
pub fn to_excel_xml(table: &DataTable) -> String {
    let mut out = String::new();
    out.push_str("<?xml version=\"1.0\"?>\n");
    out.push_str("<?mso-application progid=\"Excel.Sheet\"?>\n");
    out.push_str("<Workbook xmlns=\"urn:schemas-microsoft-com:office:spreadsheet\" ");
    out.push_str("xmlns:ss=\"urn:schemas-microsoft-com:office:spreadsheet\">\n");
    out.push_str("<Worksheet ss:Name=\"Data\"><Table>\n<Row>");
    for column in &table.columns {
        out.push_str("<Cell><Data ss:Type=\"String\">");
        out.push_str(&xml_escape(column));
        out.push_str("</Data></Cell>");
    }
    out.push_str("</Row>\n");
    for row in &table.rows {
        out.push_str("<Row>");
        for cell in row {
            match cell {
                Cell::Number(value) => {
                    out.push_str("<Cell><Data ss:Type=\"Number\">");
                    out.push_str(&value.to_string());
                    out.push_str("</Data></Cell>");
                }
                Cell::Boolean(value) => {
                    out.push_str("<Cell><Data ss:Type=\"Boolean\">");
                    out.push_str(if *value { "1" } else { "0" });
                    out.push_str("</Data></Cell>");
                }
                Cell::Text(value) => {
                    out.push_str("<Cell><Data ss:Type=\"String\">");
                    out.push_str(&xml_escape(value));
                    out.push_str("</Data></Cell>");
                }
                Cell::Empty => out.push_str("<Cell/>")
            }
        }
        out.push_str("</Row>\n");
    }
    out.push_str("</Table></Worksheet></Workbook>\n");
    out
}

fn xml_escape(value: &str) -> String {
    value.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exports_typed_excel_cells() {
        let mut table = DataTable::new("results", vec!["n".into(), "prime".into()]);
        table.add_row(vec![Cell::Number(7.0), Cell::Boolean(true)]).unwrap();
        let xml = to_excel_xml(&table);
        assert!(xml.contains("ss:Type=\"Number\""));
        assert!(xml.contains("ss:Type=\"Boolean\""));
        assert!(xml.contains("7"));
    }
}
