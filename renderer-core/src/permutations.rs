//! Structures for handling our material and pipeline permutations

#[derive(Default, Debug, Clone)]
pub struct FaceSides<T> {
    pub single: T,
    pub double: T,
}

impl<T> FaceSides<T> {
    pub fn iter_mut(&mut self) -> [&mut T; 2] {
        [&mut self.single, &mut self.double]
    }
}

#[derive(Default, Debug, Clone)]
pub struct BlendMode<T> {
    pub opaque: T,
    pub alpha_clipped: T,
    pub alpha_blended: T,
}

impl<T> BlendMode<T> {
    pub fn iter_mut(&mut self) -> [&mut T; 3] {
        [
            &mut self.opaque,
            &mut self.alpha_clipped,
            &mut self.alpha_blended,
        ]
    }
}

#[derive(Default, Debug, Clone)]
pub struct ModelTypes<T> {
    pub stationary: T,
    pub animated: T,
}

impl<T> ModelTypes<T> {
    pub fn iter_mut(&mut self) -> [&mut T; 2] {
        [&mut self.stationary, &mut self.animated]
    }
}
