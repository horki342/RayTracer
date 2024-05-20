use ray_tracer::draw::shapes::{self, Sphere};
use ray_tracer::math::{color, point, tuple, vector};

mod projs;

fn main() {
    let s = Sphere::default();
    println!("{:#?}", s);
}
