use rand::Rng;

pub struct Aimbot {
    sensitivity: f32,
}

impl Aimbot {
    pub fn new(sensitivity: f32) -> Self {
        Aimbot { sensitivity }
    }

    pub fn aim(&self, target_x: f32, target_y: f32) -> (f32, f32) {
        let mut rng = rand::thread_rng();
        let offset_x = rng.gen_range(-self.sensitivity..self.sensitivity);
        let offset_y = rng.gen_range(-self.sensitivity..self.sensitivity);
        (target_x + offset_x, target_y + offset_y)
    }
}

pub struct ESP {
    enabled: bool,
}

impl ESP {
    pub fn new() -> Self {
        ESP { enabled: false }
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

pub struct SpeedHack {
    multiplier: f32,
}

impl SpeedHack {
    pub fn new(multiplier: f32) -> Self {
        SpeedHack { multiplier }
    }

    pub fn apply(&self, base_speed: f32) -> f32 {
        base_speed * self.multiplier
    }
}