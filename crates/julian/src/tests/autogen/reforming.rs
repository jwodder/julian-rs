mod jdn_to_reforming {
    use crate::{Calendar, Month};

    #[test]
    fn reform_2299161_jdn_2299159() {
        let cal = Calendar::reforming(2299161).unwrap();
        let date = cal.at_jdn(2299159);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 3);
        assert_eq!(date.day_ordinal(), 3);
        assert_eq!(date.day_ordinal0(), 2);
        assert_eq!(date.ordinal(), 276);
        assert_eq!(date.ordinal0(), 275);
        assert_eq!(date.julian_day_number(), 2299159);
    }

    #[test]
    fn reform_2299161_jdn_2299160() {
        let cal = Calendar::reforming(2299161).unwrap();
        let date = cal.at_jdn(2299160);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 4);
        assert_eq!(date.day_ordinal(), 4);
        assert_eq!(date.day_ordinal0(), 3);
        assert_eq!(date.ordinal(), 277);
        assert_eq!(date.ordinal0(), 276);
        assert_eq!(date.julian_day_number(), 2299160);
    }

    #[test]
    fn reform_2299161_jdn_2299161() {
        let cal = Calendar::reforming(2299161).unwrap();
        let date = cal.at_jdn(2299161);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 15);
        assert_eq!(date.day_ordinal(), 5);
        assert_eq!(date.day_ordinal0(), 4);
        assert_eq!(date.ordinal(), 278);
        assert_eq!(date.ordinal0(), 277);
        assert_eq!(date.julian_day_number(), 2299161);
    }

    #[test]
    fn reform_2299161_jdn_2299162() {
        let cal = Calendar::reforming(2299161).unwrap();
        let date = cal.at_jdn(2299162);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 16);
        assert_eq!(date.day_ordinal(), 6);
        assert_eq!(date.day_ordinal0(), 5);
        assert_eq!(date.ordinal(), 279);
        assert_eq!(date.ordinal0(), 278);
        assert_eq!(date.julian_day_number(), 2299162);
    }

    #[test]
    fn reform_2299161_jdn_2299178() {
        let cal = Calendar::reforming(2299161).unwrap();
        let date = cal.at_jdn(2299178);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 295);
        assert_eq!(date.ordinal0(), 294);
        assert_eq!(date.julian_day_number(), 2299178);
    }

    #[test]
    fn reform_2299620_jdn_2299618() {
        let cal = Calendar::reforming(2299620).unwrap();
        let date = cal.at_jdn(2299618);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1584);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 5);
        assert_eq!(date.day_ordinal(), 5);
        assert_eq!(date.day_ordinal0(), 4);
        assert_eq!(date.ordinal(), 5);
        assert_eq!(date.ordinal0(), 4);
        assert_eq!(date.julian_day_number(), 2299618);
    }

    #[test]
    fn reform_2299620_jdn_2299619() {
        let cal = Calendar::reforming(2299620).unwrap();
        let date = cal.at_jdn(2299619);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1584);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 6);
        assert_eq!(date.day_ordinal(), 6);
        assert_eq!(date.day_ordinal0(), 5);
        assert_eq!(date.ordinal(), 6);
        assert_eq!(date.ordinal0(), 5);
        assert_eq!(date.julian_day_number(), 2299619);
    }

    #[test]
    fn reform_2299620_jdn_2299620() {
        let cal = Calendar::reforming(2299620).unwrap();
        let date = cal.at_jdn(2299620);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1584);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 17);
        assert_eq!(date.day_ordinal(), 7);
        assert_eq!(date.day_ordinal0(), 6);
        assert_eq!(date.ordinal(), 7);
        assert_eq!(date.ordinal0(), 6);
        assert_eq!(date.julian_day_number(), 2299620);
    }

    #[test]
    fn reform_2299620_jdn_2299621() {
        let cal = Calendar::reforming(2299620).unwrap();
        let date = cal.at_jdn(2299621);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1584);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 18);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 8);
        assert_eq!(date.ordinal0(), 7);
        assert_eq!(date.julian_day_number(), 2299621);
    }

    #[test]
    fn reform_2299620_jdn_2299663() {
        let cal = Calendar::reforming(2299620).unwrap();
        let date = cal.at_jdn(2299663);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1584);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 29);
        assert_eq!(date.day_ordinal0(), 28);
        assert_eq!(date.ordinal(), 50);
        assert_eq!(date.ordinal0(), 49);
        assert_eq!(date.julian_day_number(), 2299663);
    }

    #[test]
    fn reform_2299620_jdn_2299664() {
        let cal = Calendar::reforming(2299620).unwrap();
        let date = cal.at_jdn(2299664);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1584);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 51);
        assert_eq!(date.ordinal0(), 50);
        assert_eq!(date.julian_day_number(), 2299664);
    }

    #[test]
    fn reform_2310076_jdn_2309900() {
        let cal = Calendar::reforming(2310076).unwrap();
        let date = cal.at_jdn(2309900);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1612);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 29);
        assert_eq!(date.day_ordinal0(), 28);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 2309900);
    }

    #[test]
    fn reform_2310076_jdn_2310074() {
        let cal = Calendar::reforming(2310076).unwrap();
        let date = cal.at_jdn(2310074);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1612);
        assert_eq!(date.month(), Month::August);
        assert_eq!(date.day(), 21);
        assert_eq!(date.day_ordinal(), 21);
        assert_eq!(date.day_ordinal0(), 20);
        assert_eq!(date.ordinal(), 234);
        assert_eq!(date.ordinal0(), 233);
        assert_eq!(date.julian_day_number(), 2310074);
    }

    #[test]
    fn reform_2310076_jdn_2310075() {
        let cal = Calendar::reforming(2310076).unwrap();
        let date = cal.at_jdn(2310075);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1612);
        assert_eq!(date.month(), Month::August);
        assert_eq!(date.day(), 22);
        assert_eq!(date.day_ordinal(), 22);
        assert_eq!(date.day_ordinal0(), 21);
        assert_eq!(date.ordinal(), 235);
        assert_eq!(date.ordinal0(), 234);
        assert_eq!(date.julian_day_number(), 2310075);
    }

    #[test]
    fn reform_2310076_jdn_2310076() {
        let cal = Calendar::reforming(2310076).unwrap();
        let date = cal.at_jdn(2310076);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1612);
        assert_eq!(date.month(), Month::September);
        assert_eq!(date.day(), 2);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 236);
        assert_eq!(date.ordinal0(), 235);
        assert_eq!(date.julian_day_number(), 2310076);
    }

    #[test]
    fn reform_2310076_jdn_2310077() {
        let cal = Calendar::reforming(2310076).unwrap();
        let date = cal.at_jdn(2310077);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1612);
        assert_eq!(date.month(), Month::September);
        assert_eq!(date.day(), 3);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 237);
        assert_eq!(date.ordinal0(), 236);
        assert_eq!(date.julian_day_number(), 2310077);
    }

    #[test]
    fn reform_2310076_jdn_2310105() {
        let cal = Calendar::reforming(2310076).unwrap();
        let date = cal.at_jdn(2310105);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1612);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 265);
        assert_eq!(date.ordinal0(), 264);
        assert_eq!(date.julian_day_number(), 2310105);
    }

    #[test]
    fn reform_2342018_jdn_2342016() {
        let cal = Calendar::reforming(2342018).unwrap();
        let date = cal.at_jdn(2342016);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 3);
        assert_eq!(date.day_ordinal(), 3);
        assert_eq!(date.day_ordinal0(), 2);
        assert_eq!(date.ordinal(), 34);
        assert_eq!(date.ordinal0(), 33);
        assert_eq!(date.julian_day_number(), 2342016);
    }

    #[test]
    fn reform_2342018_jdn_2342017() {
        let cal = Calendar::reforming(2342018).unwrap();
        let date = cal.at_jdn(2342017);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 4);
        assert_eq!(date.day_ordinal(), 4);
        assert_eq!(date.day_ordinal0(), 3);
        assert_eq!(date.ordinal(), 35);
        assert_eq!(date.ordinal0(), 34);
        assert_eq!(date.julian_day_number(), 2342017);
    }

    #[test]
    fn reform_2342018_jdn_2342018() {
        let cal = Calendar::reforming(2342018).unwrap();
        let date = cal.at_jdn(2342018);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 15);
        assert_eq!(date.day_ordinal(), 5);
        assert_eq!(date.day_ordinal0(), 4);
        assert_eq!(date.ordinal(), 36);
        assert_eq!(date.ordinal0(), 35);
        assert_eq!(date.julian_day_number(), 2342018);
    }

    #[test]
    fn reform_2342018_jdn_2342019() {
        let cal = Calendar::reforming(2342018).unwrap();
        let date = cal.at_jdn(2342019);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 16);
        assert_eq!(date.day_ordinal(), 6);
        assert_eq!(date.day_ordinal0(), 5);
        assert_eq!(date.ordinal(), 37);
        assert_eq!(date.ordinal0(), 36);
        assert_eq!(date.julian_day_number(), 2342019);
    }

    #[test]
    fn reform_2342018_jdn_2342032() {
        let cal = Calendar::reforming(2342018).unwrap();
        let date = cal.at_jdn(2342032);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 50);
        assert_eq!(date.ordinal0(), 49);
        assert_eq!(date.julian_day_number(), 2342032);
    }

    #[test]
    fn reform_2342032_jdn_2342030() {
        let cal = Calendar::reforming(2342032).unwrap();
        let date = cal.at_jdn(2342030);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 17);
        assert_eq!(date.day_ordinal(), 17);
        assert_eq!(date.day_ordinal0(), 16);
        assert_eq!(date.ordinal(), 48);
        assert_eq!(date.ordinal0(), 47);
        assert_eq!(date.julian_day_number(), 2342030);
    }

    #[test]
    fn reform_2342032_jdn_2342031() {
        let cal = Calendar::reforming(2342032).unwrap();
        let date = cal.at_jdn(2342031);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 18);
        assert_eq!(date.day_ordinal(), 18);
        assert_eq!(date.day_ordinal0(), 17);
        assert_eq!(date.ordinal(), 49);
        assert_eq!(date.ordinal0(), 48);
        assert_eq!(date.julian_day_number(), 2342031);
    }

    #[test]
    fn reform_2342032_jdn_2342032() {
        let cal = Calendar::reforming(2342032).unwrap();
        let date = cal.at_jdn(2342032);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 50);
        assert_eq!(date.ordinal0(), 49);
        assert_eq!(date.julian_day_number(), 2342032);
    }

    #[test]
    fn reform_2342032_jdn_2342033() {
        let cal = Calendar::reforming(2342032).unwrap();
        let date = cal.at_jdn(2342033);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 2);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 51);
        assert_eq!(date.ordinal0(), 50);
        assert_eq!(date.julian_day_number(), 2342033);
    }

    #[test]
    fn reform_2342304_jdn_2342042() {
        let cal = Calendar::reforming(2342304).unwrap();
        let date = cal.at_jdn(2342042);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 29);
        assert_eq!(date.day_ordinal0(), 28);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 2342042);
    }

    #[test]
    fn reform_2342304_jdn_2342302() {
        let cal = Calendar::reforming(2342304).unwrap();
        let date = cal.at_jdn(2342302);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 15);
        assert_eq!(date.day_ordinal(), 15);
        assert_eq!(date.day_ordinal0(), 14);
        assert_eq!(date.ordinal(), 320);
        assert_eq!(date.ordinal0(), 319);
        assert_eq!(date.julian_day_number(), 2342302);
    }

    #[test]
    fn reform_2342304_jdn_2342303() {
        let cal = Calendar::reforming(2342304).unwrap();
        let date = cal.at_jdn(2342303);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 16);
        assert_eq!(date.day_ordinal(), 16);
        assert_eq!(date.day_ordinal0(), 15);
        assert_eq!(date.ordinal(), 321);
        assert_eq!(date.ordinal0(), 320);
        assert_eq!(date.julian_day_number(), 2342303);
    }

    #[test]
    fn reform_2342304_jdn_2342304() {
        let cal = Calendar::reforming(2342304).unwrap();
        let date = cal.at_jdn(2342304);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 28);
        assert_eq!(date.day_ordinal(), 17);
        assert_eq!(date.day_ordinal0(), 16);
        assert_eq!(date.ordinal(), 322);
        assert_eq!(date.ordinal0(), 321);
        assert_eq!(date.julian_day_number(), 2342304);
    }

    #[test]
    fn reform_2342304_jdn_2342305() {
        let cal = Calendar::reforming(2342304).unwrap();
        let date = cal.at_jdn(2342305);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 18);
        assert_eq!(date.day_ordinal0(), 17);
        assert_eq!(date.ordinal(), 323);
        assert_eq!(date.ordinal0(), 322);
        assert_eq!(date.julian_day_number(), 2342305);
    }

    #[test]
    fn reform_2344534_jdn_2344532() {
        let cal = Calendar::reforming(2344534).unwrap();
        let date = cal.at_jdn(2344532);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1706);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 24);
        assert_eq!(date.day_ordinal(), 24);
        assert_eq!(date.day_ordinal0(), 23);
        assert_eq!(date.ordinal(), 358);
        assert_eq!(date.ordinal0(), 357);
        assert_eq!(date.julian_day_number(), 2344532);
    }

    #[test]
    fn reform_2344534_jdn_2344533() {
        let cal = Calendar::reforming(2344534).unwrap();
        let date = cal.at_jdn(2344533);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1706);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 25);
        assert_eq!(date.day_ordinal(), 25);
        assert_eq!(date.day_ordinal0(), 24);
        assert_eq!(date.ordinal(), 359);
        assert_eq!(date.ordinal0(), 358);
        assert_eq!(date.julian_day_number(), 2344533);
    }

    #[test]
    fn reform_2344534_jdn_2344534() {
        let cal = Calendar::reforming(2344534).unwrap();
        let date = cal.at_jdn(2344534);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1707);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 6);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2344534);
    }

    #[test]
    fn reform_2344534_jdn_2344535() {
        let cal = Calendar::reforming(2344534).unwrap();
        let date = cal.at_jdn(2344535);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1707);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 2);
        assert_eq!(date.ordinal0(), 1);
        assert_eq!(date.julian_day_number(), 2344535);
    }

    #[test]
    fn reform_2421639_jdn_2421637() {
        let cal = Calendar::reforming(2421639).unwrap();
        let date = cal.at_jdn(2421637);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1918);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 30);
        assert_eq!(date.day_ordinal(), 30);
        assert_eq!(date.day_ordinal0(), 29);
        assert_eq!(date.ordinal(), 30);
        assert_eq!(date.ordinal0(), 29);
        assert_eq!(date.julian_day_number(), 2421637);
    }

    #[test]
    fn reform_2421639_jdn_2421638() {
        let cal = Calendar::reforming(2421639).unwrap();
        let date = cal.at_jdn(2421638);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1918);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 31);
        assert_eq!(date.ordinal0(), 30);
        assert_eq!(date.julian_day_number(), 2421638);
    }

    #[test]
    fn reform_2421639_jdn_2421639() {
        let cal = Calendar::reforming(2421639).unwrap();
        let date = cal.at_jdn(2421639);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1918);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 14);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 32);
        assert_eq!(date.ordinal0(), 31);
        assert_eq!(date.julian_day_number(), 2421639);
    }

    #[test]
    fn reform_2421639_jdn_2421640() {
        let cal = Calendar::reforming(2421639).unwrap();
        let date = cal.at_jdn(2421640);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1918);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 15);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 33);
        assert_eq!(date.ordinal0(), 32);
        assert_eq!(date.julian_day_number(), 2421640);
    }

    #[test]
    fn reform_2421639_jdn_2421654() {
        let cal = Calendar::reforming(2421639).unwrap();
        let date = cal.at_jdn(2421654);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1918);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 47);
        assert_eq!(date.ordinal0(), 46);
        assert_eq!(date.julian_day_number(), 2421654);
    }

    #[test]
    fn reform_2460316_jdn_2460314() {
        let cal = Calendar::reforming(2460316).unwrap();
        let date = cal.at_jdn(2460314);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 2023);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 22);
        assert_eq!(date.day_ordinal(), 22);
        assert_eq!(date.day_ordinal0(), 21);
        assert_eq!(date.ordinal(), 356);
        assert_eq!(date.ordinal0(), 355);
        assert_eq!(date.julian_day_number(), 2460314);
    }

    #[test]
    fn reform_2460316_jdn_2460315() {
        let cal = Calendar::reforming(2460316).unwrap();
        let date = cal.at_jdn(2460315);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 2023);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 23);
        assert_eq!(date.day_ordinal(), 23);
        assert_eq!(date.day_ordinal0(), 22);
        assert_eq!(date.ordinal(), 357);
        assert_eq!(date.ordinal0(), 356);
        assert_eq!(date.julian_day_number(), 2460315);
    }

    #[test]
    fn reform_2460316_jdn_2460316() {
        let cal = Calendar::reforming(2460316).unwrap();
        let date = cal.at_jdn(2460316);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 6);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2460316);
    }

    #[test]
    fn reform_2460316_jdn_2460317() {
        let cal = Calendar::reforming(2460316).unwrap();
        let date = cal.at_jdn(2460317);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 2);
        assert_eq!(date.ordinal0(), 1);
        assert_eq!(date.julian_day_number(), 2460317);
    }

    #[test]
    fn reform_2460316_jdn_2460370() {
        let cal = Calendar::reforming(2460316).unwrap();
        let date = cal.at_jdn(2460370);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 29);
        assert_eq!(date.day_ordinal0(), 28);
        assert_eq!(date.ordinal(), 55);
        assert_eq!(date.ordinal0(), 54);
        assert_eq!(date.julian_day_number(), 2460370);
    }

    #[test]
    fn reform_2460316_jdn_2460371() {
        let cal = Calendar::reforming(2460316).unwrap();
        let date = cal.at_jdn(2460371);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 56);
        assert_eq!(date.ordinal0(), 55);
        assert_eq!(date.julian_day_number(), 2460371);
    }

    #[test]
    fn reform_2460682_jdn_2460383() {
        let cal = Calendar::reforming(2460682).unwrap();
        let date = cal.at_jdn(2460383);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 29);
        assert_eq!(date.day_ordinal0(), 28);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 2460383);
    }

    #[test]
    fn reform_2460682_jdn_2460680() {
        let cal = Calendar::reforming(2460682).unwrap();
        let date = cal.at_jdn(2460680);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 22);
        assert_eq!(date.day_ordinal(), 22);
        assert_eq!(date.day_ordinal0(), 21);
        assert_eq!(date.ordinal(), 357);
        assert_eq!(date.ordinal0(), 356);
        assert_eq!(date.julian_day_number(), 2460680);
    }

    #[test]
    fn reform_2460682_jdn_2460681() {
        let cal = Calendar::reforming(2460682).unwrap();
        let date = cal.at_jdn(2460681);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 23);
        assert_eq!(date.day_ordinal(), 23);
        assert_eq!(date.day_ordinal0(), 22);
        assert_eq!(date.ordinal(), 358);
        assert_eq!(date.ordinal0(), 357);
        assert_eq!(date.julian_day_number(), 2460681);
    }

    #[test]
    fn reform_2460682_jdn_2460682() {
        let cal = Calendar::reforming(2460682).unwrap();
        let date = cal.at_jdn(2460682);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 2025);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 6);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2460682);
    }

    #[test]
    fn reform_2460682_jdn_2460683() {
        let cal = Calendar::reforming(2460682).unwrap();
        let date = cal.at_jdn(2460683);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 2025);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 2);
        assert_eq!(date.ordinal0(), 1);
        assert_eq!(date.julian_day_number(), 2460683);
    }

    #[test]
    fn reform_3145930_jdn_3145928() {
        let cal = Calendar::reforming(3145930).unwrap();
        let date = cal.at_jdn(3145928);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 3901);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 30);
        assert_eq!(date.day_ordinal(), 30);
        assert_eq!(date.day_ordinal0(), 29);
        assert_eq!(date.ordinal(), 30);
        assert_eq!(date.ordinal0(), 29);
        assert_eq!(date.julian_day_number(), 3145928);
    }

    #[test]
    fn reform_3145930_jdn_3145929() {
        let cal = Calendar::reforming(3145930).unwrap();
        let date = cal.at_jdn(3145929);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 3901);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 31);
        assert_eq!(date.ordinal0(), 30);
        assert_eq!(date.julian_day_number(), 3145929);
    }

    #[test]
    fn reform_3145930_jdn_3145930() {
        let cal = Calendar::reforming(3145930).unwrap();
        let date = cal.at_jdn(3145930);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 3901);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 32);
        assert_eq!(date.ordinal0(), 31);
        assert_eq!(date.julian_day_number(), 3145930);
    }

    #[test]
    fn reform_3145930_jdn_3145931() {
        let cal = Calendar::reforming(3145930).unwrap();
        let date = cal.at_jdn(3145931);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 3901);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 2);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 33);
        assert_eq!(date.ordinal0(), 32);
        assert_eq!(date.julian_day_number(), 3145931);
    }

    #[test]
    fn reform_19582149_jdn_19582147() {
        let cal = Calendar::reforming(19582149).unwrap();
        let date = cal.at_jdn(19582147);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 48900);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 30);
        assert_eq!(date.day_ordinal(), 30);
        assert_eq!(date.day_ordinal0(), 29);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 19582147);
    }

    #[test]
    fn reform_19582149_jdn_19582148() {
        let cal = Calendar::reforming(19582149).unwrap();
        let date = cal.at_jdn(19582148);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 48900);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 366);
        assert_eq!(date.ordinal0(), 365);
        assert_eq!(date.julian_day_number(), 19582148);
    }

    #[test]
    fn reform_19582149_jdn_19582149() {
        let cal = Calendar::reforming(19582149).unwrap();
        let date = cal.at_jdn(19582149);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 48902);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 19582149);
    }

    #[test]
    fn reform_19582149_jdn_19582150() {
        let cal = Calendar::reforming(19582149).unwrap();
        let date = cal.at_jdn(19582150);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 48902);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 2);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 2);
        assert_eq!(date.ordinal0(), 1);
        assert_eq!(date.julian_day_number(), 19582150);
    }
}

mod reforming_to_jdn {
    use crate::{Calendar, Month};

    #[test]
    fn reform_2299161_jdn_2299159_ymd() {
        let cal = Calendar::reforming(2299161).unwrap();
        let date = cal.at_ymd(1582, Month::October, 3).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 3);
        assert_eq!(date.day_ordinal(), 3);
        assert_eq!(date.day_ordinal0(), 2);
        assert_eq!(date.ordinal(), 276);
        assert_eq!(date.ordinal0(), 275);
        assert_eq!(date.julian_day_number(), 2299159);
    }

    #[test]
    fn reform_2299161_jdn_2299159_ordinal() {
        let cal = Calendar::reforming(2299161).unwrap();
        let date = cal.at_ordinal_date(1582, 276).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 3);
        assert_eq!(date.day_ordinal(), 3);
        assert_eq!(date.day_ordinal0(), 2);
        assert_eq!(date.ordinal(), 276);
        assert_eq!(date.ordinal0(), 275);
        assert_eq!(date.julian_day_number(), 2299159);
    }

    #[test]
    fn reform_2299161_jdn_2299160_ymd() {
        let cal = Calendar::reforming(2299161).unwrap();
        let date = cal.at_ymd(1582, Month::October, 4).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 4);
        assert_eq!(date.day_ordinal(), 4);
        assert_eq!(date.day_ordinal0(), 3);
        assert_eq!(date.ordinal(), 277);
        assert_eq!(date.ordinal0(), 276);
        assert_eq!(date.julian_day_number(), 2299160);
    }

    #[test]
    fn reform_2299161_jdn_2299160_ordinal() {
        let cal = Calendar::reforming(2299161).unwrap();
        let date = cal.at_ordinal_date(1582, 277).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 4);
        assert_eq!(date.day_ordinal(), 4);
        assert_eq!(date.day_ordinal0(), 3);
        assert_eq!(date.ordinal(), 277);
        assert_eq!(date.ordinal0(), 276);
        assert_eq!(date.julian_day_number(), 2299160);
    }

    #[test]
    fn reform_2299161_jdn_2299161_ymd() {
        let cal = Calendar::reforming(2299161).unwrap();
        let date = cal.at_ymd(1582, Month::October, 15).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 15);
        assert_eq!(date.day_ordinal(), 5);
        assert_eq!(date.day_ordinal0(), 4);
        assert_eq!(date.ordinal(), 278);
        assert_eq!(date.ordinal0(), 277);
        assert_eq!(date.julian_day_number(), 2299161);
    }

    #[test]
    fn reform_2299161_jdn_2299161_ordinal() {
        let cal = Calendar::reforming(2299161).unwrap();
        let date = cal.at_ordinal_date(1582, 278).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 15);
        assert_eq!(date.day_ordinal(), 5);
        assert_eq!(date.day_ordinal0(), 4);
        assert_eq!(date.ordinal(), 278);
        assert_eq!(date.ordinal0(), 277);
        assert_eq!(date.julian_day_number(), 2299161);
    }

    #[test]
    fn reform_2299161_jdn_2299162_ymd() {
        let cal = Calendar::reforming(2299161).unwrap();
        let date = cal.at_ymd(1582, Month::October, 16).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 16);
        assert_eq!(date.day_ordinal(), 6);
        assert_eq!(date.day_ordinal0(), 5);
        assert_eq!(date.ordinal(), 279);
        assert_eq!(date.ordinal0(), 278);
        assert_eq!(date.julian_day_number(), 2299162);
    }

    #[test]
    fn reform_2299161_jdn_2299162_ordinal() {
        let cal = Calendar::reforming(2299161).unwrap();
        let date = cal.at_ordinal_date(1582, 279).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 16);
        assert_eq!(date.day_ordinal(), 6);
        assert_eq!(date.day_ordinal0(), 5);
        assert_eq!(date.ordinal(), 279);
        assert_eq!(date.ordinal0(), 278);
        assert_eq!(date.julian_day_number(), 2299162);
    }

    #[test]
    fn reform_2299161_jdn_2299178_ymd() {
        let cal = Calendar::reforming(2299161).unwrap();
        let date = cal.at_ymd(1582, Month::November, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 295);
        assert_eq!(date.ordinal0(), 294);
        assert_eq!(date.julian_day_number(), 2299178);
    }

    #[test]
    fn reform_2299161_jdn_2299178_ordinal() {
        let cal = Calendar::reforming(2299161).unwrap();
        let date = cal.at_ordinal_date(1582, 295).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 295);
        assert_eq!(date.ordinal0(), 294);
        assert_eq!(date.julian_day_number(), 2299178);
    }

    #[test]
    fn reform_2299620_jdn_2299618_ymd() {
        let cal = Calendar::reforming(2299620).unwrap();
        let date = cal.at_ymd(1584, Month::January, 5).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1584);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 5);
        assert_eq!(date.day_ordinal(), 5);
        assert_eq!(date.day_ordinal0(), 4);
        assert_eq!(date.ordinal(), 5);
        assert_eq!(date.ordinal0(), 4);
        assert_eq!(date.julian_day_number(), 2299618);
    }

    #[test]
    fn reform_2299620_jdn_2299618_ordinal() {
        let cal = Calendar::reforming(2299620).unwrap();
        let date = cal.at_ordinal_date(1584, 5).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1584);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 5);
        assert_eq!(date.day_ordinal(), 5);
        assert_eq!(date.day_ordinal0(), 4);
        assert_eq!(date.ordinal(), 5);
        assert_eq!(date.ordinal0(), 4);
        assert_eq!(date.julian_day_number(), 2299618);
    }

    #[test]
    fn reform_2299620_jdn_2299619_ymd() {
        let cal = Calendar::reforming(2299620).unwrap();
        let date = cal.at_ymd(1584, Month::January, 6).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1584);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 6);
        assert_eq!(date.day_ordinal(), 6);
        assert_eq!(date.day_ordinal0(), 5);
        assert_eq!(date.ordinal(), 6);
        assert_eq!(date.ordinal0(), 5);
        assert_eq!(date.julian_day_number(), 2299619);
    }

    #[test]
    fn reform_2299620_jdn_2299619_ordinal() {
        let cal = Calendar::reforming(2299620).unwrap();
        let date = cal.at_ordinal_date(1584, 6).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1584);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 6);
        assert_eq!(date.day_ordinal(), 6);
        assert_eq!(date.day_ordinal0(), 5);
        assert_eq!(date.ordinal(), 6);
        assert_eq!(date.ordinal0(), 5);
        assert_eq!(date.julian_day_number(), 2299619);
    }

    #[test]
    fn reform_2299620_jdn_2299620_ymd() {
        let cal = Calendar::reforming(2299620).unwrap();
        let date = cal.at_ymd(1584, Month::January, 17).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1584);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 17);
        assert_eq!(date.day_ordinal(), 7);
        assert_eq!(date.day_ordinal0(), 6);
        assert_eq!(date.ordinal(), 7);
        assert_eq!(date.ordinal0(), 6);
        assert_eq!(date.julian_day_number(), 2299620);
    }

    #[test]
    fn reform_2299620_jdn_2299620_ordinal() {
        let cal = Calendar::reforming(2299620).unwrap();
        let date = cal.at_ordinal_date(1584, 7).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1584);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 17);
        assert_eq!(date.day_ordinal(), 7);
        assert_eq!(date.day_ordinal0(), 6);
        assert_eq!(date.ordinal(), 7);
        assert_eq!(date.ordinal0(), 6);
        assert_eq!(date.julian_day_number(), 2299620);
    }

    #[test]
    fn reform_2299620_jdn_2299621_ymd() {
        let cal = Calendar::reforming(2299620).unwrap();
        let date = cal.at_ymd(1584, Month::January, 18).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1584);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 18);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 8);
        assert_eq!(date.ordinal0(), 7);
        assert_eq!(date.julian_day_number(), 2299621);
    }

    #[test]
    fn reform_2299620_jdn_2299621_ordinal() {
        let cal = Calendar::reforming(2299620).unwrap();
        let date = cal.at_ordinal_date(1584, 8).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1584);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 18);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 8);
        assert_eq!(date.ordinal0(), 7);
        assert_eq!(date.julian_day_number(), 2299621);
    }

    #[test]
    fn reform_2299620_jdn_2299663_ymd() {
        let cal = Calendar::reforming(2299620).unwrap();
        let date = cal.at_ymd(1584, Month::February, 29).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1584);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 29);
        assert_eq!(date.day_ordinal0(), 28);
        assert_eq!(date.ordinal(), 50);
        assert_eq!(date.ordinal0(), 49);
        assert_eq!(date.julian_day_number(), 2299663);
    }

    #[test]
    fn reform_2299620_jdn_2299663_ordinal() {
        let cal = Calendar::reforming(2299620).unwrap();
        let date = cal.at_ordinal_date(1584, 50).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1584);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 29);
        assert_eq!(date.day_ordinal0(), 28);
        assert_eq!(date.ordinal(), 50);
        assert_eq!(date.ordinal0(), 49);
        assert_eq!(date.julian_day_number(), 2299663);
    }

    #[test]
    fn reform_2299620_jdn_2299664_ymd() {
        let cal = Calendar::reforming(2299620).unwrap();
        let date = cal.at_ymd(1584, Month::March, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1584);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 51);
        assert_eq!(date.ordinal0(), 50);
        assert_eq!(date.julian_day_number(), 2299664);
    }

    #[test]
    fn reform_2299620_jdn_2299664_ordinal() {
        let cal = Calendar::reforming(2299620).unwrap();
        let date = cal.at_ordinal_date(1584, 51).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1584);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 51);
        assert_eq!(date.ordinal0(), 50);
        assert_eq!(date.julian_day_number(), 2299664);
    }

    #[test]
    fn reform_2310076_jdn_2309900_ymd() {
        let cal = Calendar::reforming(2310076).unwrap();
        let date = cal.at_ymd(1612, Month::February, 29).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1612);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 29);
        assert_eq!(date.day_ordinal0(), 28);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 2309900);
    }

    #[test]
    fn reform_2310076_jdn_2309900_ordinal() {
        let cal = Calendar::reforming(2310076).unwrap();
        let date = cal.at_ordinal_date(1612, 60).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1612);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 29);
        assert_eq!(date.day_ordinal0(), 28);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 2309900);
    }

    #[test]
    fn reform_2310076_jdn_2310074_ymd() {
        let cal = Calendar::reforming(2310076).unwrap();
        let date = cal.at_ymd(1612, Month::August, 21).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1612);
        assert_eq!(date.month(), Month::August);
        assert_eq!(date.day(), 21);
        assert_eq!(date.day_ordinal(), 21);
        assert_eq!(date.day_ordinal0(), 20);
        assert_eq!(date.ordinal(), 234);
        assert_eq!(date.ordinal0(), 233);
        assert_eq!(date.julian_day_number(), 2310074);
    }

    #[test]
    fn reform_2310076_jdn_2310074_ordinal() {
        let cal = Calendar::reforming(2310076).unwrap();
        let date = cal.at_ordinal_date(1612, 234).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1612);
        assert_eq!(date.month(), Month::August);
        assert_eq!(date.day(), 21);
        assert_eq!(date.day_ordinal(), 21);
        assert_eq!(date.day_ordinal0(), 20);
        assert_eq!(date.ordinal(), 234);
        assert_eq!(date.ordinal0(), 233);
        assert_eq!(date.julian_day_number(), 2310074);
    }

    #[test]
    fn reform_2310076_jdn_2310075_ymd() {
        let cal = Calendar::reforming(2310076).unwrap();
        let date = cal.at_ymd(1612, Month::August, 22).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1612);
        assert_eq!(date.month(), Month::August);
        assert_eq!(date.day(), 22);
        assert_eq!(date.day_ordinal(), 22);
        assert_eq!(date.day_ordinal0(), 21);
        assert_eq!(date.ordinal(), 235);
        assert_eq!(date.ordinal0(), 234);
        assert_eq!(date.julian_day_number(), 2310075);
    }

    #[test]
    fn reform_2310076_jdn_2310075_ordinal() {
        let cal = Calendar::reforming(2310076).unwrap();
        let date = cal.at_ordinal_date(1612, 235).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1612);
        assert_eq!(date.month(), Month::August);
        assert_eq!(date.day(), 22);
        assert_eq!(date.day_ordinal(), 22);
        assert_eq!(date.day_ordinal0(), 21);
        assert_eq!(date.ordinal(), 235);
        assert_eq!(date.ordinal0(), 234);
        assert_eq!(date.julian_day_number(), 2310075);
    }

    #[test]
    fn reform_2310076_jdn_2310076_ymd() {
        let cal = Calendar::reforming(2310076).unwrap();
        let date = cal.at_ymd(1612, Month::September, 2).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1612);
        assert_eq!(date.month(), Month::September);
        assert_eq!(date.day(), 2);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 236);
        assert_eq!(date.ordinal0(), 235);
        assert_eq!(date.julian_day_number(), 2310076);
    }

    #[test]
    fn reform_2310076_jdn_2310076_ordinal() {
        let cal = Calendar::reforming(2310076).unwrap();
        let date = cal.at_ordinal_date(1612, 236).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1612);
        assert_eq!(date.month(), Month::September);
        assert_eq!(date.day(), 2);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 236);
        assert_eq!(date.ordinal0(), 235);
        assert_eq!(date.julian_day_number(), 2310076);
    }

    #[test]
    fn reform_2310076_jdn_2310077_ymd() {
        let cal = Calendar::reforming(2310076).unwrap();
        let date = cal.at_ymd(1612, Month::September, 3).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1612);
        assert_eq!(date.month(), Month::September);
        assert_eq!(date.day(), 3);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 237);
        assert_eq!(date.ordinal0(), 236);
        assert_eq!(date.julian_day_number(), 2310077);
    }

    #[test]
    fn reform_2310076_jdn_2310077_ordinal() {
        let cal = Calendar::reforming(2310076).unwrap();
        let date = cal.at_ordinal_date(1612, 237).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1612);
        assert_eq!(date.month(), Month::September);
        assert_eq!(date.day(), 3);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 237);
        assert_eq!(date.ordinal0(), 236);
        assert_eq!(date.julian_day_number(), 2310077);
    }

    #[test]
    fn reform_2310076_jdn_2310105_ymd() {
        let cal = Calendar::reforming(2310076).unwrap();
        let date = cal.at_ymd(1612, Month::October, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1612);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 265);
        assert_eq!(date.ordinal0(), 264);
        assert_eq!(date.julian_day_number(), 2310105);
    }

    #[test]
    fn reform_2310076_jdn_2310105_ordinal() {
        let cal = Calendar::reforming(2310076).unwrap();
        let date = cal.at_ordinal_date(1612, 265).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1612);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 265);
        assert_eq!(date.ordinal0(), 264);
        assert_eq!(date.julian_day_number(), 2310105);
    }

    #[test]
    fn reform_2342018_jdn_2342016_ymd() {
        let cal = Calendar::reforming(2342018).unwrap();
        let date = cal.at_ymd(1700, Month::February, 3).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 3);
        assert_eq!(date.day_ordinal(), 3);
        assert_eq!(date.day_ordinal0(), 2);
        assert_eq!(date.ordinal(), 34);
        assert_eq!(date.ordinal0(), 33);
        assert_eq!(date.julian_day_number(), 2342016);
    }

    #[test]
    fn reform_2342018_jdn_2342016_ordinal() {
        let cal = Calendar::reforming(2342018).unwrap();
        let date = cal.at_ordinal_date(1700, 34).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 3);
        assert_eq!(date.day_ordinal(), 3);
        assert_eq!(date.day_ordinal0(), 2);
        assert_eq!(date.ordinal(), 34);
        assert_eq!(date.ordinal0(), 33);
        assert_eq!(date.julian_day_number(), 2342016);
    }

    #[test]
    fn reform_2342018_jdn_2342017_ymd() {
        let cal = Calendar::reforming(2342018).unwrap();
        let date = cal.at_ymd(1700, Month::February, 4).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 4);
        assert_eq!(date.day_ordinal(), 4);
        assert_eq!(date.day_ordinal0(), 3);
        assert_eq!(date.ordinal(), 35);
        assert_eq!(date.ordinal0(), 34);
        assert_eq!(date.julian_day_number(), 2342017);
    }

    #[test]
    fn reform_2342018_jdn_2342017_ordinal() {
        let cal = Calendar::reforming(2342018).unwrap();
        let date = cal.at_ordinal_date(1700, 35).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 4);
        assert_eq!(date.day_ordinal(), 4);
        assert_eq!(date.day_ordinal0(), 3);
        assert_eq!(date.ordinal(), 35);
        assert_eq!(date.ordinal0(), 34);
        assert_eq!(date.julian_day_number(), 2342017);
    }

    #[test]
    fn reform_2342018_jdn_2342018_ymd() {
        let cal = Calendar::reforming(2342018).unwrap();
        let date = cal.at_ymd(1700, Month::February, 15).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 15);
        assert_eq!(date.day_ordinal(), 5);
        assert_eq!(date.day_ordinal0(), 4);
        assert_eq!(date.ordinal(), 36);
        assert_eq!(date.ordinal0(), 35);
        assert_eq!(date.julian_day_number(), 2342018);
    }

    #[test]
    fn reform_2342018_jdn_2342018_ordinal() {
        let cal = Calendar::reforming(2342018).unwrap();
        let date = cal.at_ordinal_date(1700, 36).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 15);
        assert_eq!(date.day_ordinal(), 5);
        assert_eq!(date.day_ordinal0(), 4);
        assert_eq!(date.ordinal(), 36);
        assert_eq!(date.ordinal0(), 35);
        assert_eq!(date.julian_day_number(), 2342018);
    }

    #[test]
    fn reform_2342018_jdn_2342019_ymd() {
        let cal = Calendar::reforming(2342018).unwrap();
        let date = cal.at_ymd(1700, Month::February, 16).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 16);
        assert_eq!(date.day_ordinal(), 6);
        assert_eq!(date.day_ordinal0(), 5);
        assert_eq!(date.ordinal(), 37);
        assert_eq!(date.ordinal0(), 36);
        assert_eq!(date.julian_day_number(), 2342019);
    }

    #[test]
    fn reform_2342018_jdn_2342019_ordinal() {
        let cal = Calendar::reforming(2342018).unwrap();
        let date = cal.at_ordinal_date(1700, 37).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 16);
        assert_eq!(date.day_ordinal(), 6);
        assert_eq!(date.day_ordinal0(), 5);
        assert_eq!(date.ordinal(), 37);
        assert_eq!(date.ordinal0(), 36);
        assert_eq!(date.julian_day_number(), 2342019);
    }

    #[test]
    fn reform_2342018_jdn_2342032_ymd() {
        let cal = Calendar::reforming(2342018).unwrap();
        let date = cal.at_ymd(1700, Month::March, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 50);
        assert_eq!(date.ordinal0(), 49);
        assert_eq!(date.julian_day_number(), 2342032);
    }

    #[test]
    fn reform_2342018_jdn_2342032_ordinal() {
        let cal = Calendar::reforming(2342018).unwrap();
        let date = cal.at_ordinal_date(1700, 50).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 50);
        assert_eq!(date.ordinal0(), 49);
        assert_eq!(date.julian_day_number(), 2342032);
    }

    #[test]
    fn reform_2342032_jdn_2342030_ymd() {
        let cal = Calendar::reforming(2342032).unwrap();
        let date = cal.at_ymd(1700, Month::February, 17).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 17);
        assert_eq!(date.day_ordinal(), 17);
        assert_eq!(date.day_ordinal0(), 16);
        assert_eq!(date.ordinal(), 48);
        assert_eq!(date.ordinal0(), 47);
        assert_eq!(date.julian_day_number(), 2342030);
    }

    #[test]
    fn reform_2342032_jdn_2342030_ordinal() {
        let cal = Calendar::reforming(2342032).unwrap();
        let date = cal.at_ordinal_date(1700, 48).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 17);
        assert_eq!(date.day_ordinal(), 17);
        assert_eq!(date.day_ordinal0(), 16);
        assert_eq!(date.ordinal(), 48);
        assert_eq!(date.ordinal0(), 47);
        assert_eq!(date.julian_day_number(), 2342030);
    }

    #[test]
    fn reform_2342032_jdn_2342031_ymd() {
        let cal = Calendar::reforming(2342032).unwrap();
        let date = cal.at_ymd(1700, Month::February, 18).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 18);
        assert_eq!(date.day_ordinal(), 18);
        assert_eq!(date.day_ordinal0(), 17);
        assert_eq!(date.ordinal(), 49);
        assert_eq!(date.ordinal0(), 48);
        assert_eq!(date.julian_day_number(), 2342031);
    }

    #[test]
    fn reform_2342032_jdn_2342031_ordinal() {
        let cal = Calendar::reforming(2342032).unwrap();
        let date = cal.at_ordinal_date(1700, 49).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 18);
        assert_eq!(date.day_ordinal(), 18);
        assert_eq!(date.day_ordinal0(), 17);
        assert_eq!(date.ordinal(), 49);
        assert_eq!(date.ordinal0(), 48);
        assert_eq!(date.julian_day_number(), 2342031);
    }

    #[test]
    fn reform_2342032_jdn_2342032_ymd() {
        let cal = Calendar::reforming(2342032).unwrap();
        let date = cal.at_ymd(1700, Month::March, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 50);
        assert_eq!(date.ordinal0(), 49);
        assert_eq!(date.julian_day_number(), 2342032);
    }

    #[test]
    fn reform_2342032_jdn_2342032_ordinal() {
        let cal = Calendar::reforming(2342032).unwrap();
        let date = cal.at_ordinal_date(1700, 50).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 50);
        assert_eq!(date.ordinal0(), 49);
        assert_eq!(date.julian_day_number(), 2342032);
    }

    #[test]
    fn reform_2342032_jdn_2342033_ymd() {
        let cal = Calendar::reforming(2342032).unwrap();
        let date = cal.at_ymd(1700, Month::March, 2).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 2);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 51);
        assert_eq!(date.ordinal0(), 50);
        assert_eq!(date.julian_day_number(), 2342033);
    }

    #[test]
    fn reform_2342032_jdn_2342033_ordinal() {
        let cal = Calendar::reforming(2342032).unwrap();
        let date = cal.at_ordinal_date(1700, 51).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 2);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 51);
        assert_eq!(date.ordinal0(), 50);
        assert_eq!(date.julian_day_number(), 2342033);
    }

    #[test]
    fn reform_2342304_jdn_2342042_ymd() {
        let cal = Calendar::reforming(2342304).unwrap();
        let date = cal.at_ymd(1700, Month::February, 29).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 29);
        assert_eq!(date.day_ordinal0(), 28);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 2342042);
    }

    #[test]
    fn reform_2342304_jdn_2342042_ordinal() {
        let cal = Calendar::reforming(2342304).unwrap();
        let date = cal.at_ordinal_date(1700, 60).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 29);
        assert_eq!(date.day_ordinal0(), 28);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 2342042);
    }

    #[test]
    fn reform_2342304_jdn_2342302_ymd() {
        let cal = Calendar::reforming(2342304).unwrap();
        let date = cal.at_ymd(1700, Month::November, 15).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 15);
        assert_eq!(date.day_ordinal(), 15);
        assert_eq!(date.day_ordinal0(), 14);
        assert_eq!(date.ordinal(), 320);
        assert_eq!(date.ordinal0(), 319);
        assert_eq!(date.julian_day_number(), 2342302);
    }

    #[test]
    fn reform_2342304_jdn_2342302_ordinal() {
        let cal = Calendar::reforming(2342304).unwrap();
        let date = cal.at_ordinal_date(1700, 320).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 15);
        assert_eq!(date.day_ordinal(), 15);
        assert_eq!(date.day_ordinal0(), 14);
        assert_eq!(date.ordinal(), 320);
        assert_eq!(date.ordinal0(), 319);
        assert_eq!(date.julian_day_number(), 2342302);
    }

    #[test]
    fn reform_2342304_jdn_2342303_ymd() {
        let cal = Calendar::reforming(2342304).unwrap();
        let date = cal.at_ymd(1700, Month::November, 16).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 16);
        assert_eq!(date.day_ordinal(), 16);
        assert_eq!(date.day_ordinal0(), 15);
        assert_eq!(date.ordinal(), 321);
        assert_eq!(date.ordinal0(), 320);
        assert_eq!(date.julian_day_number(), 2342303);
    }

    #[test]
    fn reform_2342304_jdn_2342303_ordinal() {
        let cal = Calendar::reforming(2342304).unwrap();
        let date = cal.at_ordinal_date(1700, 321).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 16);
        assert_eq!(date.day_ordinal(), 16);
        assert_eq!(date.day_ordinal0(), 15);
        assert_eq!(date.ordinal(), 321);
        assert_eq!(date.ordinal0(), 320);
        assert_eq!(date.julian_day_number(), 2342303);
    }

    #[test]
    fn reform_2342304_jdn_2342304_ymd() {
        let cal = Calendar::reforming(2342304).unwrap();
        let date = cal.at_ymd(1700, Month::November, 28).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 28);
        assert_eq!(date.day_ordinal(), 17);
        assert_eq!(date.day_ordinal0(), 16);
        assert_eq!(date.ordinal(), 322);
        assert_eq!(date.ordinal0(), 321);
        assert_eq!(date.julian_day_number(), 2342304);
    }

    #[test]
    fn reform_2342304_jdn_2342304_ordinal() {
        let cal = Calendar::reforming(2342304).unwrap();
        let date = cal.at_ordinal_date(1700, 322).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 28);
        assert_eq!(date.day_ordinal(), 17);
        assert_eq!(date.day_ordinal0(), 16);
        assert_eq!(date.ordinal(), 322);
        assert_eq!(date.ordinal0(), 321);
        assert_eq!(date.julian_day_number(), 2342304);
    }

    #[test]
    fn reform_2342304_jdn_2342305_ymd() {
        let cal = Calendar::reforming(2342304).unwrap();
        let date = cal.at_ymd(1700, Month::November, 29).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 18);
        assert_eq!(date.day_ordinal0(), 17);
        assert_eq!(date.ordinal(), 323);
        assert_eq!(date.ordinal0(), 322);
        assert_eq!(date.julian_day_number(), 2342305);
    }

    #[test]
    fn reform_2342304_jdn_2342305_ordinal() {
        let cal = Calendar::reforming(2342304).unwrap();
        let date = cal.at_ordinal_date(1700, 323).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 18);
        assert_eq!(date.day_ordinal0(), 17);
        assert_eq!(date.ordinal(), 323);
        assert_eq!(date.ordinal0(), 322);
        assert_eq!(date.julian_day_number(), 2342305);
    }

    #[test]
    fn reform_2344534_jdn_2344532_ymd() {
        let cal = Calendar::reforming(2344534).unwrap();
        let date = cal.at_ymd(1706, Month::December, 24).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1706);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 24);
        assert_eq!(date.day_ordinal(), 24);
        assert_eq!(date.day_ordinal0(), 23);
        assert_eq!(date.ordinal(), 358);
        assert_eq!(date.ordinal0(), 357);
        assert_eq!(date.julian_day_number(), 2344532);
    }

    #[test]
    fn reform_2344534_jdn_2344532_ordinal() {
        let cal = Calendar::reforming(2344534).unwrap();
        let date = cal.at_ordinal_date(1706, 358).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1706);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 24);
        assert_eq!(date.day_ordinal(), 24);
        assert_eq!(date.day_ordinal0(), 23);
        assert_eq!(date.ordinal(), 358);
        assert_eq!(date.ordinal0(), 357);
        assert_eq!(date.julian_day_number(), 2344532);
    }

    #[test]
    fn reform_2344534_jdn_2344533_ymd() {
        let cal = Calendar::reforming(2344534).unwrap();
        let date = cal.at_ymd(1706, Month::December, 25).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1706);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 25);
        assert_eq!(date.day_ordinal(), 25);
        assert_eq!(date.day_ordinal0(), 24);
        assert_eq!(date.ordinal(), 359);
        assert_eq!(date.ordinal0(), 358);
        assert_eq!(date.julian_day_number(), 2344533);
    }

    #[test]
    fn reform_2344534_jdn_2344533_ordinal() {
        let cal = Calendar::reforming(2344534).unwrap();
        let date = cal.at_ordinal_date(1706, 359).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1706);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 25);
        assert_eq!(date.day_ordinal(), 25);
        assert_eq!(date.day_ordinal0(), 24);
        assert_eq!(date.ordinal(), 359);
        assert_eq!(date.ordinal0(), 358);
        assert_eq!(date.julian_day_number(), 2344533);
    }

    #[test]
    fn reform_2344534_jdn_2344534_ymd() {
        let cal = Calendar::reforming(2344534).unwrap();
        let date = cal.at_ymd(1707, Month::January, 6).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1707);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 6);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2344534);
    }

    #[test]
    fn reform_2344534_jdn_2344534_ordinal() {
        let cal = Calendar::reforming(2344534).unwrap();
        let date = cal.at_ordinal_date(1707, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1707);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 6);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2344534);
    }

    #[test]
    fn reform_2344534_jdn_2344535_ymd() {
        let cal = Calendar::reforming(2344534).unwrap();
        let date = cal.at_ymd(1707, Month::January, 7).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1707);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 2);
        assert_eq!(date.ordinal0(), 1);
        assert_eq!(date.julian_day_number(), 2344535);
    }

    #[test]
    fn reform_2344534_jdn_2344535_ordinal() {
        let cal = Calendar::reforming(2344534).unwrap();
        let date = cal.at_ordinal_date(1707, 2).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1707);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 2);
        assert_eq!(date.ordinal0(), 1);
        assert_eq!(date.julian_day_number(), 2344535);
    }

    #[test]
    fn reform_2421639_jdn_2421637_ymd() {
        let cal = Calendar::reforming(2421639).unwrap();
        let date = cal.at_ymd(1918, Month::January, 30).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1918);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 30);
        assert_eq!(date.day_ordinal(), 30);
        assert_eq!(date.day_ordinal0(), 29);
        assert_eq!(date.ordinal(), 30);
        assert_eq!(date.ordinal0(), 29);
        assert_eq!(date.julian_day_number(), 2421637);
    }

    #[test]
    fn reform_2421639_jdn_2421637_ordinal() {
        let cal = Calendar::reforming(2421639).unwrap();
        let date = cal.at_ordinal_date(1918, 30).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1918);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 30);
        assert_eq!(date.day_ordinal(), 30);
        assert_eq!(date.day_ordinal0(), 29);
        assert_eq!(date.ordinal(), 30);
        assert_eq!(date.ordinal0(), 29);
        assert_eq!(date.julian_day_number(), 2421637);
    }

    #[test]
    fn reform_2421639_jdn_2421638_ymd() {
        let cal = Calendar::reforming(2421639).unwrap();
        let date = cal.at_ymd(1918, Month::January, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1918);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 31);
        assert_eq!(date.ordinal0(), 30);
        assert_eq!(date.julian_day_number(), 2421638);
    }

    #[test]
    fn reform_2421639_jdn_2421638_ordinal() {
        let cal = Calendar::reforming(2421639).unwrap();
        let date = cal.at_ordinal_date(1918, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1918);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 31);
        assert_eq!(date.ordinal0(), 30);
        assert_eq!(date.julian_day_number(), 2421638);
    }

    #[test]
    fn reform_2421639_jdn_2421639_ymd() {
        let cal = Calendar::reforming(2421639).unwrap();
        let date = cal.at_ymd(1918, Month::February, 14).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1918);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 14);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 32);
        assert_eq!(date.ordinal0(), 31);
        assert_eq!(date.julian_day_number(), 2421639);
    }

    #[test]
    fn reform_2421639_jdn_2421639_ordinal() {
        let cal = Calendar::reforming(2421639).unwrap();
        let date = cal.at_ordinal_date(1918, 32).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1918);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 14);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 32);
        assert_eq!(date.ordinal0(), 31);
        assert_eq!(date.julian_day_number(), 2421639);
    }

    #[test]
    fn reform_2421639_jdn_2421640_ymd() {
        let cal = Calendar::reforming(2421639).unwrap();
        let date = cal.at_ymd(1918, Month::February, 15).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1918);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 15);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 33);
        assert_eq!(date.ordinal0(), 32);
        assert_eq!(date.julian_day_number(), 2421640);
    }

    #[test]
    fn reform_2421639_jdn_2421640_ordinal() {
        let cal = Calendar::reforming(2421639).unwrap();
        let date = cal.at_ordinal_date(1918, 33).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1918);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 15);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 33);
        assert_eq!(date.ordinal0(), 32);
        assert_eq!(date.julian_day_number(), 2421640);
    }

    #[test]
    fn reform_2421639_jdn_2421654_ymd() {
        let cal = Calendar::reforming(2421639).unwrap();
        let date = cal.at_ymd(1918, Month::March, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1918);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 47);
        assert_eq!(date.ordinal0(), 46);
        assert_eq!(date.julian_day_number(), 2421654);
    }

    #[test]
    fn reform_2421639_jdn_2421654_ordinal() {
        let cal = Calendar::reforming(2421639).unwrap();
        let date = cal.at_ordinal_date(1918, 47).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1918);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 47);
        assert_eq!(date.ordinal0(), 46);
        assert_eq!(date.julian_day_number(), 2421654);
    }

    #[test]
    fn reform_2460316_jdn_2460314_ymd() {
        let cal = Calendar::reforming(2460316).unwrap();
        let date = cal.at_ymd(2023, Month::December, 22).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 2023);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 22);
        assert_eq!(date.day_ordinal(), 22);
        assert_eq!(date.day_ordinal0(), 21);
        assert_eq!(date.ordinal(), 356);
        assert_eq!(date.ordinal0(), 355);
        assert_eq!(date.julian_day_number(), 2460314);
    }

    #[test]
    fn reform_2460316_jdn_2460314_ordinal() {
        let cal = Calendar::reforming(2460316).unwrap();
        let date = cal.at_ordinal_date(2023, 356).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 2023);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 22);
        assert_eq!(date.day_ordinal(), 22);
        assert_eq!(date.day_ordinal0(), 21);
        assert_eq!(date.ordinal(), 356);
        assert_eq!(date.ordinal0(), 355);
        assert_eq!(date.julian_day_number(), 2460314);
    }

    #[test]
    fn reform_2460316_jdn_2460315_ymd() {
        let cal = Calendar::reforming(2460316).unwrap();
        let date = cal.at_ymd(2023, Month::December, 23).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 2023);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 23);
        assert_eq!(date.day_ordinal(), 23);
        assert_eq!(date.day_ordinal0(), 22);
        assert_eq!(date.ordinal(), 357);
        assert_eq!(date.ordinal0(), 356);
        assert_eq!(date.julian_day_number(), 2460315);
    }

    #[test]
    fn reform_2460316_jdn_2460315_ordinal() {
        let cal = Calendar::reforming(2460316).unwrap();
        let date = cal.at_ordinal_date(2023, 357).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 2023);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 23);
        assert_eq!(date.day_ordinal(), 23);
        assert_eq!(date.day_ordinal0(), 22);
        assert_eq!(date.ordinal(), 357);
        assert_eq!(date.ordinal0(), 356);
        assert_eq!(date.julian_day_number(), 2460315);
    }

    #[test]
    fn reform_2460316_jdn_2460316_ymd() {
        let cal = Calendar::reforming(2460316).unwrap();
        let date = cal.at_ymd(2024, Month::January, 6).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 6);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2460316);
    }

    #[test]
    fn reform_2460316_jdn_2460316_ordinal() {
        let cal = Calendar::reforming(2460316).unwrap();
        let date = cal.at_ordinal_date(2024, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 6);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2460316);
    }

    #[test]
    fn reform_2460316_jdn_2460317_ymd() {
        let cal = Calendar::reforming(2460316).unwrap();
        let date = cal.at_ymd(2024, Month::January, 7).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 2);
        assert_eq!(date.ordinal0(), 1);
        assert_eq!(date.julian_day_number(), 2460317);
    }

    #[test]
    fn reform_2460316_jdn_2460317_ordinal() {
        let cal = Calendar::reforming(2460316).unwrap();
        let date = cal.at_ordinal_date(2024, 2).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 2);
        assert_eq!(date.ordinal0(), 1);
        assert_eq!(date.julian_day_number(), 2460317);
    }

    #[test]
    fn reform_2460316_jdn_2460370_ymd() {
        let cal = Calendar::reforming(2460316).unwrap();
        let date = cal.at_ymd(2024, Month::February, 29).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 29);
        assert_eq!(date.day_ordinal0(), 28);
        assert_eq!(date.ordinal(), 55);
        assert_eq!(date.ordinal0(), 54);
        assert_eq!(date.julian_day_number(), 2460370);
    }

    #[test]
    fn reform_2460316_jdn_2460370_ordinal() {
        let cal = Calendar::reforming(2460316).unwrap();
        let date = cal.at_ordinal_date(2024, 55).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 29);
        assert_eq!(date.day_ordinal0(), 28);
        assert_eq!(date.ordinal(), 55);
        assert_eq!(date.ordinal0(), 54);
        assert_eq!(date.julian_day_number(), 2460370);
    }

    #[test]
    fn reform_2460316_jdn_2460371_ymd() {
        let cal = Calendar::reforming(2460316).unwrap();
        let date = cal.at_ymd(2024, Month::March, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 56);
        assert_eq!(date.ordinal0(), 55);
        assert_eq!(date.julian_day_number(), 2460371);
    }

    #[test]
    fn reform_2460316_jdn_2460371_ordinal() {
        let cal = Calendar::reforming(2460316).unwrap();
        let date = cal.at_ordinal_date(2024, 56).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 56);
        assert_eq!(date.ordinal0(), 55);
        assert_eq!(date.julian_day_number(), 2460371);
    }

    #[test]
    fn reform_2460682_jdn_2460383_ymd() {
        let cal = Calendar::reforming(2460682).unwrap();
        let date = cal.at_ymd(2024, Month::February, 29).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 29);
        assert_eq!(date.day_ordinal0(), 28);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 2460383);
    }

    #[test]
    fn reform_2460682_jdn_2460383_ordinal() {
        let cal = Calendar::reforming(2460682).unwrap();
        let date = cal.at_ordinal_date(2024, 60).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 29);
        assert_eq!(date.day_ordinal0(), 28);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 2460383);
    }

    #[test]
    fn reform_2460682_jdn_2460680_ymd() {
        let cal = Calendar::reforming(2460682).unwrap();
        let date = cal.at_ymd(2024, Month::December, 22).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 22);
        assert_eq!(date.day_ordinal(), 22);
        assert_eq!(date.day_ordinal0(), 21);
        assert_eq!(date.ordinal(), 357);
        assert_eq!(date.ordinal0(), 356);
        assert_eq!(date.julian_day_number(), 2460680);
    }

    #[test]
    fn reform_2460682_jdn_2460680_ordinal() {
        let cal = Calendar::reforming(2460682).unwrap();
        let date = cal.at_ordinal_date(2024, 357).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 22);
        assert_eq!(date.day_ordinal(), 22);
        assert_eq!(date.day_ordinal0(), 21);
        assert_eq!(date.ordinal(), 357);
        assert_eq!(date.ordinal0(), 356);
        assert_eq!(date.julian_day_number(), 2460680);
    }

    #[test]
    fn reform_2460682_jdn_2460681_ymd() {
        let cal = Calendar::reforming(2460682).unwrap();
        let date = cal.at_ymd(2024, Month::December, 23).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 23);
        assert_eq!(date.day_ordinal(), 23);
        assert_eq!(date.day_ordinal0(), 22);
        assert_eq!(date.ordinal(), 358);
        assert_eq!(date.ordinal0(), 357);
        assert_eq!(date.julian_day_number(), 2460681);
    }

    #[test]
    fn reform_2460682_jdn_2460681_ordinal() {
        let cal = Calendar::reforming(2460682).unwrap();
        let date = cal.at_ordinal_date(2024, 358).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 23);
        assert_eq!(date.day_ordinal(), 23);
        assert_eq!(date.day_ordinal0(), 22);
        assert_eq!(date.ordinal(), 358);
        assert_eq!(date.ordinal0(), 357);
        assert_eq!(date.julian_day_number(), 2460681);
    }

    #[test]
    fn reform_2460682_jdn_2460682_ymd() {
        let cal = Calendar::reforming(2460682).unwrap();
        let date = cal.at_ymd(2025, Month::January, 6).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 2025);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 6);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2460682);
    }

    #[test]
    fn reform_2460682_jdn_2460682_ordinal() {
        let cal = Calendar::reforming(2460682).unwrap();
        let date = cal.at_ordinal_date(2025, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 2025);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 6);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2460682);
    }

    #[test]
    fn reform_2460682_jdn_2460683_ymd() {
        let cal = Calendar::reforming(2460682).unwrap();
        let date = cal.at_ymd(2025, Month::January, 7).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 2025);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 2);
        assert_eq!(date.ordinal0(), 1);
        assert_eq!(date.julian_day_number(), 2460683);
    }

    #[test]
    fn reform_2460682_jdn_2460683_ordinal() {
        let cal = Calendar::reforming(2460682).unwrap();
        let date = cal.at_ordinal_date(2025, 2).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 2025);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 2);
        assert_eq!(date.ordinal0(), 1);
        assert_eq!(date.julian_day_number(), 2460683);
    }

    #[test]
    fn reform_3145930_jdn_3145928_ymd() {
        let cal = Calendar::reforming(3145930).unwrap();
        let date = cal.at_ymd(3901, Month::January, 30).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 3901);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 30);
        assert_eq!(date.day_ordinal(), 30);
        assert_eq!(date.day_ordinal0(), 29);
        assert_eq!(date.ordinal(), 30);
        assert_eq!(date.ordinal0(), 29);
        assert_eq!(date.julian_day_number(), 3145928);
    }

    #[test]
    fn reform_3145930_jdn_3145928_ordinal() {
        let cal = Calendar::reforming(3145930).unwrap();
        let date = cal.at_ordinal_date(3901, 30).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 3901);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 30);
        assert_eq!(date.day_ordinal(), 30);
        assert_eq!(date.day_ordinal0(), 29);
        assert_eq!(date.ordinal(), 30);
        assert_eq!(date.ordinal0(), 29);
        assert_eq!(date.julian_day_number(), 3145928);
    }

    #[test]
    fn reform_3145930_jdn_3145929_ymd() {
        let cal = Calendar::reforming(3145930).unwrap();
        let date = cal.at_ymd(3901, Month::January, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 3901);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 31);
        assert_eq!(date.ordinal0(), 30);
        assert_eq!(date.julian_day_number(), 3145929);
    }

    #[test]
    fn reform_3145930_jdn_3145929_ordinal() {
        let cal = Calendar::reforming(3145930).unwrap();
        let date = cal.at_ordinal_date(3901, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 3901);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 31);
        assert_eq!(date.ordinal0(), 30);
        assert_eq!(date.julian_day_number(), 3145929);
    }

    #[test]
    fn reform_3145930_jdn_3145930_ymd() {
        let cal = Calendar::reforming(3145930).unwrap();
        let date = cal.at_ymd(3901, Month::March, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 3901);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 32);
        assert_eq!(date.ordinal0(), 31);
        assert_eq!(date.julian_day_number(), 3145930);
    }

    #[test]
    fn reform_3145930_jdn_3145930_ordinal() {
        let cal = Calendar::reforming(3145930).unwrap();
        let date = cal.at_ordinal_date(3901, 32).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 3901);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 32);
        assert_eq!(date.ordinal0(), 31);
        assert_eq!(date.julian_day_number(), 3145930);
    }

    #[test]
    fn reform_3145930_jdn_3145931_ymd() {
        let cal = Calendar::reforming(3145930).unwrap();
        let date = cal.at_ymd(3901, Month::March, 2).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 3901);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 2);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 33);
        assert_eq!(date.ordinal0(), 32);
        assert_eq!(date.julian_day_number(), 3145931);
    }

    #[test]
    fn reform_3145930_jdn_3145931_ordinal() {
        let cal = Calendar::reforming(3145930).unwrap();
        let date = cal.at_ordinal_date(3901, 33).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 3901);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 2);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 33);
        assert_eq!(date.ordinal0(), 32);
        assert_eq!(date.julian_day_number(), 3145931);
    }

    #[test]
    fn reform_19582149_jdn_19582147_ymd() {
        let cal = Calendar::reforming(19582149).unwrap();
        let date = cal.at_ymd(48900, Month::December, 30).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 48900);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 30);
        assert_eq!(date.day_ordinal(), 30);
        assert_eq!(date.day_ordinal0(), 29);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 19582147);
    }

    #[test]
    fn reform_19582149_jdn_19582147_ordinal() {
        let cal = Calendar::reforming(19582149).unwrap();
        let date = cal.at_ordinal_date(48900, 365).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 48900);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 30);
        assert_eq!(date.day_ordinal(), 30);
        assert_eq!(date.day_ordinal0(), 29);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 19582147);
    }

    #[test]
    fn reform_19582149_jdn_19582148_ymd() {
        let cal = Calendar::reforming(19582149).unwrap();
        let date = cal.at_ymd(48900, Month::December, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 48900);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 366);
        assert_eq!(date.ordinal0(), 365);
        assert_eq!(date.julian_day_number(), 19582148);
    }

    #[test]
    fn reform_19582149_jdn_19582148_ordinal() {
        let cal = Calendar::reforming(19582149).unwrap();
        let date = cal.at_ordinal_date(48900, 366).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 48900);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 366);
        assert_eq!(date.ordinal0(), 365);
        assert_eq!(date.julian_day_number(), 19582148);
    }

    #[test]
    fn reform_19582149_jdn_19582149_ymd() {
        let cal = Calendar::reforming(19582149).unwrap();
        let date = cal.at_ymd(48902, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 48902);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 19582149);
    }

    #[test]
    fn reform_19582149_jdn_19582149_ordinal() {
        let cal = Calendar::reforming(19582149).unwrap();
        let date = cal.at_ordinal_date(48902, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 48902);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 19582149);
    }

    #[test]
    fn reform_19582149_jdn_19582150_ymd() {
        let cal = Calendar::reforming(19582149).unwrap();
        let date = cal.at_ymd(48902, Month::January, 2).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 48902);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 2);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 2);
        assert_eq!(date.ordinal0(), 1);
        assert_eq!(date.julian_day_number(), 19582150);
    }

    #[test]
    fn reform_19582149_jdn_19582150_ordinal() {
        let cal = Calendar::reforming(19582149).unwrap();
        let date = cal.at_ordinal_date(48902, 2).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 48902);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 2);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 2);
        assert_eq!(date.ordinal0(), 1);
        assert_eq!(date.julian_day_number(), 19582150);
    }
}
