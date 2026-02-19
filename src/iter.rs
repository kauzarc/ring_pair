use core::iter::FusedIterator;

/// Iterator over references to the elements of a ring pair, from oldest to newest.
///
/// Created by calling `.iter()` on a [`RingPair`](crate::RingPair) or
/// [`BoxedRingPair`](crate::BoxedRingPair).
#[derive(Clone)]
pub struct Iter<'a, T> {
    data: [&'a T; 2],
    front: usize,
    back: usize,
}

impl<'a, T> Iter<'a, T> {
    pub(crate) fn new(older: &'a T, newer: &'a T) -> Self {
        Self {
            data: [older, newer],
            front: 0,
            back: 2,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front < self.back {
            let item = self.data[self.front];
            self.front += 1;
            Some(item)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.back - self.front;
        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for Iter<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front < self.back {
            self.back -= 1;
            Some(self.data[self.back])
        } else {
            None
        }
    }
}

impl<T> ExactSizeIterator for Iter<'_, T> {}

impl<T> FusedIterator for Iter<'_, T> {}
