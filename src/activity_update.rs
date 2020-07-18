use gdnative::prelude::*;
use discord_game_sdk::Activity;
#[derive(ToVariant, FromVariant)]
/// Struct used to convert Godot Variant types to discord_game_sdk::Activity
/// see discord_game_sdk::Activity for what this represents
pub struct ActivityUpdate {
    pub state: Option<String>,
    pub large_image_key: Option<String>,
    pub details: Option<String>,
    pub large_image_tooltip: Option<String>,
    pub start_timestamp: Option<i64>,
    pub end_timestamp: Option<i64>
}

impl Default for ActivityUpdate {
    fn default() -> Self {
        Self {
            state: Some(String::new()),
            large_image_key: Some(String::from("default")),
            details: Some(String::new()),
            large_image_tooltip: Some(String::new()),
            start_timestamp: None,
            end_timestamp: None
        }
    }
}

impl ActivityUpdate {
    pub fn new() -> Self {
        ActivityUpdate::default()
    }
    /// Converts an ActivityUpdate into a discord_game_sdk native Activity
    pub fn to_activity_update(&self) -> Activity {
        
        let mut act = Activity::empty();
                
        act.with_state(&self.state.to_owned().unwrap_or_default())
            .with_large_image_key(&self.large_image_key.to_owned().unwrap_or_default())
            .with_details(&self.details.to_owned().unwrap_or_default())
            .with_large_image_tooltip(&self.large_image_tooltip.to_owned().unwrap_or_default());
        match self.start_timestamp {
            Some(timestamp) => {
                act.with_start_time(timestamp);
            }
            None => {}
        }
        match self.end_timestamp {
            Some(timestamp) => {
                act.with_end_time(timestamp);
            }
            None => {}
        }
        return act;
    }
}
