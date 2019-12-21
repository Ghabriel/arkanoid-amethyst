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

const OPENING_TRACK: &str = "audio/OpeningSelection.ogg";
const IN_GAME_TRACKS: &[&str] = &[
    "audio/Computer_Music_All-Stars_-_Wheres_My_Jetpack.ogg",
    "audio/Computer_Music_All-Stars_-_Albatross_v2.ogg",
];

const SELECT_OPTION_SOUND: &str = "audio/select_option.wav";
const BOUNCE_SOUND: &str = "audio/bounce.ogg";
const GAMEOVER_SOUND: &str = "audio/gameover.ogg";

pub struct Music {
    pub opening: SourceHandle,
    pub in_game: Cycle<IntoIter<SourceHandle>>,
}

pub struct Sounds {
    pub select_option_sfx: SourceHandle,
    pub bounce_sfx: SourceHandle,
    pub gameover_sfx: SourceHandle,
}

pub fn initialise_audio(world: &mut World) {
    let (sound_effects, music) = {
        let loader = world.read_resource::<Loader>();

        let opening = loader.load(OPENING_TRACK, OggFormat, (), &world.read_resource());

        let in_game = IN_GAME_TRACKS
            .iter()
            .map(|file| loader.load(*file, OggFormat, (), &world.read_resource()))
            .collect::<Vec<_>>()
            .into_iter()
            .cycle();

        let music = Music {
            opening,
            in_game,
        };

        let sound_effects = Sounds {
            select_option_sfx: loader.load(SELECT_OPTION_SOUND, WavFormat, (), &world.read_resource()),
            bounce_sfx: loader.load(BOUNCE_SOUND, OggFormat, (), &world.read_resource()),
            gameover_sfx: loader.load(GAMEOVER_SOUND, OggFormat, (), &world.read_resource()),
        };

        (sound_effects, music)
    };

    world.insert(sound_effects);
    world.insert(music);
}

pub fn play_sound<O>(
    sound: &SourceHandle,
    storage: &AssetStorage<Source>,
    output: &Option<O>,
)
    where
        O: Deref<Target = Output>,
{
    match (storage.get(&sound), output) {
        (Some(sound), Some(output)) => output.play_once(sound, 1.0),
        _ => {},
    }
}
