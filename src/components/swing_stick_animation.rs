use bevy::prelude::*;

#[derive(Component)]
pub struct SwingStickAnimation {
    pub handle: Option<Handle<AnimationClip>>,
}

impl Default for SwingStickAnimation {
    fn default() -> Self {
        Self { handle: None }
    }
}
