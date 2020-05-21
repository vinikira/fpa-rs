#[derive(Debug, Default)]
pub struct WeightingFactors {
    pub referenced_logical_file: [u32; 3],
    pub external_query: [u32; 3],
    pub external_output: [u32; 3],
    pub external_input: [u32; 3],
}
