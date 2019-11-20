// matux
// Copyright (C) SOFe
//
// Licensed under the Apache License, Version 2.0 (the License);
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an AS IS BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// Asserts that an expression follows a pattern.
///
/// # Example
/// ```
/// enum A {
///     B(&'static str),
///     C(u32),
/// }
///
/// let c = A::C(3);
/// let inner = matux::unwrap!(c, A::C(x) => x);
/// assert_eq!(inner, 3u32);
/// ```
///
/// ```should_panic
/// enum A {
///     B(&'static str),
///     C(u32),
/// }
///
/// let c = A::C(3);
/// matux::unwrap!(c, A::B(x) => x); // Error: "A::B(x) does not match c"
/// ```
#[macro_export]
macro_rules! unwrap {
    ($src:expr, $pat:pat => $proj:expr) => {
        match $src {
            $pat => $proj,
            _ => panic!(concat!(
                stringify!($src),
                " does not match ",
                stringify!($pat),
            )),
        }
    };
}

/// Asserts that an expression follows a pattern; otherwise fail with an error message.
///
/// # Example
/// ```
/// #[derive(Debug)]
/// enum A {
///     B(&'static str),
///     C(u32),
/// }
///
/// let c = A::C(3);
/// let inner = matux::expect!(c, A::C(x) => x, "c is not A::C but {:?}", c);
/// assert_eq!(inner, 3u32);
/// ```
///
/// ```should_panic
/// #[derive(Debug)]
/// enum A {
///     B(&'static str),
///     C(u32),
/// }
///
/// let c = A::C(3);
/// matux::expect!(c, A::B(x) => x, "c is not A::B but {:?}", c); // Panic: "c is not A::B but C(3)"
/// ```
#[macro_export]
macro_rules! expect {
    ($src:expr, $pat:pat => $proj:expr, $msg:literal $(, $args:expr)*) => {
        match $src {
            $pat => $proj,
            _ => panic!($msg, $($args),*),
        }
    };
}
