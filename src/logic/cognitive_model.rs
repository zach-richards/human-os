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


enum FocusState {
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
    score: f32,
    state: FocusState,
}

impl CognitiveModel {
    pub fn new() -> Self {
        Self {
            score: 0.50,
            state: FocusState::from_score(score);
        }
    }

    fn calc_score() -> f32 {
        let norm_kps: f32 = min(sys_info.kps / 5.0, 1.0);

        let norm_switch: f32 = min(SwitchRate / 3.0, 1.0);
        let switch_score: f32 = 1 - norm_switch;

        let idle_ratio: f32 = idle_seconds / 60;
        let idle_score: f32 = 1 - idle_ratio;

        let backspace_ratio: f32 = sys_info.backspace_count / kps;
        let norm_backspace: f32 = min(backspace_ratio / 0.25, 1.0);
        let backspace_score: f32 = 1 - norm_backspace;

        FocusScore =
            (norm_kps * 0.40) + // keystrokes a min (capped at 5 KPS)
            (SwitchScore * 0.30) + // amount of switching
            (IdleScore * 0.20) + // idling
            (BackspaceScore * 0.10) // amount of backspacing
    }

    pub fn update() {
        score = calc_score(score);
        state = FocusState::from_score(score);
    }
}
