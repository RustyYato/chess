use std::{collections::HashMap, time::Duration};

use chess_engine::{DurationTimeout, Engine, Score, ThreeFold};
use chess_movegen::Board;
use tracing::{field::Visit, metadata::LevelFilter, Event, Level};
use tracing_subscriber::{
    field::RecordFields,
    fmt::{FormatEvent, FormatFields},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Registry,
};

struct CurrentDepthTabs<N>(N);

struct FormatTabbed;
struct FormatTabsTo<W>(W);

impl<W: core::fmt::Write> Visit for FormatTabsTo<W> {
    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        if field.name() == "current_depth" {
            for _ in 0..value {
                let _ = write!(self.0, "    ");
            }
        }
    }

    fn record_debug(&mut self, _: &tracing::field::Field, _: &dyn std::fmt::Debug) {}
}

impl<'writer> FormatFields<'writer> for FormatTabbed {
    fn format_fields<R: RecordFields>(
        &self,
        writer: tracing_subscriber::fmt::format::Writer<'writer>,
        fields: R,
    ) -> std::fmt::Result {
        fields.record(&mut FormatTabsTo(writer));
        Ok(())
    }
}

impl<F: for<'a> FormatFields<'a> + 'static, N: FormatEvent<Registry, F>> FormatEvent<Registry, F>
    for CurrentDepthTabs<N>
{
    fn format_event(
        &self,
        ctx: &tracing_subscriber::fmt::FmtContext<'_, Registry, F>,
        mut writer: tracing_subscriber::fmt::format::Writer<'_>,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        if event.fields().any(|field| field.name() == "current_depth") {
            FormatTabbed.format_fields(writer.by_ref(), event)?
        }

        self.0.format_event(ctx, writer, event)
    }
}

struct BasicEventFormat;
impl<F: for<'a> FormatFields<'a> + 'static> FormatEvent<Registry, F> for BasicEventFormat {
    fn format_event(
        &self,
        ctx: &tracing_subscriber::fmt::FmtContext<'_, Registry, F>,
        mut f: tracing_subscriber::fmt::format::Writer<'_>,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        use chrono::DateTime;
        use colorz::{
            xterm::{self, XtermColor},
            Colorize,
        };
        use std::time::SystemTime;

        let meta = event.metadata();

        let level_color = match *meta.level() {
            Level::DEBUG => XtermColor::Blue,
            Level::TRACE => XtermColor::Magenta,
            Level::WARN => XtermColor::Yellow,
            Level::ERROR => XtermColor::Red,
            Level::INFO => XtermColor::Green,
        };

        let now = SystemTime::now();
        let now = DateTime::<chrono::Local>::from(now);

        write!(
            f,
            "{timestamp} {level} {target}",
            timestamp = format_args!("[{now}]").fg(xterm::Gray50),
            level = format_args!("{}", meta.level()).fg(level_color),
            target = meta.module_path().unwrap_or(meta.target()),
        )?;

        if let Some(line) = meta.line() {
            write!(f, ":{line}")?;
        }

        f.write_str(" ")?;

        // ctx.
        ctx.format_fields(f.by_ref(), event)?;

        writeln!(f)?;

        Ok(())
    }
}

struct BasicFieldFormatter;
struct BasicFieldVisitor<'writer>(tracing_subscriber::fmt::format::Writer<'writer>);

impl Visit for BasicFieldVisitor<'_> {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        use colorz::{xterm, Colorize};

        let mut f = self.0.by_ref();

        let _ = if field.name() == "message" {
            write!(f, "{:?}", value)
        } else {
            write!(f, " {}={:?}", field.fg(xterm::Gray70), value)
        };
    }
}

impl<'writer> FormatFields<'writer> for BasicFieldFormatter {
    fn format_fields<R: RecordFields>(
        &self,
        writer: tracing_subscriber::fmt::format::Writer<'writer>,
        fields: R,
    ) -> std::fmt::Result {
        fields.record(&mut BasicFieldVisitor(writer));
        Ok(())
    }
}

fn main() {
    colorz::mode::set_default_stream(colorz::mode::Stream::Stdout);
    colorz::mode::set_coloring_mode_from_env();

    tracing_subscriber::fmt()
        .event_format(BasicEventFormat)
        .fmt_fields(BasicFieldFormatter)
        .map_event_format(CurrentDepthTabs)
        .with_max_level(
            std::env::var("RUST_LOG")
                .map(|level| level.parse::<LevelFilter>())
                .ok()
                .transpose()
                .expect("Invalid level")
                .unwrap_or(LevelFilter::OFF),
        )
        .with_writer(std::io::stdout)
        .finish()
        .with(tracing_enabled::GlobalEnable)
        .init();

    let mut engine = Engine::default();

    let board = "6k1/8/8/8/8/8/8/R6K w - - 0 1";
    // let board = "5k2/Q7/5K2/8/8/8/8/8 w - - 8 5";
    // let board = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0";
    // let board = "r3k2r/p1ppqpb1/Bn2pnp1/3PN3/4P3/2p2Q1p/PPPB1PPP/R3K2R w KQkq - 0 1";
    // let board = "2k5/8/2K5/8/8/8/6R1/8 w - - 0 1";
    // let board = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 0";

    let mut board = board.parse::<Board>().unwrap();

    let mut three_fold = ThreeFold::new();

    loop {
        eprintln!("{board}");
        eprintln!("{board:?}");

        // let start = std::time::Instant::now();
        let (mv, score) = engine.search(
            &board,
            &three_fold,
            DurationTimeout::new(Duration::from_millis(500)),
        );

        let Some(mv) = mv else {
            println!("DRAW (MATERIAL)");
            break;
        };
        // dbg!(start.elapsed());
        assert!(board.move_mut(mv));
        eprintln!(
            "{score:?} {mv} moves: {}, max_depth: {}",
            engine.moves_evaluated, engine.max_depth
        );
        assert_ne!(score, Score::Raw(0));

        if three_fold.add(board) {
            println!("DRAW (THREE FOLD)");
            break;
        }

        if board.legals().is_empty() {
            if board.in_check() {
                println!("WIN");
            } else {
                println!("DRAW (NO LEGAL MOVES)");
            }
            break;
        }
    }

    eprintln!("{board:?}");
}
