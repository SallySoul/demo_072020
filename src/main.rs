use macroquad::{self as mq};

mod noise_circle;

enum GameState {
    MainMenu(MainMenu),
    NoiseCircle(noise_circle::NoiseCircle)
}

struct MainMenu {
}

impl MainMenu {
    pub fn new() -> MainMenu { return MainMenu {}; }

    pub fn update(&self) {
    }
}

#[macroquad::main("TechDemo")]
async fn main() {
    let mut game_state = GameState::MainMenu(MainMenu::new());

    loop {
        match &mut game_state {
            GameState::MainMenu(m) => m.update(),
            GameState::NoiseCircle(m) => m.update(),
        }

        mq::next_frame().await
    }
}
