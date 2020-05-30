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
