use adjustment_factors::AdjustmentFactors;
use basic_functional_component::{BasicFunctionalComponent, Complexity, FunctionalClassification};
use summary::Summary;
use weighting_factors::WeightingFactors;

pub mod adjustment_factors;
pub mod basic_functional_component;
pub mod summary;
pub mod weighting_factors;

#[derive(Debug, Default)]
pub struct Project {
    basic_functional_components: Vec<BasicFunctionalComponent>,
    summary: Summary,
    adjustment_factors: AdjustmentFactors,
    weighting_factors: WeightingFactors,
    total_function_point_not_adjusted: u32,
    total_influence_factor: u32,
    final_adjustment_factor: f32,
    final_adjusted_function_points: f32,
    cost_per_hour: f32,
    total_cost: f32,
}

impl Project {
    pub fn new(bfcs: Vec<BasicFunctionalComponent>) -> Self {
        let mut project = Project::default();

	for bfc in bfcs.into_iter() {
	    project.add_bfc(bfc);
	}

        project
    }

    pub fn add_bfc(&mut self, bfc: BasicFunctionalComponent) -> &mut Self {
        self.increment_summary_table(&bfc);
        self.basic_functional_components.push(bfc);
        self.compute_tfna();

        self
    }

    pub fn set_weighting_factors(&mut self, wf: WeightingFactors) {
        self.weighting_factors = wf;
        self.compute_fafp();
    }

    pub fn set_adjustment_factors(&mut self, af: AdjustmentFactors) {
        self.adjustment_factors = af;
        self.total_influence_factor = self.adjustment_factors.sum();
        self.final_adjustment_factor = 0.65f32 + (0.01f32 * self.total_influence_factor as f32);
        self.compute_fafp();
    }

    pub fn set_cost_per_hour(&mut self, cph: f32) {
        self.cost_per_hour = cph;
        self.compute_cost();
    }

    fn compute_fafp(&mut self) {
        self.final_adjusted_function_points =
            self.total_function_point_not_adjusted as f32 * self.final_adjustment_factor;

        self.compute_cost();
    }

    fn compute_cost(&mut self) {
        self.total_cost = self.final_adjusted_function_points * self.cost_per_hour;
    }

    fn increment_summary_table(&mut self, bfc: &BasicFunctionalComponent) {
        let level = match bfc.complexity {
            Complexity::Simple => 0,
            Complexity::Middle => 1,
            Complexity::Complex => 2,
        };

        match bfc.functional_classification {
            FunctionalClassification::ExternalInput => self.summary.external_input[level] += 1,
            FunctionalClassification::ExternalOutput => self.summary.external_output[level] += 1,
            FunctionalClassification::ExternalInterfaceFile => {
                self.summary.external_interface_file[level] += 1
            }
            FunctionalClassification::InternalLogicalFile => {
                self.summary.internal_logical_file[level] += 1
            }
            FunctionalClassification::ExternalQuery => self.summary.external_query[level] += 1,
        }

        self.compute_fafp();
    }

    fn compute_tfna(&mut self) {
        let ilf_result: u32 = Self::compute_weight(
            &self.summary.internal_logical_file,
            self.weighting_factors.referenced_logical_file(),
        );

        let eif_result: u32 = Self::compute_weight(
            &self.summary.external_interface_file,
            self.weighting_factors.referenced_logical_file(),
        );

        let ei_result: u32 = Self::compute_weight(
            &self.summary.external_input,
            self.weighting_factors.external_input(),
        );

        let eo_result: u32 = Self::compute_weight(
            &self.summary.external_output,
            self.weighting_factors.external_output(),
        );

        let eq_result: u32 = Self::compute_weight(
            &self.summary.external_query,
            self.weighting_factors.external_query(),
        );

        self.total_function_point_not_adjusted =
            ilf_result + eif_result + ei_result + eo_result + eq_result;

        self.compute_fafp();
    }

    /// Compute weight and return.
    fn compute_weight(summary: &[u32], weight: &[u32]) -> u32 {
        summary
            .iter()
            .zip(weight.iter())
            .map(|(summary_weight, weighting_factor)| summary_weight * weighting_factor)
            .sum()
    }

    /// Get a reference to the project's total function point not adjusted.
    pub fn total_function_point_not_adjusted(&self) -> u32 {
        self.total_function_point_not_adjusted
    }

    /// Get a reference to the project's total influence factor.
    pub fn total_influence_factor(&self) -> u32 {
        self.total_influence_factor
    }

    /// Get a reference to the project's final adjustment factor.
    pub fn final_adjustment_factor(&self) -> f32 {
        self.final_adjustment_factor
    }

    /// Get a reference to the project's final adjusted function points.
    pub fn final_adjusted_function_points(&self) -> f32 {
        self.final_adjusted_function_points
    }

    /// Get a reference to the project's total cost.
    pub fn total_cost(&self) -> f32 {
        self.total_cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project::basic_functional_component::ElementaryDataReferenced;
    use crate::project::basic_functional_component::FileRegistry;

    #[test]
    pub fn new_project() {
        let bfc = external_input_bfc();
        let bfc2 = external_output_bfc();

        Project::new(vec![bfc, bfc2]);
    }

    #[test]
    fn project_calc() {
        let mut proj = Project::default();

        proj.set_cost_per_hour(100f32);

        let wf = WeightingFactors {
            referenced_logical_file: [7, 10, 15],
            external_query: [4, 5, 7],
            external_input: [3, 4, 6],
            external_output: [4, 5, 7],
        };

        proj.set_weighting_factors(wf);

        let af = AdjustmentFactors::new([5, 0, 3, 1, 1, 1, 1, 5, 1, 2, 5, 1, 1, 5]);

        proj.set_adjustment_factors(af);

        let mut bfc1 = BasicFunctionalComponent::new(
            "Base de dados de produtos",
            FunctionalClassification::InternalLogicalFile,
        );

        bfc1.set_red(ElementaryDataReferenced::new(4, 0));

        bfc1.set_file_registry(FileRegistry::new(1, 0));

        let mut bfc2 = BasicFunctionalComponent::new(
            "Função de Criação de Registros",
            FunctionalClassification::ExternalInput,
        );

        bfc2.set_red(ElementaryDataReferenced::new(4, 0));

        bfc2.set_file_registry(FileRegistry::new(1, 0));

        let mut bfc3 = BasicFunctionalComponent::new(
            "Função de Consulta de Registros",
            FunctionalClassification::ExternalQuery,
        );

        bfc3.set_red(ElementaryDataReferenced::new(4, 0));

        bfc3.set_file_registry(FileRegistry::new(1, 0));

        let mut bfc4 = BasicFunctionalComponent::new(
            "Função de Atualização de dados",
            FunctionalClassification::ExternalInput,
        );

        bfc4.set_red(ElementaryDataReferenced::new(3, 0));

        bfc4.set_file_registry(FileRegistry::new(1, 0));

        let mut bfc5 = BasicFunctionalComponent::new(
            "Função de Extração de dados",
            FunctionalClassification::ExternalOutput,
        );

        bfc5.set_red(ElementaryDataReferenced::new(4, 6));

        bfc5.set_file_registry(FileRegistry::new(1, 1));

        let mut bfc6 = BasicFunctionalComponent::new(
            "Arquivo de dados extraídos",
            FunctionalClassification::ExternalInterfaceFile,
        );

        bfc6.set_red(ElementaryDataReferenced::new(0, 6));

        bfc6.set_file_registry(FileRegistry::new(1, 0));

        proj.add_bfc(bfc1)
            .add_bfc(bfc2)
            .add_bfc(bfc3)
            .add_bfc(bfc4)
            .add_bfc(bfc5)
            .add_bfc(bfc6);

        assert_eq!(proj.total_function_point_not_adjusted(), 28);
        assert_eq!(proj.total_influence_factor(), 32);
        assert_eq!(format!("{:.2}", proj.final_adjustment_factor()), "0.97");
        assert_eq!(
            format!("{:.2}", proj.final_adjusted_function_points()),
            "27.16"
        );
        assert_eq!(proj.total_cost(), 2716f32);
    }

    fn external_input_bfc() -> BasicFunctionalComponent {
        BasicFunctionalComponent::new(
            "Create tables",
            FunctionalClassification::ExternalInput,
        )
    }

    fn external_output_bfc() -> BasicFunctionalComponent {
        BasicFunctionalComponent::new(
            "Lorem ipsum",
            FunctionalClassification::ExternalOutput,
        )
    }
}
