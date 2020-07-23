use gdnative::prelude::*;
use discord_game_sdk::Discord;
use discord_game_sdk::EventHandler;
use discord_game_sdk::Error;
use super::activity_update::ActivityUpdate;
#[derive(NativeClass)]
#[derive(Default)]
#[inherit(Reference)]
pub struct DiscordController {
    discord: Option<Discord<'static, MyEventHandler>>
}
struct MyEventHandler;

impl EventHandler for MyEventHandler {
}

impl DiscordController {
    fn new(_owner: &Reference) -> Self {
        DiscordController {
            discord: None
        }
    }
}

#[methods]
impl DiscordController {
    /// Exposes discord's update_activity to GDScript
    ///
    /// # Arguments
    ///
    /// * `game_id` - The discord client ID for this application
    #[export]
    pub fn init_discord(&mut self, _owner: &Reference, game_id: i64) -> i64 {
        match Discord::with_create_flags(game_id, discord_game_sdk::CreateFlags::NoRequireDiscord) {
            Ok(mut discord) => {
                godot_print!("Discord OK!");
                *discord.event_handler_mut() = Some(MyEventHandler);
                self.discord = Some(discord);
                return 0;
            }
            Err(err) => {
                godot_error!("Discord init error");
                godot_error!("Error initializing discord: {:?}", err);
            }
        }
        2
    }
    /// Exposes discord's runn_callbacks to GDScript, you should call this in _process on any node
    #[export]
    pub fn run_callbacks(&mut self, _owner: &Reference) {
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
    pub fn update_activity(&mut self, _owner: &Reference, activity: ActivityUpdate) {
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