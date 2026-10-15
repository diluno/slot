use crate::layout::Canvas;
use crate::pipeline::{SRC_H, SRC_W};
use crate::surface::OUT_H;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct GameFit {
    pub rect: (i32, i32, i32, i32),
    pub source: [f32; 4],
    pub scale: f32,
}

impl GameFit {
    pub fn size(&self) -> (u32, u32) {
        (self.rect.2.max(1) as u32, self.rect.3.max(1) as u32)
    }

    pub fn canvas(&self) -> Canvas {
        let (w, h) = self.size();
        Canvas {
            w: OUT_H as f32 * w as f32 / h as f32,
            h: OUT_H as f32,
        }
    }
}

pub fn game_fit(panel: (u32, u32), picture: (u32, u32), integer: bool) -> GameFit {
    let (pw, ph) = (picture.0.clamp(1, SRC_W), picture.1.clamp(1, SRC_H));
    let fill = (panel.0 as f32 / pw as f32).min(panel.1 as f32 / ph as f32);
    let scale = match integer {
        true => fill.floor().max(1.0),
        false => fill,
    };
    let (w, h) = (
        (pw as f32 * scale).round() as i32,
        (ph as f32 * scale).round() as i32,
    );
    GameFit {
        rect: ((panel.0 as i32 - w) / 2, (panel.1 as i32 - h) / 2, w, h),
        source: [
            (SRC_W - pw) as f32 / 2.0 / SRC_W as f32,
            (SRC_H - ph) as f32 / 2.0 / SRC_H as f32,
            pw as f32 / SRC_W as f32,
            ph as f32 / SRC_H as f32,
        ],
        scale,
    }
}
