use std::{
    cell::Cell,
    marker::PhantomData,
    sync::atomic::{AtomicBool, Ordering},
};

static IS_ENABLED: AtomicBool = AtomicBool::new(true);

#[derive(Default, Clone, Copy)]
enum LocalFlag {
    #[default]
    Global,
    Enabled,
    Disabled,
}

pub struct LocalEnableState {
    flag: LocalFlag,
    _cell: PhantomData<&'static Cell<()>>,
}

pub fn enable() {
    local_enable();
    IS_ENABLED.store(true, Ordering::Release);
}

pub fn disable() {
    local_disable();
    IS_ENABLED.store(false, Ordering::Release);
}

pub fn toggle() {
    local_toggle();
    IS_ENABLED.fetch_xor(true, Ordering::Release);
}

pub fn local_enable() {
    LOCAL_ENABLED.with(|x| x.set(LocalFlag::Enabled));
}

pub fn local_disable() {
    LOCAL_ENABLED.with(|x| x.set(LocalFlag::Disabled));
}

pub fn local_toggle() {
    LOCAL_ENABLED.with(|x| {
        x.set(match x.get() {
            LocalFlag::Global => LocalFlag::Global,
            LocalFlag::Enabled => LocalFlag::Disabled,
            LocalFlag::Disabled => LocalFlag::Enabled,
        })
    });
}

pub fn local_take() -> LocalEnableState {
    LocalEnableState {
        flag: LOCAL_ENABLED.with(Cell::take),
        _cell: PhantomData,
    }
}

pub fn restore(state: LocalEnableState) {
    LOCAL_ENABLED.with(|x| x.set(state.flag));
}

pub fn is_enabled() -> bool {
    match LOCAL_ENABLED.with(Cell::get) {
        LocalFlag::Global => IS_ENABLED.load(Ordering::Acquire),
        LocalFlag::Enabled => true,
        LocalFlag::Disabled => false,
    }
}

thread_local! {
    static LOCAL_ENABLED: Cell<LocalFlag> = const { Cell::new(LocalFlag::Global) };
}

pub struct GlobalEnable;

impl<T: tracing::Subscriber> tracing_subscriber::Layer<T> for GlobalEnable {
    fn enabled(
        &self,
        _metadata: &tracing::Metadata<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, T>,
    ) -> bool {
        is_enabled()
    }
}
