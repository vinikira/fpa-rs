use super::FunctionalClassification;

#[derive(Clone)]
pub struct DraftBasicFunctionalComponent {
    pub name: String,
    pub fc: FunctionalClassification,
}

impl DraftBasicFunctionalComponent {
    pub fn new(name: String, fc: FunctionalClassification) -> Self {
        Self { name, fc }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_draftbfc() {
        let name = "Implement database".to_string();

        let new_bfc = DraftBasicFunctionalComponent::new(
            name.clone(),
            FunctionalClassification::ExternalInput,
        );

        assert_eq!(new_bfc.name, name);
        assert_eq!(new_bfc.fc, FunctionalClassification::ExternalInput);
    }
}
