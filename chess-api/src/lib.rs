use std::path::Path;

pub use abi_stable;

use abi_stable::{
    declare_root_module_statics,
    library::{LibraryError, RootModule},
    package_version_strings, sabi_trait,
    sabi_trait::TD_Opaque,
    sabi_types::VersionStrings,
    std_types::RBox,
    RRef,
};
use chess_bitboard::{Pos, PromotionPiece};
use chess_engine::{Score, Timeout};
pub use chess_movegen::{Board, ChessMove};

#[repr(C)]
#[derive(abi_stable::StableAbi)]
#[sabi(kind(Prefix(prefix_ref = ChessApiRefRaw)))]
#[sabi(missing_field(panic))]
pub struct ChessApi {
    #[sabi(last_prefix_field)]
    _new_engine: extern "C" fn() -> ChessEngineTrait_TO<'static, RBox<()>>,
}

impl ChessApi {
    /// Create a new ChessApi
    ///
    /// # Panics
    ///
    /// This function panics if the new_engine function is non-zero sized
    pub fn new<F, E>(new_engine: F) -> ChessApi
    where
        F: Fn() -> E + Copy,
        E: ChessEngineTrait + 'static,
    {
        assert_eq!(core::mem::size_of_val(&new_engine), 0);

        extern "C" fn new_engine_mk<F, E>() -> ChessEngineTrait_TO<'static, RBox<()>>
        where
            F: Fn() -> E + Copy,
            E: ChessEngineTrait + 'static,
        {
            // SAFETY: we can materialize a F here because it is zero-sized,
            // `Copy`, and we got an instance before calling this function

            let f = unsafe { core::ptr::NonNull::<F>::dangling().as_ptr().read() };
            ChessEngineTrait_TO::from_value(f(), abi_stable::sabi_trait::prelude::TD_Opaque)
        }

        ChessApi {
            _new_engine: new_engine_mk::<F, E>,
        }
    }
}

pub struct ChessApiRef(ChessApiRefRaw);

impl ChessApiRef {
    /// Loads this module from the directory specified by `where_`,
    /// first loading the dynamic library if it wasn't already loaded.
    ///
    /// Once the root module is loaded,
    /// this will return the already loaded root module.
    ///
    /// Warnings and Errors are detailed in [`load_from`](#method.load_from),
    ///
    pub fn load_from_directory(where_: &Path) -> Result<Self, LibraryError> {
        ChessApiRefRaw::load_from_directory(where_).map(Self)
    }

    /// Loads this module from the file at `path_`,
    /// first loading the dynamic library if it wasn't already loaded.
    ///
    /// Once the root module is loaded,
    /// this will return the already loaded root module.
    ///
    /// Warnings and Errors are detailed in [`load_from`](#method.load_from),
    ///
    pub fn load_from_file(path_: &Path) -> Result<Self, LibraryError> {
        ChessApiRefRaw::load_from_file(path_).map(Self)
    }

    #[inline]
    pub fn new_engine(&self) -> ChessEngine {
        ChessEngine {
            bx: self.0._new_engine()(),
        }
    }
}

pub struct ChessEngine {
    bx: ChessEngineTrait_TO<'static, RBox<()>>,
}

impl ChessEngine {
    #[inline]
    pub fn evaluate<T: Timeout>(&mut self, timeout: &T) -> (Option<ChessMove>, Score) {
        let mv = self.bx.evaluate(TimeoutReference::new(timeout));
        (mv.chess_move(), mv.score())
    }

    #[inline]
    pub fn board(&self) -> Board {
        self.bx.board()
    }

    #[inline]
    pub fn set_board(&mut self, board: Board) {
        self.bx.set_board(board)
    }

    #[inline]
    pub fn make_move(&mut self, mv: ChessMove) -> MoveResult {
        self.bx.make_move(StableChessMove::from(mv))
    }
}

#[repr(C)]
#[derive(abi_stable::StableAbi)]
pub struct MoveResult {
    pub is_valid: bool,
    pub is_three_fold_draw: bool,
}

#[sabi_trait]
pub trait ChessEngineTrait {
    fn evaluate(&mut self, timeout: TimeoutReference<'_>) -> EvaluatedMove;

    fn board(&self) -> Board;

    fn set_board(&mut self, board: Board);

    fn make_move(&mut self, mv: StableChessMove) -> MoveResult;
}

impl RootModule for ChessApiRefRaw {
    declare_root_module_statics! {ChessApiRefRaw}
    const BASE_NAME: &'static str = "chess-bot";
    const NAME: &'static str = "chess-bot";
    const VERSION_STRINGS: VersionStrings = package_version_strings!();
}

#[repr(C)]
#[derive(Clone, Copy, abi_stable::StableAbi)]
pub struct StableChessMove {
    source: Pos,
    dest: Pos,
    piece: StableOptionalPromotionPiece,
}

#[repr(C)]
#[derive(Clone, Copy, abi_stable::StableAbi)]
struct StableOptionalChessMove {
    source: Pos,
    dest: Pos,
    piece: StableMaybeIllegalOptionalPromotionPiece,
}

#[repr(u8)]
#[derive(Clone, Copy, abi_stable::StableAbi)]
enum StableOptionalPromotionPiece {
    Knight = PromotionPiece::Knight as u8,
    Bishop = PromotionPiece::Bishop as u8,
    Rook = PromotionPiece::Rook as u8,
    Queen = PromotionPiece::Queen as u8,
    None,
}

#[repr(u8)]
#[derive(Clone, Copy, abi_stable::StableAbi)]
enum StableMaybeIllegalOptionalPromotionPiece {
    Knight = PromotionPiece::Knight as u8,
    Bishop = PromotionPiece::Bishop as u8,
    Rook = PromotionPiece::Rook as u8,
    Queen = PromotionPiece::Queen as u8,
    None,
    Illegal,
}

impl From<StableOptionalChessMove> for Option<ChessMove> {
    #[inline]
    fn from(value: StableOptionalChessMove) -> Self {
        Some(ChessMove {
            source: value.source,
            dest: value.dest,
            piece: match value.piece {
                StableMaybeIllegalOptionalPromotionPiece::Knight => Some(PromotionPiece::Knight),
                StableMaybeIllegalOptionalPromotionPiece::Bishop => Some(PromotionPiece::Bishop),
                StableMaybeIllegalOptionalPromotionPiece::Rook => Some(PromotionPiece::Rook),
                StableMaybeIllegalOptionalPromotionPiece::Queen => Some(PromotionPiece::Queen),
                StableMaybeIllegalOptionalPromotionPiece::None => None,
                StableMaybeIllegalOptionalPromotionPiece::Illegal => return None,
            },
        })
    }
}

impl From<ChessMove> for StableOptionalChessMove {
    #[inline]
    fn from(value: ChessMove) -> Self {
        StableOptionalChessMove {
            source: value.source,
            dest: value.dest,
            piece: match value.piece {
                Some(PromotionPiece::Knight) => StableMaybeIllegalOptionalPromotionPiece::Knight,
                Some(PromotionPiece::Bishop) => StableMaybeIllegalOptionalPromotionPiece::Bishop,
                Some(PromotionPiece::Rook) => StableMaybeIllegalOptionalPromotionPiece::Rook,
                Some(PromotionPiece::Queen) => StableMaybeIllegalOptionalPromotionPiece::Queen,
                None => StableMaybeIllegalOptionalPromotionPiece::None,
            },
        }
    }
}

impl From<Option<ChessMove>> for StableOptionalChessMove {
    #[inline]
    fn from(value: Option<ChessMove>) -> Self {
        match value {
            Some(value) => value.into(),
            None => StableOptionalChessMove {
                source: Pos::A1,
                dest: Pos::A1,
                piece: StableMaybeIllegalOptionalPromotionPiece::Illegal,
            },
        }
    }
}

impl From<StableChessMove> for ChessMove {
    #[inline]
    fn from(value: StableChessMove) -> Self {
        ChessMove {
            source: value.source,
            dest: value.dest,
            piece: match value.piece {
                StableOptionalPromotionPiece::Knight => Some(PromotionPiece::Knight),
                StableOptionalPromotionPiece::Bishop => Some(PromotionPiece::Bishop),
                StableOptionalPromotionPiece::Rook => Some(PromotionPiece::Rook),
                StableOptionalPromotionPiece::Queen => Some(PromotionPiece::Queen),
                StableOptionalPromotionPiece::None => None,
            },
        }
    }
}

impl From<ChessMove> for StableChessMove {
    #[inline]
    fn from(value: ChessMove) -> Self {
        StableChessMove {
            source: value.source,
            dest: value.dest,
            piece: match value.piece {
                Some(PromotionPiece::Knight) => StableOptionalPromotionPiece::Knight,
                Some(PromotionPiece::Bishop) => StableOptionalPromotionPiece::Bishop,
                Some(PromotionPiece::Rook) => StableOptionalPromotionPiece::Rook,
                Some(PromotionPiece::Queen) => StableOptionalPromotionPiece::Queen,
                None => StableOptionalPromotionPiece::None,
            },
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, abi_stable::StableAbi)]
enum StableScore {
    Min,
    BlackMateIn(u16),
    Raw(i32),
    WhiteMateIn(u16),
    Max,
}

#[repr(C)]
#[derive(Clone, Copy, abi_stable::StableAbi)]
pub struct EvaluatedMove {
    chess_move: StableOptionalChessMove,
    score: StableScore,
}

impl EvaluatedMove {
    #[inline]
    pub fn new(mv: Option<ChessMove>, score: Score) -> Self {
        Self {
            chess_move: mv.into(),
            score: match score {
                Score::Min => StableScore::Min,
                Score::BlackMateIn(x) => StableScore::BlackMateIn(x),
                Score::Raw(x) => StableScore::Raw(x),
                Score::WhiteMateIn(x) => StableScore::WhiteMateIn(x),
                Score::Max => StableScore::Max,
            },
        }
    }

    #[inline]
    pub fn chess_move(self) -> Option<ChessMove> {
        self.chess_move.into()
    }

    #[inline]
    pub fn score(self) -> Score {
        match self.score {
            StableScore::Min => Score::Min,
            StableScore::BlackMateIn(x) => Score::BlackMateIn(x),
            StableScore::Raw(x) => Score::Raw(x),
            StableScore::WhiteMateIn(x) => Score::WhiteMateIn(x),
            StableScore::Max => Score::Max,
        }
    }
}

#[repr(transparent)]
#[derive(abi_stable::StableAbi)]
pub struct TimeoutReference<'a> {
    ptr: StableTimeout_TO<'a, RRef<'a, ()>>,
}

impl<'a> TimeoutReference<'a> {
    pub fn new<T: 'a + Timeout>(timeout: &'a T) -> Self {
        Self {
            ptr: StableTimeout_TO::from_ptr(timeout, TD_Opaque),
        }
    }
}

#[sabi_trait]
pub trait StableTimeout {
    #[must_use]
    fn is_complete(&self) -> bool;
}

impl<T: ?Sized + Timeout> StableTimeout for T {
    #[inline]
    fn is_complete(&self) -> bool {
        Timeout::is_complete(self)
    }
}

impl Timeout for TimeoutReference<'_> {
    #[inline]
    fn is_complete(&self) -> bool {
        self.ptr.is_complete()
    }
}
