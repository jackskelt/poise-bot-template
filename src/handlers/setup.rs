use crate::{commands, serenity::Context, Data, Error};
use tokio::time::Instant;
use tracing::{error, info, warn};

pub async fn handle(ctx: &Context) -> Result<Data, Error> {
    register_guild_commands(ctx).await;

    Ok(Data {})
}

async fn register_guild_commands(ctx: &Context) {
    let start_time = Instant::now();

    let guild_id = match std::env::var("GUILD_ID") {
        Ok(value) => value,
        Err(_) => {
            warn!("No GUILD_ID found in env");
            return;
        }
    };

    let guild_commands = commands::get_commands();

    let guild_commands_count = guild_commands.len();

    if guild_commands_count == 0 {
        warn!("No guild command to register");
        return;
    }

    match poise::builtins::register_in_guild(
        &ctx.http,
        &guild_commands,
        guild_id.parse().expect("GUILD_ID to be valid"),
    )
    .await
    {
        Ok(_) => {
            let elapsed_time = start_time.elapsed();

            info!("Registered {guild_commands_count} command(s) in {guild_id}. Elapsed {elapsed_time:.2?}");
        }
        Err(reason) => {
            error!("Couldn't register guild commands: {reason:?}");
            panic!("{reason:?}");
        }
    }
}
