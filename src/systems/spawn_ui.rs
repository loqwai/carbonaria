use bevy::prelude::*;

use crate::components::ScoreUI;

pub fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::FlexEnd,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            // Score UI
            parent
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
                            bottom: Val::Px(5.0),
                            left: Val::Px(15.0),
                            ..default()
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(ScoreUI {});
        });
}
