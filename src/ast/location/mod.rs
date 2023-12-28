use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

pub mod position;
pub use position::Position;

#[derive(Copy, Clone, Debug, Eq, Serialize, Deserialize)]
pub struct Locatable<T> {
    inner: T,
    position: Position,
}

impl<T> Locatable<T> {
    pub fn new(inner: T, position: Position) -> Self {
        Self { inner, position }
    }

    /// Maps a `Locatable<T>` to `Locatable<U>` by applying a function to the underlying element.
    /// Useful when upleveling the element (such as wrapping a Header1) while the position remains
    /// unchanged.
    #[inline]
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Locatable<U> {
        Locatable::new(f(self.inner), self.position)
    }

    /// Converts from `&Locatable<T>` to `Locatable<&T>`
    pub fn as_ref(&self) -> Locatable<&T> {
        Locatable {
            inner: &self.inner,
            position: self.position,
        }
    }

    /// Converts from `&mut Locatable<T>` to `Locatable<&mut T>`
    pub fn as_mut(&mut self) -> Locatable<&mut T> {
        Locatable {
            inner: &mut self.inner,
            position: self.position,
        }
    }

    /// Converts from `&Locatable<T>` to `&T`
    pub fn as_inner(&self) -> &T {
        &self.inner
    }

    /// Converts from `&mut Locatable<T>` to `&mut T`
    pub fn as_mut_inner(&mut self) -> &mut T {
        &mut self.inner
    }

    /// Converts from `Locatable<T>` to `T`
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Returns a copy of the position associated with the inner value
    pub fn position(&self) -> Position {
        self.position
    }
}

impl<T> Locatable<Option<T>> {
    /// Transposes a `Locatable` of an [`Option`] into an [`Option`] of a `Locatable`.
    pub fn transpose(self) -> Option<Locatable<T>> {
        let position = self.position();
        match self.into_inner() {
            Some(inner) => Some(Locatable::new(inner, position)),
            _ => None,
        }
    }
}

impl<T: PartialEq> PartialEq for Locatable<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T: PartialEq> PartialEq<T> for Locatable<T> {
    fn eq(&self, other: &T) -> bool {
        &self.inner == other
    }
}

impl<T: Hash> Hash for Locatable<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl<T> From<T> for Locatable<T> {
    /// Creates around `T`, using a default position
    fn from(t: T) -> Self {
        Self::new(t, Default::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::location::position::Point;
    use std::collections::HashSet;

    #[test]
    fn map_should_transform_inner_value_and_keep_position() {
        let le = Locatable::new(3, Position::new(Point::new(1, 1, 0), Point::new(1, 4, 3)));
        let mapped_le = le.map(|c| c + 1);
        assert_eq!(*mapped_le.as_inner(), 4);
        assert_eq!(
            mapped_le.position(),
            Position::new(Point::new(1, 1, 0), Point::new(1, 4, 3),)
        );
    }

    #[test]
    fn equality_with_other_should_only_use_inner_value() {
        let le1 = Locatable::new(3, Position::new(Point::new(1, 1, 0), Point::new(1, 4, 3)));
        let le2 = Locatable::new(3, Position::default());
        assert_eq!(le1, le2);
    }

    #[test]
    fn equality_with_inner_type_should_only_use_inner_value() {
        let le = Locatable::new(3, Position::new(Point::new(1, 1, 0), Point::new(1, 4, 3)));
        let inner = 3;
        assert_eq!(le, inner);
        assert_ne!(le, inner + 1);
    }

    #[test]
    fn hashing_should_only_use_inner_value() {
        let le1 = Locatable::new(3, Position::new(Point::new(1, 1, 0), Point::new(1, 4, 3)));
        let le2 = Locatable::new(3, Position::default());
        let le3 = Locatable::new(3, Position::new(Point::new(1, 1, 0), Point::new(1, 4, 3)));
        let le4 = Locatable::new(3, Position::new(Point::new(1, 1, 0), Point::new(1, 4, 3)));

        let mut m = HashSet::new();
        m.insert(le1);

        let le = m
            .get(&le2)
            .expect("Failed to retrieve Located with another Located");
        assert_eq!(*le.as_inner(), 3);
        assert_eq!(
            le.position(),
            Position::new(Point::new(1, 1, 0), Point::new(1, 4, 3),)
        );

        assert_eq!(m.get(&le3), None);

        let le = m
            .get(&le4)
            .expect("Failed to retrieve Located with another Located");
        assert_eq!(*le.as_inner(), 3);
        assert_eq!(
            le.position(),
            Position::new(Point::new(1, 1, 0), Point::new(1, 4, 3),)
        );
    }

    #[test]
    fn as_ref_should_return_new_element_with_ref_and_same_position() {
        #[derive(Debug, PartialEq, Eq)]
        struct Test(usize);

        let le = Locatable::new(
            Test(5),
            Position::new(Point::new(1, 1, 0), Point::new(1, 4, 3)),
        );
        let le_ref = le.as_ref();

        assert_eq!(le_ref.inner, &Test(5));
        assert_eq!(
            le_ref.position(),
            Position::new(Point::new(1, 1, 0), Point::new(1, 4, 3),)
        );
    }

    #[test]
    fn as_mut_should_return_new_element_with_mut_and_same_position() {
        #[derive(Debug, PartialEq, Eq)]
        struct Test(usize);

        let mut le = Locatable::new(
            Test(5),
            Position::new(Point::new(1, 1, 0), Point::new(1, 4, 3)),
        );
        let le_mut = le.as_mut();

        assert_eq!(le_mut.inner, &mut Test(5));
        assert_eq!(
            le_mut.position(),
            Position::new(Point::new(1, 1, 0), Point::new(1, 4, 3),)
        );
    }

    #[test]
    fn as_inner_should_return_new_element_with_ref_to_inner_and_same_position() {
        #[derive(Debug, PartialEq, Eq)]
        struct Test(usize);

        let le = Locatable::new(
            Test(5),
            Position::new(Point::new(1, 1, 0), Point::new(1, 4, 3)),
        );
        let inner = le.as_inner();

        assert_eq!(inner, &Test(5));
    }

    #[test]
    fn as_mut_inner_should_return_new_element_with_mut_ref_to_inner_and_same_position() {
        #[derive(Debug, PartialEq, Eq)]
        struct Test(usize);

        let mut le = Locatable::new(
            Test(5),
            Position::new(Point::new(1, 1, 0), Point::new(1, 4, 3)),
        );
        let inner = le.as_mut_inner();

        assert_eq!(inner, &mut Test(5));
    }

    #[test]
    fn into_inner_should_return_inner_value_as_owned() {
        #[derive(Debug, PartialEq, Eq)]
        struct Test(usize);

        let le = Located::new(
            Test(5),
            Position::new(Point::new(1, 1, 0), Point::new(1, 4, 3)),
        );
        let inner = le.into_inner();

        assert_eq!(inner, Test(5));
    }
}
