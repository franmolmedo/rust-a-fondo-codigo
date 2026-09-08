//! Equivalent lookup strategies and a UTF-8 debugging exercise.

use std::collections::HashSet;

/// Return up to `max_scalars` Unicode scalar values, not grapheme clusters.
pub fn preview(text: &str, max_scalars: usize) -> &str {
    let end = text
        .char_indices()
        .nth(max_scalars)
        .map_or(text.len(), |(offset, _)| offset);
    &text[..end]
}

pub fn count_linear(values: &[u32], queries: &[u32]) -> usize {
    queries
        .iter()
        .filter(|query| values.contains(query))
        .count()
}

pub fn count_hashed(values: &HashSet<u32>, queries: &[u32]) -> usize {
    queries
        .iter()
        .filter(|query| values.contains(query))
        .count()
}

pub struct SortedIndex(Vec<u32>);

impl SortedIndex {
    pub fn new(values: &[u32]) -> Self {
        let mut sorted = values.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        Self(sorted)
    }

    pub fn count(&self, queries: &[u32]) -> usize {
        queries
            .iter()
            .filter(|query| self.0.binary_search(query).is_ok())
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preview_uses_utf8_boundaries() {
        assert_eq!(preview("", 8), "");
        assert_eq!(preview("Rust", 0), "");
        assert_eq!(preview("Rust", 2), "Ru");
        assert_eq!(preview("a€z", 2), "a€");
        assert_eq!(preview("東京", 1), "東");
        assert_eq!(preview("Rust", usize::MAX), "Rust");
    }

    #[test]
    fn preview_does_not_promise_grapheme_segmentation() {
        assert_eq!(preview("e\u{301}", 1), "e");
        assert_eq!(preview("e\u{301}", 2), "e\u{301}");
    }

    #[test]
    fn all_strategies_count_queries_not_duplicate_values() {
        for values in [vec![], vec![3], vec![8, 3, 3, u32::MAX, 0]] {
            let queries = [3, 3, 1, 0, u32::MAX];
            let expected = count_linear(&values, &queries);
            assert_eq!(SortedIndex::new(&values).count(&queries), expected);
            assert_eq!(
                count_hashed(&values.into_iter().collect(), &queries),
                expected
            );
        }
    }

    #[test]
    fn empty_queries_have_no_hits() {
        assert_eq!(count_linear(&[1], &[]), 0);
        assert_eq!(SortedIndex::new(&[1]).count(&[]), 0);
        assert_eq!(count_hashed(&HashSet::from([1]), &[]), 0);
    }

    #[test]
    fn strategies_agree_over_many_small_inputs() {
        for mask in 0_u32..256 {
            let values: Vec<_> = (0..8).filter(|bit| mask & (1 << bit) != 0).collect();
            let queries: Vec<_> = (0..12).rev().collect();
            let expected = count_linear(&values, &queries);
            assert_eq!(SortedIndex::new(&values).count(&queries), expected);
            assert_eq!(
                count_hashed(&values.into_iter().collect(), &queries),
                expected
            );
        }
    }
}
