use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub enum DataValue { Null, Boolean(bool), Number(f64), Text(String), Json(String) }

#[derive(Debug, Clone)]
pub struct InputField { pub name: String, pub data_type: String, pub required: bool }

#[derive(Debug, Clone)]
pub struct InputRecord { pub fields: BTreeMap<String, DataValue> }

#[derive(Debug, Clone)]
pub struct OutputRecord { pub status: u16, pub fields: BTreeMap<String, DataValue> }

impl InputRecord { pub fn new() -> Self { Self { fields: BTreeMap::new() } } pub fn insert(&mut self, name: impl Into<String>, value: DataValue) { self.fields.insert(name.into(), value); } }
impl OutputRecord { pub fn ok() -> Self { Self { status: 200, fields: BTreeMap::new() } } pub fn insert(&mut self, name: impl Into<String>, value: DataValue) { self.fields.insert(name.into(), value); } }

pub trait InputAdapter { fn read(&self, input: &str) -> Result<InputRecord, String>; }
pub trait OutputAdapter { fn write(&self, output: &OutputRecord) -> Result<String, String>; }

pub struct TextInput;
impl InputAdapter for TextInput { fn read(&self, input: &str) -> Result<InputRecord, String> { let mut record=InputRecord::new(); record.insert("input", DataValue::Text(input.to_owned())); Ok(record) } }

pub struct JsonOutput;
impl OutputAdapter for JsonOutput { fn write(&self, output: &OutputRecord) -> Result<String, String> { let fields=output.fields.iter().map(|(k,v)| format!("\"{}\":\"{:?}\"", k, v)).collect::<Vec<_>>().join(","); Ok(format!("{{\"status\":{},\"data\":{{{}}}}}", output.status, fields)) } }
