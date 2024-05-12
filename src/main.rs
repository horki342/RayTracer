use ray_tracer::{
    self,
    math::linalg::{self, iden4, MatrixMethods},
};

fn main() {
    let c = ray_tracer::draw::Canvas::blank(500, 500);
    c.to_ppm("img", "test.ppm");
}
