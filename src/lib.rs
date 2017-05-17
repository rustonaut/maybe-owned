//TODO move to external crate
use std::convert::From;
use std::clone::Clone;
use std::ops::Deref;
use std::default::Default;


use self::MaybeOwned::*;

#[derive(Debug)]
pub enum MaybeOwned<'a, T: 'a> {
    Owned(T),
    Borrowed(&'a T)
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



impl<'a, T> Clone for MaybeOwned<'a, T> where T: Clone {
    fn clone(&self) -> MaybeOwned<'a, T> {
        match *self {
            Owned(ref v) => Owned(v.clone()),
            Borrowed(v) => Borrowed(v)
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


impl<'a, T> Default for MaybeOwned<'a, T> where T: Default {
    fn default() -> Self {
        Owned(T::default())
    }
}

impl<'a, T> MaybeOwned<'a, T> {
    pub fn is_owned(&self) -> bool {
        match *self {
            Owned(_) => true,
            Borrowed(_) => false
        }
    }
}

impl<'a, T> MaybeOwned<'a, T> where T: Clone {
    pub fn into_inner(self) -> T {
        match self {
            Owned(v) => v,
            Borrowed(v) => v.clone()
        }
    }
    pub fn into_owning(self) -> MaybeOwned<'a, T> {
        match self {
            Owned(v) => Owned(v),
            Borrowed(v) => Owned(v.clone())
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
    fn into_owning() {
        let data = TestType::default();
        let maybe: MaybeOwned<TestType> = (&data).into();
        assert!(!maybe.is_owned());

        let maybe = maybe.into_owning();
        assert!(maybe.is_owned());
    }

    #[test]
    fn into_inner() {
        let data = vec![1u32,2];
        let maybe: MaybeOwned<Vec<u32>> = (&data).into();
        assert_eq!(data, maybe.into_inner());
    }


}