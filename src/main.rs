use macroquad::{self as mq, *};
use noise::{NoiseFn, OpenSimplex};
use ncollide2d::pipeline::world::CollisionWorld;
use std::process::exit;

struct CollisionObjectData {
    pub velocity: Option<Vec2>
}

#[macroquad::main("TechDemo")]
async fn main() {
    let background_color = mq::Color::new(0.3, 0.89, 0.56, 1.0);
    let mut score = 0 as usize;
    let speed = 0.3;
    let mut last_update = get_time();

    let inner_proportion = 0.2;
    let outer_proportion = 0.5;

    let noise = OpenSimplex::new();

    let mut world = CollisionWorld<f32, CollisionObjectData>::new(0.02);

    struct Spikey {
        vertices: Vec<Vec2>
    }

    // Add hull

    // Add Balls

    loop {
        if is_key_down(KeyCode::Escape) {
            exit(0);
        }

        let screen_center = Vec2::new(screen_width() / 2., screen_height() / 2.);
        let width = screen_width().min(screen_height());
        let noise_coef = 0.4;
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
        let mut hull = Vec::new();
        for i in 0..segments {
            // get angle
            let angle = (i as f32 / segments as f32) * 2.0 * std::f32::consts::PI;

            // get distance with noise on angle and time
            let distance = outer_proportion * width * noise_coef * noise.get([current_time, 10.0 * angle as f64]).abs() as f32 + inner_proportion * width;

            // draw point
            let cx = distance * angle.sin() + screen_center.x();
            let cy = distance * angle.cos() + screen_center.y();
            hull.push([cx, cy]);

            draw_line(cx, cy, screen_center.x(), screen_center.y(), 6.0, YELLOW);

            if i > 0 {
                draw_line(cx, cy, hull[i - 1][0], hull[i - 1][1], 6.0, BLUE);
            }
        }
        draw_line(hull[0][0], hull[0][1], hull[19][0], hull[19][1], 6.0, BLUE);


        mq::next_frame().await
    }
}
