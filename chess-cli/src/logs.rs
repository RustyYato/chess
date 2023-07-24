use tracing::{field::Visit, metadata::LevelFilter, Event};
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

pub fn init(level: i8) {
    colorz::mode::set_default_stream(colorz::mode::Stream::Stdout);
    colorz::mode::set_coloring_mode_from_env();

    tracing_subscriber::fmt()
        .event_format(colorz_tracing::BasicEventFormat)
        .fmt_fields(colorz_tracing::BasicFieldFormatter)
        .map_event_format(CurrentDepthTabs)
        .with_max_level(match level {
            0 => std::env::var("RUST_LOG")
                .map(|level| level.parse::<LevelFilter>())
                .ok()
                .transpose()
                .expect("Invalid level")
                .unwrap_or(LevelFilter::WARN),
            -1 => LevelFilter::INFO,
            1..=i8::MAX => LevelFilter::ERROR,
            -2 => LevelFilter::DEBUG,
            i8::MIN..=-3 => LevelFilter::TRACE,
        })
        .with_writer(std::io::stdout)
        .finish()
        .with(tracing_enabled::GlobalEnable)
        .init();
}
