use macroquad::{self as mq, *};
use noise::{NoiseFn, OpenSimplex};
use megaui::Id;

fn to_mq_color(c: &[f32; 4]) -> mq::Color {
    mq::Color::new(c[0], c[1], c[2], c[3])
}

fn color_from_u32(c: u32) -> [f32; 4] {
    megaui::Color::from_rgb_u32(c).into()
}

pub struct NoiseCircle {
    noise: OpenSimplex,

    // Settings Window
    settings_window_id: Id,

    // f32 Settings
    segments: usize,
    segments_f32: f32,
    inner_proportion: f32,
    outer_proportion: f32,
    noise_coefficient: f32,
    scaling_coefficient: f32,
    sim_time_velocity: f32,
    angle_noise_coefficient: f32,

    current_time_sim: f64,
    last_time: f64,

    // color Settings
    // https://coolors.co/efd9ce-dec0f1-b79ced-957fef-7161ef
    background_color: [f32; 4],
    border_color: [f32; 4],
    inner_color: [f32; 4],
    outer_color: [f32; 4],
}

impl NoiseCircle {
    pub fn new() -> NoiseCircle {
        NoiseCircle {
            noise: OpenSimplex::new(),
            settings_window_id: hash!(),

            // f32 Settings
            segments: 20usize,
            segments_f32: 20f32,
            inner_proportion: 0.2,
            outer_proportion: 0.5,
            noise_coefficient: 0.4,
            scaling_coefficient: 1.0,
            sim_time_velocity: 0.8,
            angle_noise_coefficient: 10.0,

            current_time_sim: get_time(),
            last_time: get_time(),

            // color Settings
            // https://coolors.co/efd9ce-dec0f1-b79ced-957fef-7161ef
            background_color: color_from_u32(0xEFD9CE),
            border_color: color_from_u32(0xDEC0F1),
            inner_color: color_from_u32(0xB79CED),
            outer_color: {
                let mut result = color_from_u32(0x957FEF);
                result[3] = 0.0;
                result
            }
        }
    }

    pub fn update(&mut self) {
        let screen_center = Vec2::new(screen_width() / 2., screen_height() / 2.);
        let width_min = screen_width().min(screen_height());
        let width_max = screen_width().max(screen_height());

        let current_time = get_time();
        let delta = current_time - self.last_time;
        self.last_time = current_time;
        self.current_time_sim += self.sim_time_velocity as f64 * delta;

        mq::clear_background(to_mq_color(&self.background_color));

        let mut hull = Vec::new();
        for i in 0..self.segments {
            // get angle
            let angle = (i as f32 / self.segments as f32) * 2.0 * std::f32::consts::PI;

            // get distance with noise on angle and time
            let distance = self.outer_proportion
                * width_min
                * self.noise_coefficient
                * self.noise
                    .get([
                        self.current_time_sim,
                        (self.angle_noise_coefficient * angle.cos()) as f64,
                        (self.angle_noise_coefficient * angle.sin()) as f64,
                    ])
                    .abs() as f32
                + self.inner_proportion * width_max;

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
                to_mq_color(&self.inner_color),
            );

            draw_line(
                cx,
                cy,
                angle.sin() * width_max + screen_center.x(),
                angle.cos() * width_max + screen_center.y(),
                6.0,
                to_mq_color(&self.outer_color),
            );

            if i > 0 {
                draw_line(
                    cx,
                    cy,
                    hull[i - 1][0],
                    hull[i - 1][1],
                    6.0,
                    to_mq_color(&self.border_color),
                );
            }
        }
        draw_line(
            hull[0][0],
            hull[0][1],
            hull[self.segments - 1][0],
            hull[self.segments - 1][1],
            6.0,
            to_mq_color(&self.border_color),
        );

        draw_window(
            self.settings_window_id,
            vec2(screen_width() - 300.0, 0.0),
            vec2(300., 300.),
            WindowParams {
                label: "Settings".to_string(),
                ..Default::default()
            },
            |ui| {
                ui.tree_node(hash!(), "Coefficients", |ui| {
                    // segments
                    ui.slider(hash!(), "[10 .. 200]", 10f32..200f32, &mut self.segments_f32);
                    self.segments = self.segments_f32.ceil() as usize;
                    ui.label(None, &format!(" ^ segments {}", self.segments));
                    ui.separator();

                    // inner_proportion
                    ui.slider(hash!(), "[0 .. 1]", 0f32..1.0, &mut self.inner_proportion);
                    ui.label(None, &format!(" ^ Inner Proportion"));
                    ui.separator();

                    // outer_proportion
                    ui.slider(
                        hash!(),
                        &format!("[{} .. 1.0]", self.inner_proportion),
                        self.inner_proportion..1f32,
                        &mut self.outer_proportion,
                    );
                    ui.label(None, &format!(" ^ Outer Proportion"));
                    ui.separator();

                    // noise_coefficient
                    ui.slider(hash!(), "[0 .. 1]", 0f32..1.0, &mut self.noise_coefficient);
                    ui.label(None, &format!(" ^ Noise Coefficient"));
                    ui.separator();

                    // scaling_coefficient
                    ui.slider(hash!(), "[0 .. 1]", 0f32..1.0, &mut self.scaling_coefficient);
                    ui.label(None, &format!(" ^ Scaling Coefficient"));
                    ui.separator();

                    // sim_time_velocity
                    ui.slider(hash!(), "[-1 .. 1]", -3.0f32..3.0, &mut self.sim_time_velocity);
                    ui.label(None, &format!(" ^ Simulation Time Velocity"));
                    ui.separator();

                    // angle_noise_coefficient
                    ui.slider(
                        hash!(),
                        "[0.1 .. 20.0]",
                        0.01f32..20.0,
                        &mut self.angle_noise_coefficient,
                    );
                    ui.label(None, &format!(" ^ Angle Noise Coefficient"));
                });

                ui.tree_node(hash!(), "Colors", |ui| {
                    ui.tree_node(hash!(), "Background Color", |ui| {
                        ui.slider(hash!(), "[R]", 0.0f32..1.0, &mut self.background_color[0]);
                        ui.slider(hash!(), "[G]", 0.0f32..1.0, &mut self.background_color[1]);
                        ui.slider(hash!(), "[B]", 0.0f32..1.0, &mut self.background_color[2]);
                        ui.slider(hash!(), "[A]", 0.0f32..1.0, &mut self.background_color[3]);
                    });

                    ui.tree_node(hash!(), "Border Color", |ui| {
                        ui.slider(hash!(), "[R]", 0.0f32..1.0, &mut self.border_color[0]);
                        ui.slider(hash!(), "[G]", 0.0f32..1.0, &mut self.border_color[1]);
                        ui.slider(hash!(), "[B]", 0.0f32..1.0, &mut self.border_color[2]);
                        ui.slider(hash!(), "[A]", 0.0f32..1.0, &mut self.border_color[3]);
                    });

                    ui.tree_node(hash!(), "Inner Color", |ui| {
                        ui.slider(hash!(), "[R]", 0.0f32..1.0, &mut self.inner_color[0]);
                        ui.slider(hash!(), "[G]", 0.0f32..1.0, &mut self.inner_color[1]);
                        ui.slider(hash!(), "[B]", 0.0f32..1.0, &mut self.inner_color[2]);
                        ui.slider(hash!(), "[A]", 0.0f32..1.0, &mut self.inner_color[3]);
                    });

                    ui.tree_node(hash!(), "Outer Color", |ui| {
                        ui.slider(hash!(), "[R]", 0.0f32..1.0, &mut self.outer_color[0]);
                        ui.slider(hash!(), "[G]", 0.0f32..1.0, &mut self.outer_color[1]);
                        ui.slider(hash!(), "[B]", 0.0f32..1.0, &mut self.outer_color[2]);
                        ui.slider(hash!(), "[A]", 0.0f32..1.0, &mut self.outer_color[3]);
                    });
                });
            },
        );
    }
}
