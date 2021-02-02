use {
    super::{Context, Data, Db, Dispatcher, Logger, Nakama, Presence},
    std::collections::HashMap,
};

pub type State = Option<Box<dyn Match>>;

pub struct Join {
    pub allow: bool,
    pub reason: String,
    pub state: State,
}

pub trait Match {
    fn join_attempt(
        self: Box<Self>,
        ctx: &Context,
        logger: &mut Logger,
        db: &Db,
        nk: &Nakama,
        dispatcher: &mut Dispatcher,
        tick: u64,
        presence: &Presence,
        metadata: &HashMap<String, String>,
    ) -> Join;

    fn join(
        self: Box<Self>,
        ctx: &Context,
        logger: &mut Logger,
        db: &Db,
        nk: &Nakama,
        dispatcher: &mut Dispatcher,
        tick: u64,
        presences: &[Presence],
    ) -> State;

    fn leave(
        self: Box<Self>,
        ctx: &Context,
        logger: &mut Logger,
        db: &Db,
        nk: &Nakama,
        dispatcher: &mut Dispatcher,
        tick: u64,
        presences: &[Presence],
    ) -> State;

    /// NOTE: This is named `loop` in the native Nakama server documentation.
    fn update(
        self: Box<Self>,
        ctx: &Context,
        logger: &mut Logger,
        db: &Db,
        nk: &Nakama,
        dispatcher: &mut Dispatcher,
        tick: u64,
        messages: &[Data],
    ) -> State;

    fn terminate(
        self: Box<Self>,
        ctx: &Context,
        logger: &mut Logger,
        db: &Db,
        nk: &Nakama,
        dispatcher: &mut Dispatcher,
        tick: u64,
        grace_seconds: usize,
    ) -> State;
}
