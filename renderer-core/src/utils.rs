// std::borrow::Cow has too many type restrictions to use instead of this.
// There's probably something in the std library that does the same thing tho?
pub enum BorrowedOrOwned<'a, T> {
    Owned(T),
    Borrowed(&'a T),
}

impl<'a, T> BorrowedOrOwned<'a, T> {
    pub fn get(&'a self) -> &'a T {
        match self {
            Self::Owned(value) => value,
            Self::Borrowed(reference) => reference,
        }
    }
}
