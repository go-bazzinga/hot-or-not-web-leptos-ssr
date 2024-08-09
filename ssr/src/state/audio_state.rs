use std::time::Duration;

use leptos::*;

#[derive(Clone, Copy, Debug)]
pub struct AudioState {
    pub show_mute_icon: RwSignal<bool>,
    pub muted: RwSignal<bool>,
}

impl Default for AudioState {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioState {
    pub fn new() -> Self {
        Self {
            show_mute_icon: create_rw_signal(true),
            muted: create_rw_signal(true),
        }
    }

    pub fn get() -> Self {
        let Self {
            muted,
            show_mute_icon,
        } = expect_context();

        set_timeout(
            move || {
                show_mute_icon.set(false);
            },
            Duration::from_secs(6),
        );

        Self {
            muted,
            show_mute_icon,
        }
    }
}
