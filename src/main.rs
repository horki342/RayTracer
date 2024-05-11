use ray_tracer;

fn main() {
    let c = ray_tracer::draw::Canvas::new(500, 500);
    c.to_ppm("img", "test.ppm");
}
