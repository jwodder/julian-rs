use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct Record {
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

impl Record {
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

impl<'a> Date<'a> {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=data/jdn.csv");
    let records = csv::Reader::from_path("data/jdn.csv")?
        .deserialize()
        .collect::<Result<Vec<Record>, _>>()?;
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR envvar not set");
    let mut fp = BufWriter::new(File::create(Path::new(&out_dir).join("autogen.rs"))?);

    writeln!(&mut fp, "mod jdn_to_julian {{")?;
    writeln!(&mut fp, "    use crate::{{Calendar, Month}};")?;
    writeln!(&mut fp)?;
    let mut first = true;
    for r in &records {
        if !std::mem::replace(&mut first, false) {
            writeln!(&mut fp)?;
        }
        writeln!(&mut fp, "    #[test]")?;
        writeln!(&mut fp, "    fn jdn_{}() {{", r.jdn_id())?;
        writeln!(&mut fp, "        let cal = Calendar::julian();")?;
        writeln!(&mut fp, "        let date = cal.at_jdn({});", r.jdn)?;
        writeln!(&mut fp, "        assert_eq!(date.calendar(), &cal);")?;
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
    for r in &records {
        if !std::mem::replace(&mut first, false) {
            writeln!(&mut fp)?;
        }
        writeln!(&mut fp, "    #[test]")?;
        writeln!(&mut fp, "    fn jdn_{}_ymd() {{", r.jdn_id())?;
        writeln!(&mut fp, "        let cal = Calendar::julian();")?;
        writeln!(
            &mut fp,
            "        let date = cal.at_ymd({}, Month::{}, {}).unwrap();",
            r.jyear, r.jmonth, r.jday
        )?;
        writeln!(&mut fp, "        assert_eq!(date.calendar(), &cal);")?;
        writeln!(&mut fp, "        assert!(date.is_julian());")?;
        writeln!(&mut fp, "        assert!(!date.is_gregorian());")?;
        r.julian_date().assert(&mut fp)?;
        writeln!(&mut fp, "    }}")?;
        writeln!(&mut fp)?;
        writeln!(&mut fp, "    #[test]")?;
        writeln!(&mut fp, "    fn jdn_{}_ordinal() {{", r.jdn_id())?;
        writeln!(&mut fp, "        let cal = Calendar::julian();")?;
        writeln!(
            &mut fp,
            "        let date = cal.at_ordinal_date({}, {}).unwrap();",
            r.jyear, r.jordinal
        )?;
        writeln!(&mut fp, "        assert_eq!(date.calendar(), &cal);")?;
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
    for r in &records {
        if !std::mem::replace(&mut first, false) {
            writeln!(&mut fp)?;
        }
        writeln!(&mut fp, "    #[test]")?;
        writeln!(&mut fp, "    fn jdn_{}() {{", r.jdn_id())?;
        writeln!(&mut fp, "        let cal = Calendar::gregorian();")?;
        writeln!(&mut fp, "        let date = cal.at_jdn({});", r.jdn)?;
        writeln!(&mut fp, "        assert_eq!(date.calendar(), &cal);")?;
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
    for r in &records {
        if !std::mem::replace(&mut first, false) {
            writeln!(&mut fp)?;
        }
        writeln!(&mut fp, "    #[test]")?;
        writeln!(&mut fp, "    fn jdn_{}_ymd() {{", r.jdn_id())?;
        writeln!(&mut fp, "        let cal = Calendar::gregorian();")?;
        writeln!(
            &mut fp,
            "        let date = cal.at_ymd({}, Month::{}, {}).unwrap();",
            r.gyear, r.gmonth, r.gday
        )?;
        writeln!(&mut fp, "        assert_eq!(date.calendar(), &cal);")?;
        writeln!(&mut fp, "        assert!(!date.is_julian());")?;
        writeln!(&mut fp, "        assert!(date.is_gregorian());")?;
        r.gregorian_date().assert(&mut fp)?;
        writeln!(&mut fp, "    }}")?;
        writeln!(&mut fp)?;
        writeln!(&mut fp, "    #[test]")?;
        writeln!(&mut fp, "    fn jdn_{}_ordinal() {{", r.jdn_id())?;
        writeln!(&mut fp, "        let cal = Calendar::gregorian();")?;
        writeln!(
            &mut fp,
            "        let date = cal.at_ordinal_date({}, {}).unwrap();",
            r.gyear, r.gordinal
        )?;
        writeln!(&mut fp, "        assert_eq!(date.calendar(), &cal);")?;
        writeln!(&mut fp, "        assert!(!date.is_julian());")?;
        writeln!(&mut fp, "        assert!(date.is_gregorian());")?;
        r.gregorian_date().assert(&mut fp)?;
        writeln!(&mut fp, "    }}")?;
    }
    writeln!(&mut fp, "}}")?;
    fp.flush()?;
    Ok(())
}
