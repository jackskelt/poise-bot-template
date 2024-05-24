use crate::{built_info, Context, Error};
use chrono::DateTime;
use poise::CreateReply;
use serenity::builder::{CreateEmbed, CreateEmbedFooter};
use tokio::time::Instant;
use tracing::error;

#[poise::command(slash_command)]
pub async fn info(ctx: Context<'_>) -> Result<(), Error> {
    let ping = ctx.ping().await;

    let start_time = Instant::now();

    if let Err(reason) = ctx.defer().await {
        error!("Couldn't defer interaction: {reason:?}");
        return Ok(());
    }

    let elapsed_time = start_time.elapsed();

    let last_build_timestamp = match DateTime::parse_from_rfc2822(built_info::BUILT_TIME_UTC) {
        Ok(v) => format!("<t:{}>", v.timestamp()),
        Err(_) => "`Not Found`".into(),
    };

    let rustc_version = built_info::RUSTC_VERSION
        .split_whitespace()
        .collect::<Vec<_>>()[1];

    let serenity_version = built_info::DIRECT_DEPENDENCIES
        .iter()
        .find(|(n, _)| *n == "serenity")
        .map_or("Not Found", |(_, v)| v);
    let poise_version = built_info::DIRECT_DEPENDENCIES
        .iter()
        .find(|(n, _)| *n == "poise")
        .map_or("Not Found", |(_, v)| v);

    let embed = CreateEmbed::new()
        .color(0x5b9e48)
        .title("Bot info")
        .footer(CreateEmbedFooter::new("Made with Rust"))
        .fields([
            ("🏓 Ping", format!("`{ping:.2?}`"), true),
            ("📡 Interaction ping", format!("`{elapsed_time:.2?}`"), true),
            ("🦀 Rust version", format!("`{rustc_version}`"), true),
            ("⚙ Serenity version", format!("`{serenity_version}`"), true),
            ("🧪 Poise version", format!("`{poise_version}`"), true),
            ("🏗 Last build", last_build_timestamp, true),
        ]);

    ctx.send(CreateReply {
        embeds: vec![embed],
        ..Default::default()
    })
    .await
    .unwrap();

    Ok(())
}
