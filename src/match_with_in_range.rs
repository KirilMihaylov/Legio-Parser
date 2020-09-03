use {
    crate::{Match, MatchWith, SuccessfulMatch},
    core::ops::{Index, RangeFrom, RangeTo},
};

mod private {
    pub trait Length<N> {
        fn length(&self) -> N;
    }

    impl<N, E> Length<N> for [E]
    where
        usize: Into<N>,
    {
        fn length(&self) -> N {
            self.len().into()
        }
    }

    impl<N> Length<N> for str
    where
        usize: Into<N>,
    {
        fn length(&self) -> N {
            self.len().into()
        }
    }
}

use private::Length;

/// Provides interface for matching single "dynamic" pattern .
/// This is a counter part of [`MatchStatic`].
///
/// [`MatchStatic`]: trait.MatchStatic.html
pub trait MatchWithInRange<'object, E, N, F, R> {
    /// Matches a "dynamic" pattern by taking a function instead with taking into account a minimum amount.
    fn match_min_with(&'object self, minimum: N, pattern: F) -> Match<R>;

    /// Matches a "dynamic" pattern by taking a function instead with taking into account a maximum amount.
    fn match_max_with(&'object self, maximum: N, pattern: F) -> SuccessfulMatch<R>;

    /// Matches a "dynamic" pattern by taking a function instead with taking into account a minimum and maximum amount.
    fn match_min_max_with(&'object self, minimum: N, maximum: N, pattern: F) -> Match<R>;

    /// Matches a "dynamic" pattern by taking a function instead with taking into account a exact amount.
    fn match_exact_with(&'object self, count: N, pattern: F) -> Match<R>;
}

impl<'object, E, N, F, I> MatchWithInRange<'object, E, N, F, &'object Self> for I
where
    Self: MatchWith<'object, E, F, &'object Self>,
    N: PartialOrd + Clone,
    I: Index<RangeFrom<N>, Output = Self> + Index<RangeTo<N>, Output = Self> + Length<N> + ?Sized,
{
    fn match_min_with(&'object self, minimum: N, pattern: F) -> Match<&'object Self> {
        if self.length() < minimum {
            return Match::failed();
        }

        let matched: SuccessfulMatch<&'object Self> = self.match_with(pattern);

        if minimum <= matched.matched().length() {
            matched.into()
        } else {
            Match::failed()
        }
    }

    fn match_max_with(&'object self, maximum: N, pattern: F) -> SuccessfulMatch<&'object Self> {
        if maximum <= self.length() {
            SuccessfulMatch::new(
                0,
                self[..maximum.clone()].match_with(pattern).take().1,
                &self[maximum..],
            )
        } else {
            self.match_with(pattern)
        }
    }

    fn match_min_max_with(
        &'object self,
        minimum: N,
        maximum: N,
        pattern: F,
    ) -> Match<&'object Self> {
        if maximum < minimum {
            return Match::failed();
        }

        if self.length() < minimum {
            return Match::failed();
        }

        if self.length() <= maximum {
            return self.match_min_with(minimum, pattern);
        }

        if let Ok((_, matched, _)) = self[..maximum].match_min_with(minimum, pattern).take() {
            SuccessfulMatch::new(0, matched, &self[matched.length()..]).into()
        } else {
            Match::failed()
        }
    }

    fn match_exact_with(&'object self, count: N, pattern: F) -> Match<&'object Self> {
        if self.length() < count {
            return Match::failed();
        }

        if let Ok((_, matched, _)) = self[..count.clone()].match_min_with(count, pattern).take() {
            SuccessfulMatch::new(0, matched, &self[matched.length()..]).into()
        } else {
            Match::failed()
        }
    }
}