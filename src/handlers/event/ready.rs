use serenity::{all::OnlineStatus, gateway::ActivityData};
use tracing::info;

use crate::serenity::{Context, Ready};

pub async fn handle(event: &Ready, ctx: &Context) {
    ctx.set_presence(
        Some(ActivityData::listening("chipi chipi chapa chapa")),
        OnlineStatus::Online,
    );

    let guild_count = event.guilds.len();

    info!("Online in {guild_count} guild(s)!");
}
