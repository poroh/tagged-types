// SPDX-License-Identifier: MIT

use crate::ImplementEq;
use crate::ImplementOrd;
use crate::ImplementPartialEq;
use crate::ImplementPartialOrd;
use crate::TaggedType;
use core::cmp::Ordering;

impl<V: PartialEq, T: ImplementPartialEq> PartialEq for TaggedType<V, T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.v.eq(&other.v)
    }
}

impl<V: Eq, T> Eq for TaggedType<V, T> where T: ImplementEq + ImplementPartialEq {}

impl<V: PartialOrd, T> PartialOrd for TaggedType<V, T>
where
    T: ImplementPartialOrd + ImplementPartialEq,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.v.partial_cmp(&other.v)
    }
}

impl<V: Ord, T> Ord for TaggedType<V, T>
where
    T: ImplementOrd + ImplementPartialOrd + ImplementPartialEq + ImplementEq,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.v.cmp(&other.v)
    }
}
