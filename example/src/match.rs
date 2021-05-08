use {nakama::prelude::*, std::collections::HashMap};

// // Nakama calls this before starting a new match - we should return one of our Match implementations
// fn new_match(
//     ctx: &Context,
//     logger: &mut Logger,
//     db: &Db,
//     nk: &NakamaModule,
//     params: &HashMap<String, Box<dyn Any>>,
// ) -> Init {
//     // A string label that can be used to filter matches in listing operations
//     let label = String::new();

//     // Tick rate representing the desired number of update() calls per second
//     let rate = 10;

//     // May be any Box<dyn Match> value that will store the match state as it progresses
//     let state = Box::new(MyMatch);

//     Init {
//         label,
//         rate,
//         state: Some(state),
//     }
// }

struct MyMatch;

impl Match for MyMatch {
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
    ) -> Join {
        Join {
            allow: true,
            reason: String::from("☢️☢️ radiation detected - let this one through! ☢️☢️"),
            state: Some(self),
        }
    }

    fn join(
        self: Box<Self>,
        ctx: &Context,
        logger: &mut Logger,
        db: &Db,
        nk: &NakamaModule,
        dispatcher: &mut Dispatcher,
        tick: u64,
        presences: &[Presence],
    ) -> State {
        // TODO: add each Presence in `presences` to our state (usually stored in a vec)

        Some(self)
    }

    fn leave(
        self: Box<Self>,
        ctx: &Context,
        logger: &mut Logger,
        db: &Db,
        nk: &NakamaModule,
        dispatcher: &mut Dispatcher,
        tick: u64,
        presences: &[Presence],
    ) -> State {
        // TODO: remove each Presence in `presences` from our state

        Some(self)
    }

    /// NOTE: This is named `loop` in the native Nakama server documentation
    fn update(
        self: Box<Self>,
        ctx: &Context,
        logger: &mut Logger,
        db: &Db,
        nk: &NakamaModule,
        dispatcher: &mut Dispatcher,
        tick: u64,
        messages: &[Data],
    ) -> State {
        // TODO: Dispatch network messages about spaceships and mega asteroid blasters, maybe.

        Some(self)
    }

    fn terminate(
        self: Box<Self>,
        ctx: &Context,
        logger: &mut Logger,
        db: &Db,
        nk: &NakamaModule,
        dispatcher: &mut Dispatcher,
        tick: u64,
        grace_seconds: usize,
    ) -> State {
        // 👋

        None
    }
}
