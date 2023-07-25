use std::{vec, option::Iter, rc::Rc, cell::RefCell, ops::Index, pin::Pin};
use cgmath::{Vector2, Matrix2, Rad, Deg, Angle, Basis2, Transform, Point2, Matrix3, Rotation};
use thiserror::Error;

pub struct Mechanism {
    joints: Vec<PinJoint>,
    linkages: Vec<Rc<RefCell<Linkage>>>,
}

pub struct MechInput {
    joint_index: usize,
    linkage_index: usize,
    linkage_joint_index: usize,
    rotation: Basis2<f64>,
}

#[derive(Error, Debug)]
pub enum SolveErr{
    #[error("No anker founded. To solve Mechanism, You should fix 1 more joint.")]
    NoAnker,
    #[error("Hint shortage. To solve Mechanism, You should add more hint about joint at {at}.")]
    SolveHintshortage{at: usize},
    #[error("Too much constraint on joint at {at}.")]
    TooMuchConstraint{at: usize},
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
        // count fixed joint and return error if there is no anker 
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
            //solve input
            for (i, input) in inputs.iter().enumerate() {
                if input_solved[i] == false {
                    match joints[input.joint_index].tranceform {
                        JointTranceform::FixedTo(joint_cord) => {
                            PinJoint::search_same_linkage(joints, &self.linkages[input.linkage_index]).iter().for_each(|(joint_index, linkage_joint_index)|{
                                let linkage = self.linkages[input.linkage_index].borrow();
                                let joint = &mut joints[*joint_index];
                                match joint.tranceform {
                                    JointTranceform::Floated => {
                                        joint.tranceform = JointTranceform::FixedTo(input.rotation.rotate_vector(linkage.joints[*linkage_joint_index] - linkage.joints[input.linkage_joint_index]).into() + joint_cord);
                                    },
                                    _ => {},
                                }
                            });
                            input_solved[i] = true;
                        },
                        _ => {},
                    }
                }
            }
            //solve linkage
            for (i, joint) in joints.iter_mut().enumerate() {
                match joint.tranceform {
                    JointTranceform::Floated => {
                        match {
                            // check joint could be solved
                            let mut fixed_joint_count = 0;
                            let mut fixed_joint_index = [0; 2];
                            let mut fixed_linkage_index = [0; 2];
                            let mut fixed_linkage_joint_index = [0; 2];
                            joint.linkages.iter().enumerate().for_each(|i, linkage| {
                                PinJoint::search_same_linkage(joints, linkage).iter().for_each(|(joint_index, linkage_joint_index)|{
                                    if let JointTranceform::FixedTo(_) = joints[*joint_index].tranceform {
                                        if let 3 = fixed_joint_count {
                                            return;
                                        }
                                        fixed_joint_index[fixed_joint_count] = *joint_index;
                                        fixed_linkage_index[fixed_joint_count] = i;
                                        fixed_linkage_joint_index[fixed_joint_count] = linkage_joint_index;
                                        fixed_joint_count += 1;
                                    }
                                });
                            });
                            (fixed_joint_count, fixed_joint_index, fixed_linkage_index, fixed_linkage_joint_index)
                        } {
                            //if 2 degree constraint is applied, solve joint
                            (2, fixed_joint_index, fixed_linkage_index, fixed_linkage_joint_index) => {
                                let fixed_linkage = [self.linkages[fixed_linkage_index[0]].borrow(), self.linkages[fixed_linkage_index[1]].borrow()];
                                let fixed_joint = [joints[fixed_joint_index[0]], joints[fixed_joint_index[1]]];
                                let fixed_linkage_joint = [fixed_linkage[0].joints[fixed_linkage_joint_index[0]], fixed_linkage[1].joints[fixed_linkage_joint_index[1]]];
                                
                            },
                            //if over 3 degree constraint is applied, return error
                            (count,_,_,_) if count >= 3 => {return Err(SolveErr::TooMuchConstraint { at: i })},
                            _ => {},
                        }
                    },
                    _ => {},
                }
            }
        }
        Ok(Self)
    }
}