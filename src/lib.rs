mod mech_solver;
mod scissor_solver;
use std::f64::consts::PI;
use num_traits::Float;
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

pub fn run_crosslink_test(){
    let mut backend = BitMapBackend::new("backend.png", (640, 480));
    backend.draw_rect((0, 0), (640, 480), &WHITE, true).unwrap();
    let dx_zero = 600.0;
    let dy_zero = 20.0;
    let dx_scale = 300.0;
    let dy_scale = -300.0;
    let e1 : f64 = 1.0;
    let e2 : f64 = 1.414 * 0.9;
    let e3 : f64 = 1.414 * 0.75;
    let e4 : f64 = 1.0 * 1.1;
    let i1 : f64 = 0.6;
    let i2 : f64 = 1.5;
    let step_num : isize = 20;
    let step : f64 = (i2 - i1) / (step_num as f64);
    let mut tr1 = Triangle::from_len([e1, e2, i1]);
    let mut tr2 = Triangle::from_len([e3, e4, i1]);
    tr1.a.theta = VariableF::Fixed(-PI/2.0);
    for i in 0..2{
        if(i == 0){
            tr1.c.radius = VariableF::Fixed(i1);
            tr2.c.radius = VariableF::Fixed(i1);
        }else{
            tr1.c.radius = VariableF::Fixed(i2);
            tr2.c.radius = VariableF::Fixed(i2);
        }
        let mut tr1 = tr1.clone();
        let mut tr2 = tr2.clone();
        tr1 = tr1.solve().unwrap();
        tr2.c.theta = tr1.c.theta;
        tr2 = tr2.solve().unwrap();
        //draw e1~4 black
        if let (
            VariableFRecVec2{x: VariableF::<f64>::Fixed(e1x), y: VariableF::<f64>::Fixed(e1y)}, 
            VariableFRecVec2{x: VariableF::<f64>::Fixed(e2x), y: VariableF::<f64>::Fixed(e2y)}, 
            VariableFRecVec2{x: VariableF::<f64>::Fixed(e3x), y: VariableF::<f64>::Fixed(e3y)},
            VariableFRecVec2{x: VariableF::<f64>::Fixed(e4x), y: VariableF::<f64>::Fixed(e4y)},
            VariableFRecVec2{x: VariableF::<f64>::Fixed(i1x), y: VariableF::<f64>::Fixed(i1y)},
        ) = (tr1.a.to_rec(), tr1.b.to_rec(), tr2.a.to_rec(), tr2.b.to_rec(), tr1.c.to_rec()) {
            backend.draw_line(
                ((dx_zero) as i32, (dy_zero) as i32), 
                ((e1x * dx_scale + dx_zero) as i32, (e1y * dy_scale + dy_zero) as i32), 
                &BLACK
            ).unwrap();
            backend.draw_line(
                ((e1x * dx_scale + dx_zero) as i32, (e1y * dy_scale + dy_zero) as i32), 
                ((e1x * dx_scale + dx_zero + e2x * dx_scale) as i32, (e1y * dy_scale + dy_zero + e2y * dy_scale) as i32), 
                &BLACK
            ).unwrap();
            backend.draw_line(
                ((dx_zero) as i32, (dy_zero) as i32), 
                ((e3x * dx_scale + dx_zero) as i32, (e3y * dy_scale + dy_zero) as i32), 
                &BLACK
            ).unwrap();
            backend.draw_line(
                ((e3x * dx_scale + dx_zero) as i32, (e3y * dy_scale + dy_zero) as i32), 
                ((e3x * dx_scale + dx_zero + e4x * dx_scale) as i32, (e3y * dy_scale + dy_zero + e4y * dy_scale) as i32), 
                &BLACK
            ).unwrap();
            //draw i1 BLUE
            backend.draw_line(
                ((e1x * dx_scale + dx_zero + e2x * dx_scale) as i32, (e1y * dy_scale + dy_zero + e2y * dy_scale) as i32), 
                ((i1x * dx_scale + e1x * dx_scale + e2x * dx_scale + dx_zero) as i32, (i1y * dy_scale + e1y * dy_scale + e2y * dy_scale + dy_zero) as i32), 
                &BLUE
            ).unwrap();
        }else {
            println!("not fixed");
        }

    }
    //draw path in red
    for i in 0..step_num {
        let mut tr1 = tr1.clone();
        let mut tr2 = tr2.clone();
        tr1.a.theta = VariableF::Fixed(-PI/2.0);
        tr1.c.radius = VariableF::Fixed(i1 + step * (i as f64));
        tr2.c.radius = VariableF::Fixed(i1 + step * (i as f64));
        tr1 = tr1.solve().unwrap();
        tr2.c.theta = tr1.c.theta;
        tr2 = tr2.solve().unwrap();
        if let (
            VariableFRecVec2{x: VariableF::<f64>::Fixed(e1x), y: VariableF::<f64>::Fixed(e1y)},
            VariableFRecVec2{x: VariableF::<f64>::Fixed(e2x), y: VariableF::<f64>::Fixed(e2y)},
            VariableFRecVec2{x: VariableF::<f64>::Fixed(e3x), y: VariableF::<f64>::Fixed(e3y)},
        ) = (tr1.a.to_rec(), tr1.b.to_rec(), tr2.a.to_rec()) {
            backend.draw_line(
                ((e1x * dx_scale + dx_zero + e2x * dx_scale) as i32, (e1y * dy_scale + dy_zero + e2y * dy_scale) as i32), 
                ((e3x * dx_scale + dx_zero) as i32, (e3y * dy_scale + dy_zero) as i32), 
                &RED
            ).unwrap();
        }else {
            println!("not fixed");
        }
    }
}
