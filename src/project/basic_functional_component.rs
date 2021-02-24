pub use elementary_data_referenced::ElementaryDataReferenced;
pub use file_registry::FileRegistry;

mod elementary_data_referenced;
mod file_registry;

#[derive(PartialEq, Debug, Clone)]
pub struct BasicFunctionalComponent {
    name: String,
    functional_classification: FunctionalClassification,
    elementary_data_referenced: ElementaryDataReferenced,
    file_record: FileRegistry,
    complexity: Complexity,
}

#[derive(PartialOrd, PartialEq, Debug, Clone)]
pub enum Complexity {
    Simple,
    Middle,
    Complex,
}

#[derive(PartialEq, Debug, Clone)]
pub enum FunctionalClassification {
    ExternalInput,
    ExternalOutput,
    ExternalQuery,
    InternalLogicalFile,
    ExternalInterfaceFile,
}

impl BasicFunctionalComponent {
    pub fn new(name: &'static str, fc: FunctionalClassification) -> Self {
        let mut bfc = BasicFunctionalComponent {
            name: name.to_string(),
            functional_classification: fc,
            elementary_data_referenced: ElementaryDataReferenced::default(),
            file_record: FileRegistry::default(),
            complexity: Complexity::Simple,
        };

        bfc.compute_complexity();

        bfc
    }

    /// Set the Elementary Data Referenced.
    pub fn set_edr(&mut self, red: ElementaryDataReferenced) -> &mut Self {
        self.elementary_data_referenced = red;
        self.compute_complexity();

        self
    }

    /// Set the File Registry.
    pub fn set_file_registry(&mut self, file_record: FileRegistry) -> &mut Self {
        self.file_record = file_record;
        self.compute_complexity();

        self
    }

    fn compute_complexity(&mut self) {
        self.complexity = match self.functional_classification {
            FunctionalClassification::ExternalInterfaceFile
            | FunctionalClassification::InternalLogicalFile => self.logic_files_complexity(),
            FunctionalClassification::ExternalInput => self.external_input_complexity(),
            FunctionalClassification::ExternalOutput => self.external_output_complexity(),
            FunctionalClassification::ExternalQuery => self.external_query_complexity(),
        };
    }

    fn logic_files_complexity(&self) -> Complexity {
        match self.file_record.total() {
            1 => match self.elementary_data_referenced.total() {
                1..=19 | 20..=50 => Complexity::Simple,
                _ => Complexity::Middle,
            },
            2..=5 => match self.elementary_data_referenced.total() {
                1..=19 => Complexity::Simple,
                20..=50 => Complexity::Middle,
                _ => Complexity::Complex,
            },
            _ => match self.elementary_data_referenced.total() {
                1..=19 => Complexity::Middle,
                _ => Complexity::Complex,
            },
        }
    }

    fn external_input_complexity(&self) -> Complexity {
        match self.file_record.total() {
            0..=1 => match self.elementary_data_referenced.total() {
                1..=4 | 5..=15 => Complexity::Simple,
                _ => Complexity::Middle,
            },
            2 => match self.elementary_data_referenced.total() {
                1..=4 => Complexity::Simple,
                5..=15 => Complexity::Middle,
                _ => Complexity::Complex,
            },
            _ => match self.elementary_data_referenced.total() {
                1..=4 => Complexity::Middle,
                _ => Complexity::Complex,
            },
        }
    }

    fn external_query_complexity(&self) -> Complexity {
        match self.file_record.total() {
            0..=1 => match self.elementary_data_referenced.total() {
                1..=5 | 6..=19 => Complexity::Simple,
                _ => Complexity::Middle,
            },
            2..=3 => match self.elementary_data_referenced.total() {
                1..=5 => Complexity::Simple,
                6..=19 => Complexity::Middle,
                _ => Complexity::Complex,
            },
            _ => match self.elementary_data_referenced.total() {
                1..=5 => Complexity::Middle,
                _ => Complexity::Complex,
            },
        }
    }

    fn external_output_complexity(&self) -> Complexity {
        let complexity_output = match self.file_record.output {
            0..=1 => match self.elementary_data_referenced.total() {
                1..=5 | 6..=19 => Complexity::Simple,
                _ => Complexity::Middle,
            },
            2..=3 => match self.elementary_data_referenced.total() {
                1..=5 => Complexity::Simple,
                6..=19 => Complexity::Middle,
                _ => Complexity::Complex,
            },
            _ => match self.elementary_data_referenced.total() {
                1..=5 => Complexity::Middle,
                _ => Complexity::Complex,
            },
        };

        let complexity_input = match self.file_record.input {
            0..=1 => match self.elementary_data_referenced.total() {
                1..=4 | 5..=15 => Complexity::Simple,
                _ => Complexity::Middle,
            },
            2 => match self.elementary_data_referenced.total() {
                1..=4 => Complexity::Simple,
                5..=15 => Complexity::Middle,
                _ => Complexity::Complex,
            },
            _ => match self.elementary_data_referenced.total() {
                1..=4 => Complexity::Middle,
                _ => Complexity::Complex,
            },
        };

        if complexity_output > complexity_input {
            complexity_output
        } else {
            complexity_input
        }
    }

    /// Get a reference to the basic functional component's name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get a reference to the basic functional component's functional classification.
    pub fn functional_classification(&self) -> &FunctionalClassification {
        &self.functional_classification
    }

    /// Get a reference to the basic functional component's elementary data referenced.
    pub fn elementary_data_referenced(&self) -> &ElementaryDataReferenced {
        &self.elementary_data_referenced
    }

    /// Get a reference to the basic functional component's file record.
    pub fn file_record(&self) -> &FileRegistry {
        &self.file_record
    }

    /// Get a reference to the basic functional component's complexity.
    pub fn complexity(&self) -> &Complexity {
        &self.complexity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_logic_files_complexity_correctly() {
        let mut bf =
            BasicFunctionalComponent::new("Foo BFC", FunctionalClassification::InternalLogicalFile);

        let scenarios = vec![
            (
                ElementaryDataReferenced::new(2, 3),
                FileRegistry::new(1, 0),
                Complexity::Simple,
            ),
            (
                ElementaryDataReferenced::new(20, 20),
                FileRegistry::new(1, 0),
                Complexity::Simple,
            ),
            (
                ElementaryDataReferenced::new(50, 20),
                FileRegistry::new(1, 0),
                Complexity::Middle,
            ),
            (
                ElementaryDataReferenced::new(10, 5),
                FileRegistry::new(2, 2),
                Complexity::Simple,
            ),
            (
                ElementaryDataReferenced::new(30, 5),
                FileRegistry::new(2, 2),
                Complexity::Middle,
            ),
            (
                ElementaryDataReferenced::new(30, 30),
                FileRegistry::new(2, 2),
                Complexity::Complex,
            ),
            (
                ElementaryDataReferenced::new(5, 13),
                FileRegistry::new(4, 2),
                Complexity::Middle,
            ),
            (
                ElementaryDataReferenced::new(20, 13),
                FileRegistry::new(4, 2),
                Complexity::Complex,
            ),
            (
                ElementaryDataReferenced::new(30, 23),
                FileRegistry::new(4, 2),
                Complexity::Complex,
            ),
        ];

        for (edr, fr, complexity) in scenarios {
            bf.set_edr(edr).set_file_registry(fr);

            assert_eq!(bf.complexity(), &complexity);
        }
    }

    #[test]
    fn should_return_external_input_complexity_correctly() {
        let mut bf =
            BasicFunctionalComponent::new("Foo BFC", FunctionalClassification::ExternalInput);

        let scenarios = vec![
            (
                ElementaryDataReferenced::new(1, 3),
                FileRegistry::new(1, 0),
                Complexity::Simple,
            ),
            (
                ElementaryDataReferenced::new(5, 7),
                FileRegistry::new(1, 0),
                Complexity::Simple,
            ),
            (
                ElementaryDataReferenced::new(10, 20),
                FileRegistry::new(1, 0),
                Complexity::Middle,
            ),
            (
                ElementaryDataReferenced::new(1, 2),
                FileRegistry::new(1, 1),
                Complexity::Simple,
            ),
            (
                ElementaryDataReferenced::new(7, 5),
                FileRegistry::new(1, 1),
                Complexity::Middle,
            ),
            (
                ElementaryDataReferenced::new(16, 5),
                FileRegistry::new(1, 1),
                Complexity::Complex,
            ),
            (
                ElementaryDataReferenced::new(1, 3),
                FileRegistry::new(4, 1),
                Complexity::Middle,
            ),
            (
                ElementaryDataReferenced::new(5, 13),
                FileRegistry::new(4, 1),
                Complexity::Complex,
            ),
            (
                ElementaryDataReferenced::new(15, 6),
                FileRegistry::new(4, 1),
                Complexity::Complex,
            ),
        ];

        for (edr, fr, complexity) in scenarios {
            bf.set_edr(edr).set_file_registry(fr);

            assert_eq!(bf.complexity(), &complexity);
        }
    }

    #[test]
    fn should_return_external_query_complexity_correctly() {
        let mut bf =
            BasicFunctionalComponent::new("Foo BFC", FunctionalClassification::ExternalQuery);

        let scenarios = vec![
            (
                ElementaryDataReferenced::new(1, 3),
                FileRegistry::new(0, 1),
                Complexity::Simple,
            ),
            (
                ElementaryDataReferenced::new(5, 7),
                FileRegistry::new(1, 0),
                Complexity::Simple,
            ),
            (
                ElementaryDataReferenced::new(10, 20),
                FileRegistry::new(0, 1),
                Complexity::Middle,
            ),
            (
                ElementaryDataReferenced::new(1, 4),
                FileRegistry::new(1, 2),
                Complexity::Simple,
            ),
            (
                ElementaryDataReferenced::new(10, 5),
                FileRegistry::new(2, 1),
                Complexity::Middle,
            ),
            (
                ElementaryDataReferenced::new(20, 5),
                FileRegistry::new(1, 2),
                Complexity::Complex,
            ),
            (
                ElementaryDataReferenced::new(4, 1),
                FileRegistry::new(4, 0),
                Complexity::Middle,
            ),
            (
                ElementaryDataReferenced::new(10, 9),
                FileRegistry::new(0, 4),
                Complexity::Complex,
            ),
            (
                ElementaryDataReferenced::new(20, 12),
                FileRegistry::new(5, 2),
                Complexity::Complex,
            ),
        ];

        for (edr, fr, complexity) in scenarios {
            bf.set_edr(edr).set_file_registry(fr);

            assert_eq!(bf.complexity(), &complexity);
        }
    }

    #[test]
    fn should_return_external_output_complexity_correctly() {
        let mut bf =
            BasicFunctionalComponent::new("Foo BFC", FunctionalClassification::ExternalOutput);

        let scenarios = vec![
            (
                ElementaryDataReferenced::new(1, 3),
                FileRegistry::new(0, 1),
                Complexity::Simple,
            ),
            (
                ElementaryDataReferenced::new(5, 7),
                FileRegistry::new(0, 1),
                Complexity::Simple,
            ),
            (
                ElementaryDataReferenced::new(10, 20),
                FileRegistry::new(0, 1),
                Complexity::Middle,
            ),
            (
                ElementaryDataReferenced::new(1, 4),
                FileRegistry::new(0, 2),
                Complexity::Simple,
            ),
            (
                ElementaryDataReferenced::new(10, 5),
                FileRegistry::new(0, 2),
                Complexity::Middle,
            ),
            (
                ElementaryDataReferenced::new(20, 5),
                FileRegistry::new(0, 2),
                Complexity::Complex,
            ),
            (
                ElementaryDataReferenced::new(4, 1),
                FileRegistry::new(0, 4),
                Complexity::Middle,
            ),
            (
                ElementaryDataReferenced::new(10, 9),
                FileRegistry::new(0, 4),
                Complexity::Complex,
            ),
            (
                ElementaryDataReferenced::new(20, 12),
                FileRegistry::new(0, 4),
                Complexity::Complex,
            ),
        ];

        for (edr, fr, complexity) in scenarios {
            bf.set_edr(edr).set_file_registry(fr);

            assert_eq!(&complexity, bf.complexity());
        }
    }
}
