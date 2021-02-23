#[derive(Clone, PartialEq, Debug, Default)]
pub struct FileRegistry {
    pub input: u32,
    pub output: u32,
}

impl FileRegistry {
    pub fn new(input: u32, output: u32) -> Self { Self { input, output } }

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

        let fr = FileRegistry {
            input: i,
            output: o,
        };

        assert_eq!(fr.input, i);
        assert_eq!(fr.output, o);
    }

    #[test]
    fn should_calculate_total() {
        let fr = FileRegistry {
            input: 10,
            output: 20,
        };

        assert_eq!(fr.total(), 30);
    }
}
