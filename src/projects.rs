use ray_tracer::math::utils::*;
use ray_tracer::math::*;
use ray_tracer::render::shapes::Drawable;
use ray_tracer::*;

use std::f64::consts::PI;

pub fn draw_clock() {
    let mut app = render::Renderer::new(200, 200);
    app.reset(color(0.2, 0.2, 0.2));

    for i in 0..12 {
        let p = render::shapes::Point::new(0.0, 0.0, 0.0, color(0.5, 0.5, 0.5));

        let transform = transform!(
            TUnit::Translate(50.0, 0.0, 0.0),
            TUnit::RotateZ(i as f64 * (PI / 6.0)),
            TUnit::Translate(100.0, 100.0, 0.0)
        );

        p.borrow_mut().set_transform(transform);
        app.world.add(p);
    }

    app.render();
    app.generate_ppm("clock.ppm");
}
