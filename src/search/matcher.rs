use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use std::cmp::Reverse;

#[derive(Default)]
pub struct Matcher {
    inner: SkimMatcherV2,
}

impl Matcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn score(&self, pattern: &str, candidate: &str) -> Option<i64> {
        self.inner.fuzzy_match(candidate, pattern)
    }

    pub fn rank<'a>(&self, pattern: &str, candidates: &'a [String]) -> Vec<&'a String> {
        let mut scored: Vec<_> = candidates
            .iter()
            .filter_map(|c| self.score(pattern, c).map(|s| (c, s)))
            .collect();

        scored.sort_by_key(|&(_, score)| Reverse(score));
        scored.into_iter().map(|(c, _)| c).collect()
    }
}
