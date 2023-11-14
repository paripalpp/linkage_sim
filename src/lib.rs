mod mech_solver;
use std::f64::consts::PI;

use mech_solver::triangle_solver::*;

use mech_solver::triangle_solver::variable_vector::VariableF;

use crate::mech_solver::triangle_solver::variable_vector::Variable;

pub fn run_test(){
    let mut triangle = Triangle::from_len([1.0, 1.0, 1.0]);
    triangle.c.theta = VariableF::Fixed(PI/2.0);
    triangle = triangle.solve().unwrap();
    println!("a+b+c ~= 0? : {:?}", (triangle.a + triangle.b + triangle.c).radius);
    println!("is it triangle? : {:?}", (triangle.a + triangle.b + triangle.c).radius < (1e-10).into());
}