#[derive(Clone, PartialEq, Debug, Default)]
pub struct FileRegistry {
    pub input: u32,
    pub output: u32,
}

impl FileRegistry {
    pub fn total(&self) -> u32 {
        self.input + self.output
    }
}
