use handlers::setup::handle;
use poise::{serenity_prelude as serenity, Framework, FrameworkOptions};
use tokio::time::Instant;
use tracing::{debug, error, subscriber, Level};
use tracing_subscriber::{fmt::Subscriber, EnvFilter};

use ::serenity::all::GatewayIntents;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data;

pub mod commands;
pub mod handlers;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[tokio::main]
pub async fn main() {
    initialise_subscriber(Level::INFO);

    let (framework, intents) = (initialise_framework().await, initialise_intents());

    let mut client = initialise_client(intents, framework).await;

    if let Err(reason) = client.start().await {
        error!("Couldn't start client: {reason:?}");
    }
}

fn initialise_intents() -> GatewayIntents {
    let start_time = Instant::now();

    let intents = GatewayIntents::all();

    let elapsed_time = start_time.elapsed();
    debug!("Initialised intents in {elapsed_time:.2?}");

    intents
}

async fn initialise_client(
    intents: GatewayIntents,
    framework: Framework<Data, Error>,
) -> serenity::Client {
    let start_time = Instant::now();

    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN be set.");

    let client = match serenity::Client::builder(token, intents)
        .framework(framework)
        .await
    {
        Ok(client) => client,
        Err(reason) => {
            error!("Couldn't initialise client: {reason:?}");
            panic!("{reason:?}");
        }
    };

    let elapsed_time = start_time.elapsed();
    debug!("Initialised client in {elapsed_time:.2?}");

    client
}

async fn initialise_framework() -> Framework<Data, Error> {
    let start_time = Instant::now();

    let framework = Framework::builder()
        .setup(|ctx, _, _| Box::pin(handle(ctx)))
        .options(FrameworkOptions {
            commands: commands::get_commands(),
            on_error: |e| Box::pin(handlers::error::handle(e)),
            event_handler: |ctx, event, framework, data| {
                Box::pin(handlers::event::handle(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .build();

    let elapsed_time = start_time.elapsed();
    debug!("Initialised framework in {elapsed_time:.2?}");

    framework
}

fn initialise_subscriber(level: Level) {
    let start_time = Instant::now();

    let subscriber = Subscriber::builder()
        .with_max_level(level)
        .with_env_filter(EnvFilter::from_default_env())
        .compact()
        .finish();

    match subscriber::set_global_default(subscriber) {
        Ok(_) => (),
        Err(reason) => error!("Couldn't set global default for logger: {reason:?}"),
    };

    let elapsed_time = start_time.elapsed();
    debug!("Initialised logger in {elapsed_time:.2?}");
}
