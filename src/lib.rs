#![allow(unused_imports)]

pub mod prelude {
    pub use {
        super::{
            Context, Data, Db, Dispatcher, Init, Initializer, Join, Logger, Match, NakamaModule,
            Presence, State,
        },
        nakama_derive::*,
    };
}

pub mod sys {
    pub use nakama_sys::*;
}

mod context;
mod data;
mod db;
mod dispatch;
mod initializer;
mod logger;
mod nakama_mod;
mod presence;
mod state;

pub use self::{
    context::Context,
    data::Data,
    db::Db,
    dispatch::Dispatcher,
    initializer::Initializer,
    logger::Logger,
    nakama_mod::NakamaModule,
    presence::Presence,
    state::{Join, Match, State},
};

pub struct Init {
    pub label: String,
    pub rate: usize,
    pub state: State,
}
