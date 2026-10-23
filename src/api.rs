use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiRoute {
    pub method: String,
    pub path: String,
    pub description: String,
}

#[derive(Debug, Clone, Default)]
pub struct ResearchApi {
    routes: BTreeMap<String, ApiRoute>,
}

impl ResearchApi {
    pub fn standard() -> Self {
        let mut api = Self::default();
        api.route("GET", "/api/research/tree", "Return the research hierarchy");
        api.route("POST", "/api/research/experiments", "Create an experiment specification");
        api.route("GET", "/api/research/experiments/:id", "Return experiment metadata and result");
        api.route("POST", "/api/jobs", "Submit a research job");
        api.route("GET", "/api/jobs/:id", "Return job status");
        api.route("GET", "/api/tables/:name", "Return a named research table");
        api
    }

    pub fn route(&mut self, method: &str, path: &str, description: &str) {
        let key = format!("{} {}", method.to_uppercase(), path);
        self.routes.insert(key, ApiRoute {
            method: method.to_uppercase(),
            path: path.into(),
            description: description.into(),
        });
    }

    pub fn get(&self, method: &str, path: &str) -> Option<&ApiRoute> {
        self.routes.get(&format!("{} {}", method.to_uppercase(), path))
    }

    pub fn routes(&self) -> impl Iterator<Item = &ApiRoute> {
        self.routes.values()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_api_contains_research_routes() {
        let api = ResearchApi::standard();
        assert!(api.get("GET", "/api/research/tree").is_some());
        assert!(api.get("POST", "/api/research/experiments").is_some());
        assert_eq!(api.routes().count(), 6);
    }
}
