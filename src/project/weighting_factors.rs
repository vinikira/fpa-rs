#[derive(Debug, Default)]
pub struct WeightingFactors {
    referenced_logical_file: [u32; 3],
    external_query: [u32; 3],
    external_output: [u32; 3],
    external_input: [u32; 3],
}

impl WeightingFactors {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get a reference to the weighting factors's referenced logical file.
    pub fn referenced_logical_file(&self) -> &[u32; 3] {
        &self.referenced_logical_file
    }

    /// Get a reference to the weighting factors's external query.
    pub fn external_query(&self) -> &[u32; 3] {
        &self.external_query
    }

    /// Get a reference to the weighting factors's external output.
    pub fn external_output(&self) -> &[u32; 3] {
        &self.external_output
    }

    /// Get a reference to the weighting factors's external input.
    pub fn external_input(&self) -> &[u32; 3] {
        &self.external_input
    }

    /// Set the weighting factors's referenced logical file.
    pub fn set_referenced_logical_file(&mut self, referenced_logical_file: [u32; 3]) -> &mut Self {
        self.referenced_logical_file = referenced_logical_file;
        self
    }

    /// Set the weighting factors's external query.
    pub fn set_external_query(&mut self, external_query: [u32; 3]) -> &mut Self {
        self.external_query = external_query;
        self
    }

    /// Set the weighting factors's external output.
    pub fn set_external_output(&mut self, external_output: [u32; 3]) -> &mut Self {
        self.external_output = external_output;
        self
    }

    /// Set the weighting factors's external input.
    pub fn set_external_input(&mut self, external_input: [u32; 3]) -> &mut Self {
        self.external_input = external_input;
        self
    }
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

        let mut wf = WeightingFactors::new();

        wf.set_referenced_logical_file(rlf)
            .set_external_query(eq)
            .set_external_output(eo)
            .set_external_input(ei);

        assert_eq!(wf.referenced_logical_file(), &rlf);
        assert_eq!(wf.external_query(), &eq);
        assert_eq!(wf.external_output(), &eo);
        assert_eq!(wf.external_input(), &ei);
    }
}
