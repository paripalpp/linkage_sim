mod mech_solver;
mod scissor_solver;
use std::f64::consts::PI;
use plotters::prelude::*;

use mech_solver::triangle_solver::*;

use mech_solver::triangle_solver::variable_vector::VariableF;

use crate::mech_solver::triangle_solver::variable_vector::{Variable, VariableFRecVec2, VariableFPolVec2};

pub fn run_scissor_test(){
    let mut backend = BitMapBackend::new("backend.png", (480, 480));
    backend.draw_rect((0, 0), (480, 480), &WHITE, true).unwrap();
    let origin_scissor = scissor_solver::Scissor::new(vec![
        scissor_solver::ScissorDimension{a: 1.0, b: 1.0, c: 0.6, d: 0.4},
        scissor_solver::ScissorDimension{a: 1.0, b: 1.0, c: 0.6, d: 0.4},
        scissor_solver::ScissorDimension{a: 1.0, b: 1.0, c: 0.6, d: 0.4},
        scissor_solver::ScissorDimension{a: 1.0, b: 1.0, c: 0.6, d: 0.4},
        scissor_solver::ScissorDimension{a: 1.0, b: 1.0, c: 0.5, d: 0.5},
        scissor_solver::ScissorDimension{a: 1.0, b: 1.0, c: 0.5, d: 0.5},
        scissor_solver::ScissorDimension{a: 1.0, b: 1.0, c: 0.5, d: 0.5},
        scissor_solver::ScissorDimension{a: 1.0, b: 1.0, c: 0.5, d: 0.5},
    ]);
    let mut scissor = origin_scissor.clone();
    scissor.solve(VariableFPolVec2::from(0.9, 0.0)).unwrap();
    scissor.draw(&mut backend, 100.0, &BLACK).unwrap();
    for i in 0..100{
        let mut scissor = origin_scissor.clone();
        scissor.solve(VariableFPolVec2::from(0.998 - 0.001*i as f64, 0.0)).unwrap();
        let (x,y) = scissor.get_endpoint().unwrap();
        backend.draw_pixel(((x*100.)as i32, backend.get_size().1 as i32 - (y*100.)as i32), BLUE.to_backend_color()).unwrap();
    }
}

pub fn run_triangle_test(){
    let mut backend = BitMapBackend::new("backend.png", (640, 480));
    backend.draw_rect((0, 0), (640, 480), &WHITE, true).unwrap();
    let original_triangle = Triangle::from_len([1.0, 1.0, 1.0]);
    for i in 0..5{
        // triangle.c.theta = VariableF::Fixed(PI/2.0);
        let mut triangle = original_triangle.clone();
        triangle.a.theta = VariableF::Fixed((i as f64) * 0.4* PI);
        triangle = triangle.solve().unwrap();
        println!("a: {:?}", triangle.a);
        println!("b: {:?}", triangle.b);
        println!("c: {:?}", triangle.c);
        println!("a+b+c ~= 0? : {:?}", (triangle.a + triangle.b + triangle.c).radius);
        println!("is it triangle? : {:?}", (triangle.a + triangle.b + triangle.c).radius < (1e-10).into());

        if let (
                VariableFRecVec2{x: VariableF::<f64>::Fixed(ax), y: VariableF::<f64>::Fixed(ay)}, 
                VariableFRecVec2{x: VariableF::<f64>::Fixed(bx), y: VariableF::<f64>::Fixed(by)}, 
                VariableFRecVec2{x: VariableF::<f64>::Fixed(cx), y: VariableF::<f64>::Fixed(cy)}
            ) = (triangle.a.to_rec(), triangle.b.to_rec(), triangle.c.to_rec()) {
                let originx = 320;
                let originy = 240;
                let ax = (ax * 200.0) as i32;
                let ay = -(ay * 200.0) as i32;
                let bx = (bx * 200.0) as i32;
                let by = -(by * 200.0) as i32;
                let cx = (cx * 200.0) as i32;
                let cy = -(cy * 200.0) as i32;
            backend.draw_line((originx, originy), (originx+ax,originy+ay), &RED).unwrap();
            backend.draw_line((originx+ax,originy+ay), (originx+ax+bx,originy+ay+by), &GREEN).unwrap();
            backend.draw_line((originx+ax+bx,originy+ay+by), (originx+ax+bx+cx,originy+ay+by+cy), &BLUE).unwrap();
        }else {
            println!("not fixed");
        }
    }
}
