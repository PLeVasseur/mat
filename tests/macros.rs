#![feature(use_extern_macros)]

extern crate mat;

#[test]
fn macro_mat() {
    use mat::traits::LazyMatrix;

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
    use mat::traits::LazyMatrix;

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
    use mat::traits::LazyMatrix;

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
    use mat::traits::LazyMatrix;

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