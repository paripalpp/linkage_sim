use std::{vec, option::Iter, rc::Rc, cell::RefCell, ops::Index, pin::Pin};
use cgmath::{Vector2, Matrix2, Rad, Deg, Angle, Basis2, Transform, Point2, Matrix3};
use thiserror::Error;

pub struct Mechanism {
    joints: Vec<PinJoint>,
    linkages: Vec<Rc<RefCell<Linkage>>>,
}

pub struct MechInput {
    joint_index: usize,
    linkage_index: usize,
    rotation: Basis2<f64>,
}

#[derive(Error, Debug)]
pub enum SolveErr{
    #[error("No anker founded. To solve Mechanism, You should fix 1 more joint.")]
    NoAnker,
    #[error("Hint shortage. To solve Mechanism, You should add {at} more hint.")]
    SolveHintshortage{at: usize},
}

#[derive(Clone)]
struct PinJoint {
    linkages: Vec<Rc<RefCell<Linkage>>>,
    joint_indexs: Vec<usize>,
    tranceform: JointTranceform,
}

#[derive(Clone)]
enum JointTranceform {
    FixedTo(Point2<f64>),
    TwoSolution([Point2<f64>; 2]),
    Floated,
}

/// it will solve to point that A x B is active.
/// invert is select negative one.
/// (A is vector [joint_index_from] to [joint_index_to\[0\]] and B is to [joint_index_to\[1\]])
pub struct SolveHint {
    joint_index_from: usize,
    joint_index_to: [usize; 2],
    invert: bool,
}

pub struct Linkage {
    joints: Vec<Vector2<f64>>,
    lines: Vec<[Vector2<f64>; 2]>,
}

impl Linkage {
    pub fn new() -> Self {
        Linkage{
            joints: Vec::new(),
            lines: Vec::new(),
        }
    }
    pub fn from_points(points: &[Vector2<f64>]) -> Self {
        let mut ret = Self::new();
        for (i, &point) in points.iter().enumerate() {
            ret.joints.push(point);
            if i!=0 {ret.lines.push([points[i-1], point]);}
        }
        ret
    }
    pub fn get_vector(&self, index_from: usize, index_to: usize) -> Vector2<f64> {
        self.joints[index_to] - self.joints[index_from]
    }
    pub fn get_vector_from_origin(&self, index_to: usize) -> Vector2<f64> {
        self.joints[index_to]
    }
}

impl PinJoint{
    pub fn new() -> Self {
        PinJoint {
            linkages: Vec::new(),
            joint_indexs: Vec::new(),
            tranceform: JointTranceform::Floated,
        }
    }
    fn check_index_len(linkage: &Linkage, index: usize) -> Result<(),()> {
        if linkage.joints.len() > index {
            Result::Ok(())
        } else {
            Result::Err(())
        }
    }
    pub fn from_linkage<const NUM: usize>(linkages: [Rc<RefCell<Linkage>>; NUM], indexs: [usize; NUM]) -> Self {
        for i in 0..NUM {
            Self::check_index_len(&*linkages[i].borrow(), indexs[i]).expect("index is bigger than Vector lengthof linkage!");
        }
        PinJoint {
            linkages: Vec::from(linkages),
            joint_indexs: Vec::from(indexs),
            tranceform: JointTranceform::Floated,
        }
    }
    pub fn fix(mut self, point: Point2<f64>) -> Self {
        self.tranceform = JointTranceform::FixedTo(point);
        self
    }
    pub fn add_connection(mut self, linkage: Rc<RefCell<Linkage>>, index: usize) -> Self {
        Self::check_index_len(&*linkage.borrow(), index).expect("index is bigger than Vector lengthof linkage!");
        self.linkages.push(linkage);
        self.joint_indexs.push(index);
        self
    }
    //search joint that have connection to same linkage
    //return joint index vector
    pub fn search_same_linkage(joints: Vec<Self>, linkage: &Rc<RefCell<Linkage>>) -> Vec<(usize, usize)> {
        let mut ret = Vec::new();
        for (i, joint) in joints.iter().enumerate() {
            for (j, joint_linkage) in joint.linkages.iter().enumerate() {
                if Rc::ptr_eq(&linkage, joint_linkage) {
                    ret.push((i, joint.joint_indexs[j]));
                    break;
                }
            }
        }
        ret
    }
}

impl Mechanism {
    pub fn new() -> Self {
        Mechanism {
            joints: Vec::new(),
            linkages: Vec::new(),
        }
    }
    pub fn from_linkages<const NUM: usize>(linkages: [Linkage; NUM],) -> Self {
        Mechanism {
            joints: Vec::new(),
            linkages: Vec::from(linkages.map(|linkage|Rc::new(RefCell::new(linkage)))),
        }
    }
    pub fn set_angle_input(mut self, linkage_index: usize, angle: f64) -> Self{
        self
    }
    pub fn solve(&self, inputs: Vec<MechInput>) -> Result<Self,SolveErr> {
        if {
            let mut fixed_joint = 0;
            for joint in &self.joints {
                match joint.tranceform {
                    JointTranceform::FixedTo(_) => {fixed_joint += 1},
                    _ => {},
                }
            }
            fixed_joint
        } == 0 {return Err(SolveErr::NoAnker)};
        let mut joints = self.joints.clone();
        let mut input_solved = vec![false; inputs.len()];
        let mut no_solution_count = 0;
        while {
            let mut solved_joint = 0;
            for joint in joints {
                match joint.tranceform {
                    JointTranceform::Floated => {solved_joint += 1},
                    _ => {},
                }
            }
            solved_joint
        } != joints.len() {
            for (i, input) in inputs.iter().enumerate() {
                match joints[input.joint_index].tranceform {
                    JointTranceform::FixedTo(joint_cord) => {
                        input_solved[i] = true;
                        PinJoint::search_same_linkage(joints, &self.linkages[input.linkage_index]).iter().for_each(|(joint_index, linkage_index)|{
                            let joint = &mut joints[*joint_index];
                            match joint.tranceform {
                                JointTranceform::Floated => {
                                    joint.tranceform = JointTranceform::FixedTo();
                                },
                                _ => {},
                            }
                        });
                    },
                    _ => {},
                }
            }
            for joint in &mut joints {

            }
        }
        Ok(Self)
    }
}