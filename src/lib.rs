use gdnative::prelude::*;

pub mod discord_controller;
pub mod activity_update;
pub use discord_controller::DiscordController;
pub use activity_update::ActivityUpdate;
fn init(handle: InitHandle) {
    handle.add_class::<discord_controller::DiscordController>();
}

godot_init!(init);