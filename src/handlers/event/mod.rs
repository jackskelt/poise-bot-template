use poise::FrameworkContext;
use tracing::trace;
mod ready;

use crate::{
    serenity::{Context, FullEvent},
    Data, Error,
};

pub async fn handle(
    ctx: &Context,
    event: &FullEvent,
    _framework: FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    trace!("Event received: {:?}", event.snake_case_name());
    #[allow(clippy::single_match)]
    match event {
        FullEvent::Ready { data_about_bot, .. } => {
            ready::handle(data_about_bot, ctx).await;
        }
        _ => {}
    }

    Ok(())
}
