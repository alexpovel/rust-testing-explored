//! Copied from https: //github.com/alexpovel/srgn/tree/06b17c10e4be0a12acae74a6f3b00cf7e5960414
use itertools::Itertools;
use proptest_derive::Arbitrary;
use std::{env, fmt::Debug, ops::Range, slice::Iter};

/// A collection of [`Range`]s.
///
/// This type implements a couple utility functions to work with collections of ranges.
#[derive(Debug, Clone, PartialEq, Eq, Default, Arbitrary)]
pub struct Ranges<Idx: Ord + Copy + Debug> {
    inner: Vec<Range<Idx>>,
    /// Indicates whether merging has occurred.
    ///
    /// Used only to inject a bug for demo purposes.
    merged: bool,
}

impl<Idx: Ord + Copy + Debug> Ranges<Idx> {
    /// Returns the number of ranges.
    #[must_use]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns whether the collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Merges, such that overlapping or bordering ranges are collapsed, and the number
    /// of individual elements is minimized.
    fn merge(&mut self) -> &mut Self {
        debug_assert!(self.is_sorted(), "Merging relies on sorted ranges");

        // We're potentially removing elements, so building up from a new allocation is
        // easiest.
        let capacity = self.len();
        let mut res = Vec::with_capacity(capacity);

        let mut previous: Option<Range<Idx>> = None;
        for current in {
            // Reborrow, but can drop mutability for the iteration.
            &*self
        } {
            match previous {
                Some(prev_range) => {
                    let overlaps = prev_range.end > current.start;
                    let borders = prev_range.end == current.start;
                    if overlaps || borders {
                        let start = prev_range.start;
                        let end = prev_range.end.max(current.end);

                        // Build it up. Don't push yet: there might be an unknown number
                        // of elements more to merge.
                        previous = Some(start..end);
                    } else {
                        if !self.merged {
                            // ⚠️ Purposeful fault injection
                            res.push(prev_range);
                        }
                        previous = Some(current.to_owned());
                    }
                }
                None => previous = Some(current.to_owned()),
            }
        }

        if let Some(prev_range) = previous {
            // Potentially dangling element.
            res.push(prev_range);
        }

        assert!(
            // Cheap enough to do at runtime.
            res.len() <= capacity,
            "Merging should not increase the number of ranges"
        );

        self.inner = res;
        self.inner.shrink_to_fit(); // Might have removed elements, so yield memory back

        self.merged = env::var("FAIL").is_ok(); // ⚠️ Triggers bug, caught by property testing
        self
    }

    fn is_sorted(&self) -> bool {
        self.inner.windows(2).all(|w| w[0].start <= w[1].start)
    }
}

impl<'a, Idx: Ord + Copy + Debug> IntoIterator for &'a Ranges<Idx> {
    type Item = &'a Range<Idx>;
    type IntoIter = Iter<'a, Range<Idx>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<Idx: Ord + Copy + Debug> FromIterator<Range<Idx>> for Ranges<Idx> {
    fn from_iter<I: IntoIterator<Item = Range<Idx>>>(iter: I) -> Self {
        let mut inner = iter.into_iter().collect_vec();
        inner.sort_by_key(|r| r.start);

        Self {
            inner,
            merged: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10_000))]
        #[test]
        fn test_ranges_merging_twice_is_idempotent(
            mut ranges in any::<Vec<Range<usize>>>().prop_map(Ranges::from_iter),
        ) {
            ranges.merge();
            let after_first = ranges.clone();
            ranges.merge();
            let after_second = ranges;

            prop_assert_eq!(after_first, after_second);
        }
    }
}
