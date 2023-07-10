#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Score {
    Min,
    BlackMateIn(u16),
    Raw(i16),
    WhiteMateIn(u16),
    Max,
}

const _: [(); core::mem::size_of::<Score>()] = [(); 4];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScoreKind {
    Min,
    BlackMateIn,
    Raw,
    WhiteMateIn,
    Max,
}

impl Score {
    #[inline]
    pub fn kind(&self) -> ScoreKind {
        match self {
            Score::Min => ScoreKind::Min,
            Score::BlackMateIn(_) => ScoreKind::BlackMateIn,
            Score::Raw(_) => ScoreKind::Raw,
            Score::WhiteMateIn(_) => ScoreKind::WhiteMateIn,
            Score::Max => ScoreKind::Max,
        }
    }
}

impl PartialOrd for Score {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Score {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Smaller scores mean better for black and larger scores mean better for white
        // keep track of best mate distance for better pruning and to not get stuck finding
        // a mate always 4 moves away

        match (self, other) {
            (Score::BlackMateIn(x), Score::BlackMateIn(y)) => x.cmp(y),
            (Score::Raw(x), Score::Raw(y)) => x.cmp(y),
            (Score::WhiteMateIn(x), Score::WhiteMateIn(y)) => y.cmp(x),
            _ => self.kind().cmp(&other.kind()),
        }
    }
}

#[test]
fn test_score_order() {
    assert!(Score::BlackMateIn(1) < Score::BlackMateIn(3));
    assert!(Score::WhiteMateIn(1) > Score::WhiteMateIn(3));

    assert!(Score::Raw(i16::MAX) < Score::WhiteMateIn(3));
    assert!(Score::Raw(100) > Score::Raw(3));

    assert!(Score::Raw(i16::MIN) > Score::BlackMateIn(3));
    assert!(Score::Raw(-100) < Score::Raw(-3));
}
