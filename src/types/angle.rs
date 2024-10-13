
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Angle(u8);

impl Angle {

    pub fn new(value: u8) -> Self {
        Self(value)
    }

    pub fn from_degrees(degrees: f32) -> Self {
        let normalized_degrees = degrees % 360.0;
        let steps = (normalized_degrees * 256.0 / 360.0).round() as u8;
        Angle(steps)
    }

    pub fn to_degrees(self) -> f32 {
        (self.0 as f32 * 360.0 / 256.0).rem_euclid(360.0)
    }

    pub fn as_steps(self) -> u8 {
        self.0
    }

    pub fn rotate(&mut self, degrees: f32) {
        let new_angle = self.to_degrees() + degrees;
        *self = Angle::from_degrees(new_angle);
    }
}
