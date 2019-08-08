use std::env;
use std::ffi::OsString;
use std::str::FromStr;

use clap::{App, AppSettings, Arg, ArgMatches};

use crate::errors::*;

const SCREEN_WIDTH: &str = "screen_width";
const SCREEN_WIDTH_DEFAULT: usize = 320;
const SCREEN_HEIGHT: &str = "screen_height";
const SCREEN_HEIGHT_DEFAULT: usize = 240;
const NUM_SAMPLES: &str = "num_samples";
const NUM_SAMPLES_DEFAULT: usize = 5;

pub struct Config {
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
            screen_width: args.screen_width(),
            screen_height: args.screen_height(),
            num_samples: args.num_samples(),
        }
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

    fn usize_or_default(&self, arg_name: &str, default: usize) -> usize {
        // TODO: this needs to fail on a bad parse, not fall back silently.
        self.matches.value_of_lossy(arg_name).map_or(
            default,
            |s| {
                usize::from_str(&s).unwrap_or(default)
            }
        )
    }

    fn screen_width(&self) -> usize {
        self.usize_or_default(SCREEN_WIDTH, SCREEN_WIDTH_DEFAULT)
    }

    fn screen_height(&self) -> usize {
        self.usize_or_default(SCREEN_HEIGHT, SCREEN_HEIGHT_DEFAULT)
    }

    fn num_samples(&self) -> usize {
        self.usize_or_default(NUM_SAMPLES, NUM_SAMPLES_DEFAULT)
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
            Arg::with_name(SCREEN_WIDTH)
                .long(SCREEN_WIDTH)
                .short("w")
                .takes_value(true)
                .require_equals(true)
                .help("The width of the final image in pixels.")
                
            )
        .arg(
            Arg::with_name(SCREEN_HEIGHT)
                .long(SCREEN_HEIGHT)
                .short("h")
                .takes_value(true)
                .require_equals(true)
                .help("The height of the final image in pixels.")
                
            )
        .arg(
            Arg::with_name(NUM_SAMPLES)
                .long(NUM_SAMPLES)
                .short("s")
                .takes_value(true)
                .require_equals(true)
                .help("The number of samples to trace for each pixel.")
                
            )
        .get_matches_from_safe(itr)
        .map_err(Error::from)
}
