use bevy::prelude::*;

use crate::components::HealthTarget;

#[derive(Bundle)]
pub struct HealthBundle {
    pub target: HealthTarget,

    #[bundle]
    pub text_2d_bundle: Text2dBundle,
}

impl HealthBundle {
    pub fn new(asset_server: &Res<AssetServer>) -> HealthBundle {
        HealthBundle {
            target: HealthTarget,
            text_2d_bundle: Text2dBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "asdsad".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/10100.otf"),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    }],
                    alignment: TextAlignment {
                        vertical: VerticalAlign::Bottom,
                        horizontal: HorizontalAlign::Center,
                    },
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, -40.0, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}
