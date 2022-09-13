use bevy::prelude::*;

use crate::components::ScoreUI;

pub fn spawn_score_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("spawn_score_ui");
    commands
        .spawn_bundle(TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "Score: 0".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/10100.otf"),
                        font_size: 20.0,
                        color: Color::BLACK,
                    },
                }],
                ..Default::default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ScoreUI {});
}
