use std::f64::consts::PI;

use super::math::utils::*;
use super::math::{Color, Matrix, TUnit, Transformation};

use super::render::Canvas;
use crate::render::core::{Computations, Drawable, Is, Material, PointLight, Ray, II};
use crate::render::core::{I, II as _};
use crate::render::shapes::{Plane, Point, Sphere};

use crate::render::{Camera, World};
use crate::{fassert, massert, transform, vassert};

#[test]
fn tuple_operations() {
    // Adding two tuples
    let a1 = tuple(3.0, -2.0, 5.0, 1.0);
    let a2 = tuple(-2.0, 3.0, 1.0, 0.0);
    assert_eq!(a1 + a2, tuple(1.0, 1.0, 6.0, 1.0));

    // Subtracting points and vectors from each other
    let p1 = point(3.0, 2.0, 1.0);
    let p2 = point(5.0, 6.0, 7.0);
    let p = point(3.0, 2.0, 1.0);
    let v = vector(5.0, 6.0, 7.0);
    assert_eq!(p1 - p2, vector(-2.0, -4.0, -6.0));
    assert_eq!(p - v, point(-2.0, -4.0, -6.0));

    // Zero-vector operation
    let zero = vector(0.0, 0.0, 0.0);
    let v = vector(1.0, -2.0, 3.0);
    assert_eq!(zero - v, vector(-1.0, 2.0, -3.0));

    // Negating tuples
    let a = tuple(1.0, -2.0, 3.0, -4.0);
    assert_eq!(-a, tuple(-1.0, 2.0, -3.0, 4.0));

    // Multiplying a tuple with a number
    let a = tuple(1.0, -2.0, 3.0, -4.0);
    assert_eq!(a * 3.5, tuple(3.5, -7.0, 10.5, -14.0));
    assert_eq!(3.5 * a, tuple(3.5, -7.0, 10.5, -14.0));

    // Dividing a tuple with a number
    let a = tuple(1.0, -2.0, 3.0, -4.0);
    assert_eq!(a / 2.0, tuple(0.5, -1.0, 1.5, -2.0));

    // Magnitude of vectors
    let v = vector(1.0, 0.0, 0.0);
    assert_eq!(v.magnitude(), 1.0);

    let v = vector(-1.0, -2.0, -3.0);
    assert_eq!(v.magnitude(), 14.0_f64.sqrt());

    // Dot product check <3
    let a = vector(1.0, 2.0, 3.0);
    let b = vector(2.0, 3.0, 4.0);
    assert_eq!(dot(&a, &b), 20.0);

    // Cross product check <3
    let a = vector(1.0, 2.0, 3.0);
    let b = vector(2.0, 3.0, 4.0);
    assert_eq!(cross(&a, &b), vector(-1.0, 2.0, -1.0));
}

#[test]
fn color_operations() {
    // Color creation
    let c = color(-0.5, 0.4, 1.7);
    assert_eq!(c.r, -0.5);
    assert_eq!(c.g, 0.4);
    assert_eq!(c.b, 1.7);

    // Adding Colors
    let c1 = color(0.9, 0.6, 0.75);
    let c2 = color(0.7, 0.1, 0.25);
    assert_eq!(c1 + c2, color(1.6, 0.7, 1.0));

    // Subtracting Colors
    let c1 = color(0.9, 0.6, 0.75);
    let c2 = color(0.7, 0.1, 0.25);
    assert_eq!(c1 - c2, color(0.2, 0.5, 0.5));

    // Multiplying a Color with a number
    let c = color(0.2, 0.3, 0.4);
    assert_eq!(c * 2.0, color(0.4, 0.6, 0.8));
    assert_eq!(2.0 * c, color(0.4, 0.6, 0.8));

    // Multiplying Colors with a Schur product
    let c1 = color(1.0, 0.2, 0.4);
    let c2 = color(0.9, 1.0, 0.1);
    assert_eq!(c1 * c2, color(0.9, 0.2, 0.04));
}

#[test]
fn canvas_operations() {
    // Creating a Canvas
    let c = Canvas::new(10, 20, Color::black());
    assert_eq!(c.width, 10);
    assert_eq!(c.height, 20);
    for i in 0..c.height {
        for j in 0..c.width {
            assert_eq!(c[[i, j]], Color::default());
        }
    }

    // Writing pixels to the Canvas
    let mut c = Canvas::new(10, 20, Color::black());
    let red = color(1.0, 0.0, 0.0);
    let _ = c.write(2, 3, red);
    assert_eq!(c[[2, 3]], color(1.0, 0.0, 0.0));
}

#[test]
fn matrix_operations() {
    // Matrix equality
    let a = matrix(
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
    );
    let b = matrix(
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
    );
    massert!(a, b);

    // Multiplying matrices
    let a = matrix(
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
    );
    let b = matrix(
        -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0,
    );
    let ab = matrix(
        20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0, 26.0,
        46.0, 42.0,
    );
    massert!(ab, a * b);

    // Multiplying by a tuple
    let a = matrix(
        1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
    );
    let b = tuple(1.0, 2.0, 3.0, 1.0);
    vassert!(a * b, tuple(18.0, 24.0, 33.0, 1.0));

    // Identity matrix
    let a = matrix(
        1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
    );
    let i = Matrix::identity();
    massert!(a * i, a);

    // Transposing matrices
    let a = matrix(
        0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0,
    );
    let res = matrix(
        0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0,
    );
    massert!(a.transpose(), res);

    // Inverse
    let a = matrix(
        8.0, -5.0, 9.0, 2.0, 7.0, 5.0, 6.0, 1.0, -6.0, 0.0, 9.0, 6.0, -3.0, 0.0, -9.0, -4.0,
    );
    let res = matrix(
        -0.15385, -0.15385, -0.28205, -0.53846, -0.07692, 0.12308, 0.02564, 0.03077, 0.35897,
        0.35897, 0.43590, 0.92308, -0.69231, -0.69231, -0.76923, -1.92308,
    );
    massert!(a.try_inverse().unwrap(), res);

    let a = matrix(
        9.0, 3.0, 0.0, 9.0, -5.0, -2.0, -6.0, -3.0, -4.0, 9.0, 6.0, 4.0, -7.0, 6.0, 6.0, 2.0,
    );
    let res = matrix(
        -0.04074, -0.07778, 0.14444, -0.22222, -0.07778, 0.03333, 0.36667, -0.33333, -0.02901,
        -0.14630, -0.10926, 0.12963, 0.17778, 0.06667, -0.26667, 0.33333,
    );
    massert!(a.try_inverse().unwrap(), res);
}

#[test]
fn transformations() {
    // translation
    let transform = TUnit::Translate(5.0, -3.0, 2.0).matrix();
    let p = point(-3.0, 4.0, 5.0);
    vassert!(transform * p, point(2.0, 1.0, 7.0));

    let transform = TUnit::Translate(5.0, -3.0, 2.0).matrix();
    let inv = transform.try_inverse().unwrap();
    let p = point(-3.0, 4.0, 5.0);
    vassert!(inv * p, point(-8.0, 7.0, 3.0));

    let transform = TUnit::Translate(5.0, -3.0, 2.0).matrix();
    let v = vector(-3.0, 4.0, 5.0);
    vassert!(transform * v, v);

    // scaling
    let scale = TUnit::Scale(2.0, 3.0, 4.0).matrix();
    let p = point(-4.0, 6.0, 8.0);
    vassert!(scale * p, point(-8.0, 18.0, 32.0));

    let scale = TUnit::Scale(2.0, 3.0, 4.0).matrix();
    let p = vector(-4.0, 6.0, 8.0);
    vassert!(scale * p, vector(-8.0, 18.0, 32.0));

    let transform = TUnit::Scale(2.0, 3.0, 4.0).matrix();
    let inv = transform.try_inverse().unwrap();
    let v = vector(-4.0, 6.0, 8.0);
    let res = inv * v;
    vassert!(res, vector(-2.0, 2.0, 2.0));

    let transform = TUnit::Scale(-1.0, 1.0, 1.0).matrix();
    let p = point(2.0, 3.0, 4.0);
    let res = transform * p;
    vassert!(res, point(-2.0, 3.0, 4.0));

    // rotating
    let p = point(0.0, 1.0, 0.0);
    let half_quarter = TUnit::RotateX(PI / 4.0).matrix();
    let full_quarter = TUnit::RotateX(PI / 2.0).matrix();
    let res1 = half_quarter * p;
    let res2 = full_quarter * p;
    vassert!(res1, point(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0));
    vassert!(res2, &point(0.0, 0.0, 1.0));

    let inv = half_quarter.try_inverse().unwrap();
    vassert!(
        inv * p,
        point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0)
    );

    let p = point(0.0, 0.0, 1.0);
    let half_quarter = TUnit::RotateY(PI / 4.0).matrix();
    let full_quarter = TUnit::RotateY(PI / 2.0).matrix();
    vassert!(
        half_quarter * p,
        point(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0)
    );
    vassert!(full_quarter * p, point(1.0, 0.0, 0.0));

    let p = point(0.0, 1.0, 0.0);
    let half_quarter = TUnit::RotateZ(PI / 4.0).matrix();
    let full_quarter = TUnit::RotateZ(PI / 2.0).matrix();
    vassert!(
        half_quarter * p,
        point(-2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0)
    );
    vassert!(full_quarter * p, point(-1.0, 0.0, 0.0));

    // shearing
    let transform = TUnit::Shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0).matrix();
    let p = point(2.0, 3.0, 4.0);
    vassert!(transform * p, point(5.0, 3.0, 4.0));

    let transform = TUnit::Shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0).matrix();
    let p = point(2.0, 3.0, 4.0);
    vassert!(transform * p, point(6.0, 3.0, 4.0));

    let transform = TUnit::Shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0).matrix();
    let p = point(2.0, 3.0, 4.0);
    vassert!(transform * p, point(2.0, 5.0, 4.0));

    let transform = TUnit::Shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0).matrix();
    let p = point(2.0, 3.0, 4.0);
    vassert!(transform * p, point(2.0, 7.0, 4.0));

    let transform = TUnit::Shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0).matrix();
    let p = point(2.0, 3.0, 4.0);
    vassert!(transform * p, point(2.0, 3.0, 6.0));

    let transform = TUnit::Shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0).matrix();
    let p = point(2.0, 3.0, 4.0);
    vassert!(transform * p, point(2.0, 3.0, 7.0));

    // chained transformation
    let transform = transform!(
        TUnit::RotateX(PI / 2.0),
        TUnit::Scale(5.0, 5.0, 5.0),
        TUnit::Translate(10.0, 5.0, 7.0)
    );
    let p = point(1.0, 0.0, 1.0);
    vassert!(&transform * p, point(15.0, 0.0, 7.0));
}

#[test]
fn ray_operations_and_intersections() {
    // Computing a point from a distance
    let r = Ray::new(point(2.0, 3.0, 4.0), vector(1.0, 0.0, 0.0));
    vassert!(r.pos(0.0), point(2.0, 3.0, 4.0));
    vassert!(r.pos(1.0), point(3.0, 3.0, 4.0));
    vassert!(r.pos(-1.0), point(1.0, 3.0, 4.0));
    vassert!(r.pos(2.5), point(4.5, 3.0, 4.0));

    // Ray intersecting a sphere at two points
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = Sphere::default();
    let s = s.wrap();
    let xs = Is::create(s.borrow().intersect(&r), s.clone());

    assert_eq!(xs.len(), 2);
    assert!(xs.contains(4.0));
    assert!(xs.contains(6.0));

    // A ray intersects a sphere at a tangent
    let r = Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
    let xs = Is::create(s.borrow().intersect(&r), s.clone());

    assert_eq!(xs.len(), 2);
    assert!(xs.contains(5.0));

    // A ray misses a sphere
    let r = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
    let xs = Is::create(s.borrow().intersect(&r), s.clone());

    assert_eq!(xs.len(), 0);

    // A ray originates inside a sphere
    let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
    let xs = Is::create(s.borrow().intersect(&r), s.clone());

    assert_eq!(xs.len(), 2);
    assert!(xs.contains(-1.0));
    assert!(xs.contains(1.0));

    // A sphere is behind a ray
    let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
    let s = Sphere::default();
    let s = s.wrap();
    let xs = Is::create(s.borrow().intersect(&r), s.clone());

    assert_eq!(xs.len(), 2);
    assert!(xs.contains(-6.0));
    assert!(xs.contains(-4.0));

    // The hit when all intersections have positive t
    let i1 = I::new(1.0, s.clone());
    let i2 = I::new(2.0, s.clone());
    let xs = Is::combine(&[i1.clone(), i2]);
    let i = xs.hit().unwrap();
    assert_eq!(i, &i1);

    // The hit, when interactions have negative t
    let i1 = I::new(-1.0, s.clone());
    let i2 = I::new(1.0, s.clone());
    let xs = Is::combine(&[i1, i2.clone()]);
    let i = xs.hit().unwrap();
    assert_eq!(i, &i2);

    // The hit, when intersections have negative t
    let i1 = I::new(-2.0, s.clone());
    let i2 = I::new(-1.0, s.clone());
    let xs = Is::combine(&[i1, i2]);
    let i = xs.hit();
    assert_eq!(i, None);

    // The hit is always the lowest nonnegative intersection
    let i1 = I::new(5.0, s.clone());
    let i2 = I::new(7.0, s.clone());
    let i3 = I::new(-3.0, s.clone());
    let i4 = I::new(2.0, s.clone());
    let xs = Is::combine(&[i1, i2, i3, i4.clone()]);
    let i = xs.hit().unwrap();
    assert_eq!(i, &i4);

    // Translating a ray
    let r = Ray::new(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
    let m = TUnit::Translate(3.0, 4.0, 5.0);
    let r2 = &m * r;
    vassert!(r2.origin, point(4.0, 6.0, 8.0));
    vassert!(r2.direction, vector(0.0, 1.0, 0.0));

    // Scaling a ray
    let r = Ray::new(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
    let m = TUnit::Scale(2.0, 3.0, 4.0);
    let r2 = &m * r;
    vassert!(r2.origin, point(2.0, 6.0, 12.0));
    vassert!(r2.direction, vector(0.0, 3.0, 0.0));

    // Sphere's default transformation
    let s = Sphere::default();
    massert!(s.get_transform().matrix(), Matrix::identity());

    // Changing a sphere's transformation
    let mut s = Sphere::default();
    let t = transform!(TUnit::Translate(2.0, 3.0, 4.0));
    s.set_transform(t.clone());
    massert!(s.get_transform().matrix(), t.matrix());

    // Intersecting a scaled sphere with a ray
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let mut s = Sphere::default();
    s.set_tunit(TUnit::Scale(2.0, 2.0, 2.0));
    let xs = Is::create(s.intersect(&r), s.wrap());
    assert_eq!(xs.len(), 2_usize);
    assert!(xs.contains(3.0));
    assert!(xs.contains(7.0));

    // Intersecting a translated sphere with a ray
    let mut s = Sphere::default();
    s.set_tunit(TUnit::Translate(5.0, 0.0, 0.0));
    let xs = Is::create(s.intersect(&r), s.wrap());
    assert_eq!(xs.len(), 0_usize);
}

#[test]
fn compute_surface_normals_and_reflection_vectors() {
    let s = Sphere::default();

    // The normal on a sphere at a point on the x axis
    let n = s.normal(&point(1.0, 0.0, 0.0));
    vassert!(n, vector(1.0, 0.0, 0.0));

    // The normal on a sphere at a point on the y axis
    let n = s.normal(&point(0.0, 1.0, 0.0));
    vassert!(n, vector(0.0, 1.0, 0.0));

    // The normal on a sphere at a point on the z axis
    let n = s.normal(&point(0.0, 0.0, 1.0));
    vassert!(n, vector(0.0, 0.0, 1.0));

    // The normal on a sphere at a nonaxial point AND the normal is a normalized vector
    let t = 3_f64.sqrt() / 3.0;
    let n = s.normal(&point(t, t, t));
    vassert!(n, vector(t, t, t));
    vassert!(n, n.normalize());

    // Computing the normal on a translated sphere
    let mut s = Sphere::default();
    s.set_tunit(TUnit::Translate(0.0, 1.0, 0.0));
    let n = s.normal(&point(0.0, 1.70711, -0.70711));
    vassert!(n, vector(0.0, 0.70711, -0.70711));

    // Computing the normal on a transformed sphere
    let mut s = Sphere::default();
    s.set_transform(transform!(
        TUnit::RotateZ(PI / 5.0),
        TUnit::Scale(1.0, 0.5, 1.0)
    ));
    let t = 2_f64.sqrt() / 2.0;
    let n = s.normal(&point(0.0, t, -t));
    vassert!(n, vector(0.0, 0.97014, -0.24254));

    // Reflecting a vector approaching at 45 degrees
    let v = vector(1.0, -1.0, 0.0);
    let n = vector(0.0, 1.0, 0.0);
    let r = reflect(&v, &n);
    vassert!(r, vector(1.0, 1.0, 0.0));

    // Reflecting a vector off a slanted surface
    let v = vector(0.0, -1.0, 0.0);
    let n = vector(t, t, 0.0);
    let r = reflect(&v, &n);
    vassert!(r, vector(1.0, 0.0, 0.0));
}

#[test]
fn check_phong_reflection_model_for_sphere() {
    // The default material
    let m = Material::default();
    fassert!(m.ambient, 0.1);
    fassert!(m.diffuse, 0.9);
    fassert!(m.specular, 0.9);
    assert_eq!(color(1.0, 1.0, 1.0), m.color);

    // A sphere has a default material
    let s = Sphere::default();
    assert_eq!(*s.get_material(), Material::default());

    // A sphere may be assigned a material
    let mut s = Sphere::default();
    let mut m = Material::default();
    m.ambient = 1.0;
    s.set_material(m.clone());
    assert_eq!(*s.get_material(), m);

    // Background for Phong Reflection Model
    let m = Material::default();
    let pos = point(0.0, 0.0, 0.0);

    // Lighting with the eye between the light and the surface
    let eyev = vector(0.0, 0.0, -1.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = PointLight::new(point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0));
    let result = light.shade(&m, &pos, &eyev, &normalv, false);
    assert_eq!(result, color(1.9, 1.9, 1.9));

    // Lighting with the eye between light and surface, eye offset 45 degrees
    let eyev = vector(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = PointLight::new(point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0));
    let result = light.shade(&m, &pos, &eyev, &normalv, false);
    assert_eq!(result, color(1.0, 1.0, 1.0));

    // Lighting with eye opposite surface, light offset 45 degrees
    let eyev = vector(0.0, 0.0, -1.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = PointLight::new(point(0.0, 10.0, -10.0), color(1.0, 1.0, 1.0));
    let result = light.shade(&m, &pos, &eyev, &normalv, false);
    assert_eq!(result, color(0.7364, 0.7364, 0.7364));

    // Lighting with eye in the path of the reflection vector
    let eyev = vector(0.0, -2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = PointLight::new(point(0.0, 10.0, -10.0), color(1.0, 1.0, 1.0));
    let result = light.shade(&m, &pos, &eyev, &normalv, false);
    assert_eq!(result, color(1.6364, 1.6364, 1.6364));

    // Lighting with the light behind the surface
    let eyev = vector(0.0, 0.0, -1.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = PointLight::new(point(0.0, 0.0, 10.0), color(1.0, 1.0, 1.0));
    let result = light.shade(&m, &pos, &eyev, &normalv, false);
    assert_eq!(result, color(0.1, 0.1, 0.1));
}

// #[test]
fn _scene_making_check_world_and_renderer_and_camera() {
    // Intersect a world with a ray
    let w = World::default();
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let xs = w.intersect(&r);
    assert_eq!(xs.len(), 4);
    assert!(xs.contains(4.0));
    assert!(xs.contains(4.5));
    assert!(xs.contains(5.5));
    assert!(xs.contains(6.0));

    // Precomputing the state of the intersection
    // The hit, when an intersection occurs on the outside
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let shape = Sphere::default();
    let i = I::new(4.0, shape.wrap());
    let comps = Computations::new(i.clone(), &r);
    assert_eq!(comps.t, i.t);
    vassert!(comps.p, point(0.0, 0.0, -1.0));
    vassert!(comps.e, vector(0.0, 0.0, -1.0));
    vassert!(comps.n, vector(0.0, 0.0, -1.0));
    assert_eq!(comps.inside, false);

    // The hit, when an intersection occurs on the inside
    let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
    let shape = Sphere::default();
    let i = I::new(1.0, shape.wrap());
    let comps = Computations::new(i, &r);
    vassert!(comps.p, point(0.0, 0.0, 1.0));
    vassert!(comps.e, vector(0.0, 0.0, -1.0));
    assert_eq!(comps.inside, true);
    // normal must be inverted
    vassert!(comps.n, vector(0.0, 0.0, -1.0));

    // Shading an intersection
    let w = World::default();
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let shape = w.objects[0].clone();
    let i = I::new(4.0, shape);
    let comps = Computations::new(i, &r);
    let c = w.shade_hit(comps);
    assert_eq!(c, color(0.38066, 0.47583, 0.2855));

    // Shading an intersection from the inside
    let mut w = World::default();
    w.sources[0] = PointLight::new(point(0.0, 0.25, 0.0), color(1.0, 1.0, 1.0)).wrap_box();
    let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
    let shape = w.objects[1].clone();
    let i = I::new(0.5, shape);
    let comps = Computations::new(i, &r);
    let c = w.shade_hit(comps);
    assert_eq!(c, color(0.90498, 0.90498, 0.90498));

    // The color when a ray misses
    let w = World::default();
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0));
    assert_eq!(w.calc(&r, &Color::black()), color(0.0, 0.0, 0.0));

    // The color when a ray hits
    let w = World::default();
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    assert_eq!(w.calc(&r, &Color::black()), color(0.38066, 0.47583, 0.2855));

    // The color with an intersection behind the ray
    let w = World::default();
    let outer = w.objects[0].clone();
    outer.borrow_mut().get_material_mut().ambient = 1.0;
    let inner = w.objects[1].clone();
    inner.borrow_mut().get_material_mut().ambient = 1.0;
    let r = Ray::new(point(0.0, 0.0, 0.75), vector(0.0, 0.0, -1.0));
    assert_eq!(
        w.calc(&r, &Color::black()),
        inner.borrow_mut().get_material_mut().color
    );

    // The transfomration matrix for the default orientation
    let from = point(0.0, 0.0, 0.0);
    let to = point(0.0, 0.0, -1.0);
    let up = vector(0.0, 1.0, 0.0);
    let mut t = Camera::new(0, 0, 0.0);
    t.set_view(from, to, up);
    massert!(t.vtm, Matrix::identity());

    // A view transformation matrix looking in positive Z direction
    let from = point(0.0, 0.0, 0.0);
    let to = point(0.0, 0.0, 1.0);
    let up = vector(0.0, 1.0, 0.0);
    let mut t = Camera::new(0, 0, 0.0);
    t.set_view(from, to, up);
    massert!(t.vtm, TUnit::Scale(-1.0, 1.0, -1.0).matrix());

    // The view transformation moves the world
    let from = point(0.0, 0.0, 8.0);
    let to = point(0.0, 0.0, 0.0);
    let up = vector(0.0, 1.0, 0.0);
    let mut t = Camera::new(0, 0, 0.0);
    t.set_view(from, to, up);
    massert!(t.vtm, TUnit::Translate(0.0, 0.0, -8.0).matrix());

    // An arbitrary view transformation
    let from = point(1.0, 3.0, 2.0);
    let to = point(4.0, -2.0, 8.0);
    let up = vector(1.0, 1.0, 0.0);
    let mut t = Camera::new(0, 0, 0.0);
    t.set_view(from, to, up);
    massert!(
        t.vtm,
        matrix(
            -0.50709, 0.50709, 0.67612, -2.36643, 0.76772, 0.60609, 0.12122, -2.82843, -0.35857,
            0.59761, -0.71714, 0.00000, 0.00000, 0.00000, 0.00000, 1.00000
        )
    );

    // The pixel size for a horizontal canvas
    let c = Camera::new(200, 125, PI / 2.0);
    fassert!(c.px_size, 0.01);

    // The pixel size for a vertical canvas
    let c = Camera::new(125, 200, PI / 2.0);
    fassert!(c.px_size, 0.01);

    // Constructing a ray through the center of the canvas
    let c = Camera::new(201, 101, PI / 2.0);
    let r = c.ray_for_pixel(100, 50);
    vassert!(r.origin, point(0.0, 0.0, 0.0));
    vassert!(r.direction, vector(0.0, 0.0, -1.0));

    // Constructing a ray through a corner of the canvas
    let r = c.ray_for_pixel(0, 0);
    vassert!(r.origin, point(0.0, 0.0, 0.0));
    vassert!(r.direction, vector(0.66519, 0.33259, -0.66851));

    // Constructing a ray when the camera is transformed
    let mut c = Camera::new(201, 101, PI / 2.0);
    c.vtm = *transform!(TUnit::Translate(0.0, -2.0, 5.0), TUnit::RotateY(PI / 4.0)).matrix();
    let r = c.ray_for_pixel(100, 50);
    vassert!(r.origin, point(0.0, 2.0, -5.0));
    vassert!(
        r.direction,
        vector(2_f64.sqrt() / 2.0, 0.0, -2_f64.sqrt() / 2.0)
    );
}

#[test]
fn test_shadow_casting() {
    let m = Material::default();
    let p = point(0.0, 0.0, 0.0);

    // Lighting with the surface in shadow
    let e = vector(0.0, 0.0, -1.0);
    let n = vector(0.0, 0.0, -1.0);
    let light = PointLight::new(point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0));
    let in_shadow = true;
    let res = light.shade(&m, &p, &e, &n, in_shadow);
    assert_eq!(res, color(0.1, 0.1, 0.1));

    // There is no shadow when nothing is collienar with point and light
    let w = World::default();
    let p = point(0.0, 10.0, 0.0);
    assert_eq!(w.is_shadowed(&p), false);

    // The shadow when an object is between the point and the light
    let p = point(10.0, -10.0, 10.0);
    assert_eq!(w.is_shadowed(&p), true);

    // There is no shadow when an object is behind the light
    let p = point(-20.0, 20.0, -20.0);
    assert_eq!(w.is_shadowed(&p), false);

    // There is no shadow when an object is behind the point
    let p = point(-2.0, 2.0, -2.0);
    assert_eq!(w.is_shadowed(&p), false);

    // shade_hit() is given an intersection in shadow
    let mut w = World::new();
    let l = PointLight::new(point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0));
    w.add_src(l.wrap_box());

    let s1 = Sphere::default();
    w.add_obj(s1.wrap());

    let s2 = Sphere::default().wrap();
    s2.borrow_mut().set_tunit(TUnit::Translate(0.0, 0.0, 10.0));
    w.add_obj(s2.clone());

    let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
    let i = I::new(4.0, s2.clone());
    let comps = Computations::new(i, &r);
    let c = w.shade_hit(comps);
    assert_eq!(c, color(0.1, 0.1, 0.1));
}

#[test]
fn check_planes() {
    // The normal of a plane is constant everywhere
    let p = Plane::default();
    let n1 = p.local_normal(&point(0.0, 0.0, 0.0));
    let n2 = p.local_normal(&point(10.0, 0.0, -10.0));
    let n3 = p.local_normal(&point(-5.0, 0.0, 150.0));
    vassert!(n1, vector(0.0, 1.0, 0.0));
    vassert!(n2, vector(0.0, 1.0, 0.0));
    vassert!(n3, vector(0.0, 1.0, 0.0));

    // Intersect with a ray parallel to the plane
    let r = Ray::new(point(0.0, 10.0, 0.0), vector(0.0, 0.0, 1.0));
    let i = p.local_intersect(&r);
    assert!(i.len() == 0); 
    
    // Intersect with a coplanar ray
    let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
    let i = p.local_intersect(&r);
    assert!(i.len() == 0);

    // A ray intersecting a plane from above
    let r = Ray::new(point(0.0, 1.0, 0.0), vector(0.0, -1.0, 0.0));
    let i = p.local_intersect(&r);
    assert_eq!(i.len(), 1);
    assert!(i.contains(&1.0));
}
