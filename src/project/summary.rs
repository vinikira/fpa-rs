#[derive(Debug, Default)]
pub struct Summary {
    pub internal_logical_file: [u32; 3],
    pub external_interface_file: [u32; 3],
    pub external_query: [u32; 3],
    pub external_output: [u32; 3],
    pub external_input: [u32; 3],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create() {
        let ilf = [1, 2, 3];
        let elf = [4, 5, 6];
        let eq = [7, 8, 9];
        let eo = [10, 11, 12];
        let ei = [13, 14, 15];

        let summary = Summary {
            internal_logical_file: ilf,
            external_interface_file: elf,
            external_query: eq,
            external_output: eo,
            external_input: ei,
        };

        assert_eq!(summary.internal_logical_file, ilf);
        assert_eq!(summary.external_interface_file, elf);
        assert_eq!(summary.external_query, eq);
        assert_eq!(summary.external_output, eo);
        assert_eq!(summary.external_input, ei);
    }
}
