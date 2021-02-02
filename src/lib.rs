pub mod prelude {
    pub use super::{nakama, Context, Initializer, Init, Db, Dispatcher, State, Logger,Join, Match, Nakama, Presence, Data};
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
    data::Data,
    context::Context, db::Db, dispatch::Dispatcher, logger::Logger,
    macros::{Init},
    init::Initializer,
    nakama_mod::NakamaModule as Nakama, presence::Presence, state::{Match, State, Join},
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
