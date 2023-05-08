mod sim{
    use std::{vec, option::Iter, rc::Rc};
    use cgmath::{Vector2, Matrix2, Rad, Deg, Angle, Basis2, Rotation, Rotation2, Point2};

    struct LinkageConnection {
        index_joint_from: usize,
        linkage_joint_to: Rc<Linkage>,
        index_joint_to: usize,
    }
    pub struct Linkage {
        joints: Vec<Vector2<f64>>,
        lines: Vec<[Vector2<f64>; 2]>,
        rotation: Basis2<f64>,
        tranceform: Vector2<f64>,
        connection: Vec<LinkageConnection>,
    }

    impl Linkage {
        pub fn new() -> Self {
            Linkage{
                joints: Vec::new(),
                lines: Vec::new(),
                rotation: Rotation2::from_angle(Rad(0.0)),
                tranceform: Vector2 { x: 0.0, y: 0.0 },
                connection: Vec::new(),
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