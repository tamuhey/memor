//! An attribute macro to memoize function calls
//!
//! # Usage
//!
//! Just add `#[memo]` to your function.
//!
//! ```
//! use memor::memo;
//! #[memo]
//! fn fib(n: i64) -> i64 {
//!     if n == 0 || n == 1 {
//!         n
//!     } else {
//!         fib(n - 1) + fib(n - 2)
//!     }
//! }
//!
//! assert_eq!(12586269025, fib(50));
//! ```
//!
//! Various functions can be memoized.
//! Because the arguments are saved into keys of `std::collections::HashMap` internally,
//! this macro can be applied to functions all of whose arguments implments `Hash` and `Eq`.
//!
//! ```
//! use memor::memo;
//! #[derive(Hash, Eq, PartialEq)]
//! struct Foo {
//!     a: usize,
//!     b: usize,
//! }
//!
//! #[memo]
//! fn foo(Foo { a, b }: Foo, c: usize) -> usize {
//!     if a == 0 || b == 0 || c == 0 {
//!         1
//!     } else {
//!         foo(Foo { a, b: b - 1 }, c)
//!             .wrapping_add(foo(Foo { a: a - 1, b }, c))
//!             .wrapping_add(foo(Foo { a, b }, c - 1))
//!     }
//! }
//!
//! assert_eq!(foo(Foo { a: 50, b: 50 }, 50), 6753084261833197057);
//! ```
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, parse_quote, Expr, FnArg, ItemFn, ReturnType, Type};

#[proc_macro_attribute]
pub fn memo(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let mut key_types = Vec::<Type>::new();
    let mut keys = Vec::<Expr>::new();
    input.sig.inputs.iter().for_each(|x| match x {
        FnArg::Typed(pat) => {
            key_types.push((*pat.ty).clone());
            let p = &pat.pat;
            keys.push(parse_quote!(#p));
        }
        _ => unimplemented!(),
    });

    let key_type: Type = parse_quote! {(#(#key_types),*)};
    let ret_type: Type = match &input.sig.output {
        ReturnType::Type(_, ty) => (**ty).clone(),
        _ => panic!("required: return type"),
    };
    let memo_type: Type = parse_quote!(std::collections::HashMap<#key_type, #ret_type>);
    let memo_name = format_ident!("{}_MEMO", format!("{}", input.sig.ident).to_uppercase());

    let fn_sig = input.sig;
    let fn_block = input.block;

    let expanded = quote! {
            thread_local!(
                static #memo_name: std::cell::RefCell<#memo_type> =
                    std::cell::RefCell::new(std::collections::HashMap::new())
            );

            #fn_sig {
                if let Some(ret) = #memo_name.with(|memo| memo.borrow().get(&(#(#keys),*)).cloned()) {
                    return ret.clone();
                }
                let ret: #ret_type = (||#fn_block)();
                #memo_name.with(|memo| {
                    memo.borrow_mut().insert((#(#keys),*), ret.clone());
                });
                ret
            }
    };

    expanded.into()
}
