use colorz::Colorize;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Score {
    Min,
    BlackMateIn(u16),
    Raw(i32),
    WhiteMateIn(u16),
    Max,
}

impl core::fmt::Debug for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.sign_plus() {
            return match self {
                Self::Min => write!(f, "Min"),
                Self::BlackMateIn(arg0) => f.debug_tuple("BlackMateIn").field(arg0).finish(),
                Self::Raw(arg0) => {
                    if *arg0 == 0 {
                        write!(f, "Raw(0)")
                    } else {
                        f.debug_tuple("Raw").field(arg0).finish()
                    }
                }
                Self::WhiteMateIn(arg0) => f.debug_tuple("WhiteMateIn").field(arg0).finish(),
                Self::Max => write!(f, "Max"),
            };
        }

        write!(f, "{:+?}", self.bright_yellow())
    }
}

const _: [(); core::mem::size_of::<Score>()] = [(); 8];

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

    assert!(Score::Raw(i32::MAX) < Score::WhiteMateIn(3));
    assert!(Score::Raw(100) > Score::Raw(3));

    assert!(Score::Raw(i32::MIN) > Score::BlackMateIn(3));
    assert!(Score::Raw(-100) < Score::Raw(-3));
}
