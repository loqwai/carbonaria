use bevy::prelude::*;

use crate::components::HealthTarget;

#[derive(Bundle)]
pub struct HealthBundle {
    pub target: HealthTarget,

    #[bundle]
    pub text_2d_bundle: Text2dBundle,
}

impl HealthBundle {
    pub fn new(asset_server: &Res<AssetServer>, target: Entity) -> HealthBundle {
        HealthBundle {
            target: HealthTarget(target),
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
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}
