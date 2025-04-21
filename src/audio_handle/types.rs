use kira::{
    effect::filter::{FilterBuilder, FilterHandle, FilterMode},
    effect::EffectBuilder,
    modulator::tweener::{TweenerBuilder, TweenerHandle},
    sound::static_sound::StaticSoundData,
    track::{TrackBuilder, TrackHandle},
    AudioManager, AudioManagerSettings, Easing, Mapping, Tween, Value,
};
use std::{collections::HashMap, time::Duration};

pub struct AudioHandle {
    pub manager: AudioManager,
    // hashmap of tracks.
    tracks: HashMap<String, TrackEffectHandle>,
}

pub struct TrackEffectHandle {
    pub handle: TrackHandle,
    pub tmp_value: f64,
    pub tweener: TweenerHandle,
}

impl AudioHandle {
    pub fn new() -> Self {
        let manager = AudioManager::new(AudioManagerSettings::default()).unwrap();
        Self {
            manager,
            tracks: HashMap::new(),
        }
    }

    pub fn add_audio(&mut self) {
        let initial_value = 0.0;
        let tweener = self
            .manager
            .add_modulator(TweenerBuilder { initial_value })
            .unwrap();

        let filter_builder = FilterBuilder::new().cutoff(Value::from_modulator(
            &tweener,
            Mapping {
                input_range: (0.0, 1.0),
                output_range: (500.0, 20_000.0),
                easing: Easing::Linear,
            },
        ));
        let mut track = self
            .manager
            .add_sub_track(TrackBuilder::new().with_effect(filter_builder))
            .unwrap();

        let hash_code = "aladdin.ogg";
        let sound_data = StaticSoundData::from_file("./assets/aladdin.ogg");
        match sound_data {
            Ok(sound_data) => match track.play(sound_data) {
                Ok(_) => {
                    println!("Sound played successfully");
                    track.set_volume(1.0, Tween::default());
                    self.tracks.insert(
                        hash_code.to_string(),
                        TrackEffectHandle {
                            handle: track,
                            tmp_value: initial_value,
                            tweener,
                        },
                    );
                }
                Err(err) => {
                    println!("Error playing sound: {}", err);
                }
            },
            Err(err) => {
                println!("Error loading sound data: {}", err);
            }
        }
    }

    pub fn change_cutoff(&mut self) {
        let hash = "aladdin.ogg";
        let track_effect_handle = self.tracks.get_mut(hash).unwrap();
        track_effect_handle.tmp_value += 0.1;
        println!("New cutoff value: {}", track_effect_handle.tmp_value);
        track_effect_handle.tweener.set(
            track_effect_handle.tmp_value,
            Tween {
                duration: Duration::from_secs(3),
                ..Default::default()
            },
        )
    }
}
