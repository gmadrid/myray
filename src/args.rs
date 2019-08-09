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
        Ok(Args::parse()?.into())
    }
}

impl<'a> From<Args<'a>> for Config {
    fn from(args: Args) -> Self {
        Config {
            scale: num_to_scale(args.value_or_default(SCALE)),
            screen_width: args.value_or_default(SCREEN_WIDTH),
            screen_height: args.value_or_default(SCREEN_HEIGHT),
            num_samples: args.value_or_default(NUM_SAMPLES),
        }
    }
}

fn num_to_scale(num: usize) -> Scale {
    // TODO: Again, shouldn't fail silently.
    match num {
        0 => Scale::FitScreen,
        2 => Scale::X2,
        4 => Scale::X4,
        8 => Scale::X8,
        16 => Scale::X16,
        32 => Scale::X32,
        1 | _ => Scale::X1,
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

    fn value_or_default(&self, desc: (&str, usize)) -> usize {
        // TODO: this needs to fail on a bad parse, not fall back silently.
        self.matches
            .value_of_lossy(desc.0)
            .map_or(desc.1, |s| usize::from_str(&s).unwrap_or(desc.1))
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
