use std::path::PathBuf;
use std::str::FromStr;

use minifb::Scale;

use crate::errors::*;
use crate::vec3::Vec3;
use crate::world::Worlds;

#[derive(StructOpt, Debug)]
#[structopt()]
pub struct Config {
    /// Hue for the background color.
    #[structopt(long, default_value = "205")]
    pub hue: f32,

    /// Point at the head of the camera ray.
    #[structopt(long, default_value = "(0,0,-1)")]
    pub look_at: Vec3, // TODO

    /// The origin point of the camera normal ray.
    #[structopt(long, default_value = "(0,0,0)")]
    pub look_from: Vec3, // TODO

    /// TODO: complete this description.
    #[structopt(long, default_value = "(0,1,0)")]
    pub look_up: Vec3, // TODO

    /// Max depth for scattered/reflected rays.
    #[structopt(long, default_value = "50", visible_alias = "md")]
    pub max_depth: u8,

    /// The number of sample paths to trace for each output pixel.
    #[structopt(long, default_value = "5", visible_alias = "ns")]
    pub num_samples: u8,

    /// Scale at which to display the rendered images.
    /// Valid values are 0 (fit to screen), 1, 2, 4, 8, 16, & 32.
    #[structopt(long, default_value = "1", parse(try_from_str = "string_to_scale"))]
    pub scale: Scale, // TODO

    /// Height of the final image in pixels.
    #[structopt(long, short = "h", default_value = "240", visible_alias = "sh")]
    pub screen_height: usize,

    /// Width of the final image in pixels.
    #[structopt(long, short = "w", default_value = "320", visible_alias = "sw")]
    pub screen_width: usize,

    /// Write a YAML description of the World to this file before rendering.
    #[structopt(long = "write_world", short = "o", parse(from_os_str))]
    pub write_world: Option<PathBuf>,

    /// Use a pre-defined world. Valid values are "threeballs" and "random".
    #[structopt(long, conflicts_with = "world_files")]
    pub world: Option<Worlds>, // TODO

    /// YAML files containing world descriptions. Multiple files will be merged.
    #[structopt(multiple = true, parse(from_os_str))]
    pub world_files: Vec<PathBuf>,
}

fn num_to_scale(num: usize) -> Result<Scale> {
    match num {
        0 => Ok(Scale::FitScreen),
        1 => Ok(Scale::X1),
        2 => Ok(Scale::X2),
        4 => Ok(Scale::X4),
        8 => Ok(Scale::X8),
        16 => Ok(Scale::X16),
        32 => Ok(Scale::X32),
        _ => Err(ErrorKind::InvalidParam(
            num as f32,
            "Only 0, 1, 2, 4, 8, 16, & 32 are permitted for scale.".to_string(),
        )
        .into()),
    }
}

fn string_to_scale(s: &str) -> Result<Scale> {
    let num = usize::from_str(s)?;
    num_to_scale(num)
}
