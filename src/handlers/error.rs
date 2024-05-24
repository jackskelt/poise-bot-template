use poise::FrameworkError;
use tracing::{debug, error};

use crate::{Data, Error};

pub async fn handle(err: FrameworkError<'_, Data, Error>) {
    match err {
        FrameworkError::Setup { error, .. } => error!("Framework Setup: {error:?}"),
        FrameworkError::Command { error, .. } => error!("Command: {error:?}"),
        FrameworkError::EventHandler { error, event, .. } => {
            error!("Event {}: {error:?}", event.snake_case_name())
        }
        FrameworkError::ArgumentParse { error, input, .. } => {
            error!("Parsing({input:?}): {error:?}")
        }
        FrameworkError::CommandPanic { payload, .. } => error!("Command panic: {payload:?}"),
        FrameworkError::UnknownInteraction { interaction, .. } => {
            error!("Unknown Interaction \"{}\":", interaction.data.name);
            debug!("Unknown Interaction Data: {:?}", interaction.data);
        }
        _ => {}
    }
}
