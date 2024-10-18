#![no_std]
#![doc = include_str!("README.md")]

/// Split a string into a pair of first character and the rest.
pub fn split_first_char(text: &str) -> Option<(char, &'_ str)> {
    let mut iter = text.chars();
    let head = iter.next()?;
    let rest = iter.as_str();
    Some((head, rest))
}

/// Split a string into a pair of initial part and the last character.
pub fn split_last_char(text: &str) -> Option<(&'_ str, char)> {
    let mut iter = text.chars();
    let last = iter.next_back()?;
    let init = iter.as_str();
    Some((init, last))
}

mod sealed {
    pub trait Sealed {}
    impl<'a> Sealed for &'a str {}
}

/// Extension trait that provides methods to split character from a `&str`.
pub trait SplitCharFromStr<'a>: sealed::Sealed + Sized {
    /// Split a string into a pair of first character and the rest.
    fn split_first_char(self) -> Option<(char, &'a str)>;
    /// Split a string into a pair of initial part and the last character.
    fn split_last_char(self) -> Option<(&'a str, char)>;
}

impl<'a> SplitCharFromStr<'a> for &'a str {
    fn split_first_char(self) -> Option<(char, &'a str)> {
        split_first_char(self)
    }

    fn split_last_char(self) -> Option<(&'a str, char)> {
        split_last_char(self)
    }
}

pub use split_first_char as first;
pub use split_last_char as last;
pub use SplitCharFromStr as Extension;

#[cfg(test)]
mod test {
    use crate::{split_first_char, split_last_char, SplitCharFromStr};
    use core::ops::Deref;

    #[test]
    fn function() {
        assert_eq!(split_first_char("abc"), Some(('a', "bc")));
        assert_eq!(split_first_char("x"), Some(('x', "")));
        assert_eq!(split_first_char(""), None);
        assert_eq!(split_last_char("abc"), Some(("ab", 'c')));
        assert_eq!(split_last_char("x"), Some(("", 'x')));
        assert_eq!(split_last_char(""), None);
    }

    #[test]
    fn method() {
        assert_eq!("abc".split_first_char(), Some(('a', "bc")));
        assert_eq!("x".split_first_char(), Some(('x', "")));
        assert_eq!("".split_first_char(), None);
        assert_eq!("abc".split_last_char(), Some(("ab", 'c')));
        assert_eq!("x".split_last_char(), Some(("", 'x')));
        assert_eq!("".split_last_char(), None);
    }

    #[test]
    fn lifetime() {
        fn _use_function(text: &str) -> Option<(&'_ str, &'_ str)> {
            let (_, tail) = split_first_char(text)?;
            let (init, _) = split_last_char(text)?;
            Some((tail, init))
        }

        fn _use_method(text: &str) -> Option<(&'_ str, &'_ str)> {
            let (_, tail) = text.split_first_char()?;
            let (init, _) = text.split_last_char()?;
            Some((tail, init))
        }
    }

    #[test]
    fn deref() {
        enum Input {
            None,
            One,
            Many,
        }

        impl Deref for Input {
            type Target = str;
            fn deref(&self) -> &Self::Target {
                match self {
                    Input::None => "",
                    Input::One => "x",
                    Input::Many => "abc",
                }
            }
        }

        assert_eq!(Input::None.split_first_char(), None);
        assert_eq!(Input::One.split_first_char(), Some(('x', "")));
        assert_eq!(Input::Many.split_first_char(), Some(('a', "bc")));
        assert_eq!(Input::None.split_last_char(), None);
        assert_eq!(Input::One.split_last_char(), Some(("", 'x')));
        assert_eq!(Input::Many.split_last_char(), Some(("ab", 'c')));
    }
}
