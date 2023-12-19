use core::slice;

use plotters::{prelude::*, backend};
use crate::mech_solver;

use mech_solver::triangle_solver::{Triangle,variable_vector::{self, VariableF}};
// ScissorDimension has 4 length
// a : length of the element right up to the right
// b : length of the element right up to the left
// c : distance from a origin to cross point of a and b
// d : distance from b origin to cross point of a and b
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ScissorDimension{
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
}

#[derive(Clone)]
struct ScissorElement{
    a: variable_vector::VariableFPolVec2,
    b: variable_vector::VariableFPolVec2,
    c: variable_vector::VariableFPolVec2,
    d: variable_vector::VariableFPolVec2,
}

impl ScissorElement{
    // input is a vector from the  a origin to the b origin
    // output is a vector from the b endpoint to the a endpoint
    // you can use output as input of next element
    fn solve(&mut self, input: variable_vector::VariableFPolVec2) -> Result<variable_vector::VariableFPolVec2, std::io::Error>{
        let triangle = Triangle::new(self.c, -self.d, -input).solve()?;
        self.c = triangle.a;
        self.d = -triangle.b;
        self.a.theta = self.c.theta;
        self.b.theta = self.d.theta;
        Ok(self.a - self.b -input)
    }
}

#[no_mangle]
pub extern "C" fn create_scissor_dimension_array(size: usize) -> *const ScissorDimension {
    let vec: Vec<ScissorDimension> = vec![ScissorDimension{a:1.0, b:1.0, c:0.5, d:0.5}; size];
    let slice = vec.into_boxed_slice();
    Box::into_raw(slice) as *const ScissorDimension
}

#[no_mangle]
pub extern "C" fn get_scissor_dimension_array_element(array: *const ScissorDimension, index: usize) -> ScissorDimension {
    let slice = unsafe { std::slice::from_raw_parts(array, index + 1) };
    slice[index]
}

#[no_mangle]
pub extern "C" fn set_scissor_dimension_array_element(array: *mut ScissorDimension, index: usize, value: ScissorDimension) {
    unsafe {
        // 簡素化のためにindexが範囲内にあるか確認するなど、必要なチェックをチェックを省略しています。
        
        let array_ptr = array.add(index);
        array_ptr.write(value);
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct line {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct solve_scissor_return{
    pub error: isize,
    pub num_lines: usize,
    pub lines: *const line,
}

#[no_mangle]
pub extern "C" fn solve_from_scissor_dimension_array(array: *const ScissorDimension, len: usize, input_radius: f64, input_theta: f64) -> solve_scissor_return {
    let slice = unsafe { std::slice::from_raw_parts(array, len) };
    let mut scissor = Scissor::new(slice.to_vec());
    if let Err(_) = scissor.solve(variable_vector::VariableFPolVec2::from(input_radius, input_theta)) {
        return solve_scissor_return{
            error: 1,
            num_lines: 0,
            lines: std::ptr::null(),
        };
    }
    let mut lines = Vec::<line>::new();
    let mut next_vec_origin = (0.0, 0.0);
    let mut next_vec_input = 
    if let variable_vector::VariableFRecVec2{x: VariableF::Fixed(x),y: VariableF::Fixed(y)} = scissor.input.to_rec(){
        (x, y)
    }else{
        panic!("input must be fixed");
    };
    for i in 0..len{
        let element = &scissor.elements[i];
        if let (
            variable_vector::VariableFRecVec2{x: VariableF::<f64>::Fixed(ax), y: VariableF::<f64>::Fixed(ay)},
            variable_vector::VariableFRecVec2{x: VariableF::<f64>::Fixed(bx), y: VariableF::<f64>::Fixed(by)}
        ) = (element.a.to_rec(), element.b.to_rec()) {
            lines.push(line{
                x1: next_vec_origin.0,
                y1: next_vec_origin.1,
                x2: next_vec_origin.0 + ax,
                y2: next_vec_origin.1 + ay,
            });
            lines.push(line{
                x1: next_vec_origin.0 + next_vec_input.0,
                y1: next_vec_origin.1 + next_vec_input.1,
                x2: next_vec_origin.0 + next_vec_input.0 + bx,
                y2: next_vec_origin.1 + next_vec_input.1 + by,
            });
            next_vec_origin.0 = next_vec_origin.0 + next_vec_input.0 + bx;
            next_vec_origin.1 = next_vec_origin.1 + next_vec_input.1 + by;
            next_vec_input.0 = ax - next_vec_input.0 - bx;
            next_vec_input.1 = ay - next_vec_input.1 - by;
        }else{
            panic!("not fixed at element {}", i);
        }
    }
    let lines_slice = lines.into_boxed_slice();
    solve_scissor_return{
        error: 0,
        num_lines: lines_slice.len(),
        lines: Box::into_raw(lines_slice) as *const line,
    }
}

#[derive(Clone)]
pub struct Scissor{
    elements: Vec<ScissorElement>,
    input: variable_vector::VariableFPolVec2,
}

impl Scissor{
    pub fn new(dimensions: Vec<ScissorDimension>) -> Self{
        for dimension in dimensions.iter(){
            if dimension.a <= 0.0 || dimension.b <= 0.0 || dimension.c <= 0.0 || dimension.d <= 0.0{
                panic!("ScissorDimension must be positive");
            }
            if dimension.a <= dimension.c || dimension.b <= dimension.d{
                panic!("ScissorDimension must be a > c, b > d");
            }
        }
        let mut elements = Vec::new();
        for dimension in dimensions.iter(){
            let a = variable_vector::VariableFPolVec2::from_len(dimension.a);
            let b = variable_vector::VariableFPolVec2::from_len(dimension.b);
            let c = variable_vector::VariableFPolVec2::from_len(dimension.c);
            let d = variable_vector::VariableFPolVec2::from_len(dimension.d);
            elements.push(ScissorElement{a,b,c,d});
        }
        let input = variable_vector::VariableFPolVec2{radius: VariableF::Unknown, theta: VariableF::Unknown};
        Scissor{
            elements,
            input,
        }
    }
    pub fn solve(&mut self, input: variable_vector::VariableFPolVec2) -> Result<(), std::io::Error>{
        if let (VariableF::Unknown, VariableF::Unknown) = (input.radius, input.theta){
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "input must be fixed"));
        }
        self.input = input;
        let mut next_input = self.elements[0].solve(input)?;
        for element in self.elements.iter_mut().skip(1){
            next_input = element.solve(next_input)?;
        }
        Ok(())
    }
    pub fn draw(&self, plotter_backend : &mut BitMapBackend, scale: f64, color: &RGBColor) -> Result<(), std::io::Error>{
        let size = plotter_backend.get_size();
        let plot_origin = (0 as i32 / 2, size.1 as i32);

        let mut next_vec_origin = (0.0, 0.0);
        let mut next_vec_input = 
        if let variable_vector::VariableFRecVec2{x: VariableF::Fixed(x),y: VariableF::Fixed(y)} = self.input.to_rec(){
            (x, y)
        }else{
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "input must be fixed"));
        };
        for (index, element) in self.elements.iter().enumerate(){
            if let (
                variable_vector::VariableFRecVec2{x: VariableF::<f64>::Fixed(ax), y: VariableF::<f64>::Fixed(ay)},
                variable_vector::VariableFRecVec2{x: VariableF::<f64>::Fixed(bx), y: VariableF::<f64>::Fixed(by)}
            ) = (element.a.to_rec(), element.b.to_rec()) {
                plotter_backend.draw_line(
                    (plot_origin.0 + (next_vec_origin.0 * scale) as i32, plot_origin.1 - (next_vec_origin.1 * scale) as i32),
                    (plot_origin.0 + ((next_vec_origin.0 + ax) * scale) as i32, plot_origin.1 - ((next_vec_origin.1 + ay) * scale) as i32),
                    color
                ).unwrap();
                plotter_backend.draw_line(
                    (plot_origin.0 + ((next_vec_origin.0 + next_vec_input.0) * scale) as i32, plot_origin.1 - ((next_vec_origin.1 + next_vec_input.1) * scale) as i32),
                    (plot_origin.0 + ((next_vec_origin.0 + next_vec_input.0 + bx) * scale) as i32, plot_origin.1 - ((next_vec_origin.1 + next_vec_input.1 + by) * scale) as i32),
                    color
                ).unwrap();
                next_vec_origin.0 = next_vec_origin.0 + next_vec_input.0 + bx;
                next_vec_origin.1 = next_vec_origin.1 + next_vec_input.1 + by;
                next_vec_input.0 = ax - next_vec_input.0 - bx;
                next_vec_input.1 = ay - next_vec_input.1 - by;
            }else{
                return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("not fixed at element {}", index)));
            }
        }
        Ok(())
    }
    pub fn get_endpoint(&self) -> Result<(f64,f64), std::io::Error>{
        // check input is fixed
        let mut next_vec_origin = (0.0, 0.0);
        let mut next_vec_input = 
        if let variable_vector::VariableFRecVec2{x: VariableF::Fixed(x),y: VariableF::Fixed(y)} = self.input.to_rec(){
            (x, y)
        }else{
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "input must be fixed"));
        };
        // check all elements are fixed
        for (index, element) in self.elements.iter().enumerate(){
            if let (
                variable_vector::VariableFRecVec2{x: VariableF::<f64>::Fixed(ax), y: VariableF::<f64>::Fixed(ay)},
                variable_vector::VariableFRecVec2{x: VariableF::<f64>::Fixed(bx), y: VariableF::<f64>::Fixed(by)}
            ) = (element.a.to_rec(), element.b.to_rec()) {
                next_vec_origin.0 = next_vec_origin.0 + next_vec_input.0 + bx;
                next_vec_origin.1 = next_vec_origin.1 + next_vec_input.1 + by;
                next_vec_input.0 = ax - next_vec_input.0 - bx;
                next_vec_input.1 = ay - next_vec_input.1 - by;
            }else{
                return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("not fixed at element {}", index)));
            }
        }
        Ok(next_vec_origin)
    }
}