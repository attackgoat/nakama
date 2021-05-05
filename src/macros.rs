#[macro_export]
macro_rules! nakama {
    ($init:ident, $new_match:ident) => {
        static mut ALREADY_INIT: std::sync::atomic::AtomicBool =
            std::sync::atomic::AtomicBool::new(false);

        fn rs_init<F>(
            f: F,
            ctx: &nakama::sys::NkContext,
            logger: &nakama::sys::NkLogger,
            db: &nakama::sys::NkDb,
            nk: &nakama::sys::NkModule,
            initializer: &nakama::sys::NkInitializer,
        ) -> Result<(), usize>
        where
            F: Fn(
                &nakama::Context,
                &mut nakama::Logger,
                &nakama::Db,
                &nakama::Nakama,
                &nakama::Initializer,
            ) -> Result<(), usize>,
        {
            let ctx: nakama::Context = ctx.into();
            let mut logger: nakama::Logger = logger.into();
            let db = nakama::Db {};
            let nk: nakama::Nakama = nk.into();
            let initializer = nakama::Initializer {};
            f(&ctx, &mut logger, &db, &nk, &initializer)
        }

        fn rs_new_match<F>(
            f: F,
            ctx: &nakama::sys::NkContext,
            logger: &nakama::sys::NkLogger,
            db: &nakama::sys::NkDb,
            nk: &nakama::sys::NkModule,
        ) -> nakama::Init
        where
            F: Fn(
                &nakama::Context,
                &mut nakama::Logger,
                &nakama::Db,
                &nakama::Nakama,
                &std::collections::HashMap<String, Box<dyn std::any::Any>>,
            ) -> nakama::Init,
        {
            let ctx: nakama::Context = ctx.into();
            let mut logger: nakama::Logger = logger.into();
            let db = nakama::Db {};
            let nk: nakama::Nakama = nk.into();
            let params = std::collections::HashMap::new();
            f(&ctx, &mut logger, &db, &nk, &params)
        }

        #[no_mangle]
        pub extern "C" fn nk_init_match(
            ctx: nakama::sys::NkContext,
            logger: nakama::sys::NkLogger,
            db: nakama::sys::NkDb,
            nk: nakama::sys::NkModule,
        ) -> i64 {
            let _ = rs_new_match($new_match, &ctx, &logger, &db, &nk);

            1234
        }

        #[no_mangle]
        pub unsafe extern "C" fn nk_init_module(
            ctx: nakama::sys::NkContext,
            logger: nakama::sys::NkLogger,
            db: nakama::sys::NkDb,
            nk: nakama::sys::NkModule,
            initializer: nakama::sys::NkInitializer,
        ) -> usize {
            // Make sure this function has not been called in this process before
            if unsafe { ALREADY_INIT.swap(true, std::sync::atomic::Ordering::SeqCst) } {
                panic!("Nakama already initialized");
            }

            if let Err(code) = rs_init($init, &ctx, &logger, &db, &nk, &initializer) {
                code
            } else {
                0
            }
        }
    };
}

use super::State;

pub struct Init {
    pub label: String,
    pub rate: usize,
    pub state: State,
}
