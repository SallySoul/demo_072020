use macroquad::{self as mq};

mod noise_circle;

#[macroquad::main("TechDemo")]
async fn main() {
    let mut app = noise_circle::NoiseCircle::new();

    loop {
        app.update();

        mq::next_frame().await
    }
}
