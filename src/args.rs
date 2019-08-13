use std::convert::TryFrom;
use std::env;
use std::ffi::OsString;
use std::str::FromStr;

use clap::{App, AppSettings, Arg, ArgMatches};
use minifb::Scale;

use crate::errors::*;

const CAMERA_FROM: (&str, &str) = ("camera_from", "XXX");
const MAX_DEPTH: (&str, &str) = ("max_depth", "50");
const NUM_SAMPLES: (&str, &str) = ("num_samples", "5");
const SCALE: (&str, &str) = ("scale", "1");
const SCREEN_HEIGHT: (&str, &str) = ("screen_height", "240");
const SCREEN_WIDTH: (&str, &str) = ("screen_width", "320");
const WORLD: (&str, &str) = ("world", "threeballs");

pub enum Worlds {
    ThreeBalls,
    Random,
}

impl FromStr for Worlds {
    type Err = Error;

    fn from_str(s: &str) -> Result<Worlds> {
        match s.to_lowercase().as_str() {
            "threeballs" => Ok(Worlds::ThreeBalls),
            "random" => Ok(Worlds::Random),
            _ => Err(ErrorKind::ParseError(
                s.to_string(),
                "Must be 'threeballs' or 'random'.".to_string(),
            )
            .into()),
        }
    }
}

pub struct Config {
    pub max_depth: usize,
    pub scale: Scale,

    pub screen_width: usize,
    pub screen_height: usize,

    pub num_samples: usize,

    pub world: Worlds,
}

impl Config {
    pub fn new() -> Result<Config> {
        Args::parse().and_then(Config::try_from)
    }
}

impl<'a> TryFrom<Args<'a>> for Config {
    type Error = Error;

    fn try_from(args: Args<'a>) -> Result<Self> {
        Ok(Config {
            max_depth: args.parsed_value(MAX_DEPTH)?,
            scale: args.parsed_value(SCALE).and_then(num_to_scale)?,
            screen_width: args.parsed_value(SCREEN_WIDTH)?,
            screen_height: args.parsed_value(SCREEN_HEIGHT)?,
            num_samples: args.parsed_value(NUM_SAMPLES)?,
            world: args.parsed_value(WORLD)?,
        })
    }
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

struct Args<'a> {
    matches: ArgMatches<'a>,
}

impl<'a> Args<'a> {
    fn parse() -> Result<Args<'a>> {
        Ok(Args {
            matches: parse_from(env::args_os())?,
        })
    }

    fn parsed_value<T>(&self, desc: (&str, &str)) -> Result<T>
    where
        T: FromStr,
        Error: From<<T as FromStr>::Err>,
    {
        self.matches
            .value_of_lossy(desc.0)
            .ok_or_else(|| ErrorKind::MissingParam(desc.0.to_string()).into())
            .and_then(|cow| Ok(T::from_str(&cow)?))
    }
}

fn parse_from<'a, I, T>(itr: I) -> Result<ArgMatches<'a>>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    App::new(crate_name!())
        .about(crate_description!())
        .author(crate_authors!())
        .version(crate_version!())
        .setting(AppSettings::StrictUtf8)
        .setting(AppSettings::UnifiedHelpMessage)
        .arg(
            Arg::with_name(SCREEN_WIDTH.0)
                .long(SCREEN_WIDTH.0)
                .default_value(SCREEN_WIDTH.1)
                .short("w")
                .visible_alias("sw")
                .takes_value(true)
                .help("The width of the final image in pixels."),
        )
        .arg(
            Arg::with_name(SCREEN_HEIGHT.0)
                .long(SCREEN_HEIGHT.0)
                .default_value(SCREEN_HEIGHT.1)
                .short("h")
                .visible_alias("sh")
                .takes_value(true)
                .help("The height of the final image in pixels."),
        )
        .arg(
            Arg::with_name(NUM_SAMPLES.0)
                .long(NUM_SAMPLES.0)
                .default_value(NUM_SAMPLES.1)
                .short("n")
                .visible_alias("ns")
                .takes_value(true)
                .help("The number of samples to trace for each pixel."),
        )
        .arg(
            Arg::with_name(SCALE.0)
                .long(SCALE.0)
                .default_value(SCALE.1)
                .takes_value(true)
                .visible_alias("sc")
                .help("Scale for the output window.")
                .long_help("Scale the output window. Valid values are 0 (fit to screen), 1, 2, 4, 8, 16, & 32.")
        )
        .arg(
            Arg::with_name(MAX_DEPTH.0)
                .long(MAX_DEPTH.0)
                .default_value(MAX_DEPTH.1)
                .takes_value(true)
                .visible_alias("md")
                .help("Max depth for scattered rays.")
        )
        .arg(
            Arg::with_name(WORLD.0)
                .long(WORLD.0)
                .default_value(WORLD.1)
                .takes_value(true)
                .help("'threeballs' or 'random'")
        )
        .get_matches_from_safe(itr)
        .map_err(Error::from)
}
