use ray_tracing_utils::math::Vec3;

#[test]
fn vector_calculation() {
    let a = Vec3::new(0.0, 2.0, 4.0);
    let b = Vec3::new(1.0, 3.0, 5.0);
    let c = Vec3::new(3.0, -3.0, 3.0);

    assert_eq!(a + b, Vec3::new(1.0, 5.0, 9.0));
    assert_eq!(a + c, Vec3::new(3.0, -1.0, 7.0));
    assert_eq!(b + c, Vec3::new(4.0, 0.0, 8.0));
    assert_eq!(a - b, Vec3::new(-1.0, -1.0, -1.0));
    assert_eq!(a - c, Vec3::new(-3.0, 5.0, 1.0));
    assert_eq!(b - c, Vec3::new(-2.0, 6.0, 2.0));

    assert_eq!(-a, Vec3::new(0.0, -2.0, -4.0));
    assert_eq!(-c, Vec3::new(-3.0, 3.0, -3.0));
    assert_eq!(c * 2.0, Vec3::new(6.0, -6.0, 6.0));
    assert_eq!(c * -0.5, Vec3::new(-1.5, 1.5, -1.5));
    assert_eq!(c / 2.0, Vec3::new(1.5, -1.5, 1.5));

    assert_eq!(a * b, 26.0);
    assert_eq!(a * c, 6.0);
    assert_eq!(b * c, 9.0);
}
