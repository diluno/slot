#![cfg(target_os = "macos")]

use slot_gfx::{Compositor, Draw, HeadlessSurface, Layout, ScreenEffect, SRC_H, SRC_W};
use std::sync::{Mutex, MutexGuard, PoisonError};

static GL: Mutex<()> = Mutex::new(());

fn compositor() -> Option<(MutexGuard<'static, ()>, HeadlessSurface, Compositor)> {
    let guard = GL.lock().unwrap_or_else(PoisonError::into_inner);
    let surface = HeadlessSurface::new().ok()?;
    let compositor = Compositor::new(&surface).ok()?;
    Some((guard, surface, compositor))
}

fn texel(x: u32, y: u32) -> [u8; 3] {
    [(x * 7) as u8 | 1, (y * 11) as u8 | 1, 200]
}

fn pattern() -> Vec<u8> {
    (0..SRC_W * SRC_H)
        .flat_map(|i| {
            let [r, g, b] = texel(i % SRC_W, i / SRC_W);
            [b, g, r, 255]
        })
        .collect()
}

fn panel_with(c: &mut Compositor, picture: (u32, u32), integer: bool) -> Vec<u8> {
    c.fit((640, 480));
    c.set_screen_effect(ScreenEffect::None);
    c.set_screen_power(1.0);
    c.set_game_fit(picture, integer);
    c.begin_frame(Layout::Framed);
    c.upload_game(&pattern());
    c.draw_list(&[Draw::Game]);
    c.read_window((640, 480))
}

fn at(frame: &[u8], x: u32, y: u32) -> [u8; 3] {
    let i = ((y * 640 + x) * 4) as usize;
    [frame[i], frame[i + 1], frame[i + 2]]
}

#[test]
fn integer_draws_every_gba_texel_as_a_2x2_block_in_the_middle() {
    let Some((_g, _s, mut c)) = compositor() else {
        return;
    };
    let frame = panel_with(&mut c, (240, 160), true);
    assert_eq!(at(&frame, 79, 200), [0, 0, 0]);
    assert_eq!(at(&frame, 560, 200), [0, 0, 0]);
    assert_eq!(at(&frame, 300, 79), [0, 0, 0]);
    for (tx, ty) in [(0, 0), (1, 1), (37, 90), (239, 159)] {
        for (dx, dy) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
            assert_eq!(
                at(&frame, 80 + tx * 2 + dx, 80 + ty * 2 + dy),
                texel(tx, ty),
                "texel ({tx}, {ty}) corner ({dx}, {dy})"
            );
        }
    }
}

#[test]
fn integer_draws_the_game_boy_picture_at_3x_without_its_margin() {
    let Some((_g, _s, mut c)) = compositor() else {
        return;
    };
    let frame = panel_with(&mut c, (160, 144), true);
    assert_eq!(at(&frame, 79, 240), [0, 0, 0]);
    assert_eq!(at(&frame, 320, 23), [0, 0, 0]);
    for (gx, gy) in [(0, 0), (159, 143), (80, 70)] {
        for d in 0..3 {
            assert_eq!(
                at(&frame, 80 + gx * 3 + d, 24 + gy * 3 + d),
                texel(40 + gx, 8 + gy),
                "gb pixel ({gx}, {gy})"
            );
        }
    }
}

#[test]
fn aspect_fill_spans_the_whole_height_for_game_boy() {
    let Some((_g, _s, mut c)) = compositor() else {
        return;
    };
    let frame = panel_with(&mut c, (160, 144), false);
    assert_eq!(at(&frame, 320, 0), texel(120, 8));
    assert_eq!(at(&frame, 320, 479), texel(120, 151));
    assert_eq!(at(&frame, 52, 240), [0, 0, 0]);
    assert_ne!(at(&frame, 54, 240), [0, 0, 0]);
}
