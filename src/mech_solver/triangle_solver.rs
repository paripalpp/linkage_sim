pub mod variable_vector;
use std::{io::Error};

use variable_vector::*;

// triangle must be clockwise three vector  (a,b,c)
// a ↗↘ b
//   ← c
#[derive(Debug, Clone, Copy)]
pub struct Triangle{
    pub a : variable_vector::VariableFPolVec2,
    pub b : variable_vector::VariableFPolVec2,
    pub c : variable_vector::VariableFPolVec2,
}

impl variable_vector::Variable for Triangle{
    fn get_dof(&self) -> u16 {
        self.a.get_dof() + self.b.get_dof() + self.c.get_dof()
    }
}

impl Triangle{
    pub fn new(a : variable_vector::VariableFPolVec2, b : variable_vector::VariableFPolVec2, c : variable_vector::VariableFPolVec2) -> Self{
        Triangle{a,b,c}
    }
    pub fn from(dimensions : [[f64;2];3]) -> Self{
        Triangle::new(
            variable_vector::VariableFPolVec2::from(dimensions[0][0], dimensions[0][1]),
            variable_vector::VariableFPolVec2::from(dimensions[1][0], dimensions[1][1]),
            variable_vector::VariableFPolVec2::from(dimensions[2][0], dimensions[2][1]),
        )
    }
    pub fn from_len(len : [f64;3]) -> Self{
        Triangle{
            a : variable_vector::VariableFPolVec2::from_len(len[0]),
            b : variable_vector::VariableFPolVec2::from_len(len[1]),
            c : variable_vector::VariableFPolVec2::from_len(len[2]),
        }
    }
    pub fn swap_cw(mut self) -> Self{
        (self.a, self.b, self.c) = (self.c, self.a, self.b);
        self
    }
    pub fn solve(mut self) -> Result<Self, Error>{
        if self.get_dof() > 2 {return Err(Error::new(std::io::ErrorKind::Other, "solve error : dof > 2"));}
        match(self.a.get_dof(), self.b.get_dof(), self.c.get_dof()){
            (1|2, 0, 0) => self = self.solve_pattern1(),
            (0, 1|2, 0) => self = self.swap_cw().swap_cw().solve_pattern1().swap_cw(),
            (0, 0, 1|2) => self = self.swap_cw().solve_pattern1().swap_cw().swap_cw(),
            (1, 1, 0)|(0, 1, 1)|(1, 0, 1) => {
                match (self.a.radius.get_dof(), self.a.theta.get_dof(), self.b.radius.get_dof(), self.b.theta.get_dof(), self.c.radius.get_dof(), self.c.theta.get_dof()) {
                    (1,0,1,0,0,0) => self = self.solve_pattern2()?,
                    (0,0,1,0,1,0) => self = self.swap_cw().swap_cw().solve_pattern2()?.swap_cw(),
                    (1,0,0,0,1,0) => self = self.swap_cw().solve_pattern2()?.swap_cw().swap_cw(),
                    (0,1,1,0,0,0) => self = self.solve_pattern3()?,
                    (0,0,0,1,1,0) => self = self.swap_cw().swap_cw().solve_pattern3()?.swap_cw(),
                    (1,0,0,0,0,1) => self = self.swap_cw().solve_pattern3()?.swap_cw().swap_cw(),
                    (0,1,0,1,0,0) => self = self.solve_pattern4()?,
                    (0,0,0,1,0,1) => self = self.swap_cw().swap_cw().solve_pattern4()?.swap_cw(),
                    (0,1,0,0,0,1) => self = self.swap_cw().solve_pattern4()?.swap_cw().swap_cw(),
                    _ => return Err(Error::new(std::io::ErrorKind::Other, "solve error : dof pattern not found")),
                }
            },
            _ => return Err(Error::new(std::io::ErrorKind::Other, "solve error : dof pattern not found")),
        }
        // write later
        Ok(self)
    }
    // pattern1
    // one vector that has unknown length and angle exists
    // a has unknown length and angle
    fn solve_pattern1(mut self) -> Self{
        self.a = -self.b - self.c;
        self
    }
    //pattern2
    // two vectors that have unknown length exist
    // a and b has unknown length
    fn solve_pattern2(mut self) -> Result<Self, Error> {
        //normalize a and b
        let a_norm = self.a.set_radius(1.0).to_rec();
        let b_norm = self.b.set_radius(1.0).to_rec();
        let a_cross_b: VariableF<f64> = cross_product(a_norm, b_norm);
        if a_cross_b == VariableF::Fixed(0f64) {return Err(Error::new(std::io::ErrorKind::Other, "solve error : not triangle, only line"));}
        (self.a.radius, self.b.radius) = {(
            cross_product(b_norm, self.c.to_rec()) / a_cross_b,
            - cross_product(a_norm, self.c.to_rec()) / a_cross_b,
        )};
        Ok(self)
    }
    //pattern3
    // one vector that has unknown angle and one vector that has unknown length exist
    // a has unknown angle and b has unknown length
    fn solve_pattern3(self) -> Result<Self,Error> {
        // write later
        return Err(Error::new(std::io::ErrorKind::Other, "solve error : method \"solve_pattern3\" is not implemented"));
    }
    //pattern4
    // two vectors that have unknown angle exist
    // a and b has unknown angle
    fn solve_pattern4(mut self) -> Result<Self,Error> {
        if self.a.radius > self.b.radius + self.c.radius ||
            self.b.radius > self.c.radius + self.a.radius ||
            self.c.radius > self.a.radius + self.b.radius {return Err(Error::new(std::io::ErrorKind::Other, "solve error : In this condition, it CANNOT be triangle"));}
        let sol1_a_theta = -(self.c).theta + ((self.b.radius * self.b.radius - self.a.radius * self.a.radius - self.c.radius * self.c.radius) / (VariableF::from(2.0) * self.a.radius * self.c.radius)).acos();
        let sol1_b_theta = self.c.theta - ((self.a.radius * self.a.radius - self.b.radius * self.b.radius - self.c.radius * self.c.radius) / (VariableF::from(2.0) * self.b.radius * self.c.radius)).acos();
        let sol2_a_theta = -(self.c).theta - ((self.b.radius * self.b.radius - self.a.radius * self.a.radius - self.c.radius * self.c.radius) / (VariableF::from(2.0) * self.a.radius * self.c.radius)).acos();
        let sol2_b_theta = self.c.theta + ((self.a.radius * self.a.radius - self.b.radius * self.b.radius - self.c.radius * self.c.radius) / (VariableF::from(2.0) * self.b.radius * self.c.radius)).acos();
        let sol1_a = VariableFPolVec2{radius : self.a.radius, theta : sol1_a_theta};
        if cross_product(sol1_a.to_rec(), self.c.to_rec()) > VariableF::from(0.0) {
            self.a.theta = sol1_a_theta;
            self.b.theta = sol1_b_theta;
        } else {
            self.a.theta = sol2_a_theta;
            self.b.theta = sol2_b_theta;
        }
        Ok(self)
    }
}