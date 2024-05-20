use std::f64::consts::PI;

use ray_tracer::draw::shapes::Transformable;
use ray_tracer::draw::{shapes, Scene};
use ray_tracer::{math::*, transform};

pub fn draw_clock() {
    let mut scene = Scene::new(200, 200);
    scene.reset(color(0.2, 0.2, 0.2));

    for i in 0..12 {
        let mut p = shapes::Point::new(0.0, 0.0, 0.0, color(0.5, 0.5, 0.5));

        let pl = transform!(
            Transformation::Translate(50.0, 0.0, 0.0),
            Transformation::RotateZ(i as f64 * (PI / 6.0)),
            Transformation::Translate(100.0, 100.0, 0.0)
        )
        .apply_to(&mut p);

        scene.add(Box::new(p));
    }

    scene.draw("clock.ppm");
}
