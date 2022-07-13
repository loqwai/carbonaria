use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut player: ResMut<Player>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let entity = commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("player.png"),
            ..Default::default()
        })
        .id();

    player.entity = Some(entity);
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    player: Res<Player>,
    mut transforms: Query<&mut Transform>,
) {
    let mut transform = transforms.get_mut(player.entity.unwrap()).unwrap();

    let player_speed = 5.0;

    if keyboard_input.pressed(KeyCode::A) {
        transform.translation[0] -= player_speed;
    }
    if keyboard_input.pressed(KeyCode::D) {
        transform.translation[0] += player_speed;
    }

    if keyboard_input.pressed(KeyCode::W) {
        transform.translation[1] += player_speed;
    }
    if keyboard_input.pressed(KeyCode::S) {
        transform.translation[1] -= player_speed;
    }
}

#[derive(Default)]
struct Player {
    entity: Option<Entity>,
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Player>()
            .add_startup_system(setup)
            .add_system(move_player);
    }
}
