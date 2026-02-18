// cognitive_model.rs

enum FocusState {
    Flow,
    Focus,
    Neutral,
    Distracted,
    Fatigue/Strain,
}

impl FocusState {
    pub fn from_score(score: f32) -> Self {
        use FocusState::*;

        match score {
            0.80..=1.0 => Flow,
            0.60..0.79 => Focus,
            0.40..0.59 => Neutral,
            0.20..0.39 => Distracted,
            _ => Fatigue/Strain,
        }
    }
}

struct CognitiveModel {
}V
