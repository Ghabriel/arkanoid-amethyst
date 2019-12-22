use amethyst::{
    assets::{AssetStorage, Loader},
    audio::{
        Source,
        SourceHandle,
        OggFormat,
        output::Output,
        WavFormat,
    },
    ecs::{
        Read,
        ReadExpect,
        SystemData,
        World,
        WorldExt,
    },
    shred::ResourceId,
};

use std::{
    collections::HashMap,
    iter::Cycle,
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

pub struct SoundKit<'a> {
    asset_storage: Read<'a, AssetStorage<Source>>,
    sound_storage: ReadExpect<'a, SoundStorage>,
    output: Option<Read<'a, Output>>,
}

impl<'a> SoundKit<'a> {
    pub fn from_world(world: &World) -> SoundKit {
        SoundKit::fetch(world)
    }
}

impl<'a> SystemData<'a> for SoundKit<'a> {
    fn setup(world: &mut World) {
        <Read<'a, AssetStorage<Source>> as SystemData>::setup(world);
        <ReadExpect<'a, SoundStorage> as SystemData>::setup(world);
        <Option<Read<'a, Output>> as SystemData>::setup(world);
    }

    fn fetch(world: &'a World) -> Self {
        SoundKit {
            asset_storage: <Read<'a, AssetStorage<Source>> as SystemData<'a>>::fetch(world),
            sound_storage: <ReadExpect<'a, SoundStorage> as SystemData<'a>>::fetch(world),
            output: <Option<Read<'a, Output>> as SystemData<'a>>::fetch(world),
        }
    }

    fn reads() -> Vec<ResourceId> {
        let mut r = Vec::new();

        let mut reads = <Read<'a, AssetStorage<Source>> as SystemData>::reads();
        r.append(&mut reads);
        let mut reads = <ReadExpect<'a, SoundStorage> as SystemData>::reads();
        r.append(&mut reads);
        let mut reads = <Option<Read<'a, Output>> as SystemData>::reads();
        r.append(&mut reads);

        r
    }

    fn writes() -> Vec<ResourceId> {
        let mut r = Vec::new();

        let mut writes = <Read<'a, AssetStorage<Source>> as SystemData>::writes();
        r.append(&mut writes);
        let mut writes = <ReadExpect<'a, SoundStorage> as SystemData>::writes();
        r.append(&mut writes);
        let mut writes = <Option<Read<'a, Output>> as SystemData>::writes();
        r.append(&mut writes);

        r
    }
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

pub fn play_sound(sound: Sound, sound_kit: &SoundKit) {
    let SoundKit {
        asset_storage,
        sound_storage,
        output,
    } = sound_kit;
    let handle = sound_storage.sounds.get(&sound).unwrap();
    match (asset_storage.get(&handle), output) {
        (Some(sound), Some(output)) => output.play_once(sound, 1.0),
        _ => {},
    }
}
