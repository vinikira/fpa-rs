#[derive(Debug, Default)]
pub struct Summary {
    pub internal_logical_file: [u32; 3],
    pub external_interface_file: [u32; 3],
    pub external_query: [u32; 3],
    pub external_output: [u32; 3],
    pub external_input: [u32; 3],
}
