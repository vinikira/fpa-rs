pub use elementary_data_referenced::ElementaryDataReferenced;
pub use file_registry::FileRegistry;

mod elementary_data_referenced;
mod file_registry;

#[derive(PartialEq, Debug, Clone)]
pub struct BasicFunctionalComponent {
    pub name: String,
    pub functional_classification: FunctionalClassification,
    pub elementary_data_referenced: ElementaryDataReferenced,
    pub file_record: FileRegistry,
    pub complexity: Complexity,
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

    pub fn set_red(&mut self, red: ElementaryDataReferenced) {
        self.elementary_data_referenced = red;
        self.compute_complexity();
    }

    pub fn set_file_registry(&mut self, file_record: FileRegistry) {
        self.file_record = file_record;
        self.compute_complexity();
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
            0..=1 => match self.elementary_data_referenced.output {
                1..=5 | 6..=19 => Complexity::Simple,
                _ => Complexity::Middle,
            },
            2..=3 => match self.elementary_data_referenced.output {
                1..=5 => Complexity::Simple,
                6..=19 => Complexity::Middle,
                _ => Complexity::Complex,
            },
            _ => match self.elementary_data_referenced.output {
                1..=5 => Complexity::Middle,
                _ => Complexity::Complex,
            },
        };

        let complexity_input = match self.file_record.input {
            0..=1 => match self.elementary_data_referenced.input {
                1..=4 | 5..=15 => Complexity::Simple,
                _ => Complexity::Middle,
            },
            2 => match self.elementary_data_referenced.input {
                1..=4 => Complexity::Simple,
                5..=15 => Complexity::Middle,
                _ => Complexity::Complex,
            },
            _ => match self.elementary_data_referenced.input {
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
}
