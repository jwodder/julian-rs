use julian::{Date, DateParseError, JulianDate, JulianDateParseError, GREG_REFORM, UK_REFORM};
use lexopt::{Arg, Error, Parser, ValueExt};
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Eq, PartialEq)]
enum Command {
    Run(Options, Vec<Argument>),
    Help,
    Version,
}

impl Command {
    fn from_parser(mut parser: Parser) -> Result<Command, lexopt::Error> {
        let mut opts = Options::default();
        let mut dates = Vec::new();
        while let Some(arg) = parser.next()? {
            match arg {
                Arg::Short('h') | Arg::Long("help") => return Ok(Command::Help),
                Arg::Short('V') | Arg::Long("version") => return Ok(Command::Version),
                Arg::Long("ordinal") => opts.ordinal = true,
                Arg::Short('O') | Arg::Long("old-style") => {
                    opts.ospolicy = OldStylePolicy::PostReform;
                }
                Arg::Short('o') | Arg::Long("old-style-uk") => {
                    opts.ospolicy = OldStylePolicy::UkDelay;
                }
                Arg::Short('s') | Arg::Long("whole-seconds") => {
                    opts.whole_seconds = true;
                }
                Arg::Short('v') | Arg::Long("verbose") => opts.verbose = true,
                Arg::Short(c) if c.is_ascii_digit() => {
                    let mut s = String::from_iter(['-', c]);
                    if let Some(v) = parser.optional_value() {
                        s.push_str(&(v.string()?));
                    }
                    match s.parse::<Argument>() {
                        Ok(d) => dates.push(d),
                        Err(e) => {
                            return Err(Error::ParsingFailed {
                                value: s,
                                error: Box::new(e),
                            })
                        }
                    }
                }
                Arg::Value(val) => dates.push(val.parse::<Argument>()?),
                _ => return Err(arg.unexpected()),
            }
        }
        Ok(Command::Run(opts, dates))
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
            Command::Run(opts, dates) => opts.run(dates),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Options {
    ordinal: bool,
    ospolicy: OldStylePolicy,
    whole_seconds: bool,
    verbose: bool,
}

impl Options {
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

    fn run(&self, dates: Vec<Argument>) {
        if dates.is_empty() {
            let now = Date::now();
            let jd = now.to_julian_date();
            if self.verbose {
                self.print_styled(now, jd);
                print!(" = ");
            }
            self.print_julian(jd);
            println!();
        } else {
            for d in dates {
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

fn main() -> Result<(), Error> {
    Command::from_parser(Parser::from_env())?.run();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use OldStylePolicy::*;

    #[rstest]
    #[case(vec!["julian", "-h"], Command::Help)]
    #[case(vec!["julian", "--help"], Command::Help)]
    #[case(vec!["julian", "--help", "2023-04-20"], Command::Help)]
    #[case(vec!["julian", "2023-04-20", "-s", "-h"], Command::Help)]
    #[case(vec!["julian", "2023-04-20", "--help", "-V"], Command::Help)]
    #[case(vec!["julian", "-V"], Command::Version)]
    #[case(vec!["julian", "--version"], Command::Version)]
    #[case(vec!["julian", "--version", "2460055.314606"], Command::Version)]
    #[case(vec!["julian", "2460055.314606", "--ordinal", "--version"], Command::Version)]
    #[case(vec!["julian", "2460055.314606", "--version", "-h"], Command::Version)]
    #[case(
        vec!["julian"],
        Command::Run(
            Options {
                ordinal: false,
                ospolicy: Never,
                whole_seconds: false,
                verbose: false,
            },
            Vec::new(),
        )
    )]
    #[case(
        vec!["julian", "123"],
        Command::Run(
            Options {
                ordinal: false,
                ospolicy: Never,
                whole_seconds: false,
                verbose: false,
            },
            vec![Argument::JulianDate(JulianDate {days: 123, seconds: None})],
        )
    )]
    #[case(
        vec!["julian", "123:456"],
        Command::Run(
            Options {
                ordinal: false,
                ospolicy: Never,
                whole_seconds: false,
                verbose: false,
            },
            vec![Argument::JulianDate(JulianDate {days: 123, seconds: Some(456)})],
        )
    )]
    #[case(
        vec!["julian", "-123"],
        Command::Run(
            Options {
                ordinal: false,
                ospolicy: Never,
                whole_seconds: false,
                verbose: false,
            },
            vec![Argument::JulianDate(JulianDate {days: -123, seconds: None})],
        )
    )]
    #[case(
        vec!["julian", "-v", "-123"],
        Command::Run(
            Options {
                ordinal: false,
                ospolicy: Never,
                whole_seconds: false,
                verbose: true,
            },
            vec![Argument::JulianDate(JulianDate {days: -123, seconds: None})],
        )
    )]
    #[case(
        vec!["julian", "-123", "-v"],
        Command::Run(
            Options {
                ordinal: false,
                ospolicy: Never,
                whole_seconds: false,
                verbose: true,
            },
            vec![Argument::JulianDate(JulianDate {days: -123, seconds: None})],
        )
    )]
    #[case(
        vec!["julian", "-O"],
        Command::Run(
            Options {
                ordinal: false,
                ospolicy: PostReform,
                whole_seconds: false,
                verbose: false,
            },
            Vec::new(),
        )
    )]
    #[case(
        vec!["julian", "-o"],
        Command::Run(
            Options {
                ordinal: false,
                ospolicy: UkDelay,
                whole_seconds: false,
                verbose: false,
            },
            Vec::new(),
        )
    )]
    #[case(
        vec!["julian", "-o", "--old-style"],
        Command::Run(
            Options {
                ordinal: false,
                ospolicy: PostReform,
                whole_seconds: false,
                verbose: false,
            },
            Vec::new(),
        )
    )]
    #[case(
        vec!["julian", "-O", "--old-style-uk"],
        Command::Run(
            Options {
                ordinal: false,
                ospolicy: UkDelay,
                whole_seconds: false,
                verbose: false,
            },
            Vec::new(),
        )
    )]
    #[case(
        vec!["julian", "-s"],
        Command::Run(
            Options {
                ordinal: false,
                ospolicy: Never,
                whole_seconds: true,
                verbose: false,
            },
            Vec::new(),
        )
    )]
    #[case(
        vec!["julian", "--whole-seconds"],
        Command::Run(
            Options {
                ordinal: false,
                ospolicy: Never,
                whole_seconds: true,
                verbose: false,
            },
            Vec::new(),
        )
    )]
    fn test_cli_parser(#[case] argv: Vec<&str>, #[case] cmd: Command) {
        let parser = Parser::from_iter(argv);
        assert_eq!(Command::from_parser(parser).unwrap(), cmd);
    }
}
