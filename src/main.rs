use macroquad::{self as mq, *};
use std::process::exit;

#[macroquad::main("TechDemo")]
async fn main() {
    let background_color = mq::Color::new(0.3, 0.89, 0.56, 1.0);
    let mut score = 0;
    let speed = 0.3;
    let mut last_update = get_time();

    loop {
        if is_key_down(KeyCode::Escape) {
            exit(0);
        }


        let delta = get_time() - last_update;
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

        mq::next_frame().await
    }
}
