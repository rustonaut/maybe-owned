//! This crate only provides the `MaybeOwned` enum
//!
//! Take a look at it's documentation for more information.
//!
#![warn(missing_docs)]
use std::convert::From;
use std::clone::Clone;
use std::ops::Deref;
use std::default::Default;


use self::MaybeOwned::*;

/// This type provides a way to store data to which you either have a
/// reference to or which you do own.
///
/// It provides `From<T>`, `From<&'a T>` implementations and, in difference
/// to `Cow` does _not_ require `ToOwned` to be implemented which makes it
/// compatible with non cloneable data, as a draw back of this it does not
/// know that `&str` is the borrowed form of `String` and therefor you can
/// not pass a `&str` as borrowed version (you would have to use `&String`).
///
/// The main benefit lies in the ability to write API functions which accept
/// a generic parameter `E: Into<MaybeOwned<'a, T>>` as the API consumer can
/// pass `T`, `&'a T` and `MaybeOwned<'a, T>` as argument, without requiring
/// a explicite `Cow::Onwed` or a split into two functions one accepting
/// owed and the other borrowed values.
///
/// # Examples
///
/// ```
/// # use maybe_owned::MaybeOwned;
/// struct PseudoBigData(u8);
/// fn pseudo_register_fn<'a, E>(_val: E) where E: Into<MaybeOwned<'a, PseudoBigData>> { }
///
/// let data = PseudoBigData(12);
/// let data2 = PseudoBigData(13);
///
/// pseudo_register_fn(&data);
/// pseudo_register_fn(&data);
/// pseudo_register_fn(data2);
/// pseudo_register_fn(MaybeOwned::Owned(PseudoBigData(111)));
/// ```
///
/// ```
/// # use maybe_owned::MaybeOwned;
/// #[repr(C)]
/// struct OpaqueFFI {
///     ref1:  * const u8
///     //we also might want to have PhantomData etc.
/// }
///
/// // does not work as it does not implement `ToOwned`
/// // let _ = Cow::Owned(OpaqueFFI { ref1: 0 as *const u8});
///
/// // ok, MaybeOwned can do this (but can't do &str<->String as tread of)
/// let _ = MaybeOwned::Owned(OpaqueFFI { ref1: 0 as *const u8 });
/// ```
#[derive(Debug)]
pub enum MaybeOwned<'a, T: 'a> {
    /// owns T
    Owned(T),
    /// has a reference to T
    Borrowed(&'a T)
}

impl<'a, T> MaybeOwned<'a, T> {

    /// returns true if the data is owned else false
    pub fn is_owned(&self) -> bool {
        match *self {
            Owned(_) => true,
            Borrowed(_) => false
        }
    }
}

impl<'a, T> Deref for MaybeOwned<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        match *self {
            Owned(ref v) => v,
            Borrowed(v) => v
        }
    }
}

impl<'a, T> From<&'a T> for MaybeOwned<'a, T> {
    fn from(v: &'a T) -> MaybeOwned<'a, T> {
        Borrowed(v)
    }
}


impl<'a, T> From<T> for MaybeOwned<'a, T> {
    fn from(v: T) -> MaybeOwned<'a, T> {
        Owned(v)
    }
}


impl<'a, T> Default for MaybeOwned<'a, T> where T: Default {
    fn default() -> Self {
        Owned(T::default())
    }
}


impl<'a, T> Clone for MaybeOwned<'a, T> where T: Clone {
    fn clone(&self) -> MaybeOwned<'a, T> {
        match *self {
            Owned(ref v) => Owned(v.clone()),
            Borrowed(v) => Borrowed(v)
        }
    }
}


impl<'a, T> MaybeOwned<'a, T> where T: Clone {

    /// Aquires a mutable reference to owned data.
    ///
    /// Clones data if it is not already owned.
    ///
    /// ## Example
    ///
    /// ```
    /// use maybe_owned::MaybeOwned;
    ///
    /// #[derive(Clone, Debug, PartialEq, Eq)]
    /// struct PseudoBigData(u8);
    ///
    /// let data = PseudoBigData(12);
    ///
    /// let mut maybe: MaybeOwned<PseudoBigData> = (&data).into();
    /// assert_eq!(false, maybe.is_owned());
    ///
    /// {
    ///     let reference = maybe.to_mut();
    ///     assert_eq!(&mut PseudoBigData(12), reference);
    /// }
    /// assert!(maybe.is_owned());
    /// ```
    ///
    pub fn to_mut(&mut self) -> &mut T {
        match *self {
            Owned(ref mut v) => v,
            Borrowed(v) => {
                *self = Owned(v.clone());
                match *self {
                    Owned(ref mut v) => v,
                    Borrowed(..) => unreachable!()
                }
            }

        }
    }

    /// Extracts the owned data.
    ///
    /// If the data is borrowed it is cloned before being extracted.
    pub fn into_owned(self) -> T {
        match self {
            Owned(v) => v,
            Borrowed(v) => v.clone()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::MaybeOwned;

    type TestType = Vec<()>;

    fn with_into<'a, I: Into<MaybeOwned<'a, TestType>>>(v: I) -> MaybeOwned<'a, TestType> {
        v.into()
    }

    #[test]
    fn is_owned() {
        let data = TestType::default();
        assert!(MaybeOwned::Owned(data).is_owned());
    }

    #[test]
    fn into_with_owned() {
        //ty check if it accepts references
        let data = TestType::default();
        assert!(with_into(data).is_owned())

    }
    #[test]
    fn into_with_borrow() {
        //ty check if it accepts references
        let data = TestType::default();
        assert!(!with_into(&data).is_owned());
    }

    #[test]
    fn clone_owned() {
        let maybe = MaybeOwned::<TestType>::default();
        assert!(maybe.clone().is_owned());
    }

    #[test]
    fn clone_borrow() {
        let data = TestType::default();
        let maybe: MaybeOwned<TestType> = (&data).into();
        assert!(!maybe.clone().is_owned());
    }

    #[test]
    fn to_mut() {
        let data = TestType::default();
        let mut maybe: MaybeOwned<TestType> = (&data).into();
        assert!(!maybe.is_owned());
        {
            let _mut_ref = maybe.to_mut();
        }
        assert!(maybe.is_owned());
    }

    #[test]
    fn into_inner() {
        let data = vec![1u32,2];
        let maybe: MaybeOwned<Vec<u32>> = (&data).into();
        assert_eq!(data, maybe.into_owned());
    }

}