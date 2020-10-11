use std::f64::consts::PI;

use super::{Point3D, Quaternion3D, Transform3D};

#[test]
fn test_transform3d_concat() {
    assert_eq!(
        Transform3D::with_scale(2.0, 3.0, 1.0).translate(42.0, 8.0, 0.0),
        Transform3D::new([
            [2.0, 0.0, 0.0, 0.0],
            [0.0, 3.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [84.0, 24.0, 0.0, 1.0]
        ])
    );

    assert_eq!(
        Transform3D::with_translation(42.0, 8.0, 0.0).rotate(Quaternion3D::with_angle(
            90.0 / 180.0 * PI,
            0.0,
            0.0,
            1.0
        )),
        Transform3D::new([
            [0.0, 1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [42.0, 8.0, 0.0, 1.0]
        ])
    );

    assert_eq!(
        Transform3D::with_rotation(Quaternion3D::with_angle(90.0 / 180.0 * PI, 0.0, 0.0, 1.0))
            .translate(42.0, 8.0, 0.0),
        Transform3D::new([
            [0.0, 1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [-8.0, 42.0, 0.0, 1.0]
        ])
    );
}

#[test]
fn test_transform3d_rotate() {
    let transform =
        Transform3D::with_rotation(Quaternion3D::with_angle(90.0 / 180.0 * PI, 0.0, 0.0, 1.0));
    assert_eq!(
        transform,
        Transform3D::new([
            [0.0, 1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ])
    );

    let transform =
        Transform3D::with_rotation(Quaternion3D::with_angle(180.0 / 180.0 * PI, 0.0, 0.0, 1.0));
    assert_eq!(
        transform,
        Transform3D::new([
            [-1.0, 0.0, 0.0, 0.0],
            [0.0, -1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ])
    );

    let transform =
        Transform3D::with_rotation(Quaternion3D::with_angle(90.0 / 180.0 * PI, 0.0, 0.0, 2.0));
    assert_eq!(
        transform,
        Transform3D::new([
            [-1.0, 0.0, 0.0, 0.0],
            [0.0, -1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ])
    );

    let transform =
        Transform3D::with_rotation(Quaternion3D::with_angle(45.0 / 180.0 * PI, 0.0, 0.0, 1.0));
    let transform = transform.rotate(Quaternion3D::with_angle(135.0 / 180.0 * PI, 0.0, 0.0, 1.0));
    assert_eq!(
        transform,
        Transform3D::new([
            [-1.0, 0.0, 0.0, 0.0],
            [0.0, -1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ])
    );

    let transform =
        Transform3D::with_rotation(Quaternion3D::with_angle(180.0 / 180.0 * PI, 0.0, 0.0, 0.3));
    let transform = transform.rotate(Quaternion3D::with_angle(180.0 / 180.0 * PI, 0.0, 0.0, 0.7));
    assert_eq!(
        transform,
        Transform3D::new([
            [-1.0, 0.0, 0.0, 0.0],
            [0.0, -1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ])
    );
}

#[test]
fn test_transform3d_apply() {
    let transform = Transform3D::with_translation(0.0, 100.0, 0.0);
    assert_eq!(
        transform.apply(Point3D::new(42.0, 3.0, 0.0)),
        Point3D::new(42.0, 103.0, 0.0)
    );

    let transform = Transform3D::identity()
        .translate(0.0, 100.0, 0.0)
        .scale(2.0, 2.0, 1.0);

    assert_eq!(
        transform.apply(Point3D::new(42.0, 3.0, 0.0)),
        Point3D::new(84.0, 106.0, 0.0)
    );
}
