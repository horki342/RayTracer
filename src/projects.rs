use ray_tracer::math::utils::*;
use ray_tracer::math::*;
use ray_tracer::render::core::PointLight;
use ray_tracer::*;
use render::core::{Drawable, Pattern, PatternList};
use render::shapes::{Plane, Sphere};
use render::Renderer;

use std::f64::consts::PI;

pub fn draw_clock() {
    let mut app = render::Renderer::new(
        200,
        200,
        PI / 2.0,
        point(0.0, 0.0, 0.0),
        point(0.0, 0.0, -1.0),
        vector(0.0, 1.0, 0.0),
        color(0.2, 0.2, 0.2),
    );

    for i in 0..12 {
        let mut p = render::shapes::Point::new(0.0, 0.0, 0.0, color(0.5, 0.5, 0.5));

        let transform = transform!(
            TUnit::Translate(50.0, 0.0, 0.0),
            TUnit::RotateZ(i as f64 * (PI / 6.0)),
            TUnit::Translate(100.0, 100.0, 0.0)
        );

        p.set_transform(transform);
        app.world.add_point(p);
    }

    app.render();
    app.generate_ppm("clock.ppm");
}

// pub fn draw_circle() {
//     let ray_origin = point(0.0, 0.0, -5.0);
//     let wall_z = 10.0;
//     let wall_size = 7.0;
//     let cv_size = 500;
//     let px_size = wall_size / (cv_size as f64);
//     let half = wall_size / 2.0;

//     let mut cv = render::Canvas::new(cv_size, cv_size);
//     let col = color(1.0, 0.0, 0.0); // red
//     let sphere = render::shapes::Sphere::default();
//     sphere
//         .borrow_mut()
//         .set_transform(transform!(TUnit::Translate(-0.5, 0.3, 0.0)));

//     cv.reset(color(0.2, 0.2, 0.2));

//     for i in 0..cv.height {
//         let world_y = half - (i as f64) * px_size;

//         for j in 0..cv.width {
//             let world_x = -half + (j as f64) * px_size;

//             let look_at_point = point(world_x, world_y, wall_z);

//             let r = render::core::Ray::new(ray_origin, (look_at_point - ray_origin).normalize());
//             let xs = r.intersect_sphere(sphere.clone());

//             match xs.hit() {
//                 Some(_) => cv.write(j, i, col).expect("Could not draw a pixel"),
//                 None => continue,
//             }
//         }
//     }

//     cv.to_ppm("circle.ppm");
// }

// pub fn draw_sphere() {
//     let ray_origin = point(0.0, 0.0, -5.0);
//     let wall_z = 10.0;
//     let wall_size = 7.0;
//     let cv_size = 1000;
//     let px_size = wall_size / (cv_size as f64);
//     let half = wall_size / 2.0;

//     let mut cv = render::Canvas::new(cv_size, cv_size, utils::color(0.2, 0.2, 0.2));
//     let sphere = render::shapes::Sphere::default().wrap();

//     // assign color to the sphere
//     sphere
//         .borrow_mut()
//         .get_material_mut()
//         .change_color(color(1.0, 0.2, 1.0));

//     // create a light source
//     let light = PointLight::new(point(-10.0, 10.0, -10.0), color(1.0, 1.0, 1.0));

//     for i in 0..cv.height {
//         let world_y = half - (i as f64) * px_size;

//         for j in 0..cv.width {
//             let world_x = -half + (j as f64) * px_size;

//             let look_at_point = point(world_x, world_y, wall_z);

//             let r = render::core::Ray::new(ray_origin, (look_at_point - ray_origin).normalize());
//             let ts = sphere.borrow().intersect(&r);
//             let xs = Is::create_sorted(ts, sphere.clone());

//             match xs.hit() {
//                 Some(hit) => {
//                     // calculate the hit's point coordinates
//                     let point = r.pos(hit.t);
//                     let normal = hit.obj.borrow().normal(&point);
//                     let eye = -r.direction;

//                     // calculate the resultant color
//                     let color =
//                         light.shade(&hit.obj.borrow().get_material(), &point, &eye, &normal);
//                     cv.write(j, i, color)
//                         .expect("Could not write the pixel on Canvas");
//                 }
//                 None => continue,
//             }
//         }
//     }

//     cv.to_ppm("sphere.ppm");
// }

pub fn draw_spheres() {
    let mut floor = Sphere::default();
    floor.set_tunit(TUnit::Scale(10.0, 0.01, 10.0));
    floor.get_material_mut().change_color(color(1.0, 0.9, 0.9));
    floor.get_material_mut().specular = 0.0;

    let mut left_wall = Sphere::default();
    left_wall.set_transform(transform!(
        TUnit::Scale(10.0, 0.01, 10.0),
        TUnit::RotateX(PI / 2.0),
        TUnit::RotateY(-PI / 4.0),
        TUnit::Translate(0.0, 0.0, 5.0)
    ));
    left_wall.set_material(floor.get_material().clone());

    let mut right_wall = Sphere::default();
    right_wall.set_transform(transform!(
        TUnit::Scale(10.0, 0.01, 10.0),
        TUnit::RotateX(PI / 2.0),
        TUnit::RotateY(PI / 4.0),
        TUnit::Translate(0.0, 0.0, 5.0)
    ));
    right_wall.set_material(floor.get_material().clone());

    let mut middle = Sphere::default();
    middle.set_tunit(TUnit::Translate(-0.5, 1.0, 0.5));
    middle.get_material_mut().color = color(0.1, 1.0, 0.5);
    middle.get_material_mut().diffuse = 0.7;
    middle.get_material_mut().specular = 0.3;

    let mut right = Sphere::default();
    right.set_transform(transform!(
        TUnit::Scale(0.5, 0.5, 0.5),
        TUnit::Translate(1.5, 0.5, -0.5)
    ));
    right.get_material_mut().color = color(0.5, 1.0, 0.1);
    right.get_material_mut().diffuse = 0.7;
    right.get_material_mut().specular = 0.3;

    let mut left = Sphere::default();
    left.set_transform(transform!(
        TUnit::Scale(0.33, 0.33, 0.33),
        TUnit::Translate(-1.5, 0.33, -0.75)
    ));
    left.get_material_mut().color = color(1.0, 0.8, 0.1);
    left.get_material_mut().diffuse = 0.7;
    left.get_material_mut().specular = 0.3;

    let light = PointLight::new(point(-10.0, 10.0, -10.0), color(1.0, 1.0, 1.0));

    let mut app = Renderer::new(
        1000,
        500,
        PI / 3.0,
        point(0.0, 1.5, -5.0),
        point(0.0, 1.0, 0.0),
        vector(0.0, 1.0, 0.0),
        Color::black(),
    );

    let objects = vec![
        floor.wrap(),
        left_wall.wrap(),
        right_wall.wrap(),
        middle.wrap(),
        right.wrap(),
        left.wrap(),
    ];
    app.world.add_objs(objects);
    app.world.add_src(light.wrap_box());

    app.render();
    app.generate_ppm("spheres_shadows.ppm");
}


pub fn draw_spheres_and_planes() {
    let pattern = Pattern::default(PatternList::StripePattern);

    let mut floor = Plane::default();
    floor.get_material_mut().change_color(color(1.0, 0.9, 0.9));
    floor.get_material_mut().specular = 0.0;
    floor.set_pattern(pattern.clone());

    let mut middle = Sphere::default();
    middle.set_tunit(TUnit::Translate(-0.5, 1.0, 0.5));
    middle.get_material_mut().color = color(0.1, 1.0, 0.5);
    middle.get_material_mut().diffuse = 0.7;
    middle.get_material_mut().specular = 0.3;
    middle.set_pattern(pattern.clone());

    let mut right = Sphere::default();
    right.set_transform(transform!(
        TUnit::Scale(0.5, 0.5, 0.5),
        TUnit::Translate(1.5, 0.5, -0.5)
    ));
    right.get_material_mut().color = color(0.5, 1.0, 0.1);
    right.get_material_mut().diffuse = 0.7;
    right.get_material_mut().specular = 0.3;
    right.set_pattern(pattern.clone());

    let mut left = Sphere::default();
    left.set_transform(transform!(
        TUnit::Scale(0.33, 0.33, 0.33),
        TUnit::Translate(-1.5, 0.33, -0.75)
    ));
    left.get_material_mut().color = color(1.0, 0.8, 0.1);
    left.get_material_mut().diffuse = 0.7;
    left.get_material_mut().specular = 0.3;
    left.set_pattern(pattern);

    let light = PointLight::new(point(-10.0, 10.0, -10.0), color(1.0, 1.0, 1.0));

    let mut app = Renderer::new(
        500,
        250,
        PI / 3.0,
        point(0.0, 1.5, -5.0),
        point(0.0, 1.0, 0.0),
        vector(0.0, 1.0, 0.0),
        Color::black(),
    );

    let objects = vec![
        floor.wrap(),
        middle.wrap(),
        right.wrap(),
        left.wrap(),
    ];
    app.world.add_objs(objects);
    app.world.add_src(light.wrap_box());

    app.render();
    app.generate_ppm("planes_with_strapes.ppm");
}

pub fn draw_patterns() {
    let mut floor = Plane::default();
    floor.get_material_mut().change_color(color(1.0, 0.9, 0.9));
    floor.get_material_mut().specular = 0.0;
    let mut floor_pattern = Pattern::default(PatternList::StripePattern);
    floor_pattern.set_colors(color(0.83, 0.83, 0.83), color(0.9, 1.0, 1.0));
    floor.set_pattern(floor_pattern);

    let mut middle = Sphere::default();
    middle.set_tunit(TUnit::Translate(-0.5, 1.0, 0.5));
    middle.get_material_mut().color = color(0.1, 1.0, 0.5);
    middle.get_material_mut().diffuse = 0.7;
    middle.get_material_mut().specular = 0.3;
    let mut middle_pattern = Pattern::default(PatternList::GradientPattern);
    middle_pattern.set_colors(color(0.0, 0.0, 1.0), color(0.5, 0.0, 0.5));
    middle_pattern.add_tunit(TUnit::Scale(2.0, 1.0, 1.0));
    middle.set_pattern(middle_pattern);

    let mut right = Sphere::default();
    right.set_transform(transform!(
        TUnit::Scale(0.5, 0.5, 0.5),
        TUnit::Translate(1.5, 0.5, -0.5)
    ));
    right.get_material_mut().color = color(0.5, 1.0, 0.1);
    right.get_material_mut().diffuse = 0.7;
    right.get_material_mut().specular = 0.3;
    let mut right_pattern = Pattern::default(PatternList::GradientPattern);
    right_pattern.set_colors(color(1.0, 0.0, 0.0), color(1.0, 0.65, 0.0));
    right_pattern.add_tunit(TUnit::Scale(2.0, 1.0, 1.0));
    right.set_pattern(right_pattern);

    let mut left = Sphere::default();
    left.set_transform(transform!(
        TUnit::Scale(0.33, 0.33, 0.33),
        TUnit::Translate(-1.5, 0.33, -0.75)
    ));
    left.get_material_mut().color = color(1.0, 0.8, 0.1);
    left.get_material_mut().diffuse = 0.7;
    left.get_material_mut().specular = 0.3;
    let mut left_pattern = Pattern::default(PatternList::GradientPattern);
    left_pattern.set_colors(color(0.0, 0.5, 0.0), color(1.0, 1.0, 0.0));
    left_pattern.add_tunit(TUnit::Scale(2.0, 1.0, 1.0));
    left.set_pattern(left_pattern);

    let light = PointLight::new(point(-10.0, 10.0, -10.0), color(1.0, 1.0, 1.0));

    let mut app = Renderer::new(
        3000,
        1500,
        PI / 3.0,
        point(0.0, 1.5, -5.0),
        point(0.0, 1.0, 0.0),
        vector(0.0, 1.0, 0.0),
        Color::black(),
    );

    let objects = vec![
        floor.wrap(),
        middle.wrap(),
        right.wrap(),
        left.wrap(),
    ];
    app.world.add_objs(objects);
    app.world.add_src(light.wrap_box());

    app.render();
    app.generate_ppm("patterns.ppm");
}
