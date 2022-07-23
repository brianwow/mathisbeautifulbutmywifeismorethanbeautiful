use core::ops::Range;
use macroquad::{prelude::*, ui::root_ui};
use std::f32::consts::PI;

struct Slider {
    id: u64,
    label: String,
    range: Range<f32>,
    data: f32,
}

impl Slider {
    fn from(label: String, range: Range<f32>, data: f32) -> Self {
        Slider {
            id: get_time() as u64,
            label,
            range,
            data,
        }
    }

    fn update(&mut self) {
        root_ui().slider(self.id, &self.label, self.range.clone(), &mut self.data);
    }
}

const TAU: f32 = PI * 2.;

#[macroquad::main("Polygons")]
async fn main() {
    let mut slider = Slider::from(String::from("Polygons"), 3f32..20f32, 3f32);

    loop {
        set_camera(&Camera2D {
            zoom: vec2(1., screen_width() / screen_height()),
            ..Default::default()
        });

        let count = get_time().sin() as f32 * 50.;

        let color = Color::from_rgba(
            0,
            ((get_time() as f32 * TAU).cos().abs() * 255.) as u8,
            (get_time().cos().abs() * 255.) as u8,
            ((get_time() as f32).cos().abs() * 255.).max(100.) as u8,
        );
        clear_background(BLACK);

        let color2 = Color::from_rgba(
            0,
            ((get_time() as f32 * TAU).sin().abs() * 255.) as u8,
            (get_time().sin().abs() * 255.) as u8,
            (get_time().sin().abs() * 255.).max(100.) as u8,
        );

        for i in 0..=(count as i32) {
            draw_line(
                (((i % (count as i32 + 1)) as f32 * TAU / count.ceil()) as f32).cos()
                    / get_time().sin() as f32,
                (((i % (count as i32 + 1)) as f32 * TAU / count.ceil()) as f32).sin()
                    / get_time().sin() as f32,
                ((((i + 1) % (count as i32 + 1)) as f32 * TAU / count.ceil()) as f32).cos()
                    / get_time().sin() as f32,
                ((((i + 1) % (count as i32 + 1)) as f32 * TAU / count.ceil()) as f32).sin()
                    / get_time().sin() as f32,
                0.005f32,
                color,
            );

            draw_line(
                (((i % (count as i32 + 1)) as f32 * TAU / count.ceil()) as f32).cos() / 2.,
                (((i % (count as i32 + 1)) as f32 * TAU / count.ceil()) as f32).sin() / 2.,
                ((((i + 1) % (count as i32 + 1)) as f32 * TAU / count.ceil()) as f32).cos() / 2.,
                ((((i + 1) % (count as i32 + 1)) as f32 * TAU / count.ceil()) as f32).sin() / 2.,
                0.005f32,
                color2,
            );
        }

        for i in (count as i32)..50 {
            draw_line(
                ((i as f32 * TAU) as f32).cos() / get_time().sin() as f32,
                ((i as f32 * TAU) as f32).sin() / get_time().sin() as f32,
                (((1. + i as f32) * TAU) as f32).cos() / get_time().sin() as f32,
                (((1. + i as f32) * TAU) as f32).sin() / get_time().sin() as f32,
                0.005f32,
                color,
            );

            draw_line(
                ((i as f32 * TAU / count.ceil()) as f32).cos() / 1.5,
                ((i as f32 * TAU / count.ceil()) as f32).sin() / 1.5,
                (((1. + i as f32) * TAU / count.ceil()) as f32).cos() / 1.5,
                (((1. + i as f32) * TAU / count.ceil()) as f32).sin() / 1.5,
                0.005f32,
                color2,
            );
        }

        for i in (count as i32 + count as i32)..50 {
            draw_circle(
                ((i as f32 * PI / count.ceil()) as f32).sin() / (get_time().sin() * 1.2) as f32,
                ((i as f32 * PI / count.ceil()) as f32).cos() / (get_time().sin() * 1.2) as f32,
                0.01f32 * get_time().sin() as f32,
                color2,
            );

            draw_circle(
                ((i as f32 * TAU / count.ceil()) as f32).sin()
                    / (get_time().cos() * 1.2) as f32
                    / 5.,
                ((i as f32 * TAU / count.ceil()) as f32).cos()
                    / (get_time().cos() * 1.2) as f32
                    / 5.,
                0.01f32 * get_time().cos() as f32,
                color2,
            );

            draw_circle_lines(
                ((i as f32 * PI / count.ceil()) as f32).cos() / 3.,
                ((i as f32 * PI / count.ceil()) as f32).sin() / 3.,
                0.02f32 * get_time().cos() as f32,
                0.005f32,
                color,
            );
        }

        for i in 0..=(count as i32) {
            draw_circle(
                ((i as f32 * TAU / count.ceil()) as f32).sin() / (get_time().sin() * 1.1) as f32,
                ((i as f32 * TAU / count.ceil()) as f32).cos() / (get_time().sin() * 1.1) as f32,
                0.01f32,
                color2,
            );

            draw_circle(
                ((i as f32 * TAU / count.ceil()) as f32).sin()
                    / (get_time().cos() * 1.1).abs() as f32
                    / 5.,
                ((i as f32 * TAU / count.ceil()) as f32).cos()
                    / (get_time().cos() * 1.1).abs() as f32
                    / 5.,
                0.01f32,
                color2,
            );

            draw_circle_lines(
                ((i as f32 * TAU / count.ceil()) as f32).cos() / 2.,
                ((i as f32 * TAU / count.ceil()) as f32).sin() / 2.,
                0.02f32,
                0.005f32,
                color,
            );
        }

        set_default_camera();
        //slider.update();

        next_frame().await
    }
}
