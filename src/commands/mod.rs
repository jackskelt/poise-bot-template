use poise::Command;

use crate::{Data, Error};

pub mod misc;

pub fn get_commands() -> Vec<Command<Data, Error>> {
    vec![misc::get_commands()].into_iter().flatten().collect()
}
