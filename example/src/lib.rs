#![allow(unused_variables)]

mod rpc;

use nakama::prelude::*;

/// Nakama calls this function once during startup.
#[module_initializer]
fn my_example_module(
    ctx: Context,
    logger: Logger,
    db: Db,
    nk: NakamaModule,
    initializer: Initializer,
) {
    // Get something from the context and log it
    let exe_mode = ctx.value("execution_mode").unwrap();
    logger.info(format!("🦀: Execution mode {}", &exe_mode));

    // Initialize some RPC calls
    initializer.register_rpc("FindOrCreateMatch", rpc::find_or_create_match).unwrap();
    initializer.register_rpc("PurchaseLoot", rpc::purchase_loot).unwrap();
}
