#[derive(Default, Debug)]
pub struct AdjustmentFactors {
    teleprocessing: u8,
    distributed_processing: u8,
    performance: u8,
    machine_load: u8,
    transaction_volume: u8,
    online_data_input: u8,
    online_updates: u8,
    user_end_eficiency: u8,
    processing_complexity: u8,
    code_reuse: u8,
    implementation_facility: u8,
    operation_facility: u8,
    maintenance_facility: u8,
    operation_in_multiple_locations: u8,
}

impl AdjustmentFactors {
    pub fn new(factors_vec: [u8; 14]) -> Self {
        let mut af = AdjustmentFactors::default();

        af.teleprocessing = factors_vec[0];
        af.distributed_processing = factors_vec[1];
        af.performance = factors_vec[2];
        af.machine_load = factors_vec[3];
        af.transaction_volume = factors_vec[4];
        af.online_data_input = factors_vec[5];
        af.online_updates = factors_vec[6];
        af.user_end_eficiency = factors_vec[7];
        af.processing_complexity = factors_vec[8];
        af.code_reuse = factors_vec[9];
        af.implementation_facility = factors_vec[10];
        af.operation_facility = factors_vec[11];
        af.maintenance_facility = factors_vec[12];
        af.operation_in_multiple_locations = factors_vec[13];

        af
    }

    pub fn sum(&self) -> u32 {
        (self.teleprocessing
            + self.distributed_processing
            + self.performance
            + self.machine_load
            + self.transaction_volume
            + self.online_data_input
            + self.online_updates
            + self.user_end_eficiency
            + self.processing_complexity
            + self.code_reuse
            + self.implementation_facility
            + self.operation_facility
            + self.maintenance_facility
            + self.operation_in_multiple_locations) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_sum() {
        let af = AdjustmentFactors::new([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);

        assert_eq!(af.sum(), 105);
    }
}
