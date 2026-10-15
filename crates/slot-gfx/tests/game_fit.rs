use slot_gfx::game_fit;

#[test]
fn integer_puts_a_gba_picture_at_2x_on_the_rg28xx() {
    let fit = game_fit((640, 480), (240, 160), true);
    assert_eq!(fit.rect, (80, 80, 480, 320));
    assert_eq!(fit.source, [0.0, 0.0, 1.0, 1.0]);
    assert_eq!(fit.canvas(), slot_gfx::Canvas { w: 720.0, h: 480.0 });
}

#[test]
fn integer_puts_a_game_boy_picture_at_3x_and_crops_the_margin() {
    let fit = game_fit((640, 480), (160, 144), true);
    assert_eq!(fit.rect, (80, 24, 480, 432));
    assert_eq!(
        fit.source,
        [40.0 / 240.0, 8.0 / 160.0, 160.0 / 240.0, 144.0 / 160.0]
    );
    assert_eq!(fit.canvas().h, 480.0);
    assert!((fit.canvas().w - 533.33).abs() < 0.01);
}

#[test]
fn aspect_fills_the_width_for_gba() {
    let fit = game_fit((640, 480), (240, 160), false);
    assert_eq!(fit.rect, (0, 26, 640, 427));
}

#[test]
fn aspect_fills_the_height_for_game_boy() {
    let fit = game_fit((640, 480), (160, 144), false);
    assert_eq!(fit.rect, (53, 0, 533, 480));
}

#[test]
fn integer_never_drops_below_1x() {
    let fit = game_fit((200, 100), (240, 160), true);
    assert_eq!(fit.scale, 1.0);
}
