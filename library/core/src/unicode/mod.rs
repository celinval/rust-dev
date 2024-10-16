#![unstable(feature = "unicode_internals", issue = "none")]
#![allow(missing_docs)]

// for use in alloc, not re-exported in std.
#[rustfmt::skip]
pub use unicode_data::case_ignorable::lookup as Case_Ignorable;
pub use unicode_data::cased::lookup as Cased;
pub use unicode_data::conversions;

#[rustfmt::skip]
pub(crate) use unicode_data::alphabetic::lookup as Alphabetic;
pub(crate) use unicode_data::cc::lookup as Cc;
pub(crate) use unicode_data::grapheme_extend::lookup as Grapheme_Extend;
pub(crate) use unicode_data::lowercase::lookup as Lowercase;
pub(crate) use unicode_data::n::lookup as N;
pub(crate) use unicode_data::uppercase::lookup as Uppercase;
pub(crate) use unicode_data::white_space::lookup as White_Space;

pub(crate) mod printable;
mod unicode_data;

/// The version of [Unicode](https://www.unicode.org/) that the Unicode parts of
/// `char` and `str` methods are based on.
///
/// New versions of Unicode are released regularly and subsequently all methods
/// in the standard library depending on Unicode are updated. Therefore the
/// behavior of some `char` and `str` methods and the value of this constant
/// changes over time. This is *not* considered to be a breaking change.
///
/// The version numbering scheme is explained in
/// [Unicode 11.0 or later, Section 3.1 Versions of the Unicode Standard](https://www.unicode.org/versions/Unicode11.0.0/ch03.pdf#page=4).
#[stable(feature = "unicode_version", since = "1.45.0")]
pub const UNICODE_VERSION: (u8, u8, u8) = unicode_data::UNICODE_VERSION;

#[cfg(kani)]
mod verify {
    use super::conversions::{to_upper, to_lower};
    use wrappers::*;
    use crate::kani;
    use safety::*;

    /// Wrapper functions used to verify auto-generated functions from this module.
    ///
    /// The files in this module are auto-generated by a script, so they are harder to annotate.
    /// Instead, for each function that we want to verify, we create a wrapper function with
    /// contracts.
    mod wrappers {
        use super::*;
        use crate::ub_checks;

        /// Wraps `conversions::to_upper` function.
        ///
        /// ```no_run
        /// pub fn to_upper(c: char) -> [char; 3] {
        /// # todo!()
        /// }
        /// ```
        #[ensures(|res| ub_checks::can_dereference(res))]
        pub fn to_upper_wrapper(c: char) -> [char; 3] {
            to_upper(c)
        }

        /// Wraps `conversions::to_lower` function.
        ///
        /// ```no_run
        /// pub fn to_lower(c: char) -> [char; 3] {
        /// # todo!()
        /// }
        /// ```
        #[ensures(|res| ub_checks::can_dereference(res))]
        pub fn to_lower_wrapper(c: char) -> [char; 3] {
            to_lower(c)
        }
    }

    #[kani::proof_for_contract(to_upper_wrapper)]
    fn check_to_upper_safety() {
        let _ = to_upper_wrapper(kani::any());
    }

    #[kani::proof_for_contract(to_lower_wrapper)]
    fn check_to_lower_safety() {
        let _ = to_lower_wrapper(kani::any());
    }
}
