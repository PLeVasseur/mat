extern crate mat;

#[test]
fn mat_gen() {
    use mat::traits::Matrix;

    let a: mat::MatGen<f32, mat::typenum::U3, mat::typenum::U2> = Default::default();

    assert_eq!(a.get(0,0), 0.0);
    assert_eq!(a.get(0,1), 0.0);
    assert_eq!(a.get(1,0), 0.0);
    assert_eq!(a.get(1,1), 0.0);
    assert_eq!(a.get(2,0), 0.0);
    assert_eq!(a.get(2,1), 0.0);
}
