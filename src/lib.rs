use image::{ImageBuffer, Rgba, RgbaImage};

#[derive(Debug, Clone, Copy)]
pub enum WaveFormMode {
    Half,
    Full,
    FullSymmetry,
}

#[derive(Debug, Clone, Copy)]
pub enum ScaleMode {
    Linear,
    Logarithm,
}

#[derive(Debug)]
pub struct GenerateParams {
    pub image_width: u32,
    pub image_height: u32,
    pub bar_width: u32,
    pub bar_padding: u32,
    pub wave_form_mode: WaveFormMode,
    pub scale_mode: ScaleMode,
    pub fill_color: Rgba<u8>,
}

pub fn generate_waveform(
    samples: &[i16],
    param: &GenerateParams,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut img = RgbaImage::new(param.image_width, param.image_height);

    macro_rules! put_rect {
        ($left: expr, $top: expr, $width: expr, $height: expr) => {
            for i in $left..$left + $width {
                for j in $top..$top + $height {
                    img.put_pixel(i, j, param.fill_color);
                }
            }
        };
    }

    let plot_height = match param.wave_form_mode {
        WaveFormMode::FullSymmetry | WaveFormMode::Half => param.image_height,
        WaveFormMode::Full => param.image_height / 2,
    };
    let max_plot_value = match param.wave_form_mode {
        WaveFormMode::FullSymmetry | WaveFormMode::Half => i16::MAX as f64 - i16::MIN as f64,
        WaveFormMode::Full => -(i16::MIN as f64),
    };
    let unit_height: f64 = match param.scale_mode {
        ScaleMode::Linear => plot_height as f64 / max_plot_value,
        ScaleMode::Logarithm => plot_height as f64 / max_plot_value.log2(),
    };

    macro_rules! vertical_scale {
        ($height: expr) => {
            match param.scale_mode {
                ScaleMode::Linear => (unit_height * $height) as u32,
                ScaleMode::Logarithm => {
                    if $height < 1.0 {
                        0
                    } else {
                        ($height.log2() * unit_height) as u32
                    }
                }
            }
        };
    }

    let bar_cnt = ((param.image_width + param.bar_padding) as f64
        / (param.bar_width + param.bar_padding) as f64)
        .floor() as usize;
    let bar_per_sample = bar_cnt as f64 / samples.len() as f64;

    let mut current_bar = 0;
    let mut current_bar_float = 0_f64;
    let mut current_min = i16::MAX;
    let mut current_max = i16::MIN;

    for sample in samples.iter() {
        current_min = std::cmp::min(current_min, *sample);
        current_max = std::cmp::max(current_max, *sample);
        if current_bar_float.floor() != (current_bar_float + bar_per_sample).floor() {
            let left = current_bar * (param.bar_width + param.bar_padding);
            let width = param.bar_width;
            match param.wave_form_mode {
                WaveFormMode::Half | WaveFormMode::FullSymmetry => {
                    let height = vertical_scale!(current_max as f64 - current_min as f64);
                    let mut top = param.image_height - height;
                    if let WaveFormMode::FullSymmetry = param.wave_form_mode {
                        top /= 2
                    };
                    let top = top;
                    put_rect!(left, top, width, height);
                }
                WaveFormMode::Full => {
                    let upper_height_abs = vertical_scale!((current_max as f64).abs());
                    let upper_pos = if current_max > 0 {
                        plot_height - upper_height_abs
                    } else {
                        plot_height + upper_height_abs
                    };
                    let lower_height_abs = vertical_scale!((current_min as f64).abs());
                    let lower_pos = if current_min > 0 {
                        plot_height - lower_height_abs
                    } else {
                        plot_height + lower_height_abs
                    };
                    put_rect!(left, upper_pos, width, lower_pos - upper_pos);
                }
            };
            current_bar += 1;
            current_min = i16::MAX;
            current_max = i16::MIN;
        }
        current_bar_float += bar_per_sample;
    }

    img
}
