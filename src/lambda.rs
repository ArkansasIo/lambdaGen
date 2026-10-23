use std::collections::HashMap;

pub type NumberLambda = Box<dyn Fn(i64) -> bool + Send + Sync>;

pub struct LambdaRegistry {
    predicates: HashMap<String, NumberLambda>,
}

impl LambdaRegistry {
    pub fn new() -> Self {
        Self { predicates: HashMap::new() }
    }

    pub fn define<F>(&mut self, name: impl Into<String>, predicate: F)
    where
        F: Fn(i64) -> bool + Send + Sync + 'static,
    {
        self.predicates.insert(name.into(), Box::new(predicate));
    }

    pub fn call(&self, name: &str, value: i64) -> Option<bool> {
        self.predicates.get(name).map(|predicate| predicate(value))
    }
}

impl Default for LambdaRegistry {
    fn default() -> Self {
        Self::new()
    }
}
