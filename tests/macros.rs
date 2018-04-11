#![feature(use_extern_macros)]

extern crate mat;

#[test]
fn macro_mat() {
    use mat::traits::Matrix;

    let a = mat::mat![
        [1.0, 2.0],
        [3.0, 4.0],
        [5.0, 6.0],
    ];

    assert_eq!(a.get(0,0), 1.0);
    assert_eq!(a.get(0,1), 2.0);
    assert_eq!(a.get(1,0), 3.0);
    assert_eq!(a.get(1,1), 4.0);
    assert_eq!(a.get(2,0), 5.0);
    assert_eq!(a.get(2,1), 6.0);
}

#[test]
fn macro_mult_mat() {
    use mat::traits::Matrix;

    // 2 by 3 matrix
    let a = mat::mat![
        [1.0, 2.0, 3.0],
        [3.0, 4.0, 5.0],
    ];

    // 3 by 2 matrix
    let b = mat::mat![
        [1.0, 2.0],
        [3.0, 4.0],
        [5.0, 6.0],
    ];

    // build an expression tree
    let c = &a * &b;

    // evaluate the tree
    assert_eq!(c.get(0, 0), 22.0);
    assert_eq!(c.get(0, 1), 28.0);
    assert_eq!(c.get(1, 0), 40.0);
    assert_eq!(c.get(1, 1), 52.0);
}

#[test]
fn macro_mat_gen() {
    use mat::traits::Matrix;

    let a = mat::mat_gen![
        [1.0, 2.0],
        [3.0, 4.0],
        [5.0, 6.0],
    ];

    assert_eq!(a.get(0,0), 1.0);
    assert_eq!(a.get(0,1), 2.0);
    assert_eq!(a.get(1,0), 3.0);
    assert_eq!(a.get(1,1), 4.0);
    assert_eq!(a.get(2,0), 5.0);
    assert_eq!(a.get(2,1), 6.0);
}

#[test]
fn macro_mult_mat_gen() {
    use mat::traits::Matrix;

    // 2 by 3 matrix
    let a = mat::mat_gen![
        [1.0, 2.0, 3.0],
        [3.0, 4.0, 5.0],
    ];

    // 3 by 2 matrix
    let b = mat::mat_gen![
        [1.0, 2.0],
        [3.0, 4.0],
        [5.0, 6.0],
    ];

    // build an expression tree
    let c = &a * &b;

    // evaluate the tree
    assert_eq!(c.get(0, 0), 22.0);
    assert_eq!(c.get(0, 1), 28.0);
    assert_eq!(c.get(1, 0), 40.0);
    assert_eq!(c.get(1, 1), 52.0);
}

#[test]
fn macro_mat_gen_imm() {
    use mat::traits::ImmMatrix;

    let a = mat::mat_gen_imm![
        [1.0, 2.0],
        [3.0, 4.0],
        [5.0, 6.0],
    ];

    assert_eq!(a.get(0,0), 1.0);
    assert_eq!(a.get(0,1), 2.0);
    assert_eq!(a.get(1,0), 3.0);
    assert_eq!(a.get(1,1), 4.0);
    assert_eq!(a.get(2,0), 5.0);
    assert_eq!(a.get(2,1), 6.0);
}

#[test]
fn macro_mult_mat_gen_imm() {
    use mat::traits::ImmMatrix;

    // 2 by 3 matrix
    let a = mat::mat_gen_imm![
        [1.0, 2.0, 3.0],
        [3.0, 4.0, 5.0],
    ];

    assert_eq!(a.nrows(), 2);
    assert_eq!(a.ncols(), 3);

    // 3 by 2 matrix
    let b = mat::mat_gen_imm![
        [1.0, 2.0],
        [3.0, 4.0],
        [5.0, 6.0],
    ];

    assert_eq!(b.nrows(), 3);
    assert_eq!(b.ncols(), 2);

    assert_eq!(a.ncols(), b.nrows());

    // multiplication
    let c = &a * &b;

    // evaluate the tree
    assert_eq!(c.get(0, 0), 22.0);
    assert_eq!(c.get(0, 1), 28.0);
    assert_eq!(c.get(1, 0), 40.0);
    assert_eq!(c.get(1, 1), 52.0);
}

#[test]
fn macro_add_mat_gen_imm() {
    use mat::traits::ImmMatrix;

    // 2 by 3 matrix
    let a = mat::mat_gen_imm![
        [1.0, 2.0],
        [3.0, 4.0],
    ];

    assert_eq!(a.nrows(), 2);
    assert_eq!(a.ncols(), 2);

    // 3 by 2 matrix
    let b = mat::mat_gen_imm![
        [1.0, 2.0],
        [3.0, 4.0]
    ];

    assert_eq!(b.nrows(), 2);
    assert_eq!(b.ncols(), 2);

    assert_eq!(a.ncols(), b.nrows());

    // addition
    let c = &a + &b;

    // evaluate the tree
    assert_eq!(c.get(0, 0), 2.0);
    assert_eq!(c.get(0, 1), 4.0);
    assert_eq!(c.get(1, 0), 6.0);
    assert_eq!(c.get(1, 1), 8.0);
}

#[test]
fn mat_add_assignment() {
    use mat::traits::ImmMatrix;

    let a = mat::mat_gen_imm![
        [0., 0.],
        [0., 0.],
    ];

    let b = mat::mat_gen_imm![
        [0., 0.],
        [0., 0.],
    ];

    let mut c: mat::MatGenImm<f64, mat::typenum::U2, mat::typenum::U2> = Default::default();

    c = &a + &b;
}

#[test]
fn mat_mul_assignment() {
    use mat::traits::ImmMatrix;

    let a = mat::mat_gen_imm![
        [0., 0.],
        [0., 0.],
    ];

    let b = mat::mat_gen_imm![
        [0., 0.],
        [0., 0.],
    ];

    let mut c: mat::MatGenImm<f64, mat::typenum::U2, mat::typenum::U2> = Default::default();

    c = &a * &b;
}

#[test]
fn mat_tranpose() {
    use mat::traits::ImmMatrix;
    use mat::traits::TransposeImm;

    let a = mat::mat_gen_imm![
        [1., 2.],
        [3., 4.],
    ];

    let c = a.t();

    // evaluate the tree
    assert_eq!(c.get(0, 0), 1.);
    assert_eq!(c.get(0, 1), 3.);
    assert_eq!(c.get(1, 0), 2.);
    assert_eq!(c.get(1, 1), 4.);
}

#[test]
fn mat_sub() {
    use mat::traits::ImmMatrix;

    let a = mat::mat_gen_imm![
        [1., 2.],
        [3., 4.],
    ];

    let b = mat::mat_gen_imm![
        [4., 3.],
        [2., 1.],
    ];

    let c = &a - &b;

    // evaluate the tree
    assert_eq!(c.get(0, 0), -3.);
    assert_eq!(c.get(0, 1), -1.);
    assert_eq!(c.get(1, 0),  1.);
    assert_eq!(c.get(1, 1),  3.);
}