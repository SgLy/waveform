use std::fs::File;

use image::{codecs::png::PngEncoder, EncodableLayout, ImageEncoder, Rgba};
use wavers::{read, Samples};

use waveform::{generate_waveform, GenerateParams, ScaleMode, WaveFormMode};

fn main() {
    let input_file_name = std::env::args().nth(1);
    if input_file_name.is_none() {
        panic!("Please specify input file name");
    }
    let (samples, _): (Samples<i16>, i32) = read::<i16, _>(input_file_name.unwrap()).unwrap();

    let wave_form_modes = [
        WaveFormMode::Half,
        WaveFormMode::Full,
        WaveFormMode::FullSymmetry,
    ];
    let scale_modes = [ScaleMode::Linear, ScaleMode::Logarithm];

    for wave_form_mode in wave_form_modes.iter() {
        for scale_mode in scale_modes.iter() {
            let wave_form_mode_str = match wave_form_mode {
                WaveFormMode::Full => "full",
                WaveFormMode::FullSymmetry => "full-symmetry",
                WaveFormMode::Half => "half",
            };
            let scale_mode_str = match scale_mode {
                ScaleMode::Linear => "linear",
                ScaleMode::Logarithm => "logarithm",
            };
            let output_file_name =
                format!("waveform_{}_{}.png", wave_form_mode_str, scale_mode_str);
            let height = 800
                / if let WaveFormMode::Half = wave_form_mode {
                    2
                } else {
                    1
                };
            let buffer = generate_waveform(
                &samples,
                &GenerateParams {
                    wave_form_mode: *wave_form_mode,
                    bar_padding: 5,
                    bar_width: 20,
                    image_height: height,
                    image_width: 3200,
                    fill_color: Rgba([255, 255, 255, 255]),
                    scale_mode: *scale_mode,
                },
            );

            let output_file = File::create(output_file_name).unwrap();

            let png = PngEncoder::new(output_file);
            png.write_image(
                buffer.as_bytes(),
                buffer.width(),
                buffer.height(),
                image::ExtendedColorType::Rgba8,
            )
            .unwrap();
        }
    }
}
