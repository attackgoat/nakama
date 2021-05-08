use nakama::prelude::*;

#[rpc_callback]
pub fn find_or_create_match(
    ctx: Context,
    logger: Logger,
    db: Db,
    nk: NakamaModule,
    payload: &str,
) -> Result<String, String> {
    Ok("good".to_owned())
}

#[rpc_callback]
pub fn purchase_loot(
    ctx: Context,
    logger: Logger,
    db: Db,
    nk: NakamaModule,
    payload: &str,
) -> Result<String, String> {
    Ok("yay!".to_owned())
}