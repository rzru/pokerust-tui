use serde::{Deserialize, Serialize};

use super::NamedApiResource;

#[derive(Serialize, Deserialize, Debug)]
pub struct VerboseEffect {
    pub effect: Option<String>,
    pub short_effect: Option<String>,
    pub language: Option<NamedApiResource>,
}

impl VerboseEffect {
    pub fn get_language(&self) -> String {
        self.language
            .as_ref()
            .and_then(|language| language.name.as_ref())
            .and_then(|language| Some(language.to_string()))
            .unwrap_or(String::new())
    }

    pub fn get_effect(&self) -> String {
        self.short_effect
            .as_ref()
            .unwrap_or(&"".to_string())
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::models::NamedApiResource;

    use super::VerboseEffect;

    fn get_stub_verbose_effect() -> VerboseEffect {
        VerboseEffect {
            effect: Some(String::from("effect")),
            short_effect: Some(String::from("short effect")),
            language: Some(NamedApiResource {
                name: Some(String::from("en")),
                url: None,
            }),
        }
    }

    #[test]
    fn verbose_effect_get_effect() {
        assert_eq!(
            String::from("effect"),
            get_stub_verbose_effect().get_effect()
        )
    }

    #[test]
    fn verbose_effect_get_language() {
        assert_eq!(String::from("en"), get_stub_verbose_effect().get_language())
    }
}
