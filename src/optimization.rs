use argmin::core::{Error, CostFunction, Gradient, Hessian};

use crate::scissor_solver;
use crate::mech_solver;
use crate::bezier;

use scissor_solver::{Scissor, ScissorDimension};
use mech_solver::triangle_solver::variable_vector::VariableFPolVec2;

struct ScissorOptimization {
    initial_scissor: Vec<ScissorDimension>,
    input: Vec<f64>,
    target: bezier::Curve,
}

impl CostFunction for ScissorOptimization {
    type Param = Vec<ScissorDimension>;
    type Output = f64;
    fn cost(&self, params: &Vec<ScissorDimension>) -> Result<f64, Error> {
        let mut scissor = Scissor::new(params.clone());
        let mut distance_path = 0.0;
        for i in self.input.iter() {
            scissor.solve(VariableFPolVec2::from(i.clone(), 0.0))?;
            distance_path += self.target.distance_to_point(scissor.get_endpoint()?.into());
        }
        Ok(distance_path)
    }
}

fn optimize_scissor(){

}