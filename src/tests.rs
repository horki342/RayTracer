use std::f64::consts::PI;
use std::rc::Rc;

use super::math::utils::*;
use super::math::{Color, Matrix, TUnit, Transformation};
use super::render::Canvas;

use crate::render::core::{Intersection, Ray};
use crate::render::shapes::Sphere;
use crate::{create_intersections, transform};

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
    let c = Canvas::new(10, 20);
    assert_eq!(c.width, 10);
    assert_eq!(c.height, 20);
    for i in 0..c.height {
        for j in 0..c.width {
            assert_eq!(c[[i, j]], Color::default());
        }
    }

    // Writing pixels to the Canvas
    let mut c = Canvas::new(10, 20);
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
    assert!(meq(&a, &b));

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
    assert!(meq(&ab, &(a * b)));

    // Multiplying by a tuple
    let a = matrix(
        1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
    );
    let b = tuple(1.0, 2.0, 3.0, 1.0);
    assert!(veq(&(a * b), &tuple(18.0, 24.0, 33.0, 1.0)));

    // Identity matrix
    let a = matrix(
        1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
    );
    let i = Matrix::identity();
    assert!(meq(&(a * i), &a));

    // Transposing matrices
    let a = matrix(
        0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0,
    );
    let res = matrix(
        0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0,
    );
    assert!(meq(&a.transpose(), &res));

    // Inverse
    let a = matrix(
        8.0, -5.0, 9.0, 2.0, 7.0, 5.0, 6.0, 1.0, -6.0, 0.0, 9.0, 6.0, -3.0, 0.0, -9.0, -4.0,
    );
    let res = matrix(
        -0.15385, -0.15385, -0.28205, -0.53846, -0.07692, 0.12308, 0.02564, 0.03077, 0.35897,
        0.35897, 0.43590, 0.92308, -0.69231, -0.69231, -0.76923, -1.92308,
    );
    assert!(meq(&a.try_inverse().unwrap(), &res));

    let a = matrix(
        9.0, 3.0, 0.0, 9.0, -5.0, -2.0, -6.0, -3.0, -4.0, 9.0, 6.0, 4.0, -7.0, 6.0, 6.0, 2.0,
    );
    let res = matrix(
        -0.04074, -0.07778, 0.14444, -0.22222, -0.07778, 0.03333, 0.36667, -0.33333, -0.02901,
        -0.14630, -0.10926, 0.12963, 0.17778, 0.06667, -0.26667, 0.33333,
    );
    assert!(meq(&a.try_inverse().unwrap(), &res));
}

#[test]
fn transformations() {
    // translation
    let transform = TUnit::Translate(5.0, -3.0, 2.0).matrix();
    let p = point(-3.0, 4.0, 5.0);
    assert!(veq(&(transform * p), &point(2.0, 1.0, 7.0)));

    let transform = TUnit::Translate(5.0, -3.0, 2.0).matrix();
    let inv = transform.try_inverse().unwrap();
    let p = point(-3.0, 4.0, 5.0);
    assert!(veq(&(inv * p), &point(-8.0, 7.0, 3.0)));

    let transform = TUnit::Translate(5.0, -3.0, 2.0).matrix();
    let v = vector(-3.0, 4.0, 5.0);
    assert!(veq(&(transform * v), &v));

    // scaling
    let scale = TUnit::Scale(2.0, 3.0, 4.0).matrix();
    let p = point(-4.0, 6.0, 8.0);
    assert!(veq(&(scale * p), &point(-8.0, 18.0, 32.0)));

    let scale = TUnit::Scale(2.0, 3.0, 4.0).matrix();
    let p = vector(-4.0, 6.0, 8.0);
    assert!(veq(&(scale * p), &vector(-8.0, 18.0, 32.0)));

    let transform = TUnit::Scale(2.0, 3.0, 4.0).matrix();
    let inv = transform.try_inverse().unwrap();
    let v = vector(-4.0, 6.0, 8.0);
    let res = inv * v;
    assert!(veq(&res, &vector(-2.0, 2.0, 2.0)));

    let transform = TUnit::Scale(-1.0, 1.0, 1.0).matrix();
    let p = point(2.0, 3.0, 4.0);
    let res = transform * p;
    assert!(veq(&res, &point(-2.0, 3.0, 4.0)));

    // rotating
    let p = point(0.0, 1.0, 0.0);
    let half_quarter = TUnit::RotateX(PI / 4.0).matrix();
    let full_quarter = TUnit::RotateX(PI / 2.0).matrix();
    let res1 = half_quarter * p;
    let res2 = full_quarter * p;
    assert!(veq(
        &res1,
        &point(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
    ));
    assert!(veq(&res2, &point(0.0, 0.0, 1.0)));

    let inv = half_quarter.try_inverse().unwrap();
    assert!(veq(
        &(inv * p),
        &point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0)
    ));

    let p = point(0.0, 0.0, 1.0);
    let half_quarter = TUnit::RotateY(PI / 4.0).matrix();
    let full_quarter = TUnit::RotateY(PI / 2.0).matrix();
    assert!(veq(
        &(half_quarter * p),
        &point(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0)
    ));
    assert!(veq(&(full_quarter * p), &point(1.0, 0.0, 0.0)));

    let p = point(0.0, 1.0, 0.0);
    let half_quarter = TUnit::RotateZ(PI / 4.0).matrix();
    let full_quarter = TUnit::RotateZ(PI / 2.0).matrix();
    assert!(veq(
        &(half_quarter * p),
        &point(-2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0)
    ));
    assert!(veq(&(full_quarter * p), &point(-1.0, 0.0, 0.0)));

    // shearing
    let transform = TUnit::Shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0).matrix();
    let p = point(2.0, 3.0, 4.0);
    assert!(veq(&(transform * p), &point(5.0, 3.0, 4.0)));

    let transform = TUnit::Shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0).matrix();
    let p = point(2.0, 3.0, 4.0);
    assert!(veq(&(transform * p), &point(6.0, 3.0, 4.0)));

    let transform = TUnit::Shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0).matrix();
    let p = point(2.0, 3.0, 4.0);
    assert!(veq(&(transform * p), &point(2.0, 5.0, 4.0)));

    let transform = TUnit::Shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0).matrix();
    let p = point(2.0, 3.0, 4.0);
    assert!(veq(&(transform * p), &point(2.0, 7.0, 4.0)));

    let transform = TUnit::Shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0).matrix();
    let p = point(2.0, 3.0, 4.0);
    assert!(veq(&(transform * p), &point(2.0, 3.0, 6.0)));

    let transform = TUnit::Shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0).matrix();
    let p = point(2.0, 3.0, 4.0);
    assert!(veq(&(transform * p), &point(2.0, 3.0, 7.0)));

    // chained transformation
    let transform = transform!(
        TUnit::RotateX(PI / 2.0),
        TUnit::Scale(5.0, 5.0, 5.0),
        TUnit::Translate(10.0, 5.0, 7.0)
    );
    let p = point(1.0, 0.0, 1.0);
    assert!(veq(&(&transform * p), &point(15.0, 0.0, 7.0)));
}

#[test]
fn ray_operations() {
    // Computing a point from a distance
    let r = Ray::new(point(2.0, 3.0, 4.0), vector(1.0, 0.0, 0.0));
    assert!(veq(&r.pos(0.0), &point(2.0, 3.0, 4.0)));
    assert!(veq(&r.pos(1.0), &point(3.0, 3.0, 4.0)));
    assert!(veq(&r.pos(-1.0), &point(1.0, 3.0, 4.0)));
    assert!(veq(&r.pos(2.5), &point(4.5, 3.0, 4.0)));

    // Ray intersecting a sphere at two points
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = Sphere::default();
    let xs = r.intersect_sphere(s.clone());

    assert_eq!(xs.count(), 2);
    assert!(xs.contains(4.0));
    assert!(xs.contains(6.0));

    // A ray intersects a sphere at a tangent
    let r = Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
    let xs = r.intersect_sphere(s.clone());

    assert_eq!(xs.count(), 2);
    assert!(xs.contains(5.0));

    // A ray misses a sphere
    let r = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
    let xs = r.intersect_sphere(s.clone());

    assert_eq!(xs.count(), 0);

    // A ray originates inside a sphere
    let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
    let xs = r.intersect_sphere(s.clone());

    assert_eq!(xs.count(), 2);
    assert!(xs.contains(-1.0));
    assert!(xs.contains(1.0));

    // A sphere is behind a ray
    let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
    let s = Sphere::default();
    let xs = r.intersect_sphere(s.clone());

    assert_eq!(xs.count(), 2);
    assert!(xs.contains(-6.0));
    assert!(xs.contains(-4.0));

    // The hit when all intersections have positive t
    let i1 = Intersection::new(1.0, s.clone());
    let i2 = Intersection::new(2.0, s.clone());
    let xs = create_intersections!(i1.clone(), i2);
    let i = xs.hit().unwrap();
    assert_eq!(i, &i1);

    // The hit, when interactions have negative t
    let i1 = Intersection::new(-1.0, s.clone());
    let i2 = Intersection::new(1.0, s.clone());
    let xs = create_intersections!(i1, i2.clone());
    let i = xs.hit().unwrap();
    assert_eq!(i, &i2);

    // The hit, when intersections have negative t
    let i1 = Intersection::new(-2.0, s.clone());
    let i2 = Intersection::new(-1.0, s.clone());
    let xs = create_intersections!(i1, i2);
    let i = xs.hit();
    assert_eq!(i, None);

    // The hit is always the lowest nonnegative intersection
    let i1 = Intersection::new(5.0, s.clone());
    let i2 = Intersection::new(7.0, s.clone());
    let i3 = Intersection::new(-3.0, s.clone());
    let i4 = Intersection::new(2.0, s.clone());
    let xs = create_intersections!(i1, i2, i3, i4.clone());
    let i = xs.hit().unwrap();
    assert_eq!(i, &i4);
}
