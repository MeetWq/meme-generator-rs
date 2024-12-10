pub mod canvas;
pub mod decoder;
pub mod encoder;
pub mod image;
pub mod options;
pub mod text;

use std::fs::read;

use chrono::{DateTime, Local, TimeZone};
use skia_safe::{
    scalar, surfaces, Codec, Color, Color4f, Data, FilterMode, ISize, Image, MipmapMode, Paint,
    PaintJoin, PaintStyle, SamplingOptions, Surface,
};

use crate::{config::MEME_HOME, error::Error, utils::decoder::CodecExt};

pub(crate) fn new_surface(size: impl Into<ISize>) -> Surface {
    surfaces::raster_n32_premul(size).unwrap()
}

pub(crate) fn new_paint(color: impl Into<Color4f>) -> Paint {
    let mut paint = Paint::new(color.into(), None);
    paint.set_anti_alias(true);
    paint
}

pub(crate) fn new_stroke_paint(color: impl Into<Color4f>, stroke_width: scalar) -> Paint {
    let mut paint = Paint::new(color.into(), None);
    paint.set_anti_alias(true);
    paint.set_stroke_width(stroke_width);
    paint.set_style(PaintStyle::Stroke);
    paint.set_stroke_join(PaintJoin::Round);
    paint
}

pub(crate) fn color_from_hex_code(hex_code: &str) -> Color {
    let hex_code = hex_code.trim_start_matches('#');
    let r = u8::from_str_radix(&hex_code[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex_code[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex_code[4..6], 16).unwrap();
    let a = if hex_code.len() == 8 {
        u8::from_str_radix(&hex_code[6..8], 16).unwrap()
    } else {
        255
    };
    Color::from_argb(a, r, g, b)
}

pub(crate) fn default_sampling_options() -> SamplingOptions {
    SamplingOptions::new(FilterMode::Linear, MipmapMode::Linear)
}

pub(crate) fn local_date(year: i32, month: u32, day: u32) -> DateTime<Local> {
    Local.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap()
}

pub(crate) fn load_image(path: impl Into<String>) -> Result<Image, Error> {
    let image_path = MEME_HOME.join("resources/images").join(path.into());
    let data = Data::new_copy(&read(image_path)?);
    Codec::from_data(data)
        .ok_or(Error::ImageDecodeError(None))?
        .first_frame()
}