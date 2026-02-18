// cognitive_model.rs

fn calc_score() {
    FocusScore =
        (NormKPS * 0.40) + // keystrokes a min (capped at 5 KPS)
        (SwitchScore * 0.30) + // amount of switching
        (IdleScore * 0.20) + // idling
        (BackspaceScore * 0.10) // amount of backspacing
}

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
        score: 0.50,
        state: FocusState::from_score(score);
    }

    pub update() {
        
    }
}
