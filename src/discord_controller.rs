use gdnative::prelude::*;
use discord_game_sdk::Discord;
use discord_game_sdk::EventHandler;
use discord_game_sdk::Error;
use super::activity_update::ActivityUpdate;
#[derive(NativeClass)]
#[derive(Default)]
#[inherit(Node)]
pub struct DiscordController {
    discord: Option<Discord<'static, MyEventHandler>>
}
struct MyEventHandler;

impl EventHandler for MyEventHandler {
}

impl DiscordController {
    fn new(_owner: &Node) -> Self {
        DiscordController {
            discord: None
        }
    }
}

#[methods]
impl DiscordController {
    #[export]
    fn _ready(&mut self, _owner: &Node) {
        match Discord::new(733416106123067465) {
            Ok(mut discord) => {
                *discord.event_handler_mut() = Some(MyEventHandler);
                godot_print!("Discord OK!");
                self.discord = Some(discord);
            }
            Err(err) => {
                godot_error!("Error initializing discord: {:?}", err)
            }
        }
    }
    #[export]
    fn _process(&mut self, _owner: &Node, _delta: f64) {
        match &mut self.discord {
            Some(discord) => {
                if let Err(Error::NotRunning) = discord.run_callbacks() {
                    godot_print!("Discord is not running anymore");
                    self.discord = None;
                }
            }
            None => {}
        }
    }
    /// Exposes discord's update_activity to GDScript
    ///
    /// # Arguments
    ///
    /// * `activity` - The ActivityUpdate struct (GDScript users see ActivityUpdate for dictionary fields)
    #[export]
    pub fn update_activity(&mut self, _owner: &Node, activity: ActivityUpdate) {
        match &mut self.discord {
            Some(discord) => {
                discord.update_activity(
                    &activity.to_activity_update(),
                |_discord, result| {
                    if let Err(error) = result {
                        godot_print!("Failed to update activity: {}", error);
                    }
                })
            }
            None => {}
        }

    }
}