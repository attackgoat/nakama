pub mod prelude {
    pub use super::{
        nakama, Context, Data, Db, Dispatcher, Init, Initializer, Join, Logger, Match, Nakama,
        Presence, State,
    };
}

pub mod sys {
    pub use nakama_sys::*;
}

mod context;
mod data;
mod db;
mod dispatch;
mod init;
mod logger;
mod macros;
mod nakama_mod;
mod presence;
mod state;

pub use self::{
    context::Context,
    data::Data,
    db::Db,
    dispatch::Dispatcher,
    init::Initializer,
    logger::Logger,
    macros::Init,
    nakama_mod::NakamaModule as Nakama,
    presence::Presence,
    state::{Join, Match, State},
};

//use nakama_sys::*;

// fn hello_world(v1: i64, v2: i64, s: String) -> i64 {
//     let go_s = _GoString_ {
//         p: s.as_ptr() as *const _,
//         n: s.len() as _,
//     };

//     unsafe { nakama_sys::HelloWorld(v1, v2, go_s) }
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
