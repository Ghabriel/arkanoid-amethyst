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
    collections::HashMap,
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

#[derive(Eq, Hash, PartialEq)]
pub enum Sound {
    Bounce,
    GameOver,
    SelectOption,
}

pub struct SoundStorage {
    sounds: HashMap<Sound, SourceHandle>,
}

pub fn initialise_audio(world: &mut World) {
    let (sound_storage, music) = {
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

        let mut sound_storage = SoundStorage { sounds: HashMap::new() };
        sound_storage.sounds.insert(
            Sound::Bounce,
            loader.load(BOUNCE_SOUND, OggFormat, (), &world.read_resource()),
        );

        sound_storage.sounds.insert(
            Sound::GameOver,
            loader.load(GAMEOVER_SOUND, OggFormat, (), &world.read_resource()),
        );

        sound_storage.sounds.insert(
            Sound::SelectOption,
            loader.load(SELECT_OPTION_SOUND, WavFormat, (), &world.read_resource()),
        );

        (sound_storage, music)
    };

    world.insert(sound_storage);
    world.insert(music);
}

pub fn play_sound<O>(
    sound: Sound,
    sound_storage: &SoundStorage,
    asset_storage: &AssetStorage<Source>,
    output: &Option<O>,
)
    where
        O: Deref<Target = Output>,
{
    let handle = sound_storage.sounds.get(&sound).unwrap();
    match (asset_storage.get(&handle), output) {
        (Some(sound), Some(output)) => output.play_once(sound, 1.0),
        _ => {},
    }
}
