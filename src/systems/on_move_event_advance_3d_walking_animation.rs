use bevy::prelude::*;

use crate::resources::MechWalkingAnimation;

pub fn on_move_event_advance_3d_walking_animation(
    mut commands: Commands,
    mut animation_players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
    animation: Res<MechWalkingAnimation>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    for (entity, mut animation_player) in animation_players.iter_mut() {
        // Create a new animation graph
        let mut graph = AnimationGraph::new();
        let node_index = graph.add_clip(animation.0.clone(), 1.0, graph.root);

        // Add the graph to the entity
        let graph_handle = graphs.add(graph);
        commands.entity(entity).insert(graph_handle);

        // Play the animation
        animation_player.play(node_index).repeat();
    }
}
