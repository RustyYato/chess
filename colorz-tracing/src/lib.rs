use tracing::{field::Visit, Event, Level};
use tracing_subscriber::{
    field::RecordFields,
    fmt::{FormatEvent, FormatFields},
    Registry,
};

pub struct BasicEventFormat;
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

        ctx.format_fields(f.by_ref(), event)?;

        writeln!(f)?;

        Ok(())
    }
}

pub struct BasicFieldFormatter;
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
