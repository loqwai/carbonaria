mod detect_stick_hits;
mod mob_swings_stick_if_player_gets_close;
mod move_player;
mod on_click_swing_stick;
mod on_stick_hit_increment_points;
mod on_stick_hit_kill;
mod on_stick_hit_subtract_health;
mod spawn_camera;
mod spawn_mobs;
mod spawn_next_tile_for_rooms;
mod spawn_player;
mod spawn_room;
mod spawn_score_ui;
mod swing_stick;
mod update_health_ui;
mod update_score_ui;

pub use detect_stick_hits::detect_stick_hits;
pub use mob_swings_stick_if_player_gets_close::mob_swings_stick_if_player_gets_close;
pub use move_player::move_player;
pub use on_click_swing_stick::on_click_swing_stick;
pub use on_stick_hit_increment_points::on_stick_hit_increment_points;
pub use on_stick_hit_kill::on_stick_hit_kill;
pub use on_stick_hit_subtract_health::on_stick_hit_subtract_health;
pub use spawn_camera::spawn_camera;
pub use spawn_mobs::spawn_mobs;
pub use spawn_next_tile_for_rooms::spawn_next_tile_for_rooms;
pub use spawn_player::spawn_player;
pub use spawn_room::spawn_room;
pub use spawn_score_ui::spawn_score_ui;
pub use swing_stick::begin_swing_stick_animation;
pub use swing_stick::maybe_end_swing_stick_animation;
pub use update_health_ui::update_health_ui;
pub use update_score_ui::update_score_ui;
