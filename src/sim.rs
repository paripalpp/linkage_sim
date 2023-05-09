mod sim{
    use std::{vec, option::Iter, rc::Rc};
    use cgmath::{Vector2, Matrix2, Rad, Deg, Angle, Basis2, Transform, Point2, Matrix3};

    struct PinJoint {
        linkage: Vec<Rc<Linkage>>,
        joint_index: Vec<usize>,
        tranceform: Point2<f64>,
    }
    pub struct Linkage {
        joints: Vec<Vector2<f64>>,
        lines: Vec<[Vector2<f64>; 2]>,
        tranceform: Matrix3<f64>
    }

    impl Linkage {
        pub fn new() -> Self {
            Linkage{
                joints: Vec::new(),
                lines: Vec::new(),
                tranceform: Transform::<Point2<f64>>::look_at_rh(Point2::from([0.,0.]), Point2::from([0.,0.]), Vector2::from([0.,0.])),
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
        pub fn get_joints(&self) -> Vec<Vector2<f64>>{
            let mut ret = Vec::new();
            for joint in &self.joints {
                ret.push(self.rotation.rotate_vector(joint.clone()) + self.tranceform);
            }
            ret
        }
        pub fn add_connection(self, index_from: usize, linkage_to: Rc<Self>, index_to: usize) -> Self {
            linkage_to.connection.push(LinkageConnection { index_joint_from: index_to, linkage_joint_to: Rc::new(&self), index_joint_to: index_from });
            self.connection.push(LinkageConnection { index_joint_from: index_from, linkage_joint_to: linkage_to, index_joint_to: index_to });
            self
        }
    }
}