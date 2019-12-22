use amethyst::{
    assets::{PrefabData, ProgressCounter},
    core::Transform,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    Error,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PrefabData)]
pub struct BrickPrefab {
    brick: Brick,
    transform: Transform,
    // SpriteRender can't be present in a prefab, so it's added later, in GameState.
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum BrickKind {
    Standard,
    FastForward,
    BallSplit,
}

#[derive(Clone, Debug, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct Brick {
    pub kind: BrickKind,
    pub width: f32,
    pub height: f32,
}

impl Component for Brick {
    type Storage = DenseVecStorage<Self>;
}
