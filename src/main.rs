use julian::{ncal, Calendar, Date, Jdnum, ReformingError};
use lexopt::{Arg, Parser, ValueExt};
use std::collections::BTreeMap;
use std::fmt::{self, Write};
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
                Arg::Short('c') | Arg::Long("countries") => return Ok(Command::Countries),
                Arg::Short('h') | Arg::Long("help") => return Ok(Command::Help),
                Arg::Short('V') | Arg::Long("version") => return Ok(Command::Version),
                Arg::Short('j') | Arg::Long("julian") => opts.calendar = Calendar::JULIAN,
                Arg::Short('J') | Arg::Long("json") => opts.json = true,
                Arg::Short('o') | Arg::Long("ordinal") => opts.ordinal = true,
                Arg::Short('q') | Arg::Long("quiet") => opts.quiet = true,
                Arg::Short('r') | Arg::Long("reformation") => {
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
                    let cal = Calendar::reforming(reform)
                        .expect("ncal reformation date should be valid reformation date");
                    let last_julian = cal
                        .last_julian_date()
                        .expect("reforming calendar should have last Julian date");
                    let first_gregorian = cal
                        .first_gregorian_date()
                        .expect("reforming calendar should have first Gregorian date");
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
                println!("  -c, --countries   List the country codes accepted by the --reformation option");
                println!();
                println!(
                    "  -j, --julian      Read & write dates in the Julian calendar instead of the"
                );
                println!("                    Gregorian");
                println!();
                println!("  -J, --json        Output JSON");
                println!();
                println!(
                    "  -o, --ordinal     Output calendar dates in the form \"YYYY-JJJ\", where the"
                );
                println!(
                    "                    part after the hyphen is the day of the year from 001 to"
                );
                println!("                    366 (the ordinal date)");
                println!();
                println!("  -q, --quiet       Do not print the input value before each output value.  Do");
                println!("                    not print \"JDN\" before Julian day numbers.");
                println!();
                println!("  -r <jdn>, --reformation <jdn>");
                println!("                    Read & write dates using a reforming calendar in which the");
                println!(
                    "                    Gregorian calendar is first observed on the date with the"
                );
                println!("                    given Julian day number");
                println!();
                println!("                    A two-letter country code may be given in place of a JDN in");
                println!(
                    "                    order to use the calendar reformation as it was observed in"
                );
                println!("                    that country.");
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
    json: bool,
    ordinal: bool,
    quiet: bool,
    style: bool,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            calendar: Calendar::GREGORIAN,
            json: false,
            ordinal: false,
            quiet: false,
            style: false,
        }
    }
}

impl Options {
    fn run(&self, args: Vec<String>) -> Result<Vec<String>, lexopt::Error> {
        let mut output = Vec::with_capacity(args.len() + 2);
        if self.json {
            output.push(json_start(self.calendar).expect("formatting a String should not fail"));
        }
        if args.is_empty() {
            let (now, _) = self
                .calendar
                .now()
                .expect("JDN for system time should fit in i32");
            output.push(
                self.date_to_jdn(now)
                    .expect("formatting a String should not fail"),
            );
        } else {
            for arg in args {
                match self.parse_arg(arg)? {
                    Argument::Date(when) => output.push(
                        self.date_to_jdn(when)
                            .expect("formatting a String should not fail"),
                    ),
                    Argument::Jdn(jdn) => output.push(
                        self.jdn_to_date(jdn)
                            .expect("formatting a String should not fail"),
                    ),
                }
            }
        }
        if self.json {
            let length = output.len();
            if length > 2 {
                for obj in output
                    .get_mut(1..(length - 1))
                    .expect("slicing Vec from 1 to length-1 should work")
                {
                    obj.push(',');
                }
            }
            if let Some(obj) = output.last_mut() {
                obj.push_str("\n    ]\n}");
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

    fn date_to_jdn(&self, when: Date) -> Result<String, fmt::Error> {
        if self.json {
            return date2json(when);
        }
        let jdn = when.julian_day_number();
        let mut s = String::new();
        if !self.quiet {
            self.fmt_date(&mut s, when)?;
            write!(&mut s, " = JDN ")?;
        }
        write!(&mut s, "{jdn}")?;
        Ok(s)
    }

    fn jdn_to_date(&self, jdn: Jdnum) -> Result<String, fmt::Error> {
        let when = self.calendar.at_jdn(jdn);
        if self.json {
            return date2json(when);
        }
        let mut s = String::new();
        if !self.quiet {
            write!(&mut s, "JDN {jdn} = ")?;
        }
        self.fmt_date(&mut s, when)?;
        Ok(s)
    }

    fn fmt_date(&self, s: &mut String, when: Date) -> Result<(), fmt::Error> {
        if self.ordinal {
            write!(s, "{when:#}")?;
        } else {
            write!(s, "{when}")?;
            if self.style && when.calendar().is_reforming() {
                if when.is_julian() {
                    write!(s, " O.S.")?;
                } else {
                    write!(s, " N.S.")?;
                }
            }
        }
        Ok(())
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

fn json_start(cal: Calendar) -> Result<String, fmt::Error> {
    let mut s = String::new();
    writeln!(&mut s, "{{")?;
    writeln!(&mut s, "    \"calendar\": {{")?;
    write!(
        &mut s,
        "{:8}\"type\": \"{}\"",
        "",
        if cal == Calendar::JULIAN {
            "julian"
        } else if cal == Calendar::GREGORIAN {
            "gregorian"
        } else {
            "reforming"
        }
    )?;
    if let Some(reform) = cal.reformation() {
        writeln!(&mut s, ",")?;
        write!(&mut s, "{:8}\"reformation\": {}", "", reform)?;
    }
    writeln!(&mut s)?;
    writeln!(&mut s, "    }},")?;
    write!(&mut s, "    \"dates\": [")?;
    Ok(s)
}

fn date2json(when: Date) -> Result<String, fmt::Error> {
    let mut s = String::new();
    writeln!(&mut s, "{:8}{{", "")?;
    writeln!(
        &mut s,
        "{:12}\"julian_day_number\": {},",
        "",
        when.julian_day_number()
    )?;
    writeln!(&mut s, "{:12}\"year\": {},", "", when.year())?;
    writeln!(&mut s, "{:12}\"month\": {},", "", when.month().number())?;
    writeln!(&mut s, "{:12}\"day\": {},", "", when.day())?;
    writeln!(&mut s, "{:12}\"ordinal\": {},", "", when.ordinal())?;
    writeln!(&mut s, "{:12}\"display\": \"{}\",", "", when)?;
    write!(&mut s, "{:12}\"ordinal_display\": \"{:#}\"", "", when)?;
    if when.calendar().is_reforming() {
        writeln!(&mut s, ",")?;
        write!(
            &mut s,
            "{:12}\"old_style\": {}",
            "",
            if when.is_julian() { "true" } else { "false" }
        )?;
    }
    writeln!(&mut s)?;
    write!(&mut s, "{:8}}}", "")?;
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use rstest::rstest;

    #[rstest]
    #[case(vec!["julian", "-h"], Command::Help)]
    #[case(vec!["julian", "--help"], Command::Help)]
    #[case(vec!["julian", "--help", "2023-04-20"], Command::Help)]
    #[case(vec!["julian", "2023-04-20", "-j", "-h"], Command::Help)]
    #[case(vec!["julian", "2023-04-20", "--help", "-V"], Command::Help)]
    #[case(vec!["julian", "2023-04-20", "-c"], Command::Countries)]
    #[case(vec!["julian", "-V"], Command::Version)]
    #[case(vec!["julian", "--version"], Command::Version)]
    #[case(vec!["julian", "-V", "-h"], Command::Version)]
    #[case(vec!["julian", "-V", "-c"], Command::Version)]
    #[case(vec!["julian", "-c"], Command::Countries)]
    #[case(vec!["julian", "--countries"], Command::Countries)]
    #[case(vec!["julian", "-c", "-h"], Command::Countries)]
    #[case(vec!["julian", "--countries", "-V"], Command::Countries)]
    #[case(vec!["julian", "-h", "-c"], Command::Help)]
    #[case(vec!["julian", "-h", "-V"], Command::Help)]
    #[case(vec!["julian", "--version", "2460055"], Command::Version)]
    #[case(vec!["julian", "2460055", "--ordinal", "--version"], Command::Version)]
    #[case(vec!["julian", "2460055", "--version", "-h"], Command::Version)]
    #[case(
        vec!["julian"],
        Command::Run(
            Options {
                calendar: Calendar::GREGORIAN,
                json: false,
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
                json: false,
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
                json: false,
                ordinal: false,
                quiet: false,
                style: false,
            },
            vec!["-123".into()],
        )
    )]
    #[case(
        vec!["julian", "-1", "23"],
        Command::Run(
            Options {
                calendar: Calendar::GREGORIAN,
                json: false,
                ordinal: false,
                quiet: false,
                style: false,
            },
            vec!["-1".into(), "23".into()],
        )
    )]
    #[case(
        vec!["julian", "-q", "-123"],
        Command::Run(
            Options {
                calendar: Calendar::GREGORIAN,
                json: false,
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
                json: false,
                ordinal: false,
                quiet: true,
                style: false,
            },
            vec!["-123".into()],
        )
    )]
    #[case(
        vec!["julian", "-j"],
        Command::Run(
            Options {
                calendar: Calendar::JULIAN,
                json: false,
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
                json: false,
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
                json: false,
                ordinal: true,
                quiet: false,
                style: false,
            },
            Vec::new(),
        )
    )]
    #[case(
        vec!["julian", "-s"],
        Command::Run(
            Options {
                calendar: Calendar::GREGORIAN,
                json: false,
                ordinal: false,
                quiet: false,
                style: true,
            },
            Vec::new(),
        )
    )]
    #[case(
        vec!["julian", "--style"],
        Command::Run(
            Options {
                calendar: Calendar::GREGORIAN,
                json: false,
                ordinal: false,
                quiet: false,
                style: true,
            },
            Vec::new(),
        )
    )]
    #[case(
        vec!["julian", "-q"],
        Command::Run(
            Options {
                calendar: Calendar::GREGORIAN,
                json: false,
                ordinal: false,
                quiet: true,
                style: false,
            },
            Vec::new(),
        )
    )]
    #[case(
        vec!["julian", "--quiet"],
        Command::Run(
            Options {
                calendar: Calendar::GREGORIAN,
                json: false,
                ordinal: false,
                quiet: true,
                style: false,
            },
            Vec::new(),
        )
    )]
    #[case(
        vec!["julian", "-r", "2299162"],
        Command::Run(
            Options {
                calendar: Calendar::reforming(2299162).unwrap(),
                json: false,
                ordinal: false,
                quiet: false,
                style: false,
            },
            Vec::new(),
        )
    )]
    #[case(
        vec!["julian", "--reformation", "2299162", "1582-10-02"],
        Command::Run(
            Options {
                calendar: Calendar::reforming(2299162).unwrap(),
                json: false,
                ordinal: false,
                quiet: false,
                style: false,
            },
            vec!["1582-10-02".into()],
        )
    )]
    #[case(
        vec!["julian", "-r", "gb"],
        Command::Run(
            Options {
                calendar: Calendar::reforming(ncal::UNITED_KINGDOM).unwrap(),
                json: false,
                ordinal: false,
                quiet: false,
                style: false,
            },
            Vec::new(),
        )
    )]
    #[case(
        vec!["julian", "-r", "GB"],
        Command::Run(
            Options {
                calendar: Calendar::reforming(ncal::UNITED_KINGDOM).unwrap(),
                json: false,
                ordinal: false,
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

    #[rstest]
    #[case("2299161", 2299161)]
    #[case("ch", ncal::SWITZERLAND)]
    #[case("CH", ncal::SWITZERLAND)]
    #[case("Ch", ncal::SWITZERLAND)]
    #[case("cH", ncal::SWITZERLAND)]
    fn test_parse_reformation(#[case] optarg: &str, #[case] reform: Jdnum) {
        let cal = parse_reformation(optarg).unwrap();
        assert!(cal.is_reforming());
        assert_eq!(cal.reformation().unwrap(), reform);
    }

    #[rstest]
    #[case("0")]
    #[case("uk")]
    #[case("Italy")]
    #[case("1582")]
    #[case("1830691")]
    #[case("2147439589")]
    fn test_parse_reformation_err(#[case] optarg: &str) {
        assert!(parse_reformation(optarg).is_err());
    }

    #[test]
    fn run_default_options() {
        let opts = Options::default();
        let dates = vec![
            "2023-04-20".into(),
            "2023-134".into(),
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
                "2023-05-14 = JDN 2460079",
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
            "2023-134".into(),
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
                "2460079",
                "1969-07-20",
                "2110701",
                "1066-10-20",
                "2344633",
                "1707-04-15"
            ]
        );
    }

    #[test]
    fn run_style() {
        let opts = Options {
            style: true,
            ..Options::default()
        };
        let dates = vec![
            "2023-04-20".into(),
            "2023-134".into(),
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
                "2023-05-14 = JDN 2460079",
                "JDN 2440423 = 1969-07-20",
                "1066-10-20 = JDN 2110701",
                "JDN 2110701 = 1066-10-20",
                "1707-04-15 = JDN 2344633",
                "JDN 2344633 = 1707-04-15"
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
            "2023-134".into(),
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
                "2023-05-14 = JDN 2460092",
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
            "2023-134".into(),
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
                "2460092",
                "1969-07-07",
                "2110701",
                "1066-10-14",
                "2344633",
                "1707-04-04"
            ]
        );
    }

    #[test]
    fn run_julian_style() {
        let opts = Options {
            calendar: Calendar::JULIAN,
            style: true,
            ..Options::default()
        };
        let dates = vec![
            "2023-04-20".into(),
            "2023-134".into(),
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
                "2023-05-14 = JDN 2460092",
                "JDN 2440423 = 1969-07-07",
                "1066-10-14 = JDN 2110701",
                "JDN 2110701 = 1066-10-14",
                "1707-04-04 = JDN 2344633",
                "JDN 2344633 = 1707-04-04"
            ]
        );
    }

    #[test]
    fn run_reforming() {
        let opts = Options {
            calendar: Calendar::REFORM1582,
            ..Options::default()
        };
        let dates = vec![
            "2023-04-20".into(),
            "2023-134".into(),
            "2440423".into(),
            "1066-10-14".into(),
            "2110701".into(),
            "1707-04-15".into(),
            "2344633".into(),
        ];
        assert_eq!(
            opts.run(dates).unwrap(),
            vec![
                "2023-04-20 = JDN 2460055",
                "2023-05-14 = JDN 2460079",
                "JDN 2440423 = 1969-07-20",
                "1066-10-14 = JDN 2110701",
                "JDN 2110701 = 1066-10-14",
                "1707-04-15 = JDN 2344633",
                "JDN 2344633 = 1707-04-15"
            ]
        );
    }

    #[test]
    fn run_reforming_quiet() {
        let opts = Options {
            calendar: Calendar::REFORM1582,
            quiet: true,
            ..Options::default()
        };
        let dates = vec![
            "2023-04-20".into(),
            "2023-134".into(),
            "2440423".into(),
            "1066-10-14".into(),
            "2110701".into(),
            "1707-04-15".into(),
            "2344633".into(),
        ];
        assert_eq!(
            opts.run(dates).unwrap(),
            vec![
                "2460055",
                "2460079",
                "1969-07-20",
                "2110701",
                "1066-10-14",
                "2344633",
                "1707-04-15"
            ]
        );
    }

    #[test]
    fn run_reforming_style() {
        let opts = Options {
            calendar: Calendar::REFORM1582,
            style: true,
            ..Options::default()
        };
        let dates = vec![
            "2023-04-20".into(),
            "2023-134".into(),
            "2440423".into(),
            "1066-10-14".into(),
            "2110701".into(),
            "1707-04-15".into(),
            "2344633".into(),
        ];
        assert_eq!(
            opts.run(dates).unwrap(),
            vec![
                "2023-04-20 N.S. = JDN 2460055",
                "2023-05-14 N.S. = JDN 2460079",
                "JDN 2440423 = 1969-07-20 N.S.",
                "1066-10-14 O.S. = JDN 2110701",
                "JDN 2110701 = 1066-10-14 O.S.",
                "1707-04-15 N.S. = JDN 2344633",
                "JDN 2344633 = 1707-04-15 N.S."
            ]
        );
    }

    #[test]
    fn run_reforming_quiet_style() {
        let opts = Options {
            calendar: Calendar::REFORM1582,
            quiet: true,
            style: true,
            ..Options::default()
        };
        let dates = vec![
            "2023-04-20".into(),
            "2023-134".into(),
            "2440423".into(),
            "1066-10-14".into(),
            "2110701".into(),
            "1707-04-15".into(),
            "2344633".into(),
        ];
        assert_eq!(
            opts.run(dates).unwrap(),
            vec![
                "2460055",
                "2460079",
                "1969-07-20 N.S.",
                "2110701",
                "1066-10-14 O.S.",
                "2344633",
                "1707-04-15 N.S."
            ]
        );
    }

    #[test]
    fn run_ordinal() {
        let opts = Options {
            ordinal: true,
            ..Options::default()
        };
        let dates = vec!["2023-04-20".into(), "2023-134".into(), "2440423".into()];
        assert_eq!(
            opts.run(dates).unwrap(),
            vec![
                "2023-110 = JDN 2460055",
                "2023-134 = JDN 2460079",
                "JDN 2440423 = 1969-201",
            ]
        );
    }

    #[test]
    fn run_ordinal_quiet() {
        let opts = Options {
            ordinal: true,
            quiet: true,
            ..Options::default()
        };
        let dates = vec!["2023-04-20".into(), "2023-134".into(), "2440423".into()];
        assert_eq!(
            opts.run(dates).unwrap(),
            vec!["2460055", "2460079", "1969-201"]
        );
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
            "2023-134".into(),
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
                "2023-134 = JDN 2460092",
                "JDN 2440423 = 1969-188",
                "1066-287 = JDN 2110701",
                "JDN 2110701 = 1066-287",
                "1707-094 = JDN 2344633",
                "JDN 2344633 = 1707-094"
            ]
        );
    }

    #[test]
    fn run_reforming_ordinal_style() {
        let opts = Options {
            calendar: Calendar::REFORM1582,
            style: true,
            ordinal: true,
            ..Options::default()
        };
        let dates = vec![
            "2023-04-20".into(),
            "2023-134".into(),
            "2440423".into(),
            "1066-10-14".into(),
            "2110701".into(),
            "1707-04-15".into(),
            "2344633".into(),
        ];
        assert_eq!(
            opts.run(dates).unwrap(),
            vec![
                "2023-110 = JDN 2460055",
                "2023-134 = JDN 2460079",
                "JDN 2440423 = 1969-201",
                "1066-287 = JDN 2110701",
                "JDN 2110701 = 1066-287",
                "1707-105 = JDN 2344633",
                "JDN 2344633 = 1707-105"
            ]
        );
    }

    #[test]
    fn run_json() {
        let opts = Options {
            json: true,
            ..Options::default()
        };
        let dates = vec![
            "2023-04-20".into(),
            "2023-134".into(),
            "2440423".into(),
            "1066-10-20".into(),
            "2110701".into(),
            "1707-04-15".into(),
            "2344633".into(),
        ];
        assert_eq!(
            opts.run(dates).unwrap().join("\n"),
            indoc! {r#"{
                "calendar": {
                    "type": "gregorian"
                },
                "dates": [
                    {
                        "julian_day_number": 2460055,
                        "year": 2023,
                        "month": 4,
                        "day": 20,
                        "ordinal": 110,
                        "display": "2023-04-20",
                        "ordinal_display": "2023-110"
                    },
                    {
                        "julian_day_number": 2460079,
                        "year": 2023,
                        "month": 5,
                        "day": 14,
                        "ordinal": 134,
                        "display": "2023-05-14",
                        "ordinal_display": "2023-134"
                    },
                    {
                        "julian_day_number": 2440423,
                        "year": 1969,
                        "month": 7,
                        "day": 20,
                        "ordinal": 201,
                        "display": "1969-07-20",
                        "ordinal_display": "1969-201"
                    },
                    {
                        "julian_day_number": 2110701,
                        "year": 1066,
                        "month": 10,
                        "day": 20,
                        "ordinal": 293,
                        "display": "1066-10-20",
                        "ordinal_display": "1066-293"
                    },
                    {
                        "julian_day_number": 2110701,
                        "year": 1066,
                        "month": 10,
                        "day": 20,
                        "ordinal": 293,
                        "display": "1066-10-20",
                        "ordinal_display": "1066-293"
                    },
                    {
                        "julian_day_number": 2344633,
                        "year": 1707,
                        "month": 4,
                        "day": 15,
                        "ordinal": 105,
                        "display": "1707-04-15",
                        "ordinal_display": "1707-105"
                    },
                    {
                        "julian_day_number": 2344633,
                        "year": 1707,
                        "month": 4,
                        "day": 15,
                        "ordinal": 105,
                        "display": "1707-04-15",
                        "ordinal_display": "1707-105"
                    }
                ]
            }"#}
        );
    }

    #[test]
    fn run_json_julian() {
        let opts = Options {
            calendar: Calendar::JULIAN,
            json: true,
            ..Options::default()
        };
        let dates = vec![
            "2023-04-20".into(),
            "2023-134".into(),
            "2440423".into(),
            "1066-10-14".into(),
            "2110701".into(),
            "1707-04-04".into(),
            "2344633".into(),
        ];
        assert_eq!(
            opts.run(dates).unwrap().join("\n"),
            indoc! {r#"{
                "calendar": {
                    "type": "julian"
                },
                "dates": [
                    {
                        "julian_day_number": 2460068,
                        "year": 2023,
                        "month": 4,
                        "day": 20,
                        "ordinal": 110,
                        "display": "2023-04-20",
                        "ordinal_display": "2023-110"
                    },
                    {
                        "julian_day_number": 2460092,
                        "year": 2023,
                        "month": 5,
                        "day": 14,
                        "ordinal": 134,
                        "display": "2023-05-14",
                        "ordinal_display": "2023-134"
                    },
                    {
                        "julian_day_number": 2440423,
                        "year": 1969,
                        "month": 7,
                        "day": 7,
                        "ordinal": 188,
                        "display": "1969-07-07",
                        "ordinal_display": "1969-188"
                    },
                    {
                        "julian_day_number": 2110701,
                        "year": 1066,
                        "month": 10,
                        "day": 14,
                        "ordinal": 287,
                        "display": "1066-10-14",
                        "ordinal_display": "1066-287"
                    },
                    {
                        "julian_day_number": 2110701,
                        "year": 1066,
                        "month": 10,
                        "day": 14,
                        "ordinal": 287,
                        "display": "1066-10-14",
                        "ordinal_display": "1066-287"
                    },
                    {
                        "julian_day_number": 2344633,
                        "year": 1707,
                        "month": 4,
                        "day": 4,
                        "ordinal": 94,
                        "display": "1707-04-04",
                        "ordinal_display": "1707-094"
                    },
                    {
                        "julian_day_number": 2344633,
                        "year": 1707,
                        "month": 4,
                        "day": 4,
                        "ordinal": 94,
                        "display": "1707-04-04",
                        "ordinal_display": "1707-094"
                    }
                ]
            }"#}
        );
    }

    #[test]
    fn run_json_reforming() {
        let opts = Options {
            calendar: Calendar::reforming(ncal::UNITED_KINGDOM).unwrap(),
            json: true,
            ..Options::default()
        };
        let dates = vec![
            "2023-04-20".into(),
            "2023-134".into(),
            "2440423".into(),
            "1066-10-14".into(),
            "2110701".into(),
            "1707-04-04".into(),
            "2344633".into(),
        ];
        assert_eq!(
            opts.run(dates).unwrap().join("\n"),
            indoc! {r#"{
                "calendar": {
                    "type": "reforming",
                    "reformation": 2361222
                },
                "dates": [
                    {
                        "julian_day_number": 2460055,
                        "year": 2023,
                        "month": 4,
                        "day": 20,
                        "ordinal": 110,
                        "display": "2023-04-20",
                        "ordinal_display": "2023-110",
                        "old_style": false
                    },
                    {
                        "julian_day_number": 2460079,
                        "year": 2023,
                        "month": 5,
                        "day": 14,
                        "ordinal": 134,
                        "display": "2023-05-14",
                        "ordinal_display": "2023-134",
                        "old_style": false
                    },
                    {
                        "julian_day_number": 2440423,
                        "year": 1969,
                        "month": 7,
                        "day": 20,
                        "ordinal": 201,
                        "display": "1969-07-20",
                        "ordinal_display": "1969-201",
                        "old_style": false
                    },
                    {
                        "julian_day_number": 2110701,
                        "year": 1066,
                        "month": 10,
                        "day": 14,
                        "ordinal": 287,
                        "display": "1066-10-14",
                        "ordinal_display": "1066-287",
                        "old_style": true
                    },
                    {
                        "julian_day_number": 2110701,
                        "year": 1066,
                        "month": 10,
                        "day": 14,
                        "ordinal": 287,
                        "display": "1066-10-14",
                        "ordinal_display": "1066-287",
                        "old_style": true
                    },
                    {
                        "julian_day_number": 2344633,
                        "year": 1707,
                        "month": 4,
                        "day": 4,
                        "ordinal": 94,
                        "display": "1707-04-04",
                        "ordinal_display": "1707-094",
                        "old_style": true
                    },
                    {
                        "julian_day_number": 2344633,
                        "year": 1707,
                        "month": 4,
                        "day": 4,
                        "ordinal": 94,
                        "display": "1707-04-04",
                        "ordinal_display": "1707-094",
                        "old_style": true
                    }
                ]
            }"#}
        );
    }

    #[test]
    fn run_json_one_arg() {
        let opts = Options {
            json: true,
            ..Options::default()
        };
        let dates = vec!["2023-04-20".into()];
        assert_eq!(
            opts.run(dates).unwrap().join("\n"),
            indoc! {r#"{
                "calendar": {
                    "type": "gregorian"
                },
                "dates": [
                    {
                        "julian_day_number": 2460055,
                        "year": 2023,
                        "month": 4,
                        "day": 20,
                        "ordinal": 110,
                        "display": "2023-04-20",
                        "ordinal_display": "2023-110"
                    }
                ]
            }"#}
        );
    }
}
