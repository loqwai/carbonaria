use bevy::prelude::*;

use crate::components::HealthTarget;

#[derive(Bundle)]
pub struct HealthBundle {
    pub target: HealthTarget,

    #[bundle]
    pub text_bundle: TextBundle,
}

impl HealthBundle {
    pub fn new(asset_server: &Res<AssetServer>, target: Entity, camera: Entity) -> HealthBundle {
        HealthBundle {
            target: HealthTarget(target, camera),
            text_bundle: TextBundle {
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
                // style: Style {
                //     position_type: PositionType::Absolute,
                //     // position: Rect {
                //     //     top: Val::Px(0.0),
                //     //     left: Val::Px(0.0),
                //     //     ..default()
                //     // },
                //     ..Default::default()
                // },
                ..Default::default()
            },
        }
    }
}
