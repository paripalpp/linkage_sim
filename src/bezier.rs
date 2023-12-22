use cgmath::{Point2, MetricSpace};
use plotters::{backend::{BitMapBackend, DrawingBackend}, style::RGBColor};

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Curve {
    p0: Point2<f64>,
    p1: Point2<f64>,
    p2: Point2<f64>,
    p3: Point2<f64>,
}

impl Curve {
    pub fn new(p0: Point2<f64>, p1: Point2<f64>, p2: Point2<f64>, p3: Point2<f64>) -> Curve {
        Curve {
            p0: p0,
            p1: p1,
            p2: p2,
            p3: p3,
        }
    }
    fn point_at(&self, t: f64) -> Point2<f64> {
        let t2 = t * t;
        let t3 = t2 * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let mt3 = mt2 * mt;
        let x = mt3 * self.p0.x + 3.0 * mt2 * t * self.p1.x + 3.0 * mt * t2 * self.p2.x + t3 * self.p3.x;
        let y = mt3 * self.p0.y + 3.0 * mt2 * t * self.p1.y + 3.0 * mt * t2 * self.p2.y + t3 * self.p3.y;
        Point2::new(x, y)
    }
    pub fn distance_to_point(&self, p: Point2<f64>) -> f64 {
        let mut min = f64::MAX;
        let mut t = 0.0;
        let step = 0.01;
        while t < 1.0 {
            let d = self.point_at(t).distance2(p);
            if d < min {
                min = d;
            }
            t += step;
        }
        min
    }
    pub fn draw(&self, plotter_backend : &mut BitMapBackend, scale: f64, color: &RGBColor){
        let size = plotter_backend.get_size();
        let plot_origin = (0 as i32 / 2, size.1 as i32);
        for i in 0..100{
            let t1 = i as f64 / 100.0;
            let p1 = self.point_at(t1);
            let x1 = (p1.x * scale) as i32;
            let y1 = (p1.y * scale) as i32;
            let t2 = (i+1) as f64 / 100.0;
            let p2 = self.point_at(t2);
            let x2 = (p2.x * scale) as i32;
            let y2 = (p2.y * scale) as i32;
            plotter_backend.draw_line((plot_origin.0 + x1, plot_origin.1 - y1), (plot_origin.0 + x2, plot_origin.1 - y2), color).unwrap();
        }
    }
}

pub fn curve_draw_test(){
    let mut backend = BitMapBackend::new("curve_test.png", (640, 480));
    backend.draw_rect((0, 0), (640, 480), &RGBColor(255,255,255), true).unwrap();
    let curve = Curve::new(
        Point2::new(0.0, 0.0),
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 1.0),
        Point2::new(1.0, 1.0),
    );
    curve.draw(&mut backend, 300.0, &RGBColor(0,0,0));
    let curve = Curve::new(
        Point2::new(0.0, 0.0),
        Point2::new(0.0, 1.0),
        Point2::new(0.0, 1.0),
        Point2::new(1.0, 1.0),
    );
    curve.draw(&mut backend, 300.0, &RGBColor(0,0,0));
    let curve = Curve::new(
        Point2::new(0.0, 0.0),
        Point2::new(0.0, 1.0),
        Point2::new(1.0, 1.0),
        Point2::new(1.0, 0.0),
    );
    curve.draw(&mut backend, 300.0, &RGBColor(0,0,0));
}