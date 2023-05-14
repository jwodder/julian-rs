use anyhow::Context;
use julian::{Calendar, Date, Jdnum};
use lexopt::{Arg, Parser, ValueExt};
use std::fmt::Write;

#[derive(Debug, Eq, PartialEq)]
enum Command {
    Run(Options, Vec<String>),
    Help,
    Version,
}

impl Command {
    fn from_parser(mut parser: Parser) -> Result<Command, lexopt::Error> {
        let mut opts = Options::default();
        let mut args = Vec::new();
        while let Some(arg) = parser.next()? {
            match arg {
                Arg::Short('h') | Arg::Long("help") => return Ok(Command::Help),
                Arg::Short('V') | Arg::Long("version") => return Ok(Command::Version),
                Arg::Short('J') | Arg::Long("julian") => opts.julian = true,
                Arg::Short('o') | Arg::Long("ordinal") => opts.ordinal = true,
                Arg::Short('q') | Arg::Long("quiet") => opts.quiet = true,
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

    fn run(self) -> anyhow::Result<()> {
        match self {
            Command::Run(opts, args) => {
                for ln in opts.run(args)? {
                    println!("{ln}");
                }
            }
            Command::Help => {
                println!("Usage: julian [<options>] [<date> ...]");
                println!();
                println!("Convert Julian day numbers to & from calendar dates");
                println!();
                println!("Options:");
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

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Options {
    julian: bool,
    ordinal: bool,
    quiet: bool,
}

impl Options {
    fn run(&self, args: Vec<String>) -> anyhow::Result<Vec<String>> {
        let mut output = Vec::with_capacity(args.len());
        let cal = if self.julian {
            Calendar::JULIAN
        } else {
            Calendar::GREGORIAN
        };
        if args.is_empty() {
            let (now, _) = cal.now().unwrap();
            let jd = now.julian_day_number();
            if !self.quiet {
                output.push(format!("{now} = {jd}"));
            } else {
                output.push(jd.to_string());
            }
        } else {
            for arg in args {
                if arg.match_indices('-').any(|(i, _)| i > 0) {
                    let when = cal
                        .parse_date(&arg)
                        .with_context(|| format!("Invalid calendar date: {arg}"))?;
                    let jdn = when.julian_day_number();
                    let mut s = String::new();
                    if !self.quiet {
                        self.fmt_date(&mut s, when);
                        write!(&mut s, " = JDN ").unwrap();
                    }
                    write!(&mut s, "{jdn}").unwrap();
                    output.push(s);
                } else {
                    let jdn = arg
                        .parse::<Jdnum>()
                        .with_context(|| format!("Invalid Julian day number: {arg}"))?;
                    let when = cal.at_jdn(jdn);
                    let mut s = String::new();
                    if !self.quiet {
                        write!(&mut s, "JDN {jdn} = ").unwrap();
                    }
                    self.fmt_date(&mut s, when);
                    output.push(s);
                }
            }
        }
        Ok(output)
    }

    fn fmt_date(&self, s: &mut String, when: Date) {
        if self.ordinal {
            write!(s, "{when:#}").unwrap();
        } else {
            write!(s, "{when}").unwrap();
        }
    }
}

fn main() -> anyhow::Result<()> {
    Command::from_parser(Parser::from_env())?.run()
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
                julian: false,
                ordinal: false,
                quiet: false,
            },
            Vec::new(),
        )
    )]
    #[case(
        vec!["julian", "123"],
        Command::Run(
            Options {
                julian: false,
                ordinal: false,
                quiet: false,
            },
            vec!["123".into()],
        )
    )]
    #[case(
        vec!["julian", "-123"],
        Command::Run(
            Options {
                julian: false,
                ordinal: false,
                quiet: false,
            },
            vec!["-123".into()],
        )
    )]
    #[case(
        vec!["julian", "-q", "-123"],
        Command::Run(
            Options {
                julian: false,
                ordinal: false,
                quiet: true,
            },
            vec!["-123".into()],
        )
    )]
    #[case(
        vec!["julian", "-123", "-q"],
        Command::Run(
            Options {
                julian: false,
                ordinal: false,
                quiet: true,
            },
            vec!["-123".into()],
        )
    )]
    #[case(
        vec!["julian", "-J"],
        Command::Run(
            Options {
                julian: true,
                ordinal: false,
                quiet: false,
            },
            Vec::new(),
        )
    )]
    #[case(
        vec!["julian", "-o"],
        Command::Run(
            Options {
                julian: false,
                ordinal: true,
                quiet: false,
            },
            Vec::new(),
        )
    )]
    #[case(
        vec!["julian", "--ordinal"],
        Command::Run(
            Options {
                julian: false,
                ordinal: true,
                quiet: false,
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
            julian: true,
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
            julian: true,
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
            julian: true,
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
