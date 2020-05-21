#[derive(Clone, PartialEq, Debug, Default)]
pub struct ElementaryDataReferenced {
    pub input: u32,
    pub output: u32,
}

impl ElementaryDataReferenced {
    pub fn total(&self) -> u32 {
        self.input + self.output
    }
}
