use amethyst::{
    ecs::{Component, DenseVecStorage},
};

pub struct Ball {
    pub radius: f32,
    pub velocity: [f32; 2],
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}
