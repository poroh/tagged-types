// SPDX-License-Identifier: MIT

use crate::ImplementEq;
use crate::ImplementOrd;
use crate::ImplementPartialEq;
use crate::ImplementPartialOrd;
use crate::TaggedType;
use std::cmp::Ordering;

impl<V, T> PartialEq for TaggedType<V, T>
where
    V: PartialEq,
    T: ImplementPartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.v.eq(&other.v)
    }
}

impl<V, T> Eq for TaggedType<V, T>
where
    V: Eq,
    T: ImplementEq + ImplementPartialEq,
{
}

impl<V, T> PartialOrd for TaggedType<V, T>
where
    V: PartialOrd,
    T: ImplementPartialOrd + ImplementPartialEq,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.v.partial_cmp(&other.v)
    }
}

impl<V, T> Ord for TaggedType<V, T>
where
    V: Ord,
    T: ImplementOrd + ImplementPartialOrd + ImplementPartialEq + ImplementEq,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.v.cmp(&other.v)
    }
}
