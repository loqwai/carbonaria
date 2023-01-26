use crate::components::{GameOverUI, Player};
use bevy::prelude::*;

pub fn on_no_players_show_game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    players: Query<&Player>,
    game_over_uis: Query<&GameOverUI>,
    mut other_uis: Query<Entity, (With<Node>, Without<GameOverUI>)>,
) {
    if !players.is_empty() || !game_over_uis.is_empty() {
        return;
    }

    other_uis.for_each_mut(|ui| commands.entity(ui).despawn_recursive());

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::Rgba {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 0.9,
            }),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    style: Style {
                        flex_grow: 0.0,
                        ..Default::default()
                    },
                    text: Text {
                        alignment: TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                        sections: vec![
                            TextSection {
                                value: "Game Over\n".to_string(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/10100.otf"),
                                    font_size: 100.0,
                                    color: Color::WHITE,
                                },
                            },
                            TextSection {
                                value: "Click to reset".to_string(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/10100.otf"),
                                    font_size: 50.0,
                                    color: Color::WHITE,
                                },
                            },
                        ],
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(GameOverUI);
        });
}
