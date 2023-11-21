use std::{ops, cmp::Ordering};
use num_traits::{Float, FromPrimitive};


pub trait Variable {
    fn get_dof(&self) -> u16;
}



//can include unknown value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl<T> ops::Add for VariableF<T>
    where T: Float + FromPrimitive 
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match (self, other) {
            (VariableF::Fixed(a), VariableF::Fixed(b)) => VariableF::Fixed(a + b),
            _ => VariableF::Unknown,
        }
    }
}

impl<T> ops::Sub for VariableF<T>
    where T: Float + FromPrimitive 
{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (VariableF::Fixed(a), VariableF::Fixed(b)) => VariableF::Fixed(a - b),
            _ => VariableF::Unknown,
        }
    }
}

impl<T> ops::Neg for VariableF<T>
    where T: Float + FromPrimitive 
{
    type Output = Self;
    fn neg(self) -> Self {
        match self {
            VariableF::Fixed(a) => VariableF::Fixed(-a),
            _ => VariableF::Unknown,
        }
    }
}

impl<T> ops::Mul for VariableF<T>
    where T: Float + FromPrimitive 
{
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (VariableF::Fixed(a), VariableF::Fixed(b)) => VariableF::Fixed(a * b),
            _ => VariableF::Unknown,
        }
    }
}

impl<T> ops::Div for VariableF<T>
    where T: Float + FromPrimitive 
{
    type Output = Self;
    fn div(self, other: Self) -> Self {
        match (self, other) {
            (VariableF::Fixed(a), VariableF::Fixed(b)) => VariableF::Fixed(a / b),
            _ => VariableF::Unknown,
        }
    }
}

impl<T> PartialOrd for VariableF<T>
    where T: Float + FromPrimitive 
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (VariableF::Fixed(a), VariableF::Fixed(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

impl<T> From<T> for VariableF<T>
    where T: Float + FromPrimitive 
{
    fn from(t: T) -> Self {
        VariableF::Fixed(t)
    }
}
impl<T> VariableF<T>
    where T: Float + FromPrimitive
{
    
    pub fn cos(&self) -> Self {
        match self {
            VariableF::Fixed(a) => VariableF::Fixed(a.cos()),
            VariableF::Unknown => VariableF::Unknown,
        }
    }
    pub fn sin(&self) -> Self {
        match self {
            VariableF::Fixed(a) => VariableF::Fixed(a.sin()),
            VariableF::Unknown => VariableF::Unknown,
        }
    }
    pub fn acos(&self) -> Self {
        match self {
            VariableF::Fixed(a) => VariableF::Fixed(a.acos()),
            VariableF::Unknown => VariableF::Unknown,
        }
    }
    pub fn asin(&self) -> Self {
        match self {
            VariableF::Fixed(a) => VariableF::Fixed(a.asin()),
            VariableF::Unknown => VariableF::Unknown,
        }
    }
}


//polar coordinated 2D vector
#[derive(Debug, Clone, Copy)]
pub struct VariableFPolVec2{
    pub radius : VariableF<f64>,
    pub theta : VariableF<f64>,
}

impl VariableFPolVec2 {
    pub fn to_rec(&self) -> VariableFRecVec2 {
        match (self.radius, self.theta) {
            (VariableF::Fixed(r), VariableF::Fixed(t)) => VariableFRecVec2{x: VariableF::Fixed(r * t.cos()), y: VariableF::Fixed(r * t.sin())},
            _ => VariableFRecVec2{x: VariableF::Unknown, y: VariableF::Unknown},
        }
    }
    pub fn set_radius(mut self, radius : f64) -> Self{
        self.radius = VariableF::Fixed(radius);
        self
    }
    pub fn set_theta(mut self, theta : f64) -> Self{
        self.theta = VariableF::Fixed(theta);
        self
    }
    pub fn from(r : f64, t : f64) -> Self{
        VariableFPolVec2{radius: VariableF::Fixed(r), theta: VariableF::Fixed(t)}
    }
    pub fn from_len(r : f64) -> Self{
        VariableFPolVec2{radius: VariableF::Fixed(r), theta: VariableF::Unknown}
    }
}

impl Variable for VariableFPolVec2 {
    fn get_dof(&self) -> u16 {
        self.radius.get_dof() + self.theta.get_dof()
    }
}

impl ops::Add for VariableFPolVec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        (self.to_rec() + other.to_rec()).to_pol()
    }
}

impl ops::Sub for VariableFPolVec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        (self.to_rec() - other.to_rec()).to_pol()
    }
}

impl ops::Neg for VariableFPolVec2
{
    type Output = Self;
    fn neg(self) -> Self {
        Self { radius: self.radius, theta: self.theta + VariableF::Fixed(std::f64::consts::PI) }
    }
}

impl ops::Mul<f64> for VariableFPolVec2
{
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        VariableFPolVec2{radius: self.radius * VariableF::Fixed(other), theta: self.theta}
    }
}

impl ops::Div<f64> for VariableFPolVec2
{
    type Output = Self;
    fn div(self, other: f64) -> Self {
        VariableFPolVec2{radius: self.radius / VariableF::Fixed(other), theta: self.theta}
    }
}


//rectangular coordinated 2D vector
#[derive(Debug, Clone, Copy)]
pub struct VariableFRecVec2{
    pub x : VariableF<f64>,
    pub y : VariableF<f64>,
}

impl VariableFRecVec2 {
    pub fn to_pol(&self) -> VariableFPolVec2 {
        match (self.x, self.y) {
            (VariableF::Fixed(x), VariableF::Fixed(y)) => VariableFPolVec2{radius: VariableF::Fixed((x * x + y * y).sqrt()), theta: VariableF::Fixed(y.atan2(x))},
            _ => VariableFPolVec2{radius: VariableF::Unknown, theta: VariableF::Unknown},
        }
    }
    pub fn from(x : f64, y : f64) -> Self{
        VariableFRecVec2{x: VariableF::Fixed(x), y: VariableF::Fixed(y)}
    }
}

pub fn cross_product(a : VariableFRecVec2, b : VariableFRecVec2) -> VariableF<f64>{
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
    fn get_dof(&self) -> u16 {
        self.x.get_dof() + self.y.get_dof()
    }
}

impl ops::Add for VariableFRecVec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        VariableFRecVec2{x: self.x + other.x, y: self.y + other.y}
    }
}

impl ops::Sub for VariableFRecVec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        VariableFRecVec2{x: self.x - other.x, y: self.y - other.y}
    }
}

impl ops::Neg for VariableFRecVec2
{
    type Output = Self;
    fn neg(self) -> Self {
        Self { x: -self.x, y: -self.y }
    }
}

impl ops::Mul<f64> for VariableFRecVec2
{
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self { x: self.x * VariableF::Fixed(other), y: self.y * VariableF::Fixed(other) }
    }
}

impl ops::Div<f64> for VariableFRecVec2
{
    type Output = Self;
    fn div(self, other: f64) -> Self {
        Self { x: self.x / VariableF::Fixed(other), y: self.y / VariableF::Fixed(other) }
    }
}