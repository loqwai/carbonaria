use bevy::prelude::*;

use crate::components::ScoreUI;

pub fn spawn_score_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "dsadasd".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/10100.otf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                }],
                ..Default::default()
            },
            style: Style {
                position_type: PositionType::Relative,
                position: UiRect {
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
