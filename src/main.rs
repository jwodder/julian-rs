use julian::{ncal, Calendar, Date, Jdnum, ReformingError};
use lexopt::{Arg, Parser, ValueExt};
use std::collections::BTreeMap;
use std::fmt::Write;
use thiserror::Error;

#[derive(Debug, Eq, PartialEq)]
enum Command {
    Run(Options, Vec<String>),
    Countries,
    Help,
    Version,
}

impl Command {
    fn from_parser(mut parser: Parser) -> Result<Command, lexopt::Error> {
        let mut opts = Options::default();
        let mut args = Vec::new();
        while let Some(arg) = parser.next()? {
            match arg {
                Arg::Short('C') | Arg::Long("countries") => return Ok(Command::Countries),
                Arg::Short('h') | Arg::Long("help") => return Ok(Command::Help),
                Arg::Short('V') | Arg::Long("version") => return Ok(Command::Version),
                Arg::Short('J') | Arg::Long("julian") => opts.calendar = Calendar::JULIAN,
                Arg::Short('o') | Arg::Long("ordinal") => opts.ordinal = true,
                Arg::Short('q') | Arg::Long("quiet") => opts.quiet = true,
                Arg::Short('R') | Arg::Long("reformation") => {
                    let optarg = parser.value()?.string()?;
                    match parse_reformation(&optarg) {
                        Ok(cal) => opts.calendar = cal,
                        Err(e) => {
                            return Err(lexopt::Error::ParsingFailed {
                                value: optarg,
                                error: Box::new(e),
                            });
                        }
                    }
                }
                Arg::Short('s') | Arg::Long("style") => opts.style = true,
                Arg::Short(c) if c.is_ascii_digit() => {
                    let mut s = String::from_iter(['-', c]);
                    if let Some(v) = parser.optional_value() {
                        s.push_str(&(v.string()?));
                    }
                    args.push(s);
                }
                Arg::Value(val) => args.push(val.string()?),
                _ => return Err(arg.unexpected()),
            }
        }
        Ok(Command::Run(opts, args))
    }

    fn run(self) -> Result<(), lexopt::Error> {
        match self {
            Command::Run(opts, args) => {
                for ln in opts.run(args)? {
                    println!("{ln}");
                }
            }
            Command::Countries => {
                println!("Code  Country         Reformation  Last Julian  First Gregorian");
                for (code, (country, reform)) in national_reformations() {
                    let cal = Calendar::reforming(reform).unwrap();
                    let last_julian = cal.last_julian_date().unwrap();
                    let first_gregorian = cal.first_gregorian_date().unwrap();
                    println!(
                        "{code}    {country:<14}  JDN {reform}  {last_julian}   {first_gregorian}"
                    );
                }
            }
            Command::Help => {
                println!("Usage: julian [<options>] [<date> ...]");
                println!();
                println!("Convert Julian day numbers to & from calendar dates");
                println!();
                println!("Options:");
                println!("  -C, --countries   List the country codes accepted by the --reformation option");
                println!();
                println!(
                    "  -J, --julian      Read & write dates in the Julian calendar instead of the"
                );
                println!("                    Gregorian");
                println!();
                println!("  -o, --ordinal     Output calendar dates in the form \"YYYY-DDD\"");
                println!();
                println!(
                    "                    The part after the hyphen is the day of the year from 001"
                );
                println!("                    to 366 (the ordinal date).");
                println!();
                println!("  -q, --quiet       Do not print the input date before each output date");
                println!();
                println!("  -R <jdn>, --reformation <jdn>");
                println!("                    Read & write dates using a reforming calendar in which the");
                println!(
                    "                    Gregorian calendar is first observed on the date with the"
                );
                println!("                    given Julian day number");
                println!();
                println!("                    A two-letter country code may be given in place of a JDN in");
                println!(
                    "                    order to use the calendar reformation as observed in that"
                );
                println!("                    country.");
                println!();
                println!("  -s, --style       Mark dates in reforming calendars as \"O.S.\" (Old Style) or");
                println!("                    \"N.S.\" (New Style)");
                println!();
                println!("  -h, --help        Display this help message and exit");
                println!("  -V, --version     Show the program version and exit");
            }
            Command::Version => {
                println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Options {
    calendar: Calendar,
    ordinal: bool,
    quiet: bool,
    style: bool,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            calendar: Calendar::GREGORIAN,
            ordinal: false,
            quiet: false,
            style: false,
        }
    }
}

impl Options {
    fn run(&self, args: Vec<String>) -> Result<Vec<String>, lexopt::Error> {
        let mut output = Vec::with_capacity(args.len());
        if args.is_empty() {
            let (now, _) = self.calendar.now().unwrap();
            output.push(self.date_to_jdn(now));
        } else {
            for arg in args {
                match self.parse_arg(arg)? {
                    Argument::Date(when) => output.push(self.date_to_jdn(when)),
                    Argument::Jdn(jdn) => output.push(self.jdn_to_date(jdn)),
                }
            }
        }
        Ok(output)
    }

    fn parse_arg(&self, s: String) -> Result<Argument, lexopt::Error> {
        if s.match_indices('-').any(|(i, _)| i > 0) {
            match self.calendar.parse_date(&s) {
                Ok(d) => Ok(Argument::Date(d)),
                Err(e) => Err(lexopt::Error::ParsingFailed {
                    value: s,
                    error: Box::new(e),
                }),
            }
        } else {
            match s.parse::<Jdnum>() {
                Ok(jdn) => Ok(Argument::Jdn(jdn)),
                Err(e) => Err(lexopt::Error::ParsingFailed {
                    value: s,
                    error: Box::new(e),
                }),
            }
        }
    }

    fn date_to_jdn(&self, when: Date) -> String {
        let jdn = when.julian_day_number();
        let mut s = String::new();
        if !self.quiet {
            self.fmt_date(&mut s, when);
            write!(&mut s, " = JDN ").unwrap();
        }
        write!(&mut s, "{jdn}").unwrap();
        s
    }

    fn jdn_to_date(&self, jdn: Jdnum) -> String {
        let when = self.calendar.at_jdn(jdn);
        let mut s = String::new();
        if !self.quiet {
            write!(&mut s, "JDN {jdn} = ").unwrap();
        }
        self.fmt_date(&mut s, when);
        s
    }

    fn fmt_date(&self, s: &mut String, when: Date) {
        if self.ordinal {
            write!(s, "{when:#}").unwrap();
        } else {
            write!(s, "{when}").unwrap();
            if self.style && when.calendar().is_reforming() {
                if when.is_julian() {
                    write!(s, " O.S.").unwrap();
                } else {
                    write!(s, " N.S.").unwrap();
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Argument {
    Date(Date),
    Jdn(Jdnum),
}

fn main() -> Result<(), lexopt::Error> {
    Command::from_parser(Parser::from_env())?.run()
}

fn parse_reformation(s: &str) -> Result<Calendar, ReformationError> {
    let reformation = if s.chars().all(char::is_alphabetic) {
        let key = s.to_ascii_uppercase();
        national_reformations()
            .get(key.as_str())
            .map(|pair| pair.1)
            .ok_or(ReformationError::CountryCode)?
    } else {
        s.parse::<Jdnum>().map_err(|_| ReformationError::Jdn)?
    };
    Ok(Calendar::reforming(reformation)?)
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
enum ReformationError {
    #[error("unrecognized county code")]
    CountryCode,
    #[error("invalid number")]
    Jdn,
    #[error(transparent)]
    Reforming(#[from] ReformingError),
}

fn national_reformations() -> BTreeMap<&'static str, (&'static str, Jdnum)> {
    BTreeMap::from([
        ("AL", ("Albania", ncal::ALBANIA)),
        ("AT", ("Austria", ncal::AUSTRIA)),
        ("AU", ("Australia", ncal::AUSTRALIA)),
        ("BE", ("Belgium", ncal::BELGIUM)),
        ("BG", ("Bulgaria", ncal::BULGARIA)),
        ("CA", ("Canada", ncal::CANADA)),
        ("CH", ("Switzerland", ncal::SWITZERLAND)),
        ("CN", ("China", ncal::CHINA)),
        ("CZ", ("Czech Republic", ncal::CZECH_REPUBLIC)),
        ("DE", ("Germany", ncal::GERMANY)),
        ("DK", ("Denmark", ncal::DENMARK)),
        ("ES", ("Spain", ncal::SPAIN)),
        ("FI", ("Finland", ncal::FINLAND)),
        ("FR", ("France", ncal::FRANCE)),
        ("GB", ("United Kingdom", ncal::UNITED_KINGDOM)),
        ("GR", ("Greece", ncal::GREECE)),
        ("HU", ("Hungary", ncal::HUNGARY)),
        ("IS", ("Iceland", ncal::ICELAND)),
        ("IT", ("Italy", ncal::ITALY)),
        ("JP", ("Japan", ncal::JAPAN)),
        ("LI", ("Lithuania", ncal::LITHUANIA)),
        ("LU", ("Luxembourg", ncal::LUXEMBOURG)),
        ("LV", ("Latvia", ncal::LATVIA)),
        ("NL", ("Netherlands", ncal::NETHERLANDS)),
        ("NO", ("Norway", ncal::NORWAY)),
        ("PL", ("Poland", ncal::POLAND)),
        ("PT", ("Portugal", ncal::PORTUGAL)),
        ("RO", ("Romania", ncal::ROMANIA)),
        ("RU", ("Russia", ncal::RUSSIA)),
        ("SI", ("Slovnia", ncal::SLOVENIA)),
        ("SE", ("Sweden", ncal::SWEDEN)),
        ("TR", ("Turkey", ncal::TURKEY)),
        ("US", ("United States", ncal::UNITED_STATES)),
        ("YU", ("Yugoslavia", ncal::YUGOSLAVIA)),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec!["julian", "-h"], Command::Help)]
    #[case(vec!["julian", "--help"], Command::Help)]
    #[case(vec!["julian", "--help", "2023-04-20"], Command::Help)]
    #[case(vec!["julian", "2023-04-20", "-J", "-h"], Command::Help)]
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
                calendar: Calendar::GREGORIAN,
                ordinal: false,
                quiet: false,
                style: false,
            },
            Vec::new(),
        )
    )]
    #[case(
        vec!["julian", "123"],
        Command::Run(
            Options {
                calendar: Calendar::GREGORIAN,
                ordinal: false,
                quiet: false,
                style: false,
            },
            vec!["123".into()],
        )
    )]
    #[case(
        vec!["julian", "-123"],
        Command::Run(
            Options {
                calendar: Calendar::GREGORIAN,
                ordinal: false,
                quiet: false,
                style: false,
            },
            vec!["-123".into()],
        )
    )]
    #[case(
        vec!["julian", "-q", "-123"],
        Command::Run(
            Options {
                calendar: Calendar::GREGORIAN,
                ordinal: false,
                quiet: true,
                style: false,
            },
            vec!["-123".into()],
        )
    )]
    #[case(
        vec!["julian", "-123", "-q"],
        Command::Run(
            Options {
                calendar: Calendar::GREGORIAN,
                ordinal: false,
                quiet: true,
                style: false,
            },
            vec!["-123".into()],
        )
    )]
    #[case(
        vec!["julian", "-J"],
        Command::Run(
            Options {
                calendar: Calendar::JULIAN,
                ordinal: false,
                quiet: false,
                style: false,
            },
            Vec::new(),
        )
    )]
    #[case(
        vec!["julian", "-o"],
        Command::Run(
            Options {
                calendar: Calendar::GREGORIAN,
                ordinal: true,
                quiet: false,
                style: false,
            },
            Vec::new(),
        )
    )]
    #[case(
        vec!["julian", "--ordinal"],
        Command::Run(
            Options {
                calendar: Calendar::GREGORIAN,
                ordinal: true,
                quiet: false,
                style: false,
            },
            Vec::new(),
        )
    )]
    fn cli_parser(#[case] argv: Vec<&str>, #[case] cmd: Command) {
        let parser = Parser::from_iter(argv);
        assert_eq!(Command::from_parser(parser).unwrap(), cmd);
    }

    #[test]
    fn run_default_options() {
        let opts = Options::default();
        let dates = vec![
            "2023-04-20".into(),
            "2440423".into(),
            "1066-10-20".into(),
            "2110701".into(),
            "1707-04-15".into(),
            "2344633".into(),
        ];
        assert_eq!(
            opts.run(dates).unwrap(),
            vec![
                "2023-04-20 = JDN 2460055",
                "JDN 2440423 = 1969-07-20",
                "1066-10-20 = JDN 2110701",
                "JDN 2110701 = 1066-10-20",
                "1707-04-15 = JDN 2344633",
                "JDN 2344633 = 1707-04-15"
            ]
        );
    }

    #[test]
    fn run_quiet() {
        let opts = Options {
            quiet: true,
            ..Options::default()
        };
        let dates = vec![
            "2023-04-20".into(),
            "2440423".into(),
            "1066-10-20".into(),
            "2110701".into(),
            "1707-04-15".into(),
            "2344633".into(),
        ];
        assert_eq!(
            opts.run(dates).unwrap(),
            vec![
                "2460055",
                "1969-07-20",
                "2110701",
                "1066-10-20",
                "2344633",
                "1707-04-15"
            ]
        );
    }

    #[test]
    fn run_julian() {
        let opts = Options {
            calendar: Calendar::JULIAN,
            ..Options::default()
        };
        let dates = vec![
            "2023-04-20".into(),
            "2440423".into(),
            "1066-10-14".into(),
            "2110701".into(),
            "1707-04-04".into(),
            "2344633".into(),
        ];
        assert_eq!(
            opts.run(dates).unwrap(),
            vec![
                "2023-04-20 = JDN 2460068",
                "JDN 2440423 = 1969-07-07",
                "1066-10-14 = JDN 2110701",
                "JDN 2110701 = 1066-10-14",
                "1707-04-04 = JDN 2344633",
                "JDN 2344633 = 1707-04-04"
            ]
        );
    }

    #[test]
    fn run_julian_quiet() {
        let opts = Options {
            calendar: Calendar::JULIAN,
            quiet: true,
            ..Options::default()
        };
        let dates = vec![
            "2023-04-20".into(),
            "2440423".into(),
            "1066-10-14".into(),
            "2110701".into(),
            "1707-04-04".into(),
            "2344633".into(),
        ];
        assert_eq!(
            opts.run(dates).unwrap(),
            vec![
                "2460068",
                "1969-07-07",
                "2110701",
                "1066-10-14",
                "2344633",
                "1707-04-04"
            ]
        );
    }

    #[test]
    fn run_ordinal() {
        let opts = Options {
            ordinal: true,
            ..Options::default()
        };
        let dates = vec!["2023-110".into(), "2440423".into()];
        assert_eq!(
            opts.run(dates).unwrap(),
            vec!["2023-110 = JDN 2460055", "JDN 2440423 = 1969-201"]
        );
    }

    #[test]
    fn run_ordinal_quiet() {
        let opts = Options {
            ordinal: true,
            quiet: true,
            ..Options::default()
        };
        let dates = vec!["2023-110".into(), "2440423".into()];
        assert_eq!(opts.run(dates).unwrap(), vec!["2460055", "1969-201"]);
    }

    #[test]
    fn run_julian_ordinal() {
        let opts = Options {
            calendar: Calendar::JULIAN,
            ordinal: true,
            ..Options::default()
        };
        let dates = vec![
            "2023-04-20".into(),
            "2440423".into(),
            "1066-10-14".into(),
            "2110701".into(),
            "1707-04-04".into(),
            "2344633".into(),
        ];
        assert_eq!(
            opts.run(dates).unwrap(),
            vec![
                "2023-110 = JDN 2460068",
                "JDN 2440423 = 1969-188",
                "1066-287 = JDN 2110701",
                "JDN 2110701 = 1066-287",
                "1707-094 = JDN 2344633",
                "JDN 2344633 = 1707-094"
            ]
        );
    }
}
