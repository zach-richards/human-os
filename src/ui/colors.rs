// colors.rs

struct Colors {
    flow: RGBA,
    focus: RGBA,
    neutral: RGBA,
    distracted: RGBA,
    fatigue: RGBA,
}

impl Colors {
    // Constructor for convenience
    fn new() -> Self {
        Self {
            flow: RGBA::new(96, 165, 250, 255),
            focus: RGBA::new(52, 211, 153, 255),
            neutral: RGBA::new(217, 231, 122, 255),
            distracted: RGBA::new(253, 230, 138, 255),
            fatigue: RGBA::new(248, 113, 113, 255),
        }
    }

    fn set(color: RGBA) {
    }
}
