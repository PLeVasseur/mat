//! Statically sized matrices for `no_std` applications
//!
//! This library provides support for creating and performing mathematical operations on *statically
//! sized* matrices. That is matrices whose dimensions are known at compile time. The main use case
//! for this library are `no_std` programs where a memory allocator is not available.
//!
//! Since the matrices are statically allocated the dimensions of the matrix are stored in the type
//! system and used to prevent invalid operations (e.g. adding a 3x4 matrix to a 4x3 matrix) at
//! compile time.
//!
//! For performance reasons all operations, except for the indexing `get` method, are lazy and
//! perform no actual computation. An expression like `a * b + c;` simply builds an *expression
//! tree*. `get` can be used to force evaluation of such a tree; see below:
//!
//! ```
//! #![feature(proc_macro)]
//!
//! use mat::mat;
//! use mat::traits::Matrix;
//!
//! // 2 by 3 matrix
//! let a = mat![
//!     [1, 2, 3],
//!     [3, 4, 5],
//! ];
//!
//! // 3 by 2 matrix
//! let b = mat![
//!     [1, 2],
//!     [3, 4],
//!     [5, 6],
//! ];
//!
//! // build an expression tree
//! let c = &a * &b;
//!
//! // partially evaluate the tree
//! assert_eq!(c.get(0, 0), 22);
//! ```
//!
//! This program does *not* allocate and compute a whole new matrix C of size 2x2; it simply
//! performs the operations required to get the element at row 0 and column 0 that such matrix C
//! would have.
//!
//! # Out of scope
//!
//! The following features are out of scope for this library.
//!
//! - Operation that require dynamic memory allocation
//! - SIMD acceleration
//! - n-dimensional arrays
//!
//! If you are looking for such features check out the [`ndarray`] crate.
//!
//! [`ndarray`]: https://crates.io/crates/ndarray
//!
//! # Development status
//!
//! This library is unlikely to see much development until support for [const generics] lands in the
//! compiler.
//!
//! [const generics]: https://github.com/rust-lang/rust/issues/44580

//#![deny(missing_docs)]
//#![deny(warnings)]
#![feature(proc_macro)]
#![feature(unsize)]
#![no_std]

extern crate mat_macros;
#[doc(hidden)]
pub extern crate typenum;
pub extern crate generic_array;

use core::ops;
use core::ops::{Mul};
use core::marker::{PhantomData, Unsize};
use core::borrow::{BorrowMut};
use core::fmt;

pub use mat_macros::mat;
pub use mat_macros::mat_gen;
pub use mat_macros::mat_gen_imm;
use typenum::{Unsigned, Prod};
use generic_array::{GenericArray, ArrayLength};

pub mod traits;

use traits::{Matrix, UnsafeGet, Zero, ImmMatrix};

/// Statically allocated (row major order) matrix
#[derive(Clone)]
pub struct Mat<T, BUFFER, NROWS, NCOLS>
where
    BUFFER: Unsize<[T]>,
    NCOLS: Unsigned,
    NROWS: Unsigned,
    T: Copy,
{
    buffer: BUFFER,
    ty: PhantomData<[T; 0]>,
    nrows: PhantomData<NROWS>,
    ncols: PhantomData<NCOLS>,
}

/// Statically allocated (row major order) matrix, generic column and row sizes
#[derive(Clone)]
pub struct MatGen<T, NROWS, NCOLS>
where
    T: Copy + Default,
    NROWS: Unsigned,
    NCOLS: Unsigned,
    NROWS: Mul<NCOLS>,
    Prod<NROWS, NCOLS>: ArrayLength<T>,
{
    data: GenericArray<T, Prod<NROWS, NCOLS>>,
}

/// Statically allocated (row major order) matrix, generic column and row sizes
#[derive(Clone)]
pub struct MatGenImm<T, NROWS, NCOLS>
    where
        T: Copy + Default,
        NROWS: Unsigned,
        NCOLS: Unsigned,
        NROWS: Mul<NCOLS>,
        Prod<NROWS, NCOLS>: ArrayLength<T>,
{
    data: GenericArray<T, Prod<NROWS, NCOLS>>,
}

/// The product of two matrices
#[derive(Clone, Copy)]
pub struct Product<L, R> {
    l: L,
    r: R,
}

/// The sum of two matrices
#[derive(Clone, Copy)]
pub struct Sum<L, R> {
    l: L,
    r: R,
}

/// The transpose of a matrix
#[derive(Clone, Copy)]
pub struct Transpose<M> {
    m: M,
}

impl<T, BUFFER, NROWS, NCOLS> Mat<T, BUFFER, NROWS, NCOLS>
where
    BUFFER: Unsize<[T]>,
    NROWS: Unsigned,
    NCOLS: Unsigned,
    T: Copy,
{
    #[doc(hidden)]
    pub unsafe fn new(buffer: BUFFER) -> Self {
        Mat {
            buffer,
            ty: PhantomData,
            nrows: PhantomData,
            ncols: PhantomData,
        }
    }
}

impl<T, NROWS, NCOLS> MatGen<T, NROWS, NCOLS>
where
    T: Copy + Default,
    NROWS: Unsigned,
    NCOLS: Unsigned,
    NROWS: Mul<NCOLS>,
    Prod<NROWS, NCOLS>: ArrayLength<T>
{
    pub fn new(data: GenericArray<T, Prod<NROWS, NCOLS>>/* type signature? */) -> Self {
        MatGen {
            data
        }
    }
}

impl<T, NROWS, NCOLS> Default for MatGen<T, NROWS, NCOLS>
where
    T: Copy + Default,
    NROWS: Unsigned,
    NCOLS: Unsigned,
    NROWS: Mul<NCOLS>,
    Prod<NROWS, NCOLS>: ArrayLength<T>,
{
    fn default() -> MatGen<T, NROWS, NCOLS> {
        MatGen {
            data: Default::default()
        }
    }
}

impl<T, NROWS, NCOLS> MatGenImm<T, NROWS, NCOLS>
    where
        T: Copy + Default,
        NROWS: Unsigned,
        NCOLS: Unsigned,
        NROWS: Mul<NCOLS>,
        Prod<NROWS, NCOLS>: ArrayLength<T>
{
    pub fn new(data: GenericArray<T, Prod<NROWS, NCOLS>>/* type signature? */) -> Self {
        MatGenImm {
            data
        }
    }
}

impl<T, NROWS, NCOLS> Default for MatGenImm<T, NROWS, NCOLS>
    where
        T: Copy + Default,
        NROWS: Unsigned,
        NCOLS: Unsigned,
        NROWS: Mul<NCOLS>,
        Prod<NROWS, NCOLS>: ArrayLength<T>,
{
    fn default() -> MatGenImm<T, NROWS, NCOLS> {
        MatGenImm {
            data: Default::default()
        }
    }
}

impl<T, BUFFER, NROWS, NCOLS> fmt::Debug for Mat<T, BUFFER, NROWS, NCOLS>
where
    BUFFER: Unsize<[T]>,
    NROWS: Unsigned,
    NCOLS: Unsigned,
    T: Copy + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut is_first = true;
        let slice: &[T] = &self.buffer;
        f.write_str("[")?;
        for row in slice.chunks(NCOLS::to_usize()) {
            if is_first {
                is_first = false;
            } else {
                f.write_str(", ")?;
            }

            write!(f, "{:?}", row)?;
        }
        f.write_str("]")
    }
}

impl<T, NROWS, NCOLS> fmt::Debug for MatGen<T, NROWS, NCOLS>
where
    T: Copy + Default + fmt::Debug,
    NROWS: Unsigned,
    NCOLS: Unsigned,
    NROWS: Mul<NCOLS>,
    Prod<NROWS, NCOLS>: ArrayLength<T>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        // account for when one of the dimensions is zero
        if NROWS::to_usize() < 1 || NCOLS::to_usize() < 1 {
            return f.write_str("[]")
        }

        let mut is_first = true;
        let slice: &[T] = &self.data.as_slice();
        f.write_str("[")?;
        for row in slice.chunks(NCOLS::to_usize()) {
            if is_first {
                is_first = false;
            } else {
                f.write_str(", ")?;
            }

            write!(f, "{:?}", row)?;
        }
        f.write_str("]")
    }
}

impl<T, NROWS, NCOLS> fmt::Debug for MatGenImm<T, NROWS, NCOLS>
    where
        T: Copy + Default + fmt::Debug,
        NROWS: Unsigned,
        NCOLS: Unsigned,
        NROWS: Mul<NCOLS>,
        Prod<NROWS, NCOLS>: ArrayLength<T>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        // account for when one of the dimensions is zero
        if NROWS::to_usize() < 1 || NCOLS::to_usize() < 1 {
            return f.write_str("[]")
        }

        let mut is_first = true;
        let slice: &[T] = &self.data.as_slice();
        f.write_str("[")?;
        for row in slice.chunks(NCOLS::to_usize()) {
            if is_first {
                is_first = false;
            } else {
                f.write_str(", ")?;
            }

            write!(f, "{:?}", row)?;
        }
        f.write_str("]")
    }
}

impl<'a, T, BUFFER, NROWS, NCOLS> Matrix for &'a Mat<T, BUFFER, NROWS, NCOLS>
where
    BUFFER: Unsize<[T]>,
    NROWS: Unsigned,
    NCOLS: Unsigned,
    T: Copy,
{
    type NROWS = NROWS;
    type NCOLS = NCOLS;
}

impl<'a, T, NROWS, NCOLS> Matrix for &'a MatGen<T, NROWS, NCOLS>
where
    T: Copy + Default,
    NROWS: Unsigned,
    NCOLS: Unsigned,
    NROWS: Mul<NCOLS>,
    Prod<NROWS, NCOLS>: ArrayLength<T>,
{
    type NROWS = NROWS;
    type NCOLS = NCOLS;
}

impl<'a, T, NROWS, NCOLS> ImmMatrix for &'a MatGenImm<T, NROWS, NCOLS>
    where
        T: Copy + Default,
        NROWS: Unsigned,
        NCOLS: Unsigned,
        NROWS: Mul<NCOLS>,
        Prod<NROWS, NCOLS>: ArrayLength<T>,
{
    type NROWS = NROWS;
    type NCOLS = NCOLS;
}

impl<'a, T, BUFFER, NROWS, NCOLS> UnsafeGet for &'a Mat<T, BUFFER, NROWS, NCOLS>
where
    BUFFER: Unsize<[T]>,
    NROWS: Unsigned,
    NCOLS: Unsigned,
    T: Copy,
{
    type Elem = T;

    unsafe fn unsafe_get(self, r: usize, c: usize) -> T {
        let slice: &[T] = &self.buffer;
        *slice.get_unchecked(r * NCOLS::to_usize() + c)
    }
}

impl<'a, T, NROWS, NCOLS> UnsafeGet for &'a MatGen<T, NROWS, NCOLS>
where
    T: Copy + Default,
    NROWS: Unsigned,
    NCOLS: Unsigned,
    NROWS: Mul<NCOLS>,
    Prod<NROWS, NCOLS>: ArrayLength<T>,
{
    type Elem = T;

    unsafe fn unsafe_get(self, r: usize, c: usize) -> T {
        let slice: &[T] = &self.data.as_slice();
        *slice.get_unchecked(r * NCOLS::to_usize() + c)
    }
}

impl<'a, T, NROWS, NCOLS> UnsafeGet for &'a MatGenImm<T, NROWS, NCOLS>
    where
        T: Copy + Default,
        NROWS: Unsigned,
        NCOLS: Unsigned,
        NROWS: Mul<NCOLS>,
        Prod<NROWS, NCOLS>: ArrayLength<T>,
{
    type Elem = T;

    unsafe fn unsafe_get(self, r: usize, c: usize) -> T {
        let slice: &[T] = &self.data.as_slice();
        *slice.get_unchecked(r * NCOLS::to_usize() + c)
    }
}

impl<'a, T, BUFFER, NROWS, NCOLS, R> ops::Mul<R> for &'a Mat<T, BUFFER, NROWS, NCOLS>
where
    BUFFER: Unsize<[T]>,
    NROWS: Unsigned,
    NCOLS: Unsigned,
    T: Copy,
    R: Matrix<NROWS = NCOLS>,
{
    type Output = Product<&'a Mat<T, BUFFER, NROWS, NCOLS>, R>;

    fn mul(self, rhs: R) -> Self::Output {
        Product { l: self, r: rhs }
    }
}

impl<'a, T, NROWS, NCOLS, R> ops::Mul<R> for &'a MatGen<T, NROWS, NCOLS>
where
    T: Copy + Default,
    NROWS: Unsigned,
    NCOLS: Unsigned,
    NROWS: Mul<NCOLS>,
    Prod<NROWS, NCOLS>: ArrayLength<T>,
    R: Matrix<NROWS = NCOLS>,
{
    type Output = Product<&'a MatGen<T, NROWS, NCOLS>, R>;

    fn mul(self, rhs: R) -> Self::Output {
        Product { l: self, r: rhs }
    }
}

impl<'a, T, NROWS, NCOLS, R> ops::Mul<R> for &'a MatGenImm<T, NROWS, NCOLS>
where
    T: Copy + Default + Zero + ops::Mul<T, Output = T> + ops::Add<T, Output = T>,
    NROWS: Unsigned,
    NCOLS: Unsigned,
    NROWS: Mul<NCOLS>,
    Prod<NROWS, NCOLS>: ArrayLength<T>,
    NROWS: Mul<R::NCOLS>,
    Prod<NROWS, R::NCOLS>: ArrayLength<T>,
    R: ImmMatrix<Elem = T, NROWS = NCOLS>
{
    type Output = MatGenImm<T, NROWS, R::NCOLS>;

    fn mul(self, rhs: R) -> Self::Output {
        let mut store: MatGenImm<T, NROWS, R::NCOLS> = Default::default();
        {
            let slice: &mut [T] = store.data.borrow_mut();

            // naive iterative algorithm -- one spot for improvement
            // either by trying to use native Rust solution or a binding
            // to a linear algebra library to get dgemm and sgemm
            // (single- and double-precision generalized matrix multiplication)
            for i in 0..NROWS::to_usize() {
                for j in 0..R::NCOLS::to_usize() {
                    let mut sum = T::zero();

                    for k in 0..NCOLS::to_usize() {
                        sum = sum + self.get(i, k) * rhs.get(k, j);
                    }
                    slice[i * R::NCOLS::to_usize() + j] = sum;
                }
            }
        }

        store
    }
}

impl<'a, T, NROWS, NCOLS, R> ops::Add<R> for &'a MatGenImm<T, NROWS, NCOLS>
    where
        T: Copy + Default + Zero + ops::Mul<T, Output = T> + ops::Add<T, Output = T>,
        NROWS: Unsigned,
        NCOLS: Unsigned,
        NROWS: Mul<NCOLS>,
        Prod<NROWS, NCOLS>: ArrayLength<T>,
        NROWS: Mul<R::NCOLS>,
        Prod<NROWS, R::NCOLS>: ArrayLength<T>,
        R: ImmMatrix<Elem = T, NROWS = NROWS, NCOLS = NCOLS>
{
    type Output = MatGenImm<T, NROWS, NCOLS>;

    fn add(self, rhs: R) -> Self::Output {
        let mut store: MatGenImm<T, NROWS, NCOLS> = Default::default();
        {
            let slice: &mut [T] = store.data.borrow_mut();

            // C = A * B
            for i in 0..NROWS::to_usize() {
                for j in 0..NCOLS::to_usize() {

                    slice[i * NCOLS::to_usize() + j] =  self.get(i, j) + rhs.get(i, j);
                }
            }
        }

        store
    }
}

impl<M> traits::Transpose for M
where
    M: Matrix,
{
}

impl<M> Matrix for Transpose<M>
where
    M: Matrix,
{
    // NOTE reversed size!
    type NROWS = M::NCOLS;
    type NCOLS = M::NROWS;
}

impl<M> UnsafeGet for Transpose<M>
where
    M: Matrix,
{
    type Elem = M::Elem;

    unsafe fn unsafe_get(self, r: usize, c: usize) -> M::Elem {
        // NOTE reversed indices!
        self.m.unsafe_get(c, r)
    }
}

impl<L, R> ops::Mul<R> for Transpose<L>
where
    L: Matrix,
    R: Matrix<NROWS = L::NROWS>,
{
    type Output = Product<Transpose<L>, R>;

    fn mul(self, rhs: R) -> Self::Output {
        Product { l: self, r: rhs }
    }
}

impl<L, R, T> Matrix for Product<L, R>
where
    L: Matrix<Elem = T>,
    R: Matrix<Elem = T>,
    T: ops::Add<T, Output = T> + ops::Mul<T, Output = T> + Copy + Zero,
{
    type NROWS = L::NROWS;
    type NCOLS = R::NCOLS;
}

impl<T, L, R> UnsafeGet for Product<L, R>
where
    L: Matrix<Elem = T>,
    R: Matrix<Elem = T>,
    T: ops::Add<T, Output = T> + ops::Mul<T, Output = T> + Copy + Zero,
{
    type Elem = T;

    unsafe fn unsafe_get(self, r: usize, c: usize) -> T {
        let mut sum = T::zero();
        for i in 0..self.l.ncols() {
            sum = sum + self.l.unsafe_get(r, i) * self.r.unsafe_get(i, c);
        }
        sum
    }
}

impl<L, R, RHS> ops::Add<RHS> for Product<L, R>
where
    L: Matrix,
    R: Matrix,
    RHS: Matrix<NROWS = L::NROWS, NCOLS = R::NCOLS>,
{
    type Output = Sum<Product<L, R>, RHS>;

    fn add(self, rhs: RHS) -> Self::Output {
        Sum { l: self, r: rhs }
    }
}

impl<T, L, R> Matrix for Sum<L, R>
where
    L: Matrix<Elem = T>,
    R: Matrix<Elem = T>,
    T: ops::Add<T, Output = T> + Copy,
{
    type NROWS = L::NROWS;
    type NCOLS = L::NCOLS;
}

impl<T, L, R> UnsafeGet for Sum<L, R>
where
    L: Matrix<Elem = T>,
    R: Matrix<Elem = T>,
    T: ops::Add<T, Output = T> + Copy,
{
    type Elem = T;

    unsafe fn unsafe_get(self, r: usize, c: usize) -> T {
        self.l.unsafe_get(r, c) + self.r.unsafe_get(r, c)
    }
}
