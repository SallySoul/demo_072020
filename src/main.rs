use macroquad::{self as mq, *};
use noise::{NoiseFn, OpenSimplex};
use std::process::exit;

fn to_mq_color(c: &[f32; 4]) -> mq::Color {
    mq::Color::new(c[0], c[1], c[2], c[3])
}

fn color_from_u32(c: u32) -> [f32; 4] {
    megaui::Color::from_rgb_u32(c).into()
}

#[macroquad::main("TechDemo")]
async fn main() {
    let noise = OpenSimplex::new();
    let mut last_time = get_time();

    // Settings Window
    let settings_window_id = hash!();

    // f32 Settings
    let mut segments = 20usize;
    let mut segments_f32 = 20f32;
    let mut inner_proportion = 0.2;
    let mut outer_proportion = 0.5;
    let mut noise_coefficient = 0.4;
    let mut scaling_coefficient = 1.0;
    let mut sim_time_velocity = 0.8;
    let mut angle_noise_coefficient = 10.0;
    let mut current_time_sim = get_time();

    // color Settings
    // https://coolors.co/efd9ce-dec0f1-b79ced-957fef-7161ef
    let mut background_color = color_from_u32(0xEFD9CE);
    let mut border_color = color_from_u32(0xDEC0F1);
    let mut inner_color = color_from_u32(0xB79CED);

    loop {
        if is_key_down(KeyCode::Escape) {
            exit(0);
        }

        let screen_center = Vec2::new(screen_width() / 2., screen_height() / 2.);
        let width_min = screen_width().min(screen_height());
        let width_max = screen_width().max(screen_height());

        let current_time = get_time();
        let delta = current_time - last_time;
        last_time = current_time;
        current_time_sim += sim_time_velocity as f64 * delta;

        mq::clear_background(to_mq_color(&background_color));

        let mut hull = Vec::new();
        for i in 0..segments {
            // get angle
            let angle = (i as f32 / segments as f32) * 2.0 * std::f32::consts::PI;

            // get distance with noise on angle and time
            let distance = outer_proportion
                * width_min
                * noise_coefficient
                * noise
                    .get([
                        current_time_sim,
                        (angle_noise_coefficient * angle.cos()) as f64,
                        (angle_noise_coefficient * angle.sin()) as f64,
                    ])
                    .abs() as f32
                + inner_proportion * width_max;

            // draw point
            let cx = distance * angle.sin() + screen_center.x();
            let cy = distance * angle.cos() + screen_center.y();
            hull.push([cx, cy]);

            draw_line(
                cx,
                cy,
                screen_center.x(),
                screen_center.y(),
                6.0,
                to_mq_color(&inner_color),
            );

            if i > 0 {
                draw_line(
                    cx,
                    cy,
                    hull[i - 1][0],
                    hull[i - 1][1],
                    6.0,
                    to_mq_color(&border_color),
                );
            }
        }
        draw_line(
            hull[0][0],
            hull[0][1],
            hull[segments - 1][0],
            hull[segments - 1][1],
            6.0,
            to_mq_color(&border_color),
        );

        draw_window(
            settings_window_id,
            vec2(screen_width() - 300.0, 0.0),
            vec2(300., 300.),
            WindowParams {
                label: "Settings".to_string(),
                ..Default::default()
            },
            |ui| {
                ui.tree_node(hash!(), "Coefficients", |ui| {
                    // segments
                    ui.slider(hash!(), "[10 .. 200]", 10f32..200f32, &mut segments_f32);
                    segments = segments_f32.ceil() as usize;
                    ui.label(None, &format!(" ^ segments {}", segments));
                    ui.separator();

                    // inner_proportion
                    ui.slider(hash!(), "[0 .. 1]", 0f32..1.0, &mut inner_proportion);
                    ui.label(None, &format!(" ^ Inner Proportion"));
                    ui.separator();

                    // outer_proportion
                    ui.slider(
                        hash!(),
                        &format!("[{} .. 1.0]", inner_proportion),
                        inner_proportion..1f32,
                        &mut outer_proportion,
                    );
                    ui.label(None, &format!(" ^ Outer Proportion"));
                    ui.separator();

                    // noise_coefficient
                    ui.slider(hash!(), "[0 .. 1]", 0f32..1.0, &mut noise_coefficient);
                    ui.label(None, &format!(" ^ Noise Coefficient"));
                    ui.separator();

                    // scaling_coefficient
                    ui.slider(hash!(), "[0 .. 1]", 0f32..1.0, &mut scaling_coefficient);
                    ui.label(None, &format!(" ^ Scaling Coefficient"));
                    ui.separator();

                    // sim_time_velocity
                    ui.slider(hash!(), "[-1 .. 1]", -3.0f32..3.0, &mut sim_time_velocity);
                    ui.label(None, &format!(" ^ Simulation Time Velocity"));
                    ui.separator();

                    // angle_noise_coefficient
                    ui.slider(
                        hash!(),
                        "[0.1 .. 20.0]",
                        0.01f32..20.0,
                        &mut angle_noise_coefficient,
                    );
                    ui.label(None, &format!(" ^ Angle Noise Coefficient"));
                });

                ui.tree_node(hash!(), "Colors", |ui| {
                    ui.tree_node(hash!(), "Background Color", |ui| {
                        ui.slider(hash!(), "[R]", 0.0f32..1.0, &mut background_color[0]);
                        ui.slider(hash!(), "[G]", 0.0f32..1.0, &mut background_color[1]);
                        ui.slider(hash!(), "[B]", 0.0f32..1.0, &mut background_color[2]);
                        ui.slider(hash!(), "[A]", 0.0f32..1.0, &mut background_color[3]);
                    });

                    ui.tree_node(hash!(), "Border Color", |ui| {
                        ui.slider(hash!(), "[R]", 0.0f32..1.0, &mut border_color[0]);
                        ui.slider(hash!(), "[G]", 0.0f32..1.0, &mut border_color[1]);
                        ui.slider(hash!(), "[B]", 0.0f32..1.0, &mut border_color[2]);
                        ui.slider(hash!(), "[A]", 0.0f32..1.0, &mut border_color[3]);
                    });

                    ui.tree_node(hash!(), "Inner Color", |ui| {
                        ui.slider(hash!(), "[R]", 0.0f32..1.0, &mut inner_color[0]);
                        ui.slider(hash!(), "[G]", 0.0f32..1.0, &mut inner_color[1]);
                        ui.slider(hash!(), "[B]", 0.0f32..1.0, &mut inner_color[2]);
                        ui.slider(hash!(), "[A]", 0.0f32..1.0, &mut inner_color[3]);
                    });
                });
            },
        );

        mq::next_frame().await
    }
}
