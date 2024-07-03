
use rand::Rng;

#[derive(Copy, Clone, Debug)]
pub struct Color([u8; 3]);

#[derive(Clone, Copy)]
pub enum RGBShift {
    R,
    G,
    B
}

impl RGBShift {
    pub fn random() -> RGBShift {
        let mut rng = rand::thread_rng();

        match rng.gen_range(0..3) {
            0 => RGBShift::R,
            1 => RGBShift::G,
            _ => RGBShift::B,
        }
    }
}

impl Color {
    pub fn random() -> Color {
        let mut rng = rand::thread_rng();
        Color([
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
        ])
    }

    pub fn new_rgb(rgb: [u8; 3]) -> Color {
        Color(rgb)
    }

    pub fn shift(&mut self, by: RGBShift) {
        let [rr, gg, bb] = &mut self.0;

        let [a, b, c] = match by {
            RGBShift::R => [rr, gg, bb],
            RGBShift::G => [gg, bb, rr],
            RGBShift::B => [bb, rr, gg],
        };

        if let Some(result) = a.checked_add(1) {
            *a = result;
        } else if let Some(result) = b.checked_add(1) {
            *a = 0;
            *b = result;
        } else if let Some(result) = c.checked_add(1) {
            *b = 0;
            *c = result;
        } else {
            *c = 0;
        }
    }

    pub fn r(&self) -> u8 {
        self.0[0]
    }
    pub fn g(&self) -> u8 {
        self.0[1]
    }
    pub fn b(&self) -> u8 {
        self.0[2]
    }
}
