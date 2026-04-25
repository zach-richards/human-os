use std::time::Instant;

use crate::sys::system;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusState {
    Flow,
    Focus,
    Neutral,
    Distracted,
    Fatigued,
}

impl FocusState {
    fn from_score(score: f32, prev: FocusState) -> Self {
        // hysteresis thresholds (prevents flicker)
        let (up_flow, down_flow) = (0.82, 0.78);
        let (up_focus, down_focus) = (0.65, 0.60);
        let (up_neutral, down_neutral) = (0.45, 0.40);
        let (up_distracted, down_distracted) = (0.25, 0.20);

        match prev {
            FocusState::Flow => {
                if score < down_flow { FocusState::Focus } else { FocusState::Flow }
            }
            FocusState::Focus => {
                if score > up_flow { FocusState::Flow }
                else if score < down_focus { FocusState::Neutral }
                else { FocusState::Focus }
            }
            FocusState::Neutral => {
                if score > up_focus { FocusState::Focus }
                else if score < down_neutral { FocusState::Distracted }
                else { FocusState::Neutral }
            }
            FocusState::Distracted => {
                if score > up_neutral { FocusState::Neutral }
                else if score < down_distracted { FocusState::Fatigued }
                else { FocusState::Distracted }
            }
            FocusState::Fatigued => {
                if score > up_distracted { FocusState::Distracted }
                else { FocusState::Fatigued }
            }
        }
    }
}

pub struct CognitiveModel {
    pub score: f32,
    pub state: FocusState,

    pub focus_over_time: Vec<(Instant, f32)>,
    pub last_update: Instant,
    last_keys: u32,
    last_switches: u32,
    last_backspaces: u32,
}

impl CognitiveModel {
    pub fn new() -> Self {
        Self {
            score: 0.5,
            state: FocusState::Neutral,
            focus_over_time: Vec::new(),
            last_update: Instant::now(),
            last_keys: 0,
            last_switches: 0,
            last_backspaces: 0,
        }
    }

    fn smooth(prev: f32, target: f32, alpha: f32) -> f32 {
        prev + alpha * (target - prev)
    }

    fn calc_score(&self, sys: &system::SystemInfo) -> f32 {
        let now = Instant::now();
        let dt = (now - self.last_update).as_secs_f32().max(0.001);

        // =========================
        // DELTA-BASED ACTIVITY
        // =========================
        let key_delta = sys.key_count.saturating_sub(self.last_keys) as f32;
        let switch_delta = sys.window_switch_count.saturating_sub(self.last_switches) as f32;
        let backspace_delta = sys.backspace_count.saturating_sub(self.last_backspaces) as f32;

        let key_rate = key_delta / dt;
        let switch_rate = switch_delta / dt;

        // =========================
        // 1. TYPING ACTIVITY (slightly less damped → easier Flow)
        // =========================
        let typing_pressure = (key_rate / 8.0).tanh();

        // =========================
        // 2. WINDOW SWITCHING
        // =========================
        let switch_pressure = (switch_rate / 2.5).tanh();
        let switch_score = 1.0 - switch_pressure;

        // =========================
        // 3. IDLE TIME (slightly more forgiving → easier Flow)
        // =========================
        let idle_seconds = sys
            .last_activity
            .map(|t| now.duration_since(t).as_secs_f32())
            .unwrap_or(30.0);

        let idle_score = 1.0 - (idle_seconds / 24.0).tanh();

        // =========================
        // 4. EDITING BEHAVIOR
        // =========================
        let backspace_ratio = if key_delta == 0.0 {
            0.0
        } else {
            backspace_delta as f32 / key_delta
        };

        let mistake_score = 1.0 - (backspace_ratio * 2.0).tanh();

        // =========================
        // FINAL WEIGHTED MODEL (slightly Flow-biased)
        // =========================
        let raw =
            typing_pressure * 0.58 +
            switch_score * 0.19 +
            idle_score * 0.20 +
            mistake_score * 0.03;

        raw.clamp(0.0, 1.0)
    }

    pub fn update(&mut self, sys: &system::SystemInfo) {
        let target = self.calc_score(sys);

        self.score = Self::smooth(self.score, target, 0.25);

        self.state = FocusState::from_score(self.score, self.state);

        self.last_update = Instant::now();
        self.focus_over_time.push((self.last_update, self.score));
        self.last_keys = sys.key_count;
        self.last_switches = sys.window_switch_count;
        self.last_backspaces = sys.backspace_count;
    }
}