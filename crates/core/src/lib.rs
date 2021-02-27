pub trait ToChoices {
    fn to_choices(&self) -> Vec<String>;
}

pub struct Question<'a> {
    pub r#type: &'a str,
    pub name: &'a str,
}
