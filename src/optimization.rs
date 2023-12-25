use argmin::core::{Error, CostFunction, Gradient, Hessian};
use cgmath::MetricSpace;

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
        const GAIN_PATH_DIFF: f64 = 3.0;
        const GAIN_LENGTH_DIFF: f64 = 1.0;
        let mut scissor = Scissor::new(params.clone());
        let mut path_diff = 0.0;
        let mut nearest_points = Vec::new();
        for i in self.input.iter() {
            scissor.solve(VariableFPolVec2::from(i.clone(), 0.0))?;
            let (distance, nearest) = self.target.distance_to_point(scissor.get_endpoint()?.into());
            path_diff += distance;
            nearest_points.push(nearest);
        }
        let mut length_diff = self.target.len();
        for i in 0..(nearest_points.len() - 1) {
            length_diff -= nearest_points[i].distance2(nearest_points[i + 1]);
        }
        Ok(path_diff / self.input.len() as f64 * GAIN_PATH_DIFF + length_diff * GAIN_LENGTH_DIFF)
    }
}

impl Gradient for ScissorOptimization {
    type Param = Vec<ScissorDimension>;
    type Gradient = Vec<f64>;
    fn gradient(&self, params: &Vec<ScissorDimension>) -> Result<Vec<f64>, Error> {
        const GAIN_PATH_DIFF: f64 = 1.0;
        const GAIN_LENGTH_DIFF: f64 = 1.0;
        let mut scissor = Scissor::new(params.clone());
        let mut path_diff = 0.0;
        let mut nearest_points = Vec::new();
        for i in self.input.iter() {
            scissor.solve(VariableFPolVec2::from(i.clone(), 0.0))?;
            let (distance, nearest) = self.target.distance_to_point(scissor.get_endpoint()?.into());
            path_diff += distance;
            nearest_points.push(nearest);
        }
        let mut length_diff = self.target.len();
        for i in 0..(nearest_points.len() - 1) {
            length_diff -= nearest_points[i].distance2(nearest_points[i + 1]);
        }
        let mut grad = Vec::new();
        for i in 0..params.len() {
            let mut params = params.clone();
            params[i] += 0.0001;
            let mut scissor = Scissor::new(params);
            let mut path_diff = 0.0;
            let mut nearest_points = Vec::new();
            for i in self.input.iter() {
                scissor.solve(VariableFPolVec2::from(i.clone(), 0.0))?;
                let (distance, nearest) = self.target.distance_to_point(scissor.get_endpoint()?.into());
                path_diff += distance;
                nearest_points.push(nearest);
            }
            let mut length_diff = self.target.len();
            for i in 0..(nearest_points.len() - 1) {
                length_diff -= nearest_points[i].distance2(nearest_points[i + 1]);
            }
            grad.push((path_diff / self.input.len() as f64 * GAIN_PATH_DIFF + length_diff * GAIN_LENGTH_DIFF - self.cost(&params)?)/0.0001);
        }
        Ok(grad)
    }
}

fn optimize_scissor(){

}