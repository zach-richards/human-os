// cognitive_model.rs

/*

KPS = keystrokes per second (over 60 seconds)
NormKPS = min(KPS / 5.0, 1.0)

SwitchRate = switches per minute
NormSwitch = min(SwitchRate / 3.0, 1.0)
SwitchScore = 1 - NormSwitch

IdleRatio = idle_seconds / 60
IdleScore = 1 - IdleRatio

BackspaceRatio = backspaces / total_keystrokes
NormBackspace = min(BackspaceRatio / 0.25, 1.0)
BackspaceScore = 1 - NormBackspace

*/

use std::time::{ Duration, Instant };

use crate::sys::system;

#[derive(Debug)]
pub enum FocusState {
    Flow,
    Focus,
    Neutral,
    Distracted,
    Fatigued,
}

impl FocusState {
    fn from_score(score: f32) -> Self {
        use FocusState::*;

        match score {
            0.80..=1.0 => Flow,
            0.60..0.79 => Focus,
            0.40..0.59 => Neutral,
            0.20..0.39 => Distracted,
            _ => Fatigued,
        }
    }
}

pub struct CognitiveModel {
    pub score: f32,
    pub state: FocusState,
}

impl CognitiveModel {
    pub fn new() -> Self {
        let initial_score = 0.50;
        Self {
            score: initial_score,
            state: FocusState::from_score(initial_score),
        }
    }

    fn calc_score(&self, sys_info: &system::SystemInfo) -> f32 {
        // Keystrokes per second normalized
        let norm_kps: f32 = (sys_info.key_count as f32 / 5.0).min(1.0);

        // Window switching normalized
        let norm_switch: f32 = (sys_info.window_switch_count as f32 / 3.0).min(1.0);
        let switch_score: f32 = 1.0 - norm_switch;

        // Idle time score
        let now = Instant::now();
        let last = sys_info.last_activity.unwrap_or_else(Instant::now);
        let idle_seconds = sys_info
            .last_activity
            .map(|last| now.duration_since(last).as_secs())
            .unwrap_or(0);
        let idle_ratio: f32 = (idle_seconds as f32 / 60.0).min(1.0); // cap at 1.0
        let idle_score: f32 = 1.0 - idle_ratio;

        // Backspace ratio score
        let backspace_ratio: f32 = sys_info.backspace_count as f32 / sys_info.key_count as f32;
        let norm_backspace: f32 = (backspace_ratio / 0.25).min(1.0);
        let backspace_score: f32 = 1.0 - norm_backspace;

        // Weighted sum
        (norm_kps * 0.40) + (switch_score * 0.30) + (idle_score * 0.20) + (backspace_score * 0.10)
    }

    pub fn update(&mut self, sys_info: &system::SystemInfo) {
        self.score = self.calc_score(sys_info);
        self.state = FocusState::from_score(self.score);
    }

    pub fn print(&self) {
        println!("Focus Score: {:.2}", self.score);
        println!("Focus State: {:?}\n", self.state);
    }
}
