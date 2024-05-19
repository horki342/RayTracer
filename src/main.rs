use ray_tracer::draw::shapes;
use ray_tracer::math::{color, point, tuple, vector};

mod projs;

fn main() {
    let col = color(0.5, 0.5, 0.5);

    let r = shapes::Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0), col);
    let s = shapes::Sphere::new(0.0, 0.0, 0.0, 1.0, col);
    let xs = r.intersect_sphere(std::rc::Rc::new(s));

    println!("{:?}", xs);

    // TODO
}
