use macroquad::{self as mq, *};
use noise::{NoiseFn, OpenSimplex};
use std::process::exit;

#[macroquad::main("TechDemo")]
async fn main() {
    let background_color = mq::Color::new(0.3, 0.89, 0.56, 1.0);
    let mut score = 0;
    let speed = 0.3;
    let mut last_update = get_time();

    let noise = OpenSimplex::new();

    loop {
        if is_key_down(KeyCode::Escape) {
            exit(0);
        }

        let screen_center = Vec2::new(screen_width() / 2., screen_height() / 2.);
        let current_time = get_time();
        let delta = current_time - last_update;
        if delta > speed {
            last_update = get_time();
            score += 1;
        }

        mq::clear_background(background_color);

        draw_text(
            format!("SCORE: {}", score).as_str(),
            10.,
            10.,
            20.,
            DARKGRAY,
        );

        let segments: usize = 20;
        for i in 0..segments {
            // get angle
            let angle = (i as f32 / segments as f32) * 2.0 * std::f32::consts::PI;

            // get distance with noise on angle and time
            let distance = 300.0 * noise.get([current_time, 10.0 * angle as f64]).abs() as f32 + 250.0;

            // draw point
            let cx = distance * angle.sin() + screen_center.x();
            let cy = distance * angle.cos() + screen_center.y();
            draw_circle(
                cx,
                cy,
                10.0,
                YELLOW,
            );
        }

        mq::next_frame().await
    }
}
