mod chaser_aimables_aim_at_other_teams;
mod chasers_follow_other_teams;
mod consume_despawn_entity_events;
mod count_ticks;
// mod detect_exit;
mod follow_player_with_camera;
mod move_bullet;
mod move_player;
mod calculate_rate_of_fire;
mod move_thing;
mod on_0_health_kill;
mod on_chest_hit_pickup;
mod on_click_and_no_player_reset;
mod on_damager_hit_subtract_health;
mod on_move_event_change_sprite_index;
mod on_no_players_show_game_over;
mod player_aimables_aim_at_cursor;
mod remove_all_entities;
mod resize_window;
mod rotate_thing;
mod shoot_gun;
mod spawn_camera;
mod spawn_crosshairs;
mod shoot_rage_quit_gun;
// mod spawn_exit;
mod spawn_mobs;
mod spawn_player;
mod spawn_powerups;
mod spawn_ui;
mod sync_mouse_position;
mod team_powerup_assigns_team;
mod time_to_live;
mod update_compass;
mod update_health_ui;
mod update_score_ui;
mod powerup_adder;
pub use chaser_aimables_aim_at_other_teams::chaser_aimables_aim_at_other_teams;
pub use chasers_follow_other_teams::chasers_follow_other_teams;
pub use consume_despawn_entity_events::consume_despawn_entity_events;
pub use count_ticks::count_ticks;
// pub use detect_exit::detect_exit;
pub use follow_player_with_camera::follow_player_with_camera;
pub use move_bullet::move_bullet;
pub use move_player::move_player;
pub use calculate_rate_of_fire::calculate_rate_of_fire;
pub use move_thing::move_thing;
pub use on_0_health_kill::on_0_health_kill;
pub use on_chest_hit_pickup::on_chest_hit_pickup;
pub use on_click_and_no_player_reset::on_click_and_no_player_reset;
pub use on_damager_hit_subtract_health::on_damager_hit_subtract_health;
pub use on_move_event_change_sprite_index::on_move_event_change_sprite_index;
pub use on_no_players_show_game_over::on_no_players_show_game_over;
pub use player_aimables_aim_at_cursor::player_aimables_aim_at_cursor;
pub use remove_all_entities::remove_all_entities;
pub use resize_window::resize_window;
pub use rotate_thing::rotate_thing;
pub use shoot_gun::shoot_gun;
pub use spawn_camera::spawn_camera;
pub use spawn_crosshairs::spawn_crosshairs;
// pub use spawn_exit::spawn_exit;
pub use spawn_mobs::spawn_mobs;
pub use spawn_player::spawn_player;
pub use spawn_powerups::spawn_powerups;
pub use spawn_ui::spawn_ui;
pub use sync_mouse_position::sync_mouse_position;
pub use team_powerup_assigns_team::team_powerup_assigns_team;
pub use shoot_rage_quit_gun::shoot_rage_quit_gun;
pub use time_to_live::time_to_live;
pub use update_compass::update_compass;
pub use update_health_ui::update_health_ui;
pub use update_score_ui::update_score_ui;
pub use powerup_adder::powerup_adder;

use crate::components::{Health, Player};
use bevy::prelude::*;

pub fn on_health_100_you_win(players: Query<&Health, With<Player>>) {
    for player in players.iter() {
        if player.0 == 100 {
            println!("You win!");
            std::process::exit(1);
        }
    }
}
