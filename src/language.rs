use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value { Number(f64), Boolean(bool), Text(String), Null }

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let { name: String, value: Value },
    Set { name: String, value: Value },
    Return(Value),
    Print(Value),
    If { condition: bool, then_branch: Vec<Statement>, else_branch: Vec<Statement> },
}

#[derive(Debug, Default)]
pub struct Environment { values: HashMap<String, Value> }

impl Environment {
    pub fn new() -> Self { Self::default() }
    pub fn set(&mut self, name: impl Into<String>, value: Value) { self.values.insert(name.into(), value); }
    pub fn get(&self, name: &str) -> Option<&Value> { self.values.get(name) }
    pub fn execute(&mut self, statement: &Statement) -> Option<Value> {
        match statement {
            Statement::Let { name, value } | Statement::Set { name, value } => { self.set(name.clone(), value.clone()); None }
            Statement::Return(value) => Some(value.clone()),
            Statement::Print(value) => { println!("{:?}", value); None }
            Statement::If { condition, then_branch, else_branch } => {
                let branch = if *condition { then_branch } else { else_branch };
                for stmt in branch { if let Some(result) = self.execute(stmt) { return Some(result); } }
                None
            }
        }
    }
}

pub fn parse_value(token: &str) -> Value {
    if let Ok(n) = token.parse::<f64>() { return Value::Number(n); }
    match token { "true" => Value::Boolean(true), "false" => Value::Boolean(false), "null" => Value::Null, _ => Value::Text(token.trim_matches('"').to_string()) }
}

pub fn tokenize(source: &str) -> Vec<String> {
    source.replace("{", " { ").replace("}", " } ").replace(";", " ; ").split_whitespace().map(str::to_owned).collect()
}
