use std::convert::TryFrom;
use std::env;
use std::ffi::OsString;
use std::str::FromStr;

use clap::{App, AppSettings, Arg, ArgMatches};
use minifb::Scale;

use crate::errors::*;

const SCREEN_WIDTH: (&str, usize) = ("screen_width", 320);
const SCREEN_HEIGHT: (&str, usize) = ("screen_height", 240);
const NUM_SAMPLES: (&str, usize) = ("num_samples", 5);
const SCALE: (&str, usize) = ("scale", 1);

pub struct Config {
    pub scale: Scale,

    pub screen_width: usize,
    pub screen_height: usize,

    pub num_samples: usize,
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
            scale: args.value_or_default(SCALE).and_then(num_to_scale)?,
            screen_width: args.value_or_default(SCREEN_WIDTH)?,
            screen_height: args.value_or_default(SCREEN_HEIGHT)?,
            num_samples: args.value_or_default(NUM_SAMPLES)?,
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

    fn value_or_default(&self, desc: (&str, usize)) -> Result<usize> {
        match self.matches.value_of_lossy(desc.0) {
            // If there is no match, then use the default.
            None => Ok(desc.1),
            // Otherwise, parse it and return any parse errors.
            Some(s) => Ok(usize::from_str(&s)?),
        }
    }
}

fn parse_from<'a, I, T>(itr: I) -> Result<ArgMatches<'a>>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    App::new("myray")
        .about("My little toy path tracer.")
        .author(crate_authors!())
        .version(crate_version!())
        .setting(AppSettings::StrictUtf8)
        .setting(AppSettings::UnifiedHelpMessage)
        .arg(
            Arg::with_name(SCREEN_WIDTH.0)
                .long(SCREEN_WIDTH.0)
                .short("w")
                .visible_alias("sw")
                .takes_value(true)
                .help("The width of the final image in pixels."),
        )
        .arg(
            Arg::with_name(SCREEN_HEIGHT.0)
                .long(SCREEN_HEIGHT.0)
                .short("h")
                .visible_alias("sh")
                .takes_value(true)
                .help("The height of the final image in pixels."),
        )
        .arg(
            Arg::with_name(NUM_SAMPLES.0)
                .long(NUM_SAMPLES.0)
                .short("n")
                .visible_alias("ns")
                .takes_value(true)
                .help("The number of samples to trace for each pixel."),
        )
        .arg(
            Arg::with_name(SCALE.0)
                .long(SCALE.0)
                .takes_value(true)
                .visible_alias("sc")
                .help("Scale for the output window.")
                .long_help("Scale the output window. Valid values are 0 (fit to screen), 1, 2, 4, 8, 16, & 32.")
        )
        .get_matches_from_safe(itr)
        .map_err(Error::from)
}
