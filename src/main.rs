mod camera;
mod geometry;
mod object;
mod scene;
mod tracer;

use std::{fs::File, io::BufWriter};

fn main() {
    env_logger::init();

    let mut movie_scene = scene::scene_from_obj_file();

    let max_reflections = 5;
    let samples_per_pixel = 256;
    let num_threads = 16;
    let gamma_correction = 1.0 / 2.0;
    let (x_res, y_res) = (16 * 16, 16 * 16);

    for frame in 0..movie_scene.n_frames {
        movie_scene.calc_frame(frame);

        let mut image = vec![0.0; x_res * y_res];

        tracer::render(
            &movie_scene.scene,
            x_res,
            y_res,
            num_threads,
            samples_per_pixel,
            max_reflections,
            &mut image,
        );

        save_to_png(
            format!("target/output-{:04}.png", frame).as_str(),
            &image,
            x_res,
            y_res,
            gamma_correction,
        );

        println!("Frame {} completed", frame);
    }
}

fn save_to_png(name: &str, image: &[f64], x_res: usize, y_res: usize, gamma_correction: f64) {
    let writer = BufWriter::new(File::create(name).unwrap());

    let mut encoder = png::Encoder::new(writer, x_res as u32, y_res as u32);
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight);

    let mut data_writer = encoder.write_header().unwrap();

    let data: Vec<u8> = image
        .iter()
        .map(|gray| (256.0 * gray.powf(gamma_correction)) as u8)
        .collect();
    data_writer.write_image_data(&data).unwrap();
}
