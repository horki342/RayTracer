use crate::draw::Canvas as cv;
use crate::math::*;

#[test]
fn tuple_operations() {
    // check operations
    let a1 = tuple(3.0, -2.0, 5.0, 1.0);

    let a2 = tuple(-2.0, 3.0, 1.0, 0.0);
    assert_eq!(a1 + a2, tuple(1.0, 1.0, 6.0, 1.0));

    let p1 = point(3.0, 2.0, 1.0);
    let p2 = point(5.0, 6.0, 7.0);
    let p = point(3.0, 2.0, 1.0);
    let v = vector(5.0, 6.0, 7.0);
    assert_eq!(p1 - p2, vector(-2.0, -4.0, -6.0));
    assert_eq!(p - v, point(-2.0, -4.0, -6.0));

    let zero = vector(0.0, 0.0, 0.0);
    let v = vector(1.0, -2.0, 3.0);
    assert_eq!(zero - v, vector(-1.0, 2.0, -3.0));

    let a = tuple(1.0, -2.0, 3.0, -4.0);
    assert_eq!(-a, tuple(-1.0, 2.0, -3.0, 4.0));

    let a = tuple(1.0, -2.0, 3.0, -4.0);
    assert_eq!(a * 3.5, tuple(3.5, -7.0, 10.5, -14.0));

    let a = tuple(1.0, -2.0, 3.0, -4.0);
    assert_eq!(3.5 * a, tuple(3.5, -7.0, 10.5, -14.0));

    let a = tuple(1.0, -2.0, 3.0, -4.0);
    assert_eq!(a / 2.0, tuple(0.5, -1.0, 1.5, -2.0));

    let v = vector(1.0, 0.0, 0.0);
    assert_eq!(v.magnitude(), 1.0);

    let v = vector(-1.0, -2.0, -3.0);
    assert_eq!(v.magnitude(), 14.0_f64.sqrt());

    let a = vector(1.0, 2.0, 3.0);
    let b = vector(2.0, 3.0, 4.0);
    assert_eq!(dot(&a, &b), 20.0);

    // check cross product
    let a = vector(1.0, 2.0, 3.0);
    let b = vector(2.0, 3.0, 4.0);
    assert_eq!(cross(&a, &b), vector(-1.0, 2.0, -1.0));
}

#[test]
fn color_operations() {
    let c = color(-0.5, 0.4, 1.7);
    assert_eq!(c.r, -0.5);
    assert_eq!(c.g, 0.4);
    assert_eq!(c.b, 1.7);

    let c1 = color(0.9, 0.6, 0.75);
    let c2 = color(0.7, 0.1, 0.25);
    assert_eq!(c1 + c2, color(1.6, 0.7, 1.0));

    let c1 = color(0.9, 0.6, 0.75);
    let c2 = color(0.7, 0.1, 0.25);
    assert_eq!(c1 - c2, color(0.2, 0.5, 0.5));

    let c = color(0.2, 0.3, 0.4);
    assert_eq!(c * 2.0, color(0.4, 0.6, 0.8));

    // multiplying colors
    let c1 = color(1.0, 0.2, 0.4);
    let c2 = color(0.9, 1.0, 0.1);
    assert_eq!(c1 * c2, color(0.9, 0.2, 0.04));
}

#[test]
fn canvas_operations() {
    // creating a canvas
    let c = cv::new(10, 20);
    assert_eq!(c.width, 10);
    assert_eq!(c.height, 20);
    for i in 0..c.height {
        for j in 0..c.width {
            assert_eq!(c[[i, j]], Color::default());
        }
    }

    // writing pixels to a canvas
    let mut c = cv::new(10, 20);
    let red = color(1.0, 0.0, 0.0);
    let _ = c.write(2, 3, red);
    assert_eq!(c[[2, 3]], color(1.0, 0.0, 0.0));
}

#[test]
fn matrix_operations() {
    // matrix equality
    let a = Matrix::new(
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
    );
    let b = Matrix::new(
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
    );
    assert!(meq(&a, &b));

    // multiplying matrices
    let a = Matrix::new(
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
    );
    let b = Matrix::new(
        -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0,
    );
    let ab = Matrix::new(
        20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0, 26.0,
        46.0, 42.0,
    );
    assert!(meq(&ab, &(a * b)));

    // multiplying by a tuple
    let a = Matrix::new(
        1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
    );
    let b = tuple(1.0, 2.0, 3.0, 1.0);
    assert!(veq(&(a * b), &tuple(18.0, 24.0, 33.0, 1.0)));

    // identity matrix
    let a = Matrix::new(
        1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
    );
    let i = Matrix::identity();
    assert!(meq(&(a * i), &a));

    // transposing matrices
    let a = Matrix::new(
        0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0,
    );
    let res = Matrix::new(
        0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0,
    );
    assert!(meq(&a.transpose(), &res));

    // inverse
    let a = Matrix::new(
        8.0, -5.0, 9.0, 2.0, 7.0, 5.0, 6.0, 1.0, -6.0, 0.0, 9.0, 6.0, -3.0, 0.0, -9.0, -4.0,
    );
    let res = Matrix::new(
        -0.15385, -0.15385, -0.28205, -0.53846, -0.07692, 0.12308, 0.02564, 0.03077, 0.35897,
        0.35897, 0.43590, 0.92308, -0.69231, -0.69231, -0.76923, -1.92308,
    );
    assert!(meq(&a.try_inverse().unwrap(), &res));

    let a = Matrix::new(
        9.0, 3.0, 0.0, 9.0, -5.0, -2.0, -6.0, -3.0, -4.0, 9.0, 6.0, 4.0, -7.0, 6.0, 6.0, 2.0,
    );
    let res = Matrix::new(
        -0.04074, -0.07778, 0.14444, -0.22222, -0.07778, 0.03333, 0.36667, -0.33333, -0.02901,
        -0.14630, -0.10926, 0.12963, 0.17778, 0.06667, -0.26667, 0.33333,
    );
    assert!(meq(&a.try_inverse().unwrap(), &res));
}

#[test]
fn transformations() {
    // translation
    let transform = Transformations::translate(5.0, -3.0, 2.0);
    let p = point(-3.0, 4.0, 5.0);
    assert!(veq(&(transform * p), &point(2.0, 1.0, 7.0)));

    let transform = Transformations::translate(5.0, -3.0, 2.0);
    let inv = transform.try_inverse().unwrap();
    let p = point(-3.0, 4.0, 5.0);
    assert!(veq(&(inv * p), &point(-8.0, 7.0, 3.0)));

    let transform = Transformations::translate(5.0, -3.0, 2.0);
    let v = vector(-3.0, 4.0, 5.0);
    assert!(veq(&(transform * v), &v));
}
