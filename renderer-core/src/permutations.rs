//! Structures for handling our material and pipeline permutations

#[derive(Default, Debug, Clone)]
pub struct FaceSides<T> {
    pub single: T,
    pub double: T,
}

#[derive(Default, Debug, Clone)]
pub struct BlendMode<T> {
    pub opaque: T,
    pub alpha_clipped: T,
    pub alpha_blended: T,
}

#[derive(Default, Debug, Clone)]
pub struct ModelTypes<T> {
    pub stationary: T,
    pub animated: T,
}
