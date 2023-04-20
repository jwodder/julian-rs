use julian::{Date, DateParseError, JulianDate, JulianDateParseError, GREG_REFORM, UK_REFORM};
use lexopt::{Arg, Error, Parser, ValueExt};
use std::str::FromStr;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum OldStylePolicy {
    #[default]
    Never,
    UkDelay,
    PostReform,
}

impl OldStylePolicy {
    fn show_old_style(self, jd: JulianDate) -> bool {
        self != OldStylePolicy::Never
            && GREG_REFORM <= jd.days
            && (jd.days < UK_REFORM || self == OldStylePolicy::PostReform)
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Command {
    Run(Arguments),
    Help,
    Version,
}

impl Command {
    fn from_parser(mut parser: Parser) -> Result<Command, lexopt::Error> {
        let mut args = Arguments::default();
        while let Some(arg) = parser.next()? {
            match arg {
                Arg::Short('h') | Arg::Long("help") => return Ok(Command::Help),
                Arg::Short('V') | Arg::Long("version") => return Ok(Command::Version),
                Arg::Long("ordinal") => args.ordinal = true,
                Arg::Short('O') | Arg::Long("old-style") => {
                    args.ospolicy = OldStylePolicy::PostReform;
                }
                Arg::Short('o') | Arg::Long("old-style-uk") => {
                    args.ospolicy = OldStylePolicy::UkDelay;
                }
                Arg::Short('s') | Arg::Long("whole-seconds") => {
                    args.whole_seconds = true;
                }
                Arg::Short('v') | Arg::Long("verbose") => args.verbose = true,
                Arg::Short(c) if c.is_ascii_digit() => {
                    let mut s = String::from_iter(['-', c]);
                    if let Some(v) = parser.optional_value() {
                        s.push_str(&(v.string()?));
                    }
                    match s.parse::<Argument>() {
                        Ok(d) => args.dates.push(d),
                        Err(e) => {
                            return Err(Error::ParsingFailed {
                                value: s,
                                error: Box::new(e),
                            })
                        }
                    }
                }
                Arg::Value(val) => args.dates.push(val.parse::<Argument>()?),
                _ => return Err(arg.unexpected()),
            }
        }
        Ok(Command::Run(args))
    }

    fn run(self) {
        match self {
            Command::Help => {
                println!("Usage: julian [<options>] [<date> ...]");
                println!();
                println!("Convert Julian dates to & from calendar dates");
                println!();
                println!("Options:");
                println!("  --ordinal         Output calendar dates in the form \"YYYY-DDD\"");
                println!();
                println!(
                    "                    The part after the hyphen is the day of the year from 001"
                );
                println!("                    to 366 (the ordinal date).");
                println!();
                println!("  -O, --old-style   Append Old Style dates to post-Reformation dates");
                println!();
                println!("  -o, --old-style-uk");
                println!(
                    "                    Append Old Style dates to dates between the Gregorian"
                );
                println!("                    Reformation and the UK's adoption thereof");
                println!();
                println!("  -s, --whole-seconds");
                println!("                    Show Julian dates with seconds as \"DDDDDD:sssss\"");
                println!();
                println!("                    The part after the colon is the number of seconds past noon");
                println!("                    as an integer.");
                println!();
                println!("  -v, --verbose     Print out the input date before each output date");
                println!();
                println!("  -h, --help        Display this help message and exit");
                println!("  -V, --version     Show the program version and exit");
            }
            Command::Version => {
                println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            }
            Command::Run(args) => args.run(),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Arguments {
    ordinal: bool,
    ospolicy: OldStylePolicy,
    whole_seconds: bool,
    verbose: bool,
    dates: Vec<Argument>,
}

impl Arguments {
    fn print_yds(&self, when: Date) {
        if self.ordinal {
            print!("{when:#}");
        } else {
            print!("{when}");
        }
    }

    fn print_styled(&self, when: Date, jd: JulianDate) {
        self.print_yds(when);
        if self.ospolicy.show_old_style(jd) {
            print!(" [O.S. ");
            self.print_yds(jd.to_julian());
            print!("]");
        }
    }

    fn print_julian(&self, jd: JulianDate) {
        if self.whole_seconds {
            print!("{jd:#}");
        } else {
            print!("{jd}");
        }
    }

    fn run(&self) {
        if self.dates.is_empty() {
            let now = Date::now();
            let jd = now.to_julian_date();
            if self.verbose {
                self.print_styled(now, jd);
                print!(" = ");
            }
            self.print_julian(jd);
            println!();
        } else {
            for &d in &self.dates {
                match d {
                    Argument::CalendarDate(when) => {
                        let jd = when.to_julian_date();
                        if self.verbose {
                            self.print_styled(when, jd);
                            print!(" = ");
                        }
                        self.print_julian(jd);
                        println!();
                    }
                    Argument::JulianDate(jd) => {
                        if self.verbose {
                            self.print_julian(jd);
                            print!(" = ");
                        }
                        let when = jd.to_gregorian();
                        self.print_styled(when, jd);
                        println!();
                    }
                }
            }
        }
    }
}

fn main() -> Result<(), Error> {
    Command::from_parser(Parser::from_env())?.run();
    Ok(())
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Argument {
    CalendarDate(Date),
    JulianDate(JulianDate),
}

impl FromStr for Argument {
    type Err = ArgumentParseError;

    fn from_str(s: &str) -> Result<Argument, ArgumentParseError> {
        if s.match_indices('-').any(|(i, _)| i > 0) {
            Ok(Argument::CalendarDate(s.parse::<Date>()?))
        } else {
            Ok(Argument::JulianDate(s.parse::<JulianDate>()?))
        }
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
enum ArgumentParseError {
    #[error(transparent)]
    CalendarDate(#[from] DateParseError),
    #[error(transparent)]
    JulianDate(#[from] JulianDateParseError),
}
