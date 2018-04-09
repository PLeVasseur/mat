#![deny(warnings)]
#![allow(unused_unsafe)]
#![feature(proc_macro)]
#![recursion_limit="128"]

extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

extern crate generic_array;

use proc_macro::TokenStream;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::synom::Synom;
use syn::{Expr, ExprArray, Ident};

#[allow(unused_imports)]
use generic_array::{arr, arr_impl};

struct Mat {
    rows: Punctuated<ExprArray, Token![,]>,
}

impl Synom for Mat {
    named!(parse -> Self, do_parse!(
        rows: call!(Punctuated::parse_terminated_nonempty) >> (Mat { rows })
    ));
}

/// A macro to construct matrices
#[proc_macro]
pub fn mat(input: TokenStream) -> TokenStream {
    let mat: Mat = syn::parse(input).unwrap();

    // check consistent number of columns
    let nrows = mat.rows.len();
    let ncols = mat.rows.iter().next().expect("BUG: zero rows").elems.len();

    for row in mat.rows.iter() {
        for (i, expr) in row.elems.iter().enumerate() {
            if i >= ncols {
                expr.span()
                    .unstable()
                    .error(format!("expected {} elements", ncols,))
                    .emit();
            }
        }
    }

    let size = nrows * ncols;
    let elems: Vec<&Expr> = mat.rows.iter().flat_map(|row| row.elems.iter()).collect();

    let nrows_ty = Ident::from(format!("U{}", nrows));
    let ncols_ty = Ident::from(format!("U{}", ncols));

    quote!(unsafe {
        extern crate mat;
        mat::Mat::<_, [_; #size], mat::typenum::#nrows_ty, mat::typenum::#ncols_ty>::new([#(#elems,)*])
    }).into()
}

/// A macro to construct matrices generic in row and length, backed by a GenericArray
#[proc_macro]
pub fn mat_gen(input: TokenStream) -> TokenStream {
    let mat: Mat = syn::parse(input).unwrap();

    // check consistent number of columns
    let nrows = mat.rows.len();
    let ncols = mat.rows.iter().next().expect("BUG: zero rows").elems.len();

    for row in mat.rows.iter() {
        for (i, expr) in row.elems.iter().enumerate() {
            if i >= ncols {
                expr.span()
                .unstable()
                .error(format!("expected {} elements", ncols,))
                .emit();
            }
        }
    }

    let size = nrows * ncols;
    let elems: Vec<&Expr> = mat.rows.iter().flat_map(|row| row.elems.iter()).collect();

    let nrows_ty = Ident::from(format!("U{}", nrows));
    let ncols_ty = Ident::from(format!("U{}", ncols));
    let size_ty = Ident::from(format!("U{}", size));

    quote!(unsafe {
        extern crate mat;

        let arr = [#(#elems,)*];
        let slice = &arr[..];
        let gen_arr : mat::generic_array::GenericArray<_, mat::typenum::#size_ty> = mat::generic_array::GenericArray::clone_from_slice(slice);

        mat::MatGen::<_, mat::typenum::#nrows_ty, mat::typenum::#ncols_ty>::new(gen_arr)
    }).into()
}