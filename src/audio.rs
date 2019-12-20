use amethyst::{
    assets::{AssetStorage, Loader},
    audio::{
        Source,
        SourceHandle,
        OggFormat,
        output::Output,
        WavFormat,
    },
    ecs::{World, WorldExt},
};

use std::{
    iter::Cycle,
    ops::Deref,
    vec::IntoIter,
};

const OPENING_TRACKS: &[&str] = &[
    "audio/OpeningSelection.ogg",
];

const SELECT_OPTION_SOUND: &str = "audio/select_option.wav";

pub struct Music {
    pub opening: Cycle<IntoIter<SourceHandle>>,
}

pub struct Sounds {
    pub select_option_sfx: SourceHandle,
}

pub fn initialise_audio(world: &mut World) {
    let (sound_effects, music) = {
        let loader = world.read_resource::<Loader>();

        let opening = OPENING_TRACKS
            .iter()
            .map(|file| loader.load(*file, OggFormat, (), &world.read_resource()))
            .collect::<Vec<_>>()
            .into_iter()
            .cycle();

        let music = Music {
            opening,
        };

        let sound_effects = Sounds {
            select_option_sfx: loader.load(SELECT_OPTION_SOUND, WavFormat, (), &world.read_resource()),
        };

        (sound_effects, music)
    };

    world.insert(sound_effects);
    world.insert(music);
}

pub fn play_select_option_sound<O>(
    sounds: &Sounds,
    storage: &AssetStorage<Source>,
    output: &Option<O>,
)
    where
        O: Deref<Target = Output>,
{
    match (storage.get(&sounds.select_option_sfx), output) {
        (Some(sound), Some(output)) => output.play_once(sound, 1.0),
        _ => {},
    }
}
