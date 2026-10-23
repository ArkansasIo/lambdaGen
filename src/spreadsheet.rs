use std::fmt::Write;

#[derive(Debug, Clone, PartialEq)]
pub enum Cell { Empty, Text(String), Number(f64), Boolean(bool) }

#[derive(Debug, Clone)]
pub struct DataTable { pub name: String, pub columns: Vec<String>, pub rows: Vec<Vec<Cell>> }

impl DataTable {
    pub fn new(name: impl Into<String>, columns: Vec<String>) -> Self { Self { name:name.into(), columns, rows:Vec::new() } }
    pub fn add_row(&mut self, row: Vec<Cell>) -> Result<(), String> { if row.len()!=self.columns.len() { return Err("row width does not match column count".into()); } self.rows.push(row); Ok(()) }
    pub fn to_csv(&self) -> String {
        let mut out=String::new(); out.push_str(&self.columns.iter().map(|c| csv_escape(c)).collect::<Vec<_>>().join(",")); out.push('\n');
        for row in &self.rows { out.push_str(&row.iter().map(cell_text).map(|s| csv_escape(&s)).collect::<Vec<_>>().join(",")); out.push('\n'); }
        out
    }
    pub fn to_html_table(&self) -> String {
        let mut out=String::from("<table><thead><tr>");
        for c in &self.columns { let _=write!(out,"<th>{}</th>",html_escape(c)); }
        out.push_str("</tr></thead><tbody>");
        for row in &self.rows { out.push_str("<tr>"); for c in row { let _=write!(out,"<td>{}</td>",html_escape(&cell_text(c))); } out.push_str("</tr>"); }
        out.push_str("</tbody></table>"); out
    }
}

fn cell_text(cell:&Cell)->String { match cell { Cell::Empty=>String::new(), Cell::Text(s)=>s.clone(), Cell::Number(n)=>n.to_string(), Cell::Boolean(b)=>b.to_string() } }
fn csv_escape(s:&str)->String { if s.contains([',','\"','\n','\r']) { format!("\"{}\"",s.replace('\"',"\"\"")) } else { s.to_owned() } }
fn html_escape(s:&str)->String { s.replace('&',"&amp;").replace('<',"&lt;").replace('>',"&gt;").replace('\"',"&quot;") }

#[derive(Debug, Clone)]
pub struct SpreadsheetExport { pub table_name: String, pub format: String, pub content: String }

impl SpreadsheetExport {
    pub fn csv(table:&DataTable)->Self { Self { table_name:table.name.clone(), format:"csv".into(), content:table.to_csv() } }
    pub fn html(table:&DataTable)->Self { Self { table_name:table.name.clone(), format:"html-table".into(), content:table.to_html_table() } }
}
