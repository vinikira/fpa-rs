#[derive(Clone, PartialEq, Debug, Default)]
pub struct ElementaryDataReferenced {
    input: u32,
    output: u32,
}

impl ElementaryDataReferenced {
    pub fn new(input: u32, output: u32) -> Self {
        Self { input, output }
    }

    pub fn total(&self) -> u32 {
        self.input + self.output
    }

    /// Get a reference to the elementary data referenced's input.
    pub fn input(&self) -> &u32 {
        &self.input
    }

    /// Get a reference to the elementary data referenced's output.
    pub fn output(&self) -> &u32 {
        &self.output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create() {
        let i = 10;
        let o = 20;

        let edr = ElementaryDataReferenced {
            input: i,
            output: o,
        };

        assert_eq!(edr.input, i);
        assert_eq!(edr.output, o);
    }

    #[test]
    fn should_calculate_total() {
        let edr = ElementaryDataReferenced {
            input: 10,
            output: 20,
        };

        assert_eq!(edr.total(), 30);
    }
}
