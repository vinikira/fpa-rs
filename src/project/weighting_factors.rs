#[derive(Debug, Default)]
pub struct WeightingFactors {
    pub referenced_logical_file: [u32; 3],
    pub external_query: [u32; 3],
    pub external_output: [u32; 3],
    pub external_input: [u32; 3],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create() {
        let rlf = [4, 5, 6];
        let eq = [7, 8, 9];
        let eo = [10, 11, 12];
        let ei = [13, 14, 15];

        let wf = WeightingFactors {
            referenced_logical_file: rlf,
            external_query: eq,
            external_output: eo,
            external_input: ei,
        };

        assert_eq!(wf.referenced_logical_file, rlf);
        assert_eq!(wf.external_query, eq);
        assert_eq!(wf.external_output, eo);
        assert_eq!(wf.external_input, ei);
    }
}
