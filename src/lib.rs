#![cfg_attr(not(test), no_std)]

//! A macro to define lambda-like macros inline.
//!
//! Syntax:
//!
//! `defmac!(` *name* [ *pattern* [, *pattern* ... ]] `=>` *expression* `)`
//!
//! *name* is the name of the new macro, followed by 0 or more patterns
//! separated by comma. A pattern can be just an argument name like `x`
//! or a pattern like `ref value`, `(x, y)` etc. Note that there is no comma
//! between the name and the first pattern.
//!
//! # Example
//!
//! ```
//! #[macro_use] extern crate defmac;
//!
//! fn main() {
//!     defmac!(mkvec iter => iter.into_iter().collect::<Vec<_>>());
//!
//!     let v = mkvec!((0..10).map(|x| x * 2));
//!
//!     defmac!(repeat ref s, n => (0..n).map(|_| &s[..]).collect::<String>());
//!
//!     let text = String::from("abc");
//!     let s = repeat!(text, 10);
//!     let t = repeat!("-", s.len());
//!     println!("{}", s);
//!     println!("{}", t);
//!
//! }
//! ```
//!
//! Did you know that macros can “capture” variables that they have in scope?
//! The capture is by name instead of by reference, so we can use
//! defmac where we cannot use closures. See the example below:
//!
//! ```
//! #[macro_use] extern crate defmac;
//!
//! fn main() {
//!     let mut result = Vec::new();
//!     let mut sum = 0.;
//!     let input = "2 2 ^ 7 b ^";
//!
//!     defmac!(push elem => result.push(elem));
//!     defmac!(double => *result.last_mut().unwrap() *= 2);
//!
//!     for ch in input.chars() {
//!         match ch {
//!             '^' => double!(),
//!             '0'...'9' => push!(ch as u32 - '0' as u32),
//!             'a'...'z' => push!(ch as u32 - 'a' as u32),
//!             _ => { }
//!         }
//!     }
//!
//!     assert_eq!(
//!         result,
//!         vec![2, 4, 7, 2]);
//! }
//! ```
//!
//! ## Rust Version
//!
//! This crate requires Rust 1.20 or later.

/// A macro to define lambda-like macros inline.
///
/// Syntax:
///
/// `defmac!(` *name* [ *pattern* [, *pattern* ... ]] `=>` *expression* `)`
///
/// *name* is the name of the new macro, followed by 0 or more patterns
/// separated by comma. A pattern can be just an argument name like `x`
/// or a pattern like `ref value`, `(x, y)` etc.
///
/// Supports arbitrary many arguments.
#[macro_export]
macro_rules! defmac {
    // nest matches final rule
    (@nest $name:ident ($dol:tt) => (
        [$($arg:ident)*] $($result_body:tt)+)
    ) => {
        macro_rules! $name {
            ($($dol $arg : expr), *) => {
                $($result_body)+
            }
        }
    };

    // nest matches entry point and recursive rule
    (@nest $name:ident ($dol:tt) => (
            [$($arg:ident)*] $($result_body:tt)+
        )
        $p1:pat $(, $p2:pat)*
    ) => {
        // `marg` is a hygienic macro argument name
        defmac!(@nest $name ($dol) => (
            [marg $($arg)*]
            match {$dol marg} { $p1 => $($result_body)+ }
        )
        $($p2),* )
    };

    // reverse patterns before passing them on to @nest
    // reverse patterns final rule
    (@revpats [$($args:tt)*] [$($pr:pat),*]) => {
        defmac!(@nest $($args)* $($pr),*)
    };

    // reverse patterns entry point and recursive rule
    (@revpats [$($args:tt)*] [$($pr:pat),*] $p1:pat $(, $p2:pat)*) => {
        defmac!(@revpats [$($args)*] [$p1 $(, $pr)*] $($p2),*)
    };

    // entry point
    ($name:ident $($p1:pat),* => $result:expr) => {
        defmac!(@revpats [$name ($) => ([] $result)] [] $($p1),*)
    };
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

        let value = "xyz";

        defmac!(none => value);
        assert_eq!(none!(), "xyz");

        defmac!(one x => x);
        assert_eq!(one!(2), 2);

        defmac!(two x, y => x + y);
        assert_eq!(two!(1., 2.), 3.);

        defmac!(three x, y, z => (x, y, z));
        assert_eq!(three!(1, (2, 3), (4, 5, 6)), (1, (2, 3), (4, 5, 6)));

        defmac!(four w, x, y, z => (w + x, z, y));
        assert_eq!(four!(3, 4, "a", "b"), (7, "b", "a"));

        defmac!(many a, b, c, d, e, f, g, h, i, j, k => (a, b + c, d + e + f,
                                                         g + h + i + j + k));
        assert_eq!(many!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11),
                   (1, 5, 15, 45));
    }

    #[test]
    fn eval_order() {
        use std::cell::Cell;
        let v = Cell::new(0);
        let f = || {
            let n = v.get();
            v.set(n + 1);
            n
        };

        defmac!(two x, y => (x, y));

        let result = two!(f(), f());
        assert_eq!(result, (0, 1));
        assert_eq!(f(), 2);
    }
}
