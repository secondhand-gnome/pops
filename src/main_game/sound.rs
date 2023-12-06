use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_kira_audio::AudioSource;
use rand::Rng;
use std::time::Duration;

use crate::asset_loader::AudioAssets;

use super::kernel::PopEvent;

pub struct SoundPlugin;

const POP_MUTE_DURATION: Duration = Duration::from_millis(125);

#[derive(Resource)]
struct PopMute {
    timer: Timer,
}

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, play_pop).insert_resource(PopMute {
            timer: Timer::new(Duration::ZERO, TimerMode::Once),
        });
    }
}

fn play_pop(
    time: Res<Time>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
    mut ev_pop: EventReader<PopEvent>,
    mut pop_mute: ResMut<PopMute>,
) {
    pop_mute.timer.tick(time.delta());

    for _ in ev_pop.read() {
        if pop_mute.timer.finished() {
            audio.play(select_pop_sound(&audio_assets));
            debug!("Play pop sound");
            pop_mute.timer = Timer::new(POP_MUTE_DURATION, TimerMode::Once);
        }
    }
}

fn select_pop_sound(audio_assets: &Res<AudioAssets>) -> Handle<AudioSource> {
    let mut rng = rand::thread_rng();
    let pop_sounds = vec![&audio_assets.pop1, &audio_assets.pop2];
    let rand_index = rng.gen_range(0..pop_sounds.len());
    return pop_sounds[rand_index].clone();
}
