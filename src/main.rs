use std::fs;
use std::path::Path;
use std::error::Error;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use opencv::prelude::*;
use opencv::core::Mat;
use opencv::imgcodecs::imread;
use csv::Writer;

const IMAGE_DIR: &str = "path/to/image/directory";
const LABELS: [&str; 3] = ["label1", "label2", "label3"];

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize SDL2 and create a window
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Labeling Tool", 800, 600)
        .position_centered()
        .build()?;
    let mut canvas = window.into_canvas().build()?;

    // Initialize OpenCV
    let _ = opencv::highgui::named_window("Image", opencv::highgui::WINDOW_AUTOSIZE)?;

    // Load the list of images from the directory
    let image_files = fs::read_dir(IMAGE_DIR)?.map(|r| r.map(|f| f.path())).collect::<Result<Vec<_>, _>>()?;

    // Initialize the CSV writer
    let mut csv_writer = Writer::from_path("labels.csv")?;

    // Loop through each image and allow the user to select a label
    for image_file in image_files {
        // Load the image using OpenCV
        let mat = imread(image_file.to_str().unwrap(), opencv::imgcodecs::IMREAD_COLOR)?;

        // Convert the OpenCV Mat to an SDL2 Texture
        let texture_creator = canvas.texture_creator();
        let surface = sdl2::surface::Surface::from_data(
            mat.data(),
            mat.cols(),
            mat.rows(),
            mat.step(),
            sdl2::pixels::PixelFormatEnum::RGB24,
        )?;
        let texture = texture_creator.create_texture_from_surface(&surface)?;

        // Display the image and allow the user to select a label
        'event_loop: loop {
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.clear();

            // Display the image
            canvas.copy(&texture, None, Some(Rect::new(0, 0, mat.cols() as u32, mat.rows() as u32)))?;

            // Display the labels
            let mut x = 10;
            let y = 10;
            for label in &LABELS {
                let texture = texture_creator.load_font("path/to/font.ttf", 24)?
                    .render(label)
                    .blended(Color::RGB(0, 0, 0))?;
                let texture_rect = Rect::new(x, y, texture.width(), texture.height());
                canvas.copy(&texture, None, Some(texture_rect))?;
                x += texture.width() as i32 + 10;
            }

            canvas.present();

            // Handle events
            for event in sdl_context.event_pump()?.poll_iter() {
                match event {
                    Event::Quit {..} => return Ok(()),
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return Ok(()),
                    Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
                        csv_writer.write_record(&[image_file.to_str().unwrap(), LABELS[0]])?;
                        break 'event_loop;
                    },
                    Event::KeyDown { keycode: Some(Keycode::Num2), .. } => {
                        csv_writer.write_record(&[image_file.to_str().unwrap(), LABELS[1]])?;
                        break 'event_loop;
                    },
                    Event::KeyDown { keycode: Some(Keycode::Num3), .. } => {
                        csv_writer.write_record(&[image_file.to_str().unwrap(), LABELS[2]])?;
                        break 'event_loop;
                    },
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

