use core::hash::{Hash, Hasher};

pub(crate) trait Buf {
    type Item;
    fn get(&self, idx: usize) -> &Self::Item;
    fn get_mut(&mut self, idx: usize) -> &mut Self::Item;
}

impl<T> Buf for [T; 2] {
    type Item = T;
    fn get(&self, idx: usize) -> &T { &self[idx] }
    fn get_mut(&mut self, idx: usize) -> &mut T { &mut self[idx] }
}

impl<T> Buf for Box<[T; 2]> {
    type Item = T;
    fn get(&self, idx: usize) -> &T { &(**self)[idx] }
    fn get_mut(&mut self, idx: usize) -> &mut T { &mut (**self)[idx] }
}

#[derive(Clone, Copy, Default)]
pub(crate) struct RingPairInner<S> {
    pub(crate) buffer: S,
    pub(crate) newest: bool,
}

impl<S: Buf> RingPairInner<S> {
    pub(crate) fn newer(&self) -> &S::Item {
        self.buffer.get(usize::from(self.newest))
    }

    pub(crate) fn older(&self) -> &S::Item {
        self.buffer.get(usize::from(!self.newest))
    }

    pub(crate) fn push(&mut self, value: S::Item) {
        self.newest = !self.newest;
        *self.buffer.get_mut(usize::from(self.newest)) = value;
    }

    pub(crate) fn push_with<F: FnOnce(&mut S::Item)>(&mut self, f: F) {
        self.newest = !self.newest;
        f(self.buffer.get_mut(usize::from(self.newest)));
    }
}


impl<S: Buf> PartialEq for RingPairInner<S>
where
    S::Item: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.older() == other.older() && self.newer() == other.newer()
    }
}

impl<S: Buf> Eq for RingPairInner<S> where S::Item: Eq {}

impl<S: Buf> Hash for RingPairInner<S>
where
    S::Item: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.older().hash(state);
        self.newer().hash(state);
    }
}
