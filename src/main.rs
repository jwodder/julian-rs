use julian::{reformations, Calendar, Date, JulianDayT, ParseDateError};
use lexopt::{Arg, Error, Parser, ValueExt};
use std::fmt::Write;
use std::num::ParseIntError;
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
                Arg::Short('j') | Arg::Long("ordinal") => opts.ordinal = true,
                Arg::Short('O') | Arg::Long("old-style") => {
                    opts.ospolicy = OldStylePolicy::PostReform;
                }
                Arg::Short('o') | Arg::Long("old-style-uk") => {
                    opts.ospolicy = OldStylePolicy::UkDelay;
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
            Command::Run(opts, dates) => {
                for ln in opts.run(dates) {
                    println!("{ln}");
                }
            }
            Command::Help => {
                println!("Usage: julian [<options>] [<date> ...]");
                println!();
                println!("Convert Julian days to & from calendar dates");
                println!();
                println!("Options:");
                println!("  -j, --ordinal     Output calendar dates in the form \"YYYY-DDD\"");
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
                println!("  -v, --verbose     Print out the input date before each output date");
                println!();
                println!("  -h, --help        Display this help message and exit");
                println!("  -V, --version     Show the program version and exit");
            }
            Command::Version => {
                println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            }
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Options {
    ordinal: bool,
    ospolicy: OldStylePolicy,
    verbose: bool,
}

impl Options {
    fn run(&self, dates: Vec<Argument>) -> Vec<String> {
        let mut output = Vec::with_capacity(dates.len());
        let cal = Calendar::gregorian_reform();
        if dates.is_empty() {
            let (now, _) = cal.now().unwrap();
            let jd = now.julian_day();
            output.push(self.show_cal_to_julian(now, jd));
        } else {
            for d in dates {
                match d {
                    Argument::CalendarDate(when) => {
                        let jd = when.julian_day();
                        output.push(self.show_cal_to_julian(when, jd));
                    }
                    Argument::JulianDay(jd) => {
                        let when = cal.at_julian_day(jd).unwrap();
                        output.push(self.show_julian_to_cal(when, jd));
                    }
                }
            }
        }
        output
    }

    fn show_cal_to_julian(&self, cal: Date, jd: JulianDayT) -> String {
        let mut s = String::new();
        if self.verbose {
            self.fmt_styled(&mut s, cal, jd);
            write!(&mut s, " = ").unwrap();
        }
        self.fmt_julian(&mut s, jd);
        s
    }

    fn show_julian_to_cal(&self, cal: Date, jd: JulianDayT) -> String {
        let mut s = String::new();
        if self.verbose {
            self.fmt_julian(&mut s, jd);
            write!(&mut s, " = ").unwrap();
        }
        self.fmt_styled(&mut s, cal, jd);
        s
    }

    fn fmt_styled(&self, s: &mut String, when: Date, jd: JulianDayT) {
        self.fmt_date(s, when);
        if self.ospolicy.show_old_style(jd) {
            write!(s, " [O.S. ").unwrap();
            self.fmt_date(s, Calendar::julian().at_julian_day(jd).unwrap());
            write!(s, "]").unwrap();
        }
    }

    fn fmt_date(&self, s: &mut String, when: Date) {
        if self.ordinal {
            write!(s, "{when:#}").unwrap();
        } else {
            write!(s, "{when}").unwrap();
        }
    }

    fn fmt_julian(&self, s: &mut String, jd: JulianDayT) {
        write!(s, "{jd}").unwrap();
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
    fn show_old_style(self, jd: JulianDayT) -> bool {
        self != OldStylePolicy::Never
            && reformations::GREGORIAN <= jd
            && (jd < reformations::UNITED_KINGDOM || self == OldStylePolicy::PostReform)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Argument {
    CalendarDate(Date),
    JulianDay(JulianDayT),
}

impl FromStr for Argument {
    type Err = ArgumentParseError;

    fn from_str(s: &str) -> Result<Argument, ArgumentParseError> {
        if s.match_indices('-').any(|(i, _)| i > 0) {
            Ok(Argument::CalendarDate(
                Calendar::gregorian_reform().parse_date(s)?,
            ))
        } else {
            Ok(Argument::JulianDay(s.parse::<JulianDayT>()?))
        }
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
enum ArgumentParseError {
    #[error(transparent)]
    CalendarDate(#[from] ParseDateError),
    #[error(transparent)]
    JulianDay(#[from] ParseIntError),
}

fn main() -> Result<(), Error> {
    Command::from_parser(Parser::from_env())?.run();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use julian::Month;
    use rstest::rstest;
    use OldStylePolicy::*;

    #[rstest]
    #[case(vec!["julian", "-h"], Command::Help)]
    #[case(vec!["julian", "--help"], Command::Help)]
    #[case(vec!["julian", "--help", "2023-04-20"], Command::Help)]
    #[case(vec!["julian", "2023-04-20", "-O", "-h"], Command::Help)]
    #[case(vec!["julian", "2023-04-20", "--help", "-V"], Command::Help)]
    #[case(vec!["julian", "-V"], Command::Version)]
    #[case(vec!["julian", "--version"], Command::Version)]
    #[case(vec!["julian", "--version", "2460055"], Command::Version)]
    #[case(vec!["julian", "2460055", "--ordinal", "--version"], Command::Version)]
    #[case(vec!["julian", "2460055", "--version", "-h"], Command::Version)]
    #[case(
        vec!["julian"],
        Command::Run(
            Options {
                ordinal: false,
                ospolicy: Never,
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
                verbose: false,
            },
            vec![Argument::JulianDay(123)],
        )
    )]
    #[case(
        vec!["julian", "-123"],
        Command::Run(
            Options {
                ordinal: false,
                ospolicy: Never,
                verbose: false,
            },
            vec![Argument::JulianDay(-123)],
        )
    )]
    #[case(
        vec!["julian", "-v", "-123"],
        Command::Run(
            Options {
                ordinal: false,
                ospolicy: Never,
                verbose: true,
            },
            vec![Argument::JulianDay(-123)],
        )
    )]
    #[case(
        vec!["julian", "-123", "-v"],
        Command::Run(
            Options {
                ordinal: false,
                ospolicy: Never,
                verbose: true,
            },
            vec![Argument::JulianDay(-123)],
        )
    )]
    #[case(
        vec!["julian", "-O"],
        Command::Run(
            Options {
                ordinal: false,
                ospolicy: PostReform,
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
                verbose: false,
            },
            Vec::new(),
        )
    )]
    #[case(
        vec!["julian", "-j"],
        Command::Run(
            Options {
                ordinal: true,
                ospolicy: Never,
                verbose: false,
            },
            Vec::new(),
        )
    )]
    #[case(
        vec!["julian", "--ordinal"],
        Command::Run(
            Options {
                ordinal: true,
                ospolicy: Never,
                verbose: false,
            },
            Vec::new(),
        )
    )]
    fn test_cli_parser(#[case] argv: Vec<&str>, #[case] cmd: Command) {
        let parser = Parser::from_iter(argv);
        assert_eq!(Command::from_parser(parser).unwrap(), cmd);
    }

    #[test]
    fn test_run_default_options() {
        let opts = Options::default();
        let cal = Calendar::gregorian_reform();
        let dates = vec![
            Argument::CalendarDate(cal.at_ymd(2023, Month::April, 20).unwrap()),
            Argument::JulianDay(2440423),
            Argument::CalendarDate(cal.at_ymd(1066, Month::October, 14).unwrap()),
            Argument::JulianDay(2110701),
            Argument::CalendarDate(cal.at_ymd(1707, Month::April, 15).unwrap()),
            Argument::JulianDay(2344633),
        ];
        assert_eq!(
            opts.run(dates),
            vec![
                "2460055",
                "1969-07-20",
                "2110701",
                "1066-10-14",
                "2344633",
                "1707-04-15"
            ]
        );
    }

    #[test]
    fn test_run_verbose() {
        let opts = Options {
            verbose: true,
            ..Options::default()
        };
        let cal = Calendar::gregorian_reform();
        let dates = vec![
            Argument::CalendarDate(cal.at_ymd(2023, Month::April, 20).unwrap()),
            Argument::JulianDay(2440423),
            Argument::CalendarDate(cal.at_ymd(1066, Month::October, 14).unwrap()),
            Argument::JulianDay(2110701),
            Argument::CalendarDate(cal.at_ymd(1707, Month::April, 15).unwrap()),
            Argument::JulianDay(2344633),
        ];
        assert_eq!(
            opts.run(dates),
            vec![
                "2023-04-20 = 2460055",
                "2440423 = 1969-07-20",
                "1066-10-14 = 2110701",
                "2110701 = 1066-10-14",
                "1707-04-15 = 2344633",
                "2344633 = 1707-04-15"
            ]
        );
    }

    #[test]
    fn test_run_old_style() {
        let opts = Options {
            ospolicy: PostReform,
            ..Options::default()
        };
        let cal = Calendar::gregorian_reform();
        let dates = vec![
            Argument::CalendarDate(cal.at_ymd(2023, Month::April, 20).unwrap()),
            Argument::JulianDay(2440423),
            Argument::CalendarDate(cal.at_ymd(1066, Month::October, 14).unwrap()),
            Argument::JulianDay(2110701),
            Argument::CalendarDate(cal.at_ymd(1707, Month::April, 15).unwrap()),
            Argument::JulianDay(2344633),
        ];
        assert_eq!(
            opts.run(dates),
            vec![
                "2460055",
                "1969-07-20 [O.S. 1969-07-07]",
                "2110701",
                "1066-10-14",
                "2344633",
                "1707-04-15 [O.S. 1707-04-04]"
            ]
        );
    }

    #[test]
    fn test_run_old_style_uk() {
        let opts = Options {
            ospolicy: UkDelay,
            ..Options::default()
        };
        let cal = Calendar::gregorian_reform();
        let dates = vec![
            Argument::CalendarDate(cal.at_ymd(2023, Month::April, 20).unwrap()),
            Argument::JulianDay(2440423),
            Argument::CalendarDate(cal.at_ymd(1066, Month::October, 14).unwrap()),
            Argument::JulianDay(2110701),
            Argument::CalendarDate(cal.at_ymd(1707, Month::April, 15).unwrap()),
            Argument::JulianDay(2344633),
        ];
        assert_eq!(
            opts.run(dates),
            vec![
                "2460055",
                "1969-07-20",
                "2110701",
                "1066-10-14",
                "2344633",
                "1707-04-15 [O.S. 1707-04-04]"
            ]
        );
    }

    #[test]
    fn test_run_old_style_verbose() {
        let opts = Options {
            ospolicy: PostReform,
            verbose: true,
            ..Options::default()
        };
        let cal = Calendar::gregorian_reform();
        let dates = vec![
            Argument::CalendarDate(cal.at_ymd(2023, Month::April, 20).unwrap()),
            Argument::JulianDay(2440423),
            Argument::CalendarDate(cal.at_ymd(1066, Month::October, 14).unwrap()),
            Argument::JulianDay(2110701),
            Argument::CalendarDate(cal.at_ymd(1707, Month::April, 15).unwrap()),
            Argument::JulianDay(2344633),
        ];
        assert_eq!(
            opts.run(dates),
            vec![
                "2023-04-20 [O.S. 2023-04-07] = 2460055",
                "2440423 = 1969-07-20 [O.S. 1969-07-07]",
                "1066-10-14 = 2110701",
                "2110701 = 1066-10-14",
                "1707-04-15 [O.S. 1707-04-04] = 2344633",
                "2344633 = 1707-04-15 [O.S. 1707-04-04]"
            ]
        );
    }

    #[test]
    fn test_run_old_style_uk_verbose() {
        let opts = Options {
            ospolicy: UkDelay,
            verbose: true,
            ..Options::default()
        };
        let cal = Calendar::gregorian_reform();
        let dates = vec![
            Argument::CalendarDate(cal.at_ymd(2023, Month::April, 20).unwrap()),
            Argument::JulianDay(2440423),
            Argument::CalendarDate(cal.at_ymd(1066, Month::October, 14).unwrap()),
            Argument::JulianDay(2110701),
            Argument::CalendarDate(cal.at_ymd(1707, Month::April, 15).unwrap()),
            Argument::JulianDay(2344633),
        ];
        assert_eq!(
            opts.run(dates),
            vec![
                "2023-04-20 = 2460055",
                "2440423 = 1969-07-20",
                "1066-10-14 = 2110701",
                "2110701 = 1066-10-14",
                "1707-04-15 [O.S. 1707-04-04] = 2344633",
                "2344633 = 1707-04-15 [O.S. 1707-04-04]"
            ]
        );
    }

    #[test]
    fn test_run_ordinal() {
        let opts = Options {
            ordinal: true,
            ..Options::default()
        };
        let dates = vec![
            Argument::CalendarDate(
                Calendar::gregorian_reform()
                    .at_ymd(2023, Month::April, 20)
                    .unwrap(),
            ),
            Argument::JulianDay(2440423),
        ];
        assert_eq!(opts.run(dates), vec!["2460055", "1969-201"]);
    }

    #[test]
    fn test_run_ordinal_verbose() {
        let opts = Options {
            ordinal: true,
            verbose: true,
            ..Options::default()
        };
        let dates = vec![
            Argument::CalendarDate(
                Calendar::gregorian_reform()
                    .at_ymd(2023, Month::April, 20)
                    .unwrap(),
            ),
            Argument::JulianDay(2440423),
        ];
        assert_eq!(
            opts.run(dates),
            vec!["2023-110 = 2460055", "2440423 = 1969-201"]
        );
    }

    #[test]
    fn test_run_old_style_verbose_ordinal() {
        let opts = Options {
            ospolicy: PostReform,
            verbose: true,
            ordinal: true,
        };
        let cal = Calendar::gregorian_reform();
        let dates = vec![
            Argument::CalendarDate(cal.at_ymd(2023, Month::April, 20).unwrap()),
            Argument::JulianDay(2440423),
            Argument::CalendarDate(cal.at_ymd(1066, Month::October, 14).unwrap()),
            Argument::JulianDay(2110701),
            Argument::CalendarDate(cal.at_ymd(1707, Month::April, 15).unwrap()),
            Argument::JulianDay(2344633),
        ];
        assert_eq!(
            opts.run(dates),
            vec![
                "2023-110 [O.S. 2023-097] = 2460055",
                "2440423 = 1969-201 [O.S. 1969-188]",
                "1066-287 = 2110701",
                "2110701 = 1066-287",
                "1707-105 [O.S. 1707-094] = 2344633",
                "2344633 = 1707-105 [O.S. 1707-094]"
            ]
        );
    }
}
