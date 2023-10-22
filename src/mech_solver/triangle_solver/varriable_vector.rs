use std::ops;
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
    pub fn get_dof(&self) -> u16 {
        match self {
            VariableF::Fixed(_)  => 0,
            VariableF::Unknown   => 1,
        }
    }
}

impl<T> ops::Add for VariableF<T>
    where T: Float + FromPrimitive 
{
    pub fn add(self, other: Self) -> Self {
        match (self, other) {
            (VariableF::Fixed(a), VariableF::Fixed(b)) => VariableF::Fixed(a + b),
            _ => VariableF::Unknown,
        }
    }
}

impl<T> ops::Sub for VariableF<T>
    where T: Float + FromPrimitive 
{
    pub fn sub(self, other: Self) -> Self {
        match (self, other) {
            (VariableF::Fixed(a), VariableF::Fixed(b)) => VariableF::Fixed(a - b),
            _ => VariableF::Unknown,
        }
    }
}

impl<T> ops::Mul for VariableF<T>
    where T: Float + FromPrimitive 
{
    pub fn mul(self, other: Self) -> Self {
        match (self, other) {
            (VariableF::Fixed(a), VariableF::Fixed(b)) => VariableF::Fixed(a * b),
            _ => VariableF::Unknown,
        }
    }
}

impl<T> ops::Div for VariableF<T>
    where T: Float + FromPrimitive 
{
    pub fn div(self, other: Self) -> Self {
        match (self, other) {
            (VariableF::Fixed(a), VariableF::Fixed(b)) => VariableF::Fixed(a / b),
            _ => VariableF::Unknown,
        }
    }
}


//polar coordinated 2D vector
#[derive(Debug, Clone, Copy)]
pub struct VariableFPolVec2{
    radius : VariableF<f64>,
    theta : VariableF<f64>,
}

pub fn to_rec(pol_rec : VariableFPolVec2) -> VariableFRecVec2 {
    match (pol_rec.radius, pol_rec.theta) {
        (VariableF::Fixed(r), VariableF::Fixed(t)) => VariableFRecVec2{x: VariableF::Fixed(r * t.cos()), y: VariableF::Fixed(r * t.sin())},
        _ => VariableFRecVec2{x: VariableF::Unknown, y: VariableF::Unknown},
    }
}

impl Variable for VariableFPolVec2 {
    pub fn get_dof(&self) -> u16 {
        self.radius.get_dof() + self.theta.get_dof()
    }
}

impl ops::Add for VariableFPolVec2 {
    pub fn add(self, other: Self) -> Self {
        (self.to_rec() + other.to_rec()).to_pol()
    }
}

impl ops::Sub for VariableFPolVec2 {
    pub fn sub(self, other: Self) -> Self {
        (self.to_rec() - other.to_rec()).to_pol()
    }
}

impl<T> ops::Mul<T> for VariableFPolVec2
    where T: Float + FromPrimitive 
{
    pub fn mul(self, other: T) -> Self {
        self.radius = self.radius * VariableF::Fixed(other);
        self
    }
}

impl<T> ops::Div<T> for VariableFPolVec2
    where T: Float + FromPrimitive 
{
    pub fn div(self, other: T) -> Self {
        self.radius = self.radius / VariableF::Fixed(other);
        self
    }
}


//rectangular coordinated 2D vector
#[derive(Debug, Clone, Copy)]
pub struct VariableFRecVec2{
    x : VariableF<f64>,
    y : VariableF<f64>,
}

pub fn to_pol(rec_pol : VariableFRecVec2) -> VariableFPolVec2 {
    match (rec_pol.x, rec_pol.y) {
        (VariableF::Fixed(x), VariableF::Fixed(y)) => VariableFPolVec2{radius: VariableF::Fixed((x * x + y * y).sqrt()), theta: VariableF::Fixed(y.atan2(x))},
        _ => VariableFPolVec2{radius: VariableF::Unknown, theta: VariableF::Unknown},
    }
}

pub fn closs_product(a : VariableFRecVec2, b : VariableFRecVec2) -> VariableF<f64>{
    match (a.x, a.y, b.x, b.y) {
        (VariableF::Fixed(ax), VariableF::Fixed(ay), VariableF::Fixed(bx), VariableF::Fixed(by)) => VariableF::Fixed(ax * by - ay * bx),
        _ => VariableF::Unknown,
    }
}

pub fn dot_product(a : VariableFRecVec2, b : VariableFRecVec2) -> VariableF<f64>{
    match (a.x, a.y, b.x, b.y) {
        (VariableF::Fixed(ax), VariableF::Fixed(ay), VariableF::Fixed(bx), VariableF::Fixed(by)) => VariableF::Fixed(ax * bx + ay * by),
        _ => VariableF::Unknown,
    }
}

impl Variable for VariableFRecVec2 {
    pub fn get_dof(&self) -> u16 {
        self.x.get_dof() + self.y.get_dof()
    }
}

impl ops::Add for VariableFRecVec2 {
    pub fn add(self, other: Self) -> Self {
        VariableFRecVec2{x: self.x + other.x, y: self.y + other.y}
    }
}

impl ops::Sub for VariableFRecVec2 {
    pub fn sub(self, other: Self) -> Self {
        VariableFRecVec2{x: self.x - other.x, y: self.y - other.y}
    }
}

impl<T> ops::Mul<T> for VariableFRecVec2
    where T: Float + FromPrimitive 
{
    pub fn mul(self, other: T) -> Self {
        Self { x: self.x * VariableF::Fixed(other), y: self.y * VariableF::Fixed(other) }
    }
}

impl<T> ops::Div<T> for VariableFRecVec2
    where T: Float + FromPrimitive 
{
    pub fn div(self, other: T) -> Self {
        Self { x: self.x / VariableF::Fixed(other), y: self.y / VariableF::Fixed(other) }
    }
}