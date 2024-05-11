use crate::draw::{self, Canvas};
use crate::math::color::Color;
use crate::math::linalg::{cross, dot, Tuple};
use crate::{color, point, tuple, vector};

#[test]
fn tuple_operations() {
    assert_eq!(
        point![4.0, -4.0, 3.0],
        Tuple {
            x: 4.0,
            y: -4.0,
            z: 3.0,
            w: 1.0
        }
    );

    assert_eq!(
        vector![4.0, -4.0, 3.0],
        Tuple {
            x: 4.0,
            y: -4.0,
            z: 3.0,
            w: 0.0
        }
    );

    // check operations
    let a1 = tuple![3.0, -2.0, 5.0, 1.0];
    let a2 = tuple![-2.0, 3.0, 1.0, 0.0];
    assert_eq!(a1 + a2, tuple![1.0, 1.0, 6.0, 1.0]);

    let p1 = point![3.0, 2.0, 1.0];
    let p2 = point![5.0, 6.0, 7.0];
    let p = point![3.0, 2.0, 1.0];
    let v = vector![5.0, 6.0, 7.0];
    assert_eq!(p1 - p2, vector![-2.0, -4.0, -6.0]);
    assert_eq!(p - v, point![-2.0, -4.0, -6.0]);

    let zero = vector![0.0, 0.0, 0.0];
    let v = vector![1.0, -2.0, 3.0];
    assert_eq!(zero - v, vector![-1.0, 2.0, -3.0]);

    let a = tuple![1.0, -2.0, 3.0, -4.0];
    assert_eq!(-a, tuple![-1.0, 2.0, -3.0, 4.0]);

    let a = tuple![1.0, -2.0, 3.0, -4.0];
    assert_eq!(a * 3.5, tuple![3.5, -7.0, 10.5, -14.0]);

    let a = tuple![1.0, -2.0, 3.0, -4.0];
    assert_eq!(3.5 * a, tuple![3.5, -7.0, 10.5, -14.0]);

    let a = tuple![1.0, -2.0, 3.0, -4.0];
    assert_eq!(a / 2.0, tuple![0.5, -1.0, 1.5, -2.0]);

    let v = vector![1.0, 0.0, 0.0];
    assert_eq!(v.mag(), 1.0);

    let v = vector![-1.0, -2.0, -3.0];
    assert_eq!(v.mag(), 14.0_f32.sqrt());

    let a = vector![1.0, 2.0, 3.0];
    let b = vector![2.0, 3.0, 4.0];
    assert_eq!(dot(&a, &b), 20.0);

    // check cross product
    let a = vector![1.0, 2.0, 3.0];
    let b = vector![2.0, 3.0, 4.0];
    assert_eq!(cross(&a, &b), vector![-1.0, 2.0, -1.0]);
}

#[test]
fn color_operations() {
    let c = color![-0.5, 0.4, 1.7];
    assert_eq!(c.red, -0.5);
    assert_eq!(c.green, 0.4);
    assert_eq!(c.blue, 1.7);

    let c1 = color![0.9, 0.6, 0.75];
    let c2 = color![0.7, 0.1, 0.25];
    assert_eq!(c1 + c2, color![1.6, 0.7, 1.0]);

    let c1 = color![0.9, 0.6, 0.75];
    let c2 = color![0.7, 0.1, 0.25];
    assert_eq!(c1 - c2, color![0.2, 0.5, 0.5]);

    let c = color![0.2, 0.3, 0.4];
    assert_eq!(c * 2.0, color![0.4, 0.6, 0.8]);

    let c = color![0.2, 0.3, 0.4];
    assert_eq!(2.0 * c, color![0.4, 0.6, 0.8]);

    // multiplying colors\
    let c1 = color![1.0, 0.2, 0.4];
    let c2 = color![0.9, 1.0, 0.1];
    assert_eq!(c1 * c2, color![0.9, 0.2, 0.04]);
}

#[test]
fn canvas_operations() {
    // creating a canvas
    let c = draw::Canvas::new(10, 20);
    assert_eq!(c.width, 10);
    assert_eq!(c.height, 20);
    for row in c.grid {
        for el in row {
            assert_eq!(el, Color::black());
        }
    }

    // writing pixels to a canvas
    let mut c = draw::Canvas::new(10, 20);
    let red = color![1.0, 0.0, 0.0];
    c.write(2, 3, red);
    assert_eq!(c.at(2, 3).clone(), color![1.0, 0.0, 0.0]);
}
