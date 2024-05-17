use ray_tracer::math::*;

fn main() {
    let transform = Transformations::translate(5.0, -3.0, 2.0);
    let p = point(-3.0, 4.0, 5.0);
    let res = point(2.0, 1.0, 7.0);

    println!("{}", transform);
    println!("{}", transform * p);
    println!("{}", res);

    println!("{}", veq(&(transform * p), &res));
}
