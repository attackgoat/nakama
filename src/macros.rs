#[macro_export]
macro_rules! nakama {
    ($init:ident) => {
        static mut ALREADY_INIT: std::sync::atomic::AtomicBool =
            std::sync::atomic::AtomicBool::new(false);

        #[no_mangle]
        pub unsafe extern "C" fn nkinit(
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
                &nakama::NakamaModule,
                &nakama::Initializer,
            ) -> Result<(), usize>,
        {
            let ctx = ctx.into();
            let mut logger = logger.into();
            let db = nakama::Db {};
            let nk = nk.into();
            let initializer = initializer.into();
            f(&ctx, &mut logger, &db, &nk, &initializer)
        }
    };
}

use super::State;

pub struct Init {
    pub label: String,
    pub rate: usize,
    pub state: State,
}
