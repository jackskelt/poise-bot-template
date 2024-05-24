use poise::Command;

use crate::{Data, Error};

pub mod info;

pub fn get_commands() -> Vec<Command<Data, Error>> {
    vec![info::info()]
}
