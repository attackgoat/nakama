use {
    super::{Context, Data, Db, Dispatcher, Logger, NakamaModule, Presence},
    std::collections::HashMap,
};

pub type State = Option<Box<dyn Match>>;

pub struct Join {
    pub allow: bool,
    pub reason: String,
    pub state: State,
}

pub trait Match {
    #[allow(clippy::too_many_arguments)]
    fn join_attempt(
        self: Box<Self>,
        ctx: &Context,
        logger: &mut Logger,
        db: &Db,
        nk: &NakamaModule,
        dispatcher: &mut Dispatcher,
        tick: u64,
        presence: &Presence,
        metadata: &HashMap<String, String>,
    ) -> Join;

    #[allow(clippy::too_many_arguments)]
    fn join(
        self: Box<Self>,
        ctx: &Context,
        logger: &mut Logger,
        db: &Db,
        nk: &NakamaModule,
        dispatcher: &mut Dispatcher,
        tick: u64,
        presences: &[Presence],
    ) -> State;

    #[allow(clippy::too_many_arguments)]
    fn leave(
        self: Box<Self>,
        ctx: &Context,
        logger: &mut Logger,
        db: &Db,
        nk: &NakamaModule,
        dispatcher: &mut Dispatcher,
        tick: u64,
        presences: &[Presence],
    ) -> State;

    /// NOTE: This is named `loop` in the native Nakama server documentation.
    #[allow(clippy::too_many_arguments)]
    fn update(
        self: Box<Self>,
        ctx: &Context,
        logger: &mut Logger,
        db: &Db,
        nk: &NakamaModule,
        dispatcher: &mut Dispatcher,
        tick: u64,
        messages: &[Data],
    ) -> State;

    #[allow(clippy::too_many_arguments)]
    fn terminate(
        self: Box<Self>,
        ctx: &Context,
        logger: &mut Logger,
        db: &Db,
        nk: &NakamaModule,
        dispatcher: &mut Dispatcher,
        tick: u64,
        grace_seconds: usize,
    ) -> State;
}
