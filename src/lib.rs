#![no_std]

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
//! Supports up to four arguments.
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
/// Supports up to four arguments.
#[macro_export]
macro_rules! defmac {
    ($name:ident => $e:expr) => {
        macro_rules! $name {
            () => { $e }
        }
    };
    ($name:ident $x:pat => $e:expr) => {
        macro_rules! $name {
            ($arg:expr) => {
                match $arg { $x => $e }
            }
        }
    };
    ($name:ident $x1:pat, $x2:pat => $e:expr) => {
        macro_rules! $name {
            ($a1:expr, $a2:expr) => {
                match $a1 { $x1 =>
                match $a2 { $x2 => $e } }
            }
        }
    };
    ($name:ident $x1:pat, $x2:pat, $x3:pat => $e:expr) => {
        macro_rules! $name {
            ($a1:expr, $a2:expr, $a3:expr) => {
                match $a1 { $x1 =>
                match $a2 { $x2 =>
                match $a3 { $x3 => $e } } }
            }
        }
    };
    ($name:ident $x1:pat, $x2:pat, $x3:pat, $x4:pat => $e:expr) => {
        macro_rules! $name {
            ($a1:expr, $a2:expr, $a3:expr, $a4:expr) => {
                match $a1 { $x1 =>
                match $a2 { $x2 =>
                match $a3 { $x3 =>
                match $a4 { $x4 => $e } } } }
            }
        }
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
    }
}
