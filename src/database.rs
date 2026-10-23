#[derive(Debug, Clone)]
pub struct DatabaseConfig { pub host: String, pub port: u16, pub database: String, pub username: String }

#[derive(Debug, Clone)]
pub struct Query { pub sql: String, pub parameters: Vec<String> }

pub trait Database { fn execute(&self, query: &Query) -> Result<u64, String>; fn query(&self, query: &Query) -> Result<Vec<Vec<String>>, String>; }

/// MySQL contract layer. A concrete driver can implement this trait without coupling the language runtime to a database client.
pub struct MySqlBackend { pub config: DatabaseConfig }
impl MySqlBackend { pub fn new(config: DatabaseConfig) -> Self { Self { config } } }
impl Database for MySqlBackend {
    fn execute(&self, _query: &Query) -> Result<u64, String> { Err("MySQL driver not connected; configure a concrete client implementation".into()) }
    fn query(&self, _query: &Query) -> Result<Vec<Vec<String>>, String> { Err("MySQL driver not connected; configure a concrete client implementation".into()) }
}
