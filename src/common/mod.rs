mod config;
mod files;
mod province;

pub use config::*;
pub use files::*;
pub use province::*;

use rand::Rng;

#[derive(Copy, Clone, Debug)]
pub struct Color([u8; 3]);

impl Color {
    pub fn random() -> Color {
        let mut rng = rand::thread_rng();
        Color([
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
        ])
    }

    pub fn shift(&mut self) {
        let [r, g, b] = &mut self.0;

        if let Some(result) = r.checked_add(1) {
            *r = result;
        } else if let Some(result) = g.checked_add(1) {
            *r = 0;
            *g = result;
        } else if let Some(result) = b.checked_add(1) {
            *g = 0;
            *b = result;
        } else {
            *b = 0;
        }
    }

    pub fn r(&self) -> u8 { self.0[0] }
    pub fn g(&self) -> u8 { self.0[1] }
    pub fn b(&self) -> u8 { self.0[2] }
}