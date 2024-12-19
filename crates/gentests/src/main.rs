/// This is a program for automatically generating a number of test cases for
/// `julian` from data files in `data/`.  Run it whenever a file in `data/`
/// changes.
///
/// Run with:
///
///     cargo run --bin gentests --features gentests -- path/to/root/of/project
///
/// where the path argument can be omitted when running from the project root
/// itself.
use anyhow::Context;
use serde::Deserialize;
use std::fs::{create_dir_all, File};
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct JdnRecord {
    jdn: i32,
    jyear: i32,
    jmonth: String,
    jday: u16,
    jordinal: u16,
    gyear: i32,
    gmonth: String,
    gday: u16,
    gordinal: u16,
}

impl JdnRecord {
    fn jdn_id(&self) -> String {
        self.jdn.to_string().replace('-', "neg_")
    }

    fn julian_date(&self) -> Date<'_> {
        Date {
            jdn: self.jdn,
            year: self.jyear,
            month: self.jmonth.as_str(),
            day: self.jday,
            ordinal: self.jordinal,
        }
    }

    fn gregorian_date(&self) -> Date<'_> {
        Date {
            jdn: self.jdn,
            year: self.gyear,
            month: self.gmonth.as_str(),
            day: self.gday,
            ordinal: self.gordinal,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Date<'a> {
    jdn: i32,
    year: i32,
    month: &'a str,
    day: u16,
    ordinal: u16,
}

impl Date<'_> {
    fn assert<W: Write>(&self, mut out: W) -> io::Result<()> {
        writeln!(out, "        assert_eq!(date.year(), {});", self.year)?;
        writeln!(
            out,
            "        assert_eq!(date.month(), Month::{});",
            self.month
        )?;
        writeln!(out, "        assert_eq!(date.day(), {});", self.day)?;
        writeln!(out, "        assert_eq!(date.day_ordinal(), {});", self.day)?;
        writeln!(
            out,
            "        assert_eq!(date.day_ordinal0(), {});",
            self.day - 1
        )?;
        writeln!(out, "        assert_eq!(date.ordinal(), {});", self.ordinal)?;
        writeln!(
            out,
            "        assert_eq!(date.ordinal0(), {});",
            self.ordinal - 1
        )?;
        writeln!(
            out,
            "        assert_eq!(date.julian_day_number(), {});",
            self.jdn
        )?;
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct ReformingRecord {
    reformation: i32,
    jdn: i32,
    year: i32,
    month: String,
    day: u16,
    ordinal: u16,
    day_ordinal: u16,
}

impl ReformingRecord {
    fn jdn_id(&self) -> String {
        self.jdn.to_string().replace('-', "neg_")
    }

    fn assert<W: Write>(&self, mut out: W) -> io::Result<()> {
        if self.jdn < self.reformation {
            writeln!(out, "        assert!(date.is_julian());")?;
            writeln!(out, "        assert!(!date.is_gregorian());")?;
        } else {
            writeln!(out, "        assert!(!date.is_julian());")?;
            writeln!(out, "        assert!(date.is_gregorian());")?;
        }
        writeln!(out, "        assert_eq!(date.year(), {});", self.year)?;
        writeln!(
            out,
            "        assert_eq!(date.month(), Month::{});",
            self.month
        )?;
        writeln!(out, "        assert_eq!(date.day(), {});", self.day)?;
        writeln!(
            out,
            "        assert_eq!(date.day_ordinal(), {});",
            self.day_ordinal
        )?;
        writeln!(
            out,
            "        assert_eq!(date.day_ordinal0(), {});",
            self.day_ordinal - 1
        )?;
        writeln!(out, "        assert_eq!(date.ordinal(), {});", self.ordinal)?;
        writeln!(
            out,
            "        assert_eq!(date.ordinal0(), {});",
            self.ordinal - 1
        )?;
        writeln!(
            out,
            "        assert_eq!(date.julian_day_number(), {});",
            self.jdn
        )?;
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let project_root = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_default();
    let outdir = project_root
        .join("crates")
        .join("julian")
        .join("src")
        .join("tests")
        .join("autogen");
    create_dir_all(&outdir)
        .with_context(|| format!("Failed to create directory at {}", outdir.display()))?;
    let data_dir = project_root.join("testdata");
    make_jdn_tests(data_dir.join("jdn.csv"), outdir.join("jdn.rs"))?;
    make_reforming_tests(data_dir.join("reforming.csv"), outdir.join("reforming.rs"))?;
    Ok(())
}

fn make_jdn_tests(inpath: PathBuf, outpath: PathBuf) -> anyhow::Result<()> {
    eprintln!("Loading {} ...", inpath.display());
    let jdn_records = csv::ReaderBuilder::new()
        .from_path(&inpath)
        .with_context(|| format!("Failed to read {}", inpath.display()))?
        .deserialize()
        .collect::<Result<Vec<JdnRecord>, _>>()
        .with_context(|| format!("Failed to deserialize {}", inpath.display()))?;
    eprintln!("Generating {} ...", outpath.display());
    let mut fp = BufWriter::new(File::create(outpath)?);

    writeln!(&mut fp, "mod jdn_to_julian {{")?;
    writeln!(&mut fp, "    use crate::{{Calendar, Month}};")?;
    writeln!(&mut fp)?;
    let mut first = true;
    for r in &jdn_records {
        if !std::mem::replace(&mut first, false) {
            writeln!(&mut fp)?;
        }
        writeln!(&mut fp, "    #[test]")?;
        writeln!(&mut fp, "    fn jdn_{}() {{", r.jdn_id())?;
        writeln!(&mut fp, "        let cal = Calendar::JULIAN;")?;
        writeln!(&mut fp, "        let date = cal.at_jdn({});", r.jdn)?;
        writeln!(&mut fp, "        assert_eq!(date.calendar(), cal);")?;
        writeln!(&mut fp, "        assert!(date.is_julian());")?;
        writeln!(&mut fp, "        assert!(!date.is_gregorian());")?;
        r.julian_date().assert(&mut fp)?;
        writeln!(&mut fp, "    }}")?;
    }
    writeln!(&mut fp, "}}")?;
    writeln!(&mut fp)?;

    writeln!(&mut fp, "mod julian_to_jdn {{")?;
    writeln!(&mut fp, "    use crate::{{Calendar, Month}};")?;
    writeln!(&mut fp)?;
    let mut first = true;
    for r in &jdn_records {
        if !std::mem::replace(&mut first, false) {
            writeln!(&mut fp)?;
        }
        writeln!(&mut fp, "    #[test]")?;
        writeln!(&mut fp, "    fn jdn_{}_ymd() {{", r.jdn_id())?;
        writeln!(&mut fp, "        let cal = Calendar::JULIAN;")?;
        writeln!(
            &mut fp,
            "        let date = cal.at_ymd({}, Month::{}, {}).unwrap();",
            r.jyear, r.jmonth, r.jday
        )?;
        writeln!(&mut fp, "        assert_eq!(date.calendar(), cal);")?;
        writeln!(&mut fp, "        assert!(date.is_julian());")?;
        writeln!(&mut fp, "        assert!(!date.is_gregorian());")?;
        r.julian_date().assert(&mut fp)?;
        writeln!(&mut fp, "    }}")?;
        writeln!(&mut fp)?;
        writeln!(&mut fp, "    #[test]")?;
        writeln!(&mut fp, "    fn jdn_{}_ordinal() {{", r.jdn_id())?;
        writeln!(&mut fp, "        let cal = Calendar::JULIAN;")?;
        writeln!(
            &mut fp,
            "        let date = cal.at_ordinal_date({}, {}).unwrap();",
            r.jyear, r.jordinal
        )?;
        writeln!(&mut fp, "        assert_eq!(date.calendar(), cal);")?;
        writeln!(&mut fp, "        assert!(date.is_julian());")?;
        writeln!(&mut fp, "        assert!(!date.is_gregorian());")?;
        r.julian_date().assert(&mut fp)?;
        writeln!(&mut fp, "    }}")?;
    }
    writeln!(&mut fp, "}}")?;
    writeln!(&mut fp)?;

    writeln!(&mut fp, "mod jdn_to_gregorian {{")?;
    writeln!(&mut fp, "    use crate::{{Calendar, Month}};")?;
    writeln!(&mut fp)?;
    let mut first = true;
    for r in &jdn_records {
        if !std::mem::replace(&mut first, false) {
            writeln!(&mut fp)?;
        }
        writeln!(&mut fp, "    #[test]")?;
        writeln!(&mut fp, "    fn jdn_{}() {{", r.jdn_id())?;
        writeln!(&mut fp, "        let cal = Calendar::GREGORIAN;")?;
        writeln!(&mut fp, "        let date = cal.at_jdn({});", r.jdn)?;
        writeln!(&mut fp, "        assert_eq!(date.calendar(), cal);")?;
        writeln!(&mut fp, "        assert!(!date.is_julian());")?;
        writeln!(&mut fp, "        assert!(date.is_gregorian());")?;
        r.gregorian_date().assert(&mut fp)?;
        writeln!(&mut fp, "    }}")?;
    }
    writeln!(&mut fp, "}}")?;
    writeln!(&mut fp)?;

    writeln!(&mut fp, "mod gregorian_to_jdn {{")?;
    writeln!(&mut fp, "    use crate::{{Calendar, Month}};")?;
    writeln!(&mut fp)?;
    let mut first = true;
    for r in &jdn_records {
        if !std::mem::replace(&mut first, false) {
            writeln!(&mut fp)?;
        }
        writeln!(&mut fp, "    #[test]")?;
        writeln!(&mut fp, "    fn jdn_{}_ymd() {{", r.jdn_id())?;
        writeln!(&mut fp, "        let cal = Calendar::GREGORIAN;")?;
        writeln!(
            &mut fp,
            "        let date = cal.at_ymd({}, Month::{}, {}).unwrap();",
            r.gyear, r.gmonth, r.gday
        )?;
        writeln!(&mut fp, "        assert_eq!(date.calendar(), cal);")?;
        writeln!(&mut fp, "        assert!(!date.is_julian());")?;
        writeln!(&mut fp, "        assert!(date.is_gregorian());")?;
        r.gregorian_date().assert(&mut fp)?;
        writeln!(&mut fp, "    }}")?;
        writeln!(&mut fp)?;
        writeln!(&mut fp, "    #[test]")?;
        writeln!(&mut fp, "    fn jdn_{}_ordinal() {{", r.jdn_id())?;
        writeln!(&mut fp, "        let cal = Calendar::GREGORIAN;")?;
        writeln!(
            &mut fp,
            "        let date = cal.at_ordinal_date({}, {}).unwrap();",
            r.gyear, r.gordinal
        )?;
        writeln!(&mut fp, "        assert_eq!(date.calendar(), cal);")?;
        writeln!(&mut fp, "        assert!(!date.is_julian());")?;
        writeln!(&mut fp, "        assert!(date.is_gregorian());")?;
        r.gregorian_date().assert(&mut fp)?;
        writeln!(&mut fp, "    }}")?;
    }
    writeln!(&mut fp, "}}")?;

    fp.flush()?;
    Ok(())
}

fn make_reforming_tests(inpath: PathBuf, outpath: PathBuf) -> anyhow::Result<()> {
    eprintln!("Loading {} ...", inpath.display());
    let reforming_records = csv::ReaderBuilder::new()
        .from_path(&inpath)
        .with_context(|| format!("Failed to read {}", inpath.display()))?
        .deserialize()
        .collect::<Result<Vec<ReformingRecord>, _>>()
        .with_context(|| format!("Failed to deserialize {}", inpath.display()))?;
    eprintln!("Generating {} ...", outpath.display());
    let mut fp = BufWriter::new(File::create(outpath)?);

    writeln!(&mut fp, "mod jdn_to_reforming {{")?;
    writeln!(&mut fp, "    use crate::{{Calendar, Month}};")?;
    writeln!(&mut fp)?;
    let mut first = true;
    for r in &reforming_records {
        if !std::mem::replace(&mut first, false) {
            writeln!(&mut fp)?;
        }
        writeln!(&mut fp, "    #[test]")?;
        writeln!(
            &mut fp,
            "    fn reform_{}_jdn_{}() {{",
            r.reformation,
            r.jdn_id()
        )?;
        writeln!(
            &mut fp,
            "        let cal = Calendar::reforming({}).unwrap();",
            r.reformation
        )?;
        writeln!(&mut fp, "        let date = cal.at_jdn({});", r.jdn)?;
        writeln!(&mut fp, "        assert_eq!(date.calendar(), cal);")?;
        r.assert(&mut fp)?;
        writeln!(&mut fp, "    }}")?;
    }
    writeln!(&mut fp, "}}")?;
    writeln!(&mut fp)?;

    writeln!(&mut fp, "mod reforming_to_jdn {{")?;
    writeln!(&mut fp, "    use crate::{{Calendar, Month}};")?;
    writeln!(&mut fp)?;
    let mut first = true;
    for r in &reforming_records {
        if !std::mem::replace(&mut first, false) {
            writeln!(&mut fp)?;
        }
        writeln!(&mut fp, "    #[test]")?;
        writeln!(
            &mut fp,
            "    fn reform_{}_jdn_{}_ymd() {{",
            r.reformation,
            r.jdn_id()
        )?;
        writeln!(
            &mut fp,
            "        let cal = Calendar::reforming({}).unwrap();",
            r.reformation
        )?;
        writeln!(
            &mut fp,
            "        let date = cal.at_ymd({}, Month::{}, {}).unwrap();",
            r.year, r.month, r.day
        )?;
        writeln!(&mut fp, "        assert_eq!(date.calendar(), cal);")?;
        r.assert(&mut fp)?;
        writeln!(&mut fp, "    }}")?;
        writeln!(&mut fp)?;
        writeln!(&mut fp, "    #[test]")?;
        writeln!(
            &mut fp,
            "    fn reform_{}_jdn_{}_ordinal() {{",
            r.reformation,
            r.jdn_id()
        )?;
        writeln!(
            &mut fp,
            "        let cal = Calendar::reforming({}).unwrap();",
            r.reformation
        )?;
        writeln!(
            &mut fp,
            "        let date = cal.at_ordinal_date({}, {}).unwrap();",
            r.year, r.ordinal,
        )?;
        writeln!(&mut fp, "        assert_eq!(date.calendar(), cal);")?;
        r.assert(&mut fp)?;
        writeln!(&mut fp, "    }}")?;
    }
    writeln!(&mut fp, "}}")?;

    fp.flush()?;
    Ok(())
}
