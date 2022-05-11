pub fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

pub trait PreparePokemonNameForDisplay {
    fn split_capitalize(self) -> Self;
}

impl PreparePokemonNameForDisplay for String {
    fn split_capitalize(self) -> Self {
        self.split("-")
            .collect::<Vec<&str>>()
            .iter()
            .map(|str| uppercase_first_letter(str))
            .collect::<Vec<String>>()
            .join(" ")
    }
}
