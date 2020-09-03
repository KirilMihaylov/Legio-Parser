use crate::SuccessfulMatch;

/// Provides interface for matching single "dynamic" pattern.
/// This is a counter part of [`MatchStatic`].
///
/// [`MatchStatic`]: trait.MatchStatic.html
pub trait MatchWith<'object, E, F, R> {
    /// Matches a "dynamic" pattern by taking a function instead.
    fn match_with(&'object self, pattern: F) -> SuccessfulMatch<R>;
}

impl<'object, E, F> MatchWith<'object, E, F, &'object Self> for [E]
where
    E: Clone,
    F: FnMut(E) -> bool,
{
    fn match_with(&'object self, mut pattern: F) -> SuccessfulMatch<&'object Self> {
        for (index, element) in self.iter().enumerate() {
            if !pattern(element.clone()) {
                return SuccessfulMatch::new(0, &self[..index], &self[index..]);
            }
        }

        SuccessfulMatch::new(0, self, &[])
    }
}

impl<'object, E, F> MatchWith<'object, &E, F, &'object Self> for [E]
where
    F: FnMut(&E) -> bool,
{
    fn match_with(&'object self, mut pattern: F) -> SuccessfulMatch<&'object Self> {
        for (index, element) in self.iter().enumerate() {
            if !pattern(element) {
                return SuccessfulMatch::new(0, &self[..index], &self[index..]);
            }
        }

        SuccessfulMatch::new(0, self, &[])
    }
}

impl<'object, F> MatchWith<'object, char, F, &'object Self> for str
where
    F: FnMut(char) -> bool,
{
    fn match_with(&'object self, mut pattern: F) -> SuccessfulMatch<&'object Self> {
        for (index, element) in self.char_indices() {
            if !pattern(element) {
                return SuccessfulMatch::new(0, &self[..index], &self[index..]);
            }
        }

        SuccessfulMatch::new(0, self, "")
    }
}

impl<'object, F> MatchWith<'object, &char, F, &'object Self> for str
where
    F: FnMut(&char) -> bool,
{
    fn match_with(&'object self, mut pattern: F) -> SuccessfulMatch<&'object Self> {
        for (index, element) in self.char_indices() {
            if !pattern(&element) {
                return SuccessfulMatch::new(0, &self[..index], &self[index..]);
            }
        }

        SuccessfulMatch::new(0, self, "")
    }
}