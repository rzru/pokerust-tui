use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NamedApiResource {
    pub name: Option<String>,
    pub url: Option<String>,
}

impl NamedApiResource {
    pub fn get_name_or_stub(&self) -> String {
        self.name.as_ref().unwrap_or(&"".to_string()).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::NamedApiResource;

    fn get_stub_named_api_resource(name: Option<String>) -> NamedApiResource {
        NamedApiResource { name, url: None }
    }

    #[test]
    fn named_api_resource_get_name_or_stub_with_name() {
        let named_api_resource = get_stub_named_api_resource(Some(String::from("test")));

        assert_eq!(named_api_resource.get_name_or_stub(), String::from("test"))
    }

    #[test]
    fn named_api_resource_get_name_or_stub_without_name() {
        let named_api_resource = get_stub_named_api_resource(None);

        assert_eq!(named_api_resource.get_name_or_stub(), String::from(""))
    }
}
