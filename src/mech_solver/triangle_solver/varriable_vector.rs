use num_traits::{Float, FromPrimitive};


pub trait Variable {
    fn get_dof(&self) -> u16;
}



//can include unknown value
#[derive(Debug, Clone, Copy)]
pub enum VariableF<T>
    where T: Float + FromPrimitive
{
    Fixed(T),
    Unknown,
}

impl<T> Variable for VariableF<T>
    where T: Float + FromPrimitive 
{
    fn get_dof(&self) -> u16 {
        match self {
            VariableF::Fixed(_)  => 0,
            VariableF::Unknown   => 1,
        }
    }
}



//polar coordinated 2D vector
#[derive(Debug, Clone, Copy)]
pub struct VariableFPolVec2{
    radius : VariableF<f64>,
    theta : VariableF<f64>,
}

impl Variable for VariableFPolVec2 {
    fn get_dof(&self) -> u16 {
        self.radius.get_dof() + self.radius.get_dof()
    }
}



//rectangular coordinated 2D vector
#[derive(Debug, Clone, Copy)]
pub struct VariableFRecVec2{
    x : VariableF<f64>,
    y : VariableF<f64>,
}

impl Variable for VariableFRecVec2 {
    fn get_dof(&self) -> u16 {
        self.x.get_dof() + self.y.get_dof()
    }
}