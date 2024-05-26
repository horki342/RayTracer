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

pub fn draw_circle() {
    let ray_origin = point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let cv_size = 500;
    let px_size = wall_size / (cv_size as f64);
    let half = wall_size / 2.0;

    let mut cv = render::Canvas::new(cv_size, cv_size);
    let col = color(1.0, 0.0, 0.0); // red
    let sphere = render::shapes::Sphere::default();
    sphere
        .borrow_mut()
        .set_transform(transform!(TUnit::Translate(-0.5, 0.3, 0.0)));

    cv.reset(color(0.2, 0.2, 0.2));

    for i in 0..cv.height {
        let world_y = half - (i as f64) * px_size;

        for j in 0..cv.width {
            let world_x = -half + (j as f64) * px_size;

            let look_at_point = point(world_x, world_y, wall_z);

            let r = render::core::Ray::new(ray_origin, (look_at_point - ray_origin).normalize());
            let xs = r.intersect_sphere(sphere.clone());

            match xs.hit() {
                Some(_) => cv.write(j, i, col).expect("Could not draw a pixel"),
                None => continue,
            }
        }
    }

    cv.to_ppm("circle.ppm");
}
