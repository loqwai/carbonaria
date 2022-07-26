use bevy::{ecs::system::QuerySingleError, prelude::*, sprite::collide_aabb::collide};

use crate::{
    components::{HitBox, Player, Wall},
    resources::PlayerResource,
};

#[derive(Debug, Error)]
enum DetectPlayerWallCollisionsError {
    QuerySingleError(QuerySingleError),
}

pub fn detect_player_wall_collisions(
    player: Res<PlayerResource>,
    set: ParamSet<(
        Query<(&mut Transform, &Player, &HitBox)>, // Player
        Query<(&Transform, &Wall, &HitBox)>,       // Walls
    )>,
) {
    if let Err(e) = fallible_detect_player_wall_collisions(player, set) {
        panic!("Error detecting collisions: {}", e);
    }
}

fn fallible_detect_player_wall_collisions(
    _player: Res<PlayerResource>,
    mut set: ParamSet<(
        Query<(&mut Transform, &Player, &HitBox)>,
        Query<(&Transform, &Wall, &HitBox)>,
    )>,
) -> Result<(), DetectPlayerWallCollisionsError> {
    let (player_loc, player_hitbox) = {
        let player_query = set.p0();
        let (transform, _, hitbox) = player_query.get_single()?;

        (transform.translation.clone(), hitbox.clone_vec2())
    };

    let mut collided = false;
    let mut vec = Vec3::new(0.0, 0.0, 0.0);

    for (transform, _, hitbox) in set.p1().iter() {
        let wall_loc = transform.translation.clone();
        let wall_hitbox = hitbox.clone_vec2();

        let collision = collide(player_loc, player_hitbox, wall_loc, wall_hitbox);

        match collision {
            None => continue,
            Some(_) => {
                let distance = wall_loc.distance(player_loc);

                collided = true;
                vec = wall_loc + (((player_loc - wall_loc) / distance) * 64.0);

                break;
            }
        }
    }

    if collided {
        let mut player_query = set.p0();
        let (mut player, _, _) = player_query.get_single_mut()?;
        player.translation = vec;
    }

    Ok(())
}
