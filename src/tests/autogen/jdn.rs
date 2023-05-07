mod jdn_to_julian {
    use crate::{Calendar, Month};

    #[test]
    fn jdn_neg_2147483648() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(-2147483648);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -5884202);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 16);
        assert_eq!(date.day_ordinal(), 16);
        assert_eq!(date.day_ordinal0(), 15);
        assert_eq!(date.ordinal(), 75);
        assert_eq!(date.ordinal0(), 74);
        assert_eq!(date.julian_day_number(), -2147483648);
    }

    #[test]
    fn jdn_neg_214725() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(-214725);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -5300);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 12);
        assert_eq!(date.day_ordinal(), 12);
        assert_eq!(date.day_ordinal0(), 11);
        assert_eq!(date.ordinal(), 43);
        assert_eq!(date.ordinal0(), 42);
        assert_eq!(date.julian_day_number(), -214725);
    }

    #[test]
    fn jdn_neg_178201() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(-178201);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -5200);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 11);
        assert_eq!(date.day_ordinal(), 11);
        assert_eq!(date.day_ordinal0(), 10);
        assert_eq!(date.ordinal(), 42);
        assert_eq!(date.ordinal0(), 41);
        assert_eq!(date.julian_day_number(), -178201);
    }

    #[test]
    fn jdn_neg_141676() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(-141676);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -5100);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 11);
        assert_eq!(date.day_ordinal(), 11);
        assert_eq!(date.day_ordinal0(), 10);
        assert_eq!(date.ordinal(), 42);
        assert_eq!(date.ordinal0(), 41);
        assert_eq!(date.julian_day_number(), -141676);
    }

    #[test]
    fn jdn_neg_105152() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(-105152);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -5000);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 10);
        assert_eq!(date.day_ordinal(), 10);
        assert_eq!(date.day_ordinal0(), 9);
        assert_eq!(date.ordinal(), 41);
        assert_eq!(date.ordinal0(), 40);
        assert_eq!(date.julian_day_number(), -105152);
    }

    #[test]
    fn jdn_neg_68628() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(-68628);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4900);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 9);
        assert_eq!(date.day_ordinal(), 9);
        assert_eq!(date.day_ordinal0(), 8);
        assert_eq!(date.ordinal(), 40);
        assert_eq!(date.ordinal0(), 39);
        assert_eq!(date.julian_day_number(), -68628);
    }

    #[test]
    fn jdn_neg_32104() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(-32104);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4800);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 39);
        assert_eq!(date.ordinal0(), 38);
        assert_eq!(date.julian_day_number(), -32104);
    }

    #[test]
    fn jdn_neg_31738() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(-31738);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4799);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 39);
        assert_eq!(date.ordinal0(), 38);
        assert_eq!(date.julian_day_number(), -31738);
    }

    #[test]
    fn jdn_neg_1402() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(-1402);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4716);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 29);
        assert_eq!(date.day_ordinal0(), 28);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), -1402);
    }

    #[test]
    fn jdn_neg_1095() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(-1095);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4715);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -1095);
    }

    #[test]
    fn jdn_neg_730() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(-730);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4714);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -730);
    }

    #[test]
    fn jdn_neg_692() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(-692);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4714);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 39);
        assert_eq!(date.ordinal0(), 38);
        assert_eq!(date.julian_day_number(), -692);
    }

    #[test]
    fn jdn_neg_365() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(-365);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4713);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -365);
    }

    #[test]
    fn jdn_neg_328() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(-328);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4713);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 7);
        assert_eq!(date.day_ordinal0(), 6);
        assert_eq!(date.ordinal(), 38);
        assert_eq!(date.ordinal0(), 37);
        assert_eq!(date.julian_day_number(), -328);
    }

    #[test]
    fn jdn_neg_327() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(-327);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4713);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 39);
        assert_eq!(date.ordinal0(), 38);
        assert_eq!(date.julian_day_number(), -327);
    }

    #[test]
    fn jdn_neg_1() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(-1);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4713);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), -1);
    }

    #[test]
    fn jdn_0() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(0);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4712);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 0);
    }

    #[test]
    fn jdn_1() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(1);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4712);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 2);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 2);
        assert_eq!(date.ordinal0(), 1);
        assert_eq!(date.julian_day_number(), 1);
    }

    #[test]
    fn jdn_37() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(37);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4712);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 7);
        assert_eq!(date.day_ordinal0(), 6);
        assert_eq!(date.ordinal(), 38);
        assert_eq!(date.ordinal0(), 37);
        assert_eq!(date.julian_day_number(), 37);
    }

    #[test]
    fn jdn_38() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(38);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4712);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 39);
        assert_eq!(date.ordinal0(), 38);
        assert_eq!(date.julian_day_number(), 38);
    }

    #[test]
    fn jdn_365() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(365);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4712);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 366);
        assert_eq!(date.ordinal0(), 365);
        assert_eq!(date.julian_day_number(), 365);
    }

    #[test]
    fn jdn_366() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(366);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4711);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 366);
    }

    #[test]
    fn jdn_404() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(404);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4711);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 39);
        assert_eq!(date.ordinal0(), 38);
        assert_eq!(date.julian_day_number(), 404);
    }

    #[test]
    fn jdn_730() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(730);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4711);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 730);
    }

    #[test]
    fn jdn_731() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(731);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4710);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 731);
    }

    #[test]
    fn jdn_768() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(768);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4710);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 7);
        assert_eq!(date.day_ordinal0(), 6);
        assert_eq!(date.ordinal(), 38);
        assert_eq!(date.ordinal0(), 37);
        assert_eq!(date.julian_day_number(), 768);
    }

    #[test]
    fn jdn_769() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(769);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4710);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 39);
        assert_eq!(date.ordinal0(), 38);
        assert_eq!(date.julian_day_number(), 769);
    }

    #[test]
    fn jdn_1095() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(1095);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4710);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 1095);
    }

    #[test]
    fn jdn_1096() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(1096);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4709);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 1096);
    }

    #[test]
    fn jdn_1133() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(1133);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4709);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 7);
        assert_eq!(date.day_ordinal0(), 6);
        assert_eq!(date.ordinal(), 38);
        assert_eq!(date.ordinal0(), 37);
        assert_eq!(date.julian_day_number(), 1133);
    }

    #[test]
    fn jdn_1134() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(1134);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4709);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 39);
        assert_eq!(date.ordinal0(), 38);
        assert_eq!(date.julian_day_number(), 1134);
    }

    #[test]
    fn jdn_1460() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(1460);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4709);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 1460);
    }

    #[test]
    fn jdn_1461() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(1461);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4708);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 1461);
    }

    #[test]
    fn jdn_1498() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(1498);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4708);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 7);
        assert_eq!(date.day_ordinal0(), 6);
        assert_eq!(date.ordinal(), 38);
        assert_eq!(date.ordinal0(), 37);
        assert_eq!(date.julian_day_number(), 1498);
    }

    #[test]
    fn jdn_1499() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(1499);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4708);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 39);
        assert_eq!(date.ordinal0(), 38);
        assert_eq!(date.julian_day_number(), 1499);
    }

    #[test]
    fn jdn_1826() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(1826);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4708);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 366);
        assert_eq!(date.ordinal0(), 365);
        assert_eq!(date.julian_day_number(), 1826);
    }

    #[test]
    fn jdn_1827() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(1827);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4707);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 1827);
    }

    #[test]
    fn jdn_1864() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(1864);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4707);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 7);
        assert_eq!(date.day_ordinal0(), 6);
        assert_eq!(date.ordinal(), 38);
        assert_eq!(date.ordinal0(), 37);
        assert_eq!(date.julian_day_number(), 1864);
    }

    #[test]
    fn jdn_1865() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(1865);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4707);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 39);
        assert_eq!(date.ordinal0(), 38);
        assert_eq!(date.julian_day_number(), 1865);
    }

    #[test]
    fn jdn_4420() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(4420);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4700);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 7);
        assert_eq!(date.day_ordinal0(), 6);
        assert_eq!(date.ordinal(), 38);
        assert_eq!(date.ordinal0(), 37);
        assert_eq!(date.julian_day_number(), 4420);
    }

    #[test]
    fn jdn_4421() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(4421);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4700);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 39);
        assert_eq!(date.ordinal0(), 38);
        assert_eq!(date.julian_day_number(), 4421);
    }

    #[test]
    fn jdn_4785() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(4785);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4699);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 6);
        assert_eq!(date.day_ordinal(), 6);
        assert_eq!(date.day_ordinal0(), 5);
        assert_eq!(date.ordinal(), 37);
        assert_eq!(date.ordinal0(), 36);
        assert_eq!(date.julian_day_number(), 4785);
    }

    #[test]
    fn jdn_4786() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(4786);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4699);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 7);
        assert_eq!(date.day_ordinal0(), 6);
        assert_eq!(date.ordinal(), 38);
        assert_eq!(date.ordinal0(), 37);
        assert_eq!(date.julian_day_number(), 4786);
    }

    #[test]
    fn jdn_40945() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(40945);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4600);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 7);
        assert_eq!(date.day_ordinal0(), 6);
        assert_eq!(date.ordinal(), 38);
        assert_eq!(date.ordinal0(), 37);
        assert_eq!(date.julian_day_number(), 40945);
    }

    #[test]
    fn jdn_77469() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(77469);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4500);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 6);
        assert_eq!(date.day_ordinal(), 6);
        assert_eq!(date.day_ordinal0(), 5);
        assert_eq!(date.ordinal(), 37);
        assert_eq!(date.ordinal0(), 36);
        assert_eq!(date.julian_day_number(), 77469);
    }

    #[test]
    fn jdn_113992() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(113992);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4400);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 4);
        assert_eq!(date.day_ordinal(), 4);
        assert_eq!(date.day_ordinal0(), 3);
        assert_eq!(date.ordinal(), 35);
        assert_eq!(date.ordinal0(), 34);
        assert_eq!(date.julian_day_number(), 113992);
    }

    #[test]
    fn jdn_113993() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(113993);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4400);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 5);
        assert_eq!(date.day_ordinal(), 5);
        assert_eq!(date.day_ordinal0(), 4);
        assert_eq!(date.ordinal(), 36);
        assert_eq!(date.ordinal0(), 35);
        assert_eq!(date.julian_day_number(), 113993);
    }

    #[test]
    fn jdn_150518() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(150518);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4300);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 5);
        assert_eq!(date.day_ordinal(), 5);
        assert_eq!(date.day_ordinal0(), 4);
        assert_eq!(date.ordinal(), 36);
        assert_eq!(date.ordinal0(), 35);
        assert_eq!(date.julian_day_number(), 150518);
    }

    #[test]
    fn jdn_1719656() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(1719656);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 29);
        assert_eq!(date.day_ordinal0(), 28);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 1719656);
    }

    #[test]
    fn jdn_1721424() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(1721424);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 1721424);
    }

    #[test]
    fn jdn_1721425() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(1721425);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 2);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 2);
        assert_eq!(date.ordinal0(), 1);
        assert_eq!(date.julian_day_number(), 1721425);
    }

    #[test]
    fn jdn_1721426() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(1721426);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 3);
        assert_eq!(date.day_ordinal(), 3);
        assert_eq!(date.day_ordinal0(), 2);
        assert_eq!(date.ordinal(), 3);
        assert_eq!(date.ordinal0(), 2);
        assert_eq!(date.julian_day_number(), 1721426);
    }

    #[test]
    fn jdn_1794166() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(1794166);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 200);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 28);
        assert_eq!(date.day_ordinal(), 28);
        assert_eq!(date.day_ordinal0(), 27);
        assert_eq!(date.ordinal(), 59);
        assert_eq!(date.ordinal0(), 58);
        assert_eq!(date.julian_day_number(), 1794166);
    }

    #[test]
    fn jdn_1794167() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(1794167);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 200);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 29);
        assert_eq!(date.day_ordinal0(), 28);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 1794167);
    }

    #[test]
    fn jdn_1830691() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(1830691);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 300);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 28);
        assert_eq!(date.day_ordinal(), 28);
        assert_eq!(date.day_ordinal0(), 27);
        assert_eq!(date.ordinal(), 59);
        assert_eq!(date.ordinal0(), 58);
        assert_eq!(date.julian_day_number(), 1830691);
    }

    #[test]
    fn jdn_1830692() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(1830692);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 300);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 29);
        assert_eq!(date.day_ordinal0(), 28);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 1830692);
    }

    #[test]
    fn jdn_1830997() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(1830997);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 300);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 30);
        assert_eq!(date.day_ordinal(), 30);
        assert_eq!(date.day_ordinal0(), 29);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 1830997);
    }

    #[test]
    fn jdn_1830998() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(1830998);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 300);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 366);
        assert_eq!(date.ordinal0(), 365);
        assert_eq!(date.julian_day_number(), 1830998);
    }

    #[test]
    fn jdn_1830999() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(1830999);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 301);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 1830999);
    }

    #[test]
    fn jdn_2159358() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(2159358);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1200);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2159358);
    }

    #[test]
    fn jdn_2195883() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(2195883);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1300);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2195883);
    }

    #[test]
    fn jdn_2232408() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(2232408);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1400);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2232408);
    }

    #[test]
    fn jdn_2268933() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(2268933);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1500);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2268933);
    }

    #[test]
    fn jdn_2269298() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(2269298);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1500);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 366);
        assert_eq!(date.ordinal0(), 365);
        assert_eq!(date.julian_day_number(), 2269298);
    }

    #[test]
    fn jdn_2298883() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(2298883);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1581);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 2298883);
    }

    #[test]
    fn jdn_2298884() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(2298884);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2298884);
    }

    #[test]
    fn jdn_2299160() {
        let cal = Calendar::JULIAN;
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
    fn jdn_2299161() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(2299161);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 5);
        assert_eq!(date.day_ordinal(), 5);
        assert_eq!(date.day_ordinal0(), 4);
        assert_eq!(date.ordinal(), 278);
        assert_eq!(date.ordinal0(), 277);
        assert_eq!(date.julian_day_number(), 2299161);
    }

    #[test]
    fn jdn_2299162() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(2299162);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 6);
        assert_eq!(date.day_ordinal(), 6);
        assert_eq!(date.day_ordinal0(), 5);
        assert_eq!(date.ordinal(), 279);
        assert_eq!(date.ordinal0(), 278);
        assert_eq!(date.julian_day_number(), 2299162);
    }

    #[test]
    fn jdn_2299238() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(2299238);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 21);
        assert_eq!(date.day_ordinal(), 21);
        assert_eq!(date.day_ordinal0(), 20);
        assert_eq!(date.ordinal(), 355);
        assert_eq!(date.ordinal0(), 354);
        assert_eq!(date.julian_day_number(), 2299238);
    }

    #[test]
    fn jdn_2299239() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(2299239);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 22);
        assert_eq!(date.day_ordinal(), 22);
        assert_eq!(date.day_ordinal0(), 21);
        assert_eq!(date.ordinal(), 356);
        assert_eq!(date.ordinal0(), 355);
        assert_eq!(date.julian_day_number(), 2299239);
    }

    #[test]
    fn jdn_2305448() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(2305448);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1599);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 22);
        assert_eq!(date.day_ordinal(), 22);
        assert_eq!(date.day_ordinal0(), 21);
        assert_eq!(date.ordinal(), 356);
        assert_eq!(date.ordinal0(), 355);
        assert_eq!(date.julian_day_number(), 2305448);
    }

    #[test]
    fn jdn_2305517() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(2305517);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1600);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 29);
        assert_eq!(date.day_ordinal0(), 28);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 2305517);
    }

    #[test]
    fn jdn_2341973() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(2341973);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1699);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 22);
        assert_eq!(date.day_ordinal(), 22);
        assert_eq!(date.day_ordinal0(), 21);
        assert_eq!(date.ordinal(), 356);
        assert_eq!(date.ordinal0(), 355);
        assert_eq!(date.julian_day_number(), 2341973);
    }

    #[test]
    fn jdn_2342032() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(2342032);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 19);
        assert_eq!(date.day_ordinal(), 19);
        assert_eq!(date.day_ordinal0(), 18);
        assert_eq!(date.ordinal(), 50);
        assert_eq!(date.ordinal0(), 49);
        assert_eq!(date.julian_day_number(), 2342032);
    }

    #[test]
    fn jdn_2342337() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(2342337);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 20);
        assert_eq!(date.day_ordinal(), 20);
        assert_eq!(date.day_ordinal0(), 19);
        assert_eq!(date.ordinal(), 355);
        assert_eq!(date.ordinal0(), 354);
        assert_eq!(date.julian_day_number(), 2342337);
    }

    #[test]
    fn jdn_2342338() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(2342338);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 21);
        assert_eq!(date.day_ordinal(), 21);
        assert_eq!(date.day_ordinal0(), 20);
        assert_eq!(date.ordinal(), 356);
        assert_eq!(date.ordinal0(), 355);
        assert_eq!(date.julian_day_number(), 2342338);
    }

    #[test]
    fn jdn_2378496() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(2378496);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1799);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 20);
        assert_eq!(date.day_ordinal(), 20);
        assert_eq!(date.day_ordinal0(), 19);
        assert_eq!(date.ordinal(), 354);
        assert_eq!(date.ordinal0(), 353);
        assert_eq!(date.julian_day_number(), 2378496);
    }

    #[test]
    fn jdn_2378497() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(2378497);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1799);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 21);
        assert_eq!(date.day_ordinal(), 21);
        assert_eq!(date.day_ordinal0(), 20);
        assert_eq!(date.ordinal(), 355);
        assert_eq!(date.ordinal0(), 354);
        assert_eq!(date.julian_day_number(), 2378497);
    }

    #[test]
    fn jdn_2378861() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(2378861);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1800);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 19);
        assert_eq!(date.day_ordinal(), 19);
        assert_eq!(date.day_ordinal0(), 18);
        assert_eq!(date.ordinal(), 354);
        assert_eq!(date.ordinal0(), 353);
        assert_eq!(date.julian_day_number(), 2378861);
    }

    #[test]
    fn jdn_2378862() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(2378862);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1800);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 20);
        assert_eq!(date.day_ordinal(), 20);
        assert_eq!(date.day_ordinal0(), 19);
        assert_eq!(date.ordinal(), 355);
        assert_eq!(date.ordinal0(), 354);
        assert_eq!(date.julian_day_number(), 2378862);
    }

    #[test]
    fn jdn_2415020() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(2415020);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1899);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 19);
        assert_eq!(date.day_ordinal(), 19);
        assert_eq!(date.day_ordinal0(), 18);
        assert_eq!(date.ordinal(), 353);
        assert_eq!(date.ordinal0(), 352);
        assert_eq!(date.julian_day_number(), 2415020);
    }

    #[test]
    fn jdn_2415021() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(2415021);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1899);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 20);
        assert_eq!(date.day_ordinal(), 20);
        assert_eq!(date.day_ordinal0(), 19);
        assert_eq!(date.ordinal(), 354);
        assert_eq!(date.ordinal0(), 353);
        assert_eq!(date.julian_day_number(), 2415021);
    }

    #[test]
    fn jdn_2415385() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(2415385);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1900);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 18);
        assert_eq!(date.day_ordinal(), 18);
        assert_eq!(date.day_ordinal0(), 17);
        assert_eq!(date.ordinal(), 353);
        assert_eq!(date.ordinal0(), 352);
        assert_eq!(date.julian_day_number(), 2415385);
    }

    #[test]
    fn jdn_2415386() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(2415386);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1900);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 19);
        assert_eq!(date.day_ordinal(), 19);
        assert_eq!(date.day_ordinal0(), 18);
        assert_eq!(date.ordinal(), 354);
        assert_eq!(date.ordinal0(), 353);
        assert_eq!(date.julian_day_number(), 2415386);
    }

    #[test]
    fn jdn_2451605() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(2451605);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 2000);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 17);
        assert_eq!(date.day_ordinal(), 17);
        assert_eq!(date.day_ordinal0(), 16);
        assert_eq!(date.ordinal(), 48);
        assert_eq!(date.ordinal0(), 47);
        assert_eq!(date.julian_day_number(), 2451605);
    }

    #[test]
    fn jdn_2451910() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(2451910);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 2000);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 18);
        assert_eq!(date.day_ordinal(), 18);
        assert_eq!(date.day_ordinal0(), 17);
        assert_eq!(date.ordinal(), 353);
        assert_eq!(date.ordinal0(), 352);
        assert_eq!(date.julian_day_number(), 2451910);
    }

    #[test]
    fn jdn_2451911() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(2451911);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 2000);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 19);
        assert_eq!(date.day_ordinal(), 19);
        assert_eq!(date.day_ordinal0(), 18);
        assert_eq!(date.ordinal(), 354);
        assert_eq!(date.ordinal0(), 353);
        assert_eq!(date.julian_day_number(), 2451911);
    }

    #[test]
    fn jdn_3145930() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(3145930);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 3901);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 32);
        assert_eq!(date.ordinal0(), 31);
        assert_eq!(date.julian_day_number(), 3145930);
    }

    #[test]
    fn jdn_19508368() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(19508368);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 48699);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 19508368);
    }

    #[test]
    fn jdn_19581784() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(19581784);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 48900);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 2);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 2);
        assert_eq!(date.ordinal0(), 1);
        assert_eq!(date.julian_day_number(), 19581784);
    }

    #[test]
    fn jdn_19581842() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(19581842);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 48900);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 29);
        assert_eq!(date.day_ordinal0(), 28);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 19581842);
    }

    #[test]
    fn jdn_19581843() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(19581843);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 48900);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 61);
        assert_eq!(date.ordinal0(), 60);
        assert_eq!(date.julian_day_number(), 19581843);
    }

    #[test]
    fn jdn_19582148() {
        let cal = Calendar::JULIAN;
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
    fn jdn_19582149() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(19582149);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 48901);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 19582149);
    }

    #[test]
    fn jdn_19617943() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(19617943);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 48999);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 19617943);
    }

    #[test]
    fn jdn_19654466() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(19654466);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 49098);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 30);
        assert_eq!(date.day_ordinal(), 30);
        assert_eq!(date.day_ordinal0(), 29);
        assert_eq!(date.ordinal(), 364);
        assert_eq!(date.ordinal0(), 363);
        assert_eq!(date.julian_day_number(), 19654466);
    }

    #[test]
    fn jdn_19654467() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(19654467);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 49098);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 19654467);
    }

    #[test]
    fn jdn_19654468() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(19654468);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 49099);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 19654468);
    }

    #[test]
    fn jdn_2147439589() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(2147439589);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 5874657);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 3);
        assert_eq!(date.day_ordinal(), 3);
        assert_eq!(date.day_ordinal0(), 2);
        assert_eq!(date.ordinal(), 62);
        assert_eq!(date.ordinal0(), 61);
        assert_eq!(date.julian_day_number(), 2147439589);
    }

    #[test]
    fn jdn_2147483647() {
        let cal = Calendar::JULIAN;
        let date = cal.at_jdn(2147483647);
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 5874777);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 17);
        assert_eq!(date.day_ordinal(), 17);
        assert_eq!(date.day_ordinal0(), 16);
        assert_eq!(date.ordinal(), 290);
        assert_eq!(date.ordinal0(), 289);
        assert_eq!(date.julian_day_number(), 2147483647);
    }
}

mod julian_to_jdn {
    use crate::{Calendar, Month};

    #[test]
    fn jdn_neg_2147483648_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-5884202, Month::March, 16).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -5884202);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 16);
        assert_eq!(date.day_ordinal(), 16);
        assert_eq!(date.day_ordinal0(), 15);
        assert_eq!(date.ordinal(), 75);
        assert_eq!(date.ordinal0(), 74);
        assert_eq!(date.julian_day_number(), -2147483648);
    }

    #[test]
    fn jdn_neg_2147483648_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-5884202, 75).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -5884202);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 16);
        assert_eq!(date.day_ordinal(), 16);
        assert_eq!(date.day_ordinal0(), 15);
        assert_eq!(date.ordinal(), 75);
        assert_eq!(date.ordinal0(), 74);
        assert_eq!(date.julian_day_number(), -2147483648);
    }

    #[test]
    fn jdn_neg_214725_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-5300, Month::February, 12).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -5300);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 12);
        assert_eq!(date.day_ordinal(), 12);
        assert_eq!(date.day_ordinal0(), 11);
        assert_eq!(date.ordinal(), 43);
        assert_eq!(date.ordinal0(), 42);
        assert_eq!(date.julian_day_number(), -214725);
    }

    #[test]
    fn jdn_neg_214725_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-5300, 43).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -5300);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 12);
        assert_eq!(date.day_ordinal(), 12);
        assert_eq!(date.day_ordinal0(), 11);
        assert_eq!(date.ordinal(), 43);
        assert_eq!(date.ordinal0(), 42);
        assert_eq!(date.julian_day_number(), -214725);
    }

    #[test]
    fn jdn_neg_178201_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-5200, Month::February, 11).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -5200);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 11);
        assert_eq!(date.day_ordinal(), 11);
        assert_eq!(date.day_ordinal0(), 10);
        assert_eq!(date.ordinal(), 42);
        assert_eq!(date.ordinal0(), 41);
        assert_eq!(date.julian_day_number(), -178201);
    }

    #[test]
    fn jdn_neg_178201_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-5200, 42).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -5200);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 11);
        assert_eq!(date.day_ordinal(), 11);
        assert_eq!(date.day_ordinal0(), 10);
        assert_eq!(date.ordinal(), 42);
        assert_eq!(date.ordinal0(), 41);
        assert_eq!(date.julian_day_number(), -178201);
    }

    #[test]
    fn jdn_neg_141676_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-5100, Month::February, 11).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -5100);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 11);
        assert_eq!(date.day_ordinal(), 11);
        assert_eq!(date.day_ordinal0(), 10);
        assert_eq!(date.ordinal(), 42);
        assert_eq!(date.ordinal0(), 41);
        assert_eq!(date.julian_day_number(), -141676);
    }

    #[test]
    fn jdn_neg_141676_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-5100, 42).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -5100);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 11);
        assert_eq!(date.day_ordinal(), 11);
        assert_eq!(date.day_ordinal0(), 10);
        assert_eq!(date.ordinal(), 42);
        assert_eq!(date.ordinal0(), 41);
        assert_eq!(date.julian_day_number(), -141676);
    }

    #[test]
    fn jdn_neg_105152_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-5000, Month::February, 10).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -5000);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 10);
        assert_eq!(date.day_ordinal(), 10);
        assert_eq!(date.day_ordinal0(), 9);
        assert_eq!(date.ordinal(), 41);
        assert_eq!(date.ordinal0(), 40);
        assert_eq!(date.julian_day_number(), -105152);
    }

    #[test]
    fn jdn_neg_105152_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-5000, 41).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -5000);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 10);
        assert_eq!(date.day_ordinal(), 10);
        assert_eq!(date.day_ordinal0(), 9);
        assert_eq!(date.ordinal(), 41);
        assert_eq!(date.ordinal0(), 40);
        assert_eq!(date.julian_day_number(), -105152);
    }

    #[test]
    fn jdn_neg_68628_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4900, Month::February, 9).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4900);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 9);
        assert_eq!(date.day_ordinal(), 9);
        assert_eq!(date.day_ordinal0(), 8);
        assert_eq!(date.ordinal(), 40);
        assert_eq!(date.ordinal0(), 39);
        assert_eq!(date.julian_day_number(), -68628);
    }

    #[test]
    fn jdn_neg_68628_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4900, 40).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4900);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 9);
        assert_eq!(date.day_ordinal(), 9);
        assert_eq!(date.day_ordinal0(), 8);
        assert_eq!(date.ordinal(), 40);
        assert_eq!(date.ordinal0(), 39);
        assert_eq!(date.julian_day_number(), -68628);
    }

    #[test]
    fn jdn_neg_32104_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4800, Month::February, 8).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4800);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 39);
        assert_eq!(date.ordinal0(), 38);
        assert_eq!(date.julian_day_number(), -32104);
    }

    #[test]
    fn jdn_neg_32104_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4800, 39).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4800);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 39);
        assert_eq!(date.ordinal0(), 38);
        assert_eq!(date.julian_day_number(), -32104);
    }

    #[test]
    fn jdn_neg_31738_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4799, Month::February, 8).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4799);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 39);
        assert_eq!(date.ordinal0(), 38);
        assert_eq!(date.julian_day_number(), -31738);
    }

    #[test]
    fn jdn_neg_31738_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4799, 39).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4799);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 39);
        assert_eq!(date.ordinal0(), 38);
        assert_eq!(date.julian_day_number(), -31738);
    }

    #[test]
    fn jdn_neg_1402_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4716, Month::February, 29).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4716);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 29);
        assert_eq!(date.day_ordinal0(), 28);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), -1402);
    }

    #[test]
    fn jdn_neg_1402_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4716, 60).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4716);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 29);
        assert_eq!(date.day_ordinal0(), 28);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), -1402);
    }

    #[test]
    fn jdn_neg_1095_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4715, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4715);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -1095);
    }

    #[test]
    fn jdn_neg_1095_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4715, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4715);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -1095);
    }

    #[test]
    fn jdn_neg_730_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4714, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4714);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -730);
    }

    #[test]
    fn jdn_neg_730_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4714, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4714);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -730);
    }

    #[test]
    fn jdn_neg_692_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4714, Month::February, 8).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4714);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 39);
        assert_eq!(date.ordinal0(), 38);
        assert_eq!(date.julian_day_number(), -692);
    }

    #[test]
    fn jdn_neg_692_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4714, 39).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4714);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 39);
        assert_eq!(date.ordinal0(), 38);
        assert_eq!(date.julian_day_number(), -692);
    }

    #[test]
    fn jdn_neg_365_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4713, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4713);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -365);
    }

    #[test]
    fn jdn_neg_365_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4713, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4713);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -365);
    }

    #[test]
    fn jdn_neg_328_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4713, Month::February, 7).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4713);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 7);
        assert_eq!(date.day_ordinal0(), 6);
        assert_eq!(date.ordinal(), 38);
        assert_eq!(date.ordinal0(), 37);
        assert_eq!(date.julian_day_number(), -328);
    }

    #[test]
    fn jdn_neg_328_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4713, 38).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4713);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 7);
        assert_eq!(date.day_ordinal0(), 6);
        assert_eq!(date.ordinal(), 38);
        assert_eq!(date.ordinal0(), 37);
        assert_eq!(date.julian_day_number(), -328);
    }

    #[test]
    fn jdn_neg_327_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4713, Month::February, 8).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4713);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 39);
        assert_eq!(date.ordinal0(), 38);
        assert_eq!(date.julian_day_number(), -327);
    }

    #[test]
    fn jdn_neg_327_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4713, 39).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4713);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 39);
        assert_eq!(date.ordinal0(), 38);
        assert_eq!(date.julian_day_number(), -327);
    }

    #[test]
    fn jdn_neg_1_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4713, Month::December, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4713);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), -1);
    }

    #[test]
    fn jdn_neg_1_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4713, 365).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4713);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), -1);
    }

    #[test]
    fn jdn_0_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4712, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4712);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 0);
    }

    #[test]
    fn jdn_0_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4712, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4712);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 0);
    }

    #[test]
    fn jdn_1_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4712, Month::January, 2).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4712);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 2);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 2);
        assert_eq!(date.ordinal0(), 1);
        assert_eq!(date.julian_day_number(), 1);
    }

    #[test]
    fn jdn_1_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4712, 2).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4712);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 2);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 2);
        assert_eq!(date.ordinal0(), 1);
        assert_eq!(date.julian_day_number(), 1);
    }

    #[test]
    fn jdn_37_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4712, Month::February, 7).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4712);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 7);
        assert_eq!(date.day_ordinal0(), 6);
        assert_eq!(date.ordinal(), 38);
        assert_eq!(date.ordinal0(), 37);
        assert_eq!(date.julian_day_number(), 37);
    }

    #[test]
    fn jdn_37_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4712, 38).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4712);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 7);
        assert_eq!(date.day_ordinal0(), 6);
        assert_eq!(date.ordinal(), 38);
        assert_eq!(date.ordinal0(), 37);
        assert_eq!(date.julian_day_number(), 37);
    }

    #[test]
    fn jdn_38_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4712, Month::February, 8).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4712);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 39);
        assert_eq!(date.ordinal0(), 38);
        assert_eq!(date.julian_day_number(), 38);
    }

    #[test]
    fn jdn_38_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4712, 39).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4712);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 39);
        assert_eq!(date.ordinal0(), 38);
        assert_eq!(date.julian_day_number(), 38);
    }

    #[test]
    fn jdn_365_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4712, Month::December, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4712);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 366);
        assert_eq!(date.ordinal0(), 365);
        assert_eq!(date.julian_day_number(), 365);
    }

    #[test]
    fn jdn_365_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4712, 366).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4712);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 366);
        assert_eq!(date.ordinal0(), 365);
        assert_eq!(date.julian_day_number(), 365);
    }

    #[test]
    fn jdn_366_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4711, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4711);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 366);
    }

    #[test]
    fn jdn_366_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4711, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4711);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 366);
    }

    #[test]
    fn jdn_404_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4711, Month::February, 8).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4711);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 39);
        assert_eq!(date.ordinal0(), 38);
        assert_eq!(date.julian_day_number(), 404);
    }

    #[test]
    fn jdn_404_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4711, 39).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4711);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 39);
        assert_eq!(date.ordinal0(), 38);
        assert_eq!(date.julian_day_number(), 404);
    }

    #[test]
    fn jdn_730_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4711, Month::December, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4711);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 730);
    }

    #[test]
    fn jdn_730_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4711, 365).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4711);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 730);
    }

    #[test]
    fn jdn_731_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4710, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4710);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 731);
    }

    #[test]
    fn jdn_731_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4710, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4710);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 731);
    }

    #[test]
    fn jdn_768_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4710, Month::February, 7).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4710);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 7);
        assert_eq!(date.day_ordinal0(), 6);
        assert_eq!(date.ordinal(), 38);
        assert_eq!(date.ordinal0(), 37);
        assert_eq!(date.julian_day_number(), 768);
    }

    #[test]
    fn jdn_768_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4710, 38).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4710);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 7);
        assert_eq!(date.day_ordinal0(), 6);
        assert_eq!(date.ordinal(), 38);
        assert_eq!(date.ordinal0(), 37);
        assert_eq!(date.julian_day_number(), 768);
    }

    #[test]
    fn jdn_769_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4710, Month::February, 8).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4710);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 39);
        assert_eq!(date.ordinal0(), 38);
        assert_eq!(date.julian_day_number(), 769);
    }

    #[test]
    fn jdn_769_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4710, 39).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4710);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 39);
        assert_eq!(date.ordinal0(), 38);
        assert_eq!(date.julian_day_number(), 769);
    }

    #[test]
    fn jdn_1095_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4710, Month::December, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4710);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 1095);
    }

    #[test]
    fn jdn_1095_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4710, 365).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4710);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 1095);
    }

    #[test]
    fn jdn_1096_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4709, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4709);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 1096);
    }

    #[test]
    fn jdn_1096_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4709, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4709);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 1096);
    }

    #[test]
    fn jdn_1133_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4709, Month::February, 7).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4709);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 7);
        assert_eq!(date.day_ordinal0(), 6);
        assert_eq!(date.ordinal(), 38);
        assert_eq!(date.ordinal0(), 37);
        assert_eq!(date.julian_day_number(), 1133);
    }

    #[test]
    fn jdn_1133_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4709, 38).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4709);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 7);
        assert_eq!(date.day_ordinal0(), 6);
        assert_eq!(date.ordinal(), 38);
        assert_eq!(date.ordinal0(), 37);
        assert_eq!(date.julian_day_number(), 1133);
    }

    #[test]
    fn jdn_1134_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4709, Month::February, 8).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4709);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 39);
        assert_eq!(date.ordinal0(), 38);
        assert_eq!(date.julian_day_number(), 1134);
    }

    #[test]
    fn jdn_1134_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4709, 39).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4709);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 39);
        assert_eq!(date.ordinal0(), 38);
        assert_eq!(date.julian_day_number(), 1134);
    }

    #[test]
    fn jdn_1460_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4709, Month::December, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4709);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 1460);
    }

    #[test]
    fn jdn_1460_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4709, 365).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4709);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 1460);
    }

    #[test]
    fn jdn_1461_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4708, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4708);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 1461);
    }

    #[test]
    fn jdn_1461_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4708, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4708);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 1461);
    }

    #[test]
    fn jdn_1498_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4708, Month::February, 7).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4708);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 7);
        assert_eq!(date.day_ordinal0(), 6);
        assert_eq!(date.ordinal(), 38);
        assert_eq!(date.ordinal0(), 37);
        assert_eq!(date.julian_day_number(), 1498);
    }

    #[test]
    fn jdn_1498_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4708, 38).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4708);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 7);
        assert_eq!(date.day_ordinal0(), 6);
        assert_eq!(date.ordinal(), 38);
        assert_eq!(date.ordinal0(), 37);
        assert_eq!(date.julian_day_number(), 1498);
    }

    #[test]
    fn jdn_1499_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4708, Month::February, 8).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4708);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 39);
        assert_eq!(date.ordinal0(), 38);
        assert_eq!(date.julian_day_number(), 1499);
    }

    #[test]
    fn jdn_1499_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4708, 39).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4708);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 39);
        assert_eq!(date.ordinal0(), 38);
        assert_eq!(date.julian_day_number(), 1499);
    }

    #[test]
    fn jdn_1826_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4708, Month::December, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4708);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 366);
        assert_eq!(date.ordinal0(), 365);
        assert_eq!(date.julian_day_number(), 1826);
    }

    #[test]
    fn jdn_1826_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4708, 366).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4708);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 366);
        assert_eq!(date.ordinal0(), 365);
        assert_eq!(date.julian_day_number(), 1826);
    }

    #[test]
    fn jdn_1827_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4707, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4707);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 1827);
    }

    #[test]
    fn jdn_1827_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4707, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4707);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 1827);
    }

    #[test]
    fn jdn_1864_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4707, Month::February, 7).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4707);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 7);
        assert_eq!(date.day_ordinal0(), 6);
        assert_eq!(date.ordinal(), 38);
        assert_eq!(date.ordinal0(), 37);
        assert_eq!(date.julian_day_number(), 1864);
    }

    #[test]
    fn jdn_1864_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4707, 38).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4707);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 7);
        assert_eq!(date.day_ordinal0(), 6);
        assert_eq!(date.ordinal(), 38);
        assert_eq!(date.ordinal0(), 37);
        assert_eq!(date.julian_day_number(), 1864);
    }

    #[test]
    fn jdn_1865_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4707, Month::February, 8).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4707);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 39);
        assert_eq!(date.ordinal0(), 38);
        assert_eq!(date.julian_day_number(), 1865);
    }

    #[test]
    fn jdn_1865_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4707, 39).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4707);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 39);
        assert_eq!(date.ordinal0(), 38);
        assert_eq!(date.julian_day_number(), 1865);
    }

    #[test]
    fn jdn_4420_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4700, Month::February, 7).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4700);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 7);
        assert_eq!(date.day_ordinal0(), 6);
        assert_eq!(date.ordinal(), 38);
        assert_eq!(date.ordinal0(), 37);
        assert_eq!(date.julian_day_number(), 4420);
    }

    #[test]
    fn jdn_4420_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4700, 38).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4700);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 7);
        assert_eq!(date.day_ordinal0(), 6);
        assert_eq!(date.ordinal(), 38);
        assert_eq!(date.ordinal0(), 37);
        assert_eq!(date.julian_day_number(), 4420);
    }

    #[test]
    fn jdn_4421_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4700, Month::February, 8).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4700);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 39);
        assert_eq!(date.ordinal0(), 38);
        assert_eq!(date.julian_day_number(), 4421);
    }

    #[test]
    fn jdn_4421_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4700, 39).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4700);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 39);
        assert_eq!(date.ordinal0(), 38);
        assert_eq!(date.julian_day_number(), 4421);
    }

    #[test]
    fn jdn_4785_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4699, Month::February, 6).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4699);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 6);
        assert_eq!(date.day_ordinal(), 6);
        assert_eq!(date.day_ordinal0(), 5);
        assert_eq!(date.ordinal(), 37);
        assert_eq!(date.ordinal0(), 36);
        assert_eq!(date.julian_day_number(), 4785);
    }

    #[test]
    fn jdn_4785_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4699, 37).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4699);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 6);
        assert_eq!(date.day_ordinal(), 6);
        assert_eq!(date.day_ordinal0(), 5);
        assert_eq!(date.ordinal(), 37);
        assert_eq!(date.ordinal0(), 36);
        assert_eq!(date.julian_day_number(), 4785);
    }

    #[test]
    fn jdn_4786_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4699, Month::February, 7).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4699);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 7);
        assert_eq!(date.day_ordinal0(), 6);
        assert_eq!(date.ordinal(), 38);
        assert_eq!(date.ordinal0(), 37);
        assert_eq!(date.julian_day_number(), 4786);
    }

    #[test]
    fn jdn_4786_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4699, 38).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4699);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 7);
        assert_eq!(date.day_ordinal0(), 6);
        assert_eq!(date.ordinal(), 38);
        assert_eq!(date.ordinal0(), 37);
        assert_eq!(date.julian_day_number(), 4786);
    }

    #[test]
    fn jdn_40945_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4600, Month::February, 7).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4600);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 7);
        assert_eq!(date.day_ordinal0(), 6);
        assert_eq!(date.ordinal(), 38);
        assert_eq!(date.ordinal0(), 37);
        assert_eq!(date.julian_day_number(), 40945);
    }

    #[test]
    fn jdn_40945_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4600, 38).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4600);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 7);
        assert_eq!(date.day_ordinal(), 7);
        assert_eq!(date.day_ordinal0(), 6);
        assert_eq!(date.ordinal(), 38);
        assert_eq!(date.ordinal0(), 37);
        assert_eq!(date.julian_day_number(), 40945);
    }

    #[test]
    fn jdn_77469_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4500, Month::February, 6).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4500);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 6);
        assert_eq!(date.day_ordinal(), 6);
        assert_eq!(date.day_ordinal0(), 5);
        assert_eq!(date.ordinal(), 37);
        assert_eq!(date.ordinal0(), 36);
        assert_eq!(date.julian_day_number(), 77469);
    }

    #[test]
    fn jdn_77469_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4500, 37).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4500);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 6);
        assert_eq!(date.day_ordinal(), 6);
        assert_eq!(date.day_ordinal0(), 5);
        assert_eq!(date.ordinal(), 37);
        assert_eq!(date.ordinal0(), 36);
        assert_eq!(date.julian_day_number(), 77469);
    }

    #[test]
    fn jdn_113992_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4400, Month::February, 4).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4400);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 4);
        assert_eq!(date.day_ordinal(), 4);
        assert_eq!(date.day_ordinal0(), 3);
        assert_eq!(date.ordinal(), 35);
        assert_eq!(date.ordinal0(), 34);
        assert_eq!(date.julian_day_number(), 113992);
    }

    #[test]
    fn jdn_113992_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4400, 35).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4400);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 4);
        assert_eq!(date.day_ordinal(), 4);
        assert_eq!(date.day_ordinal0(), 3);
        assert_eq!(date.ordinal(), 35);
        assert_eq!(date.ordinal0(), 34);
        assert_eq!(date.julian_day_number(), 113992);
    }

    #[test]
    fn jdn_113993_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4400, Month::February, 5).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4400);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 5);
        assert_eq!(date.day_ordinal(), 5);
        assert_eq!(date.day_ordinal0(), 4);
        assert_eq!(date.ordinal(), 36);
        assert_eq!(date.ordinal0(), 35);
        assert_eq!(date.julian_day_number(), 113993);
    }

    #[test]
    fn jdn_113993_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4400, 36).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4400);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 5);
        assert_eq!(date.day_ordinal(), 5);
        assert_eq!(date.day_ordinal0(), 4);
        assert_eq!(date.ordinal(), 36);
        assert_eq!(date.ordinal0(), 35);
        assert_eq!(date.julian_day_number(), 113993);
    }

    #[test]
    fn jdn_150518_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4300, Month::February, 5).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4300);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 5);
        assert_eq!(date.day_ordinal(), 5);
        assert_eq!(date.day_ordinal0(), 4);
        assert_eq!(date.ordinal(), 36);
        assert_eq!(date.ordinal0(), 35);
        assert_eq!(date.julian_day_number(), 150518);
    }

    #[test]
    fn jdn_150518_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4300, 36).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4300);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 5);
        assert_eq!(date.day_ordinal(), 5);
        assert_eq!(date.day_ordinal0(), 4);
        assert_eq!(date.ordinal(), 36);
        assert_eq!(date.ordinal0(), 35);
        assert_eq!(date.julian_day_number(), 150518);
    }

    #[test]
    fn jdn_1719656_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(-4, Month::February, 29).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 29);
        assert_eq!(date.day_ordinal0(), 28);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 1719656);
    }

    #[test]
    fn jdn_1719656_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(-4, 60).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), -4);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 29);
        assert_eq!(date.day_ordinal0(), 28);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 1719656);
    }

    #[test]
    fn jdn_1721424_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(1, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 1721424);
    }

    #[test]
    fn jdn_1721424_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(1, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 1721424);
    }

    #[test]
    fn jdn_1721425_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(1, Month::January, 2).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 2);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 2);
        assert_eq!(date.ordinal0(), 1);
        assert_eq!(date.julian_day_number(), 1721425);
    }

    #[test]
    fn jdn_1721425_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(1, 2).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 2);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 2);
        assert_eq!(date.ordinal0(), 1);
        assert_eq!(date.julian_day_number(), 1721425);
    }

    #[test]
    fn jdn_1721426_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(1, Month::January, 3).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 3);
        assert_eq!(date.day_ordinal(), 3);
        assert_eq!(date.day_ordinal0(), 2);
        assert_eq!(date.ordinal(), 3);
        assert_eq!(date.ordinal0(), 2);
        assert_eq!(date.julian_day_number(), 1721426);
    }

    #[test]
    fn jdn_1721426_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(1, 3).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 3);
        assert_eq!(date.day_ordinal(), 3);
        assert_eq!(date.day_ordinal0(), 2);
        assert_eq!(date.ordinal(), 3);
        assert_eq!(date.ordinal0(), 2);
        assert_eq!(date.julian_day_number(), 1721426);
    }

    #[test]
    fn jdn_1794166_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(200, Month::February, 28).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 200);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 28);
        assert_eq!(date.day_ordinal(), 28);
        assert_eq!(date.day_ordinal0(), 27);
        assert_eq!(date.ordinal(), 59);
        assert_eq!(date.ordinal0(), 58);
        assert_eq!(date.julian_day_number(), 1794166);
    }

    #[test]
    fn jdn_1794166_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(200, 59).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 200);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 28);
        assert_eq!(date.day_ordinal(), 28);
        assert_eq!(date.day_ordinal0(), 27);
        assert_eq!(date.ordinal(), 59);
        assert_eq!(date.ordinal0(), 58);
        assert_eq!(date.julian_day_number(), 1794166);
    }

    #[test]
    fn jdn_1794167_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(200, Month::February, 29).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 200);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 29);
        assert_eq!(date.day_ordinal0(), 28);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 1794167);
    }

    #[test]
    fn jdn_1794167_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(200, 60).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 200);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 29);
        assert_eq!(date.day_ordinal0(), 28);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 1794167);
    }

    #[test]
    fn jdn_1830691_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(300, Month::February, 28).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 300);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 28);
        assert_eq!(date.day_ordinal(), 28);
        assert_eq!(date.day_ordinal0(), 27);
        assert_eq!(date.ordinal(), 59);
        assert_eq!(date.ordinal0(), 58);
        assert_eq!(date.julian_day_number(), 1830691);
    }

    #[test]
    fn jdn_1830691_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(300, 59).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 300);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 28);
        assert_eq!(date.day_ordinal(), 28);
        assert_eq!(date.day_ordinal0(), 27);
        assert_eq!(date.ordinal(), 59);
        assert_eq!(date.ordinal0(), 58);
        assert_eq!(date.julian_day_number(), 1830691);
    }

    #[test]
    fn jdn_1830692_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(300, Month::February, 29).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 300);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 29);
        assert_eq!(date.day_ordinal0(), 28);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 1830692);
    }

    #[test]
    fn jdn_1830692_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(300, 60).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 300);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 29);
        assert_eq!(date.day_ordinal0(), 28);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 1830692);
    }

    #[test]
    fn jdn_1830997_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(300, Month::December, 30).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 300);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 30);
        assert_eq!(date.day_ordinal(), 30);
        assert_eq!(date.day_ordinal0(), 29);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 1830997);
    }

    #[test]
    fn jdn_1830997_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(300, 365).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 300);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 30);
        assert_eq!(date.day_ordinal(), 30);
        assert_eq!(date.day_ordinal0(), 29);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 1830997);
    }

    #[test]
    fn jdn_1830998_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(300, Month::December, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 300);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 366);
        assert_eq!(date.ordinal0(), 365);
        assert_eq!(date.julian_day_number(), 1830998);
    }

    #[test]
    fn jdn_1830998_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(300, 366).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 300);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 366);
        assert_eq!(date.ordinal0(), 365);
        assert_eq!(date.julian_day_number(), 1830998);
    }

    #[test]
    fn jdn_1830999_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(301, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 301);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 1830999);
    }

    #[test]
    fn jdn_1830999_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(301, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 301);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 1830999);
    }

    #[test]
    fn jdn_2159358_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(1200, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1200);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2159358);
    }

    #[test]
    fn jdn_2159358_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(1200, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1200);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2159358);
    }

    #[test]
    fn jdn_2195883_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(1300, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1300);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2195883);
    }

    #[test]
    fn jdn_2195883_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(1300, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1300);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2195883);
    }

    #[test]
    fn jdn_2232408_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(1400, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1400);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2232408);
    }

    #[test]
    fn jdn_2232408_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(1400, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1400);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2232408);
    }

    #[test]
    fn jdn_2268933_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(1500, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1500);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2268933);
    }

    #[test]
    fn jdn_2268933_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(1500, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1500);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2268933);
    }

    #[test]
    fn jdn_2269298_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(1500, Month::December, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1500);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 366);
        assert_eq!(date.ordinal0(), 365);
        assert_eq!(date.julian_day_number(), 2269298);
    }

    #[test]
    fn jdn_2269298_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(1500, 366).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1500);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 366);
        assert_eq!(date.ordinal0(), 365);
        assert_eq!(date.julian_day_number(), 2269298);
    }

    #[test]
    fn jdn_2298883_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(1581, Month::December, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1581);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 2298883);
    }

    #[test]
    fn jdn_2298883_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(1581, 365).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1581);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 2298883);
    }

    #[test]
    fn jdn_2298884_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(1582, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2298884);
    }

    #[test]
    fn jdn_2298884_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(1582, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2298884);
    }

    #[test]
    fn jdn_2299160_ymd() {
        let cal = Calendar::JULIAN;
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
    fn jdn_2299160_ordinal() {
        let cal = Calendar::JULIAN;
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
    fn jdn_2299161_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(1582, Month::October, 5).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 5);
        assert_eq!(date.day_ordinal(), 5);
        assert_eq!(date.day_ordinal0(), 4);
        assert_eq!(date.ordinal(), 278);
        assert_eq!(date.ordinal0(), 277);
        assert_eq!(date.julian_day_number(), 2299161);
    }

    #[test]
    fn jdn_2299161_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(1582, 278).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 5);
        assert_eq!(date.day_ordinal(), 5);
        assert_eq!(date.day_ordinal0(), 4);
        assert_eq!(date.ordinal(), 278);
        assert_eq!(date.ordinal0(), 277);
        assert_eq!(date.julian_day_number(), 2299161);
    }

    #[test]
    fn jdn_2299162_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(1582, Month::October, 6).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 6);
        assert_eq!(date.day_ordinal(), 6);
        assert_eq!(date.day_ordinal0(), 5);
        assert_eq!(date.ordinal(), 279);
        assert_eq!(date.ordinal0(), 278);
        assert_eq!(date.julian_day_number(), 2299162);
    }

    #[test]
    fn jdn_2299162_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(1582, 279).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 6);
        assert_eq!(date.day_ordinal(), 6);
        assert_eq!(date.day_ordinal0(), 5);
        assert_eq!(date.ordinal(), 279);
        assert_eq!(date.ordinal0(), 278);
        assert_eq!(date.julian_day_number(), 2299162);
    }

    #[test]
    fn jdn_2299238_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(1582, Month::December, 21).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 21);
        assert_eq!(date.day_ordinal(), 21);
        assert_eq!(date.day_ordinal0(), 20);
        assert_eq!(date.ordinal(), 355);
        assert_eq!(date.ordinal0(), 354);
        assert_eq!(date.julian_day_number(), 2299238);
    }

    #[test]
    fn jdn_2299238_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(1582, 355).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 21);
        assert_eq!(date.day_ordinal(), 21);
        assert_eq!(date.day_ordinal0(), 20);
        assert_eq!(date.ordinal(), 355);
        assert_eq!(date.ordinal0(), 354);
        assert_eq!(date.julian_day_number(), 2299238);
    }

    #[test]
    fn jdn_2299239_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(1582, Month::December, 22).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 22);
        assert_eq!(date.day_ordinal(), 22);
        assert_eq!(date.day_ordinal0(), 21);
        assert_eq!(date.ordinal(), 356);
        assert_eq!(date.ordinal0(), 355);
        assert_eq!(date.julian_day_number(), 2299239);
    }

    #[test]
    fn jdn_2299239_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(1582, 356).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 22);
        assert_eq!(date.day_ordinal(), 22);
        assert_eq!(date.day_ordinal0(), 21);
        assert_eq!(date.ordinal(), 356);
        assert_eq!(date.ordinal0(), 355);
        assert_eq!(date.julian_day_number(), 2299239);
    }

    #[test]
    fn jdn_2305448_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(1599, Month::December, 22).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1599);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 22);
        assert_eq!(date.day_ordinal(), 22);
        assert_eq!(date.day_ordinal0(), 21);
        assert_eq!(date.ordinal(), 356);
        assert_eq!(date.ordinal0(), 355);
        assert_eq!(date.julian_day_number(), 2305448);
    }

    #[test]
    fn jdn_2305448_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(1599, 356).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1599);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 22);
        assert_eq!(date.day_ordinal(), 22);
        assert_eq!(date.day_ordinal0(), 21);
        assert_eq!(date.ordinal(), 356);
        assert_eq!(date.ordinal0(), 355);
        assert_eq!(date.julian_day_number(), 2305448);
    }

    #[test]
    fn jdn_2305517_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(1600, Month::February, 29).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1600);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 29);
        assert_eq!(date.day_ordinal0(), 28);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 2305517);
    }

    #[test]
    fn jdn_2305517_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(1600, 60).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1600);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 29);
        assert_eq!(date.day_ordinal0(), 28);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 2305517);
    }

    #[test]
    fn jdn_2341973_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(1699, Month::December, 22).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1699);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 22);
        assert_eq!(date.day_ordinal(), 22);
        assert_eq!(date.day_ordinal0(), 21);
        assert_eq!(date.ordinal(), 356);
        assert_eq!(date.ordinal0(), 355);
        assert_eq!(date.julian_day_number(), 2341973);
    }

    #[test]
    fn jdn_2341973_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(1699, 356).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1699);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 22);
        assert_eq!(date.day_ordinal(), 22);
        assert_eq!(date.day_ordinal0(), 21);
        assert_eq!(date.ordinal(), 356);
        assert_eq!(date.ordinal0(), 355);
        assert_eq!(date.julian_day_number(), 2341973);
    }

    #[test]
    fn jdn_2342032_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(1700, Month::February, 19).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 19);
        assert_eq!(date.day_ordinal(), 19);
        assert_eq!(date.day_ordinal0(), 18);
        assert_eq!(date.ordinal(), 50);
        assert_eq!(date.ordinal0(), 49);
        assert_eq!(date.julian_day_number(), 2342032);
    }

    #[test]
    fn jdn_2342032_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(1700, 50).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 19);
        assert_eq!(date.day_ordinal(), 19);
        assert_eq!(date.day_ordinal0(), 18);
        assert_eq!(date.ordinal(), 50);
        assert_eq!(date.ordinal0(), 49);
        assert_eq!(date.julian_day_number(), 2342032);
    }

    #[test]
    fn jdn_2342337_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(1700, Month::December, 20).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 20);
        assert_eq!(date.day_ordinal(), 20);
        assert_eq!(date.day_ordinal0(), 19);
        assert_eq!(date.ordinal(), 355);
        assert_eq!(date.ordinal0(), 354);
        assert_eq!(date.julian_day_number(), 2342337);
    }

    #[test]
    fn jdn_2342337_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(1700, 355).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 20);
        assert_eq!(date.day_ordinal(), 20);
        assert_eq!(date.day_ordinal0(), 19);
        assert_eq!(date.ordinal(), 355);
        assert_eq!(date.ordinal0(), 354);
        assert_eq!(date.julian_day_number(), 2342337);
    }

    #[test]
    fn jdn_2342338_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(1700, Month::December, 21).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 21);
        assert_eq!(date.day_ordinal(), 21);
        assert_eq!(date.day_ordinal0(), 20);
        assert_eq!(date.ordinal(), 356);
        assert_eq!(date.ordinal0(), 355);
        assert_eq!(date.julian_day_number(), 2342338);
    }

    #[test]
    fn jdn_2342338_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(1700, 356).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 21);
        assert_eq!(date.day_ordinal(), 21);
        assert_eq!(date.day_ordinal0(), 20);
        assert_eq!(date.ordinal(), 356);
        assert_eq!(date.ordinal0(), 355);
        assert_eq!(date.julian_day_number(), 2342338);
    }

    #[test]
    fn jdn_2378496_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(1799, Month::December, 20).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1799);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 20);
        assert_eq!(date.day_ordinal(), 20);
        assert_eq!(date.day_ordinal0(), 19);
        assert_eq!(date.ordinal(), 354);
        assert_eq!(date.ordinal0(), 353);
        assert_eq!(date.julian_day_number(), 2378496);
    }

    #[test]
    fn jdn_2378496_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(1799, 354).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1799);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 20);
        assert_eq!(date.day_ordinal(), 20);
        assert_eq!(date.day_ordinal0(), 19);
        assert_eq!(date.ordinal(), 354);
        assert_eq!(date.ordinal0(), 353);
        assert_eq!(date.julian_day_number(), 2378496);
    }

    #[test]
    fn jdn_2378497_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(1799, Month::December, 21).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1799);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 21);
        assert_eq!(date.day_ordinal(), 21);
        assert_eq!(date.day_ordinal0(), 20);
        assert_eq!(date.ordinal(), 355);
        assert_eq!(date.ordinal0(), 354);
        assert_eq!(date.julian_day_number(), 2378497);
    }

    #[test]
    fn jdn_2378497_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(1799, 355).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1799);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 21);
        assert_eq!(date.day_ordinal(), 21);
        assert_eq!(date.day_ordinal0(), 20);
        assert_eq!(date.ordinal(), 355);
        assert_eq!(date.ordinal0(), 354);
        assert_eq!(date.julian_day_number(), 2378497);
    }

    #[test]
    fn jdn_2378861_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(1800, Month::December, 19).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1800);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 19);
        assert_eq!(date.day_ordinal(), 19);
        assert_eq!(date.day_ordinal0(), 18);
        assert_eq!(date.ordinal(), 354);
        assert_eq!(date.ordinal0(), 353);
        assert_eq!(date.julian_day_number(), 2378861);
    }

    #[test]
    fn jdn_2378861_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(1800, 354).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1800);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 19);
        assert_eq!(date.day_ordinal(), 19);
        assert_eq!(date.day_ordinal0(), 18);
        assert_eq!(date.ordinal(), 354);
        assert_eq!(date.ordinal0(), 353);
        assert_eq!(date.julian_day_number(), 2378861);
    }

    #[test]
    fn jdn_2378862_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(1800, Month::December, 20).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1800);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 20);
        assert_eq!(date.day_ordinal(), 20);
        assert_eq!(date.day_ordinal0(), 19);
        assert_eq!(date.ordinal(), 355);
        assert_eq!(date.ordinal0(), 354);
        assert_eq!(date.julian_day_number(), 2378862);
    }

    #[test]
    fn jdn_2378862_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(1800, 355).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1800);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 20);
        assert_eq!(date.day_ordinal(), 20);
        assert_eq!(date.day_ordinal0(), 19);
        assert_eq!(date.ordinal(), 355);
        assert_eq!(date.ordinal0(), 354);
        assert_eq!(date.julian_day_number(), 2378862);
    }

    #[test]
    fn jdn_2415020_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(1899, Month::December, 19).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1899);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 19);
        assert_eq!(date.day_ordinal(), 19);
        assert_eq!(date.day_ordinal0(), 18);
        assert_eq!(date.ordinal(), 353);
        assert_eq!(date.ordinal0(), 352);
        assert_eq!(date.julian_day_number(), 2415020);
    }

    #[test]
    fn jdn_2415020_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(1899, 353).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1899);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 19);
        assert_eq!(date.day_ordinal(), 19);
        assert_eq!(date.day_ordinal0(), 18);
        assert_eq!(date.ordinal(), 353);
        assert_eq!(date.ordinal0(), 352);
        assert_eq!(date.julian_day_number(), 2415020);
    }

    #[test]
    fn jdn_2415021_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(1899, Month::December, 20).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1899);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 20);
        assert_eq!(date.day_ordinal(), 20);
        assert_eq!(date.day_ordinal0(), 19);
        assert_eq!(date.ordinal(), 354);
        assert_eq!(date.ordinal0(), 353);
        assert_eq!(date.julian_day_number(), 2415021);
    }

    #[test]
    fn jdn_2415021_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(1899, 354).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1899);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 20);
        assert_eq!(date.day_ordinal(), 20);
        assert_eq!(date.day_ordinal0(), 19);
        assert_eq!(date.ordinal(), 354);
        assert_eq!(date.ordinal0(), 353);
        assert_eq!(date.julian_day_number(), 2415021);
    }

    #[test]
    fn jdn_2415385_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(1900, Month::December, 18).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1900);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 18);
        assert_eq!(date.day_ordinal(), 18);
        assert_eq!(date.day_ordinal0(), 17);
        assert_eq!(date.ordinal(), 353);
        assert_eq!(date.ordinal0(), 352);
        assert_eq!(date.julian_day_number(), 2415385);
    }

    #[test]
    fn jdn_2415385_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(1900, 353).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1900);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 18);
        assert_eq!(date.day_ordinal(), 18);
        assert_eq!(date.day_ordinal0(), 17);
        assert_eq!(date.ordinal(), 353);
        assert_eq!(date.ordinal0(), 352);
        assert_eq!(date.julian_day_number(), 2415385);
    }

    #[test]
    fn jdn_2415386_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(1900, Month::December, 19).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1900);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 19);
        assert_eq!(date.day_ordinal(), 19);
        assert_eq!(date.day_ordinal0(), 18);
        assert_eq!(date.ordinal(), 354);
        assert_eq!(date.ordinal0(), 353);
        assert_eq!(date.julian_day_number(), 2415386);
    }

    #[test]
    fn jdn_2415386_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(1900, 354).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 1900);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 19);
        assert_eq!(date.day_ordinal(), 19);
        assert_eq!(date.day_ordinal0(), 18);
        assert_eq!(date.ordinal(), 354);
        assert_eq!(date.ordinal0(), 353);
        assert_eq!(date.julian_day_number(), 2415386);
    }

    #[test]
    fn jdn_2451605_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(2000, Month::February, 17).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 2000);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 17);
        assert_eq!(date.day_ordinal(), 17);
        assert_eq!(date.day_ordinal0(), 16);
        assert_eq!(date.ordinal(), 48);
        assert_eq!(date.ordinal0(), 47);
        assert_eq!(date.julian_day_number(), 2451605);
    }

    #[test]
    fn jdn_2451605_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(2000, 48).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 2000);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 17);
        assert_eq!(date.day_ordinal(), 17);
        assert_eq!(date.day_ordinal0(), 16);
        assert_eq!(date.ordinal(), 48);
        assert_eq!(date.ordinal0(), 47);
        assert_eq!(date.julian_day_number(), 2451605);
    }

    #[test]
    fn jdn_2451910_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(2000, Month::December, 18).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 2000);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 18);
        assert_eq!(date.day_ordinal(), 18);
        assert_eq!(date.day_ordinal0(), 17);
        assert_eq!(date.ordinal(), 353);
        assert_eq!(date.ordinal0(), 352);
        assert_eq!(date.julian_day_number(), 2451910);
    }

    #[test]
    fn jdn_2451910_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(2000, 353).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 2000);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 18);
        assert_eq!(date.day_ordinal(), 18);
        assert_eq!(date.day_ordinal0(), 17);
        assert_eq!(date.ordinal(), 353);
        assert_eq!(date.ordinal0(), 352);
        assert_eq!(date.julian_day_number(), 2451910);
    }

    #[test]
    fn jdn_2451911_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(2000, Month::December, 19).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 2000);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 19);
        assert_eq!(date.day_ordinal(), 19);
        assert_eq!(date.day_ordinal0(), 18);
        assert_eq!(date.ordinal(), 354);
        assert_eq!(date.ordinal0(), 353);
        assert_eq!(date.julian_day_number(), 2451911);
    }

    #[test]
    fn jdn_2451911_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(2000, 354).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 2000);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 19);
        assert_eq!(date.day_ordinal(), 19);
        assert_eq!(date.day_ordinal0(), 18);
        assert_eq!(date.ordinal(), 354);
        assert_eq!(date.ordinal0(), 353);
        assert_eq!(date.julian_day_number(), 2451911);
    }

    #[test]
    fn jdn_3145930_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(3901, Month::February, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 3901);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 32);
        assert_eq!(date.ordinal0(), 31);
        assert_eq!(date.julian_day_number(), 3145930);
    }

    #[test]
    fn jdn_3145930_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(3901, 32).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 3901);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 32);
        assert_eq!(date.ordinal0(), 31);
        assert_eq!(date.julian_day_number(), 3145930);
    }

    #[test]
    fn jdn_19508368_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(48699, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 48699);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 19508368);
    }

    #[test]
    fn jdn_19508368_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(48699, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 48699);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 19508368);
    }

    #[test]
    fn jdn_19581784_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(48900, Month::January, 2).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 48900);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 2);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 2);
        assert_eq!(date.ordinal0(), 1);
        assert_eq!(date.julian_day_number(), 19581784);
    }

    #[test]
    fn jdn_19581784_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(48900, 2).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 48900);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 2);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 2);
        assert_eq!(date.ordinal0(), 1);
        assert_eq!(date.julian_day_number(), 19581784);
    }

    #[test]
    fn jdn_19581842_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(48900, Month::February, 29).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 48900);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 29);
        assert_eq!(date.day_ordinal0(), 28);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 19581842);
    }

    #[test]
    fn jdn_19581842_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(48900, 60).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 48900);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 29);
        assert_eq!(date.day_ordinal(), 29);
        assert_eq!(date.day_ordinal0(), 28);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 19581842);
    }

    #[test]
    fn jdn_19581843_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(48900, Month::March, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 48900);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 61);
        assert_eq!(date.ordinal0(), 60);
        assert_eq!(date.julian_day_number(), 19581843);
    }

    #[test]
    fn jdn_19581843_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(48900, 61).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 48900);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 61);
        assert_eq!(date.ordinal0(), 60);
        assert_eq!(date.julian_day_number(), 19581843);
    }

    #[test]
    fn jdn_19582148_ymd() {
        let cal = Calendar::JULIAN;
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
    fn jdn_19582148_ordinal() {
        let cal = Calendar::JULIAN;
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
    fn jdn_19582149_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(48901, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 48901);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 19582149);
    }

    #[test]
    fn jdn_19582149_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(48901, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 48901);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 19582149);
    }

    #[test]
    fn jdn_19617943_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(48999, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 48999);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 19617943);
    }

    #[test]
    fn jdn_19617943_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(48999, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 48999);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 19617943);
    }

    #[test]
    fn jdn_19654466_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(49098, Month::December, 30).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 49098);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 30);
        assert_eq!(date.day_ordinal(), 30);
        assert_eq!(date.day_ordinal0(), 29);
        assert_eq!(date.ordinal(), 364);
        assert_eq!(date.ordinal0(), 363);
        assert_eq!(date.julian_day_number(), 19654466);
    }

    #[test]
    fn jdn_19654466_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(49098, 364).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 49098);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 30);
        assert_eq!(date.day_ordinal(), 30);
        assert_eq!(date.day_ordinal0(), 29);
        assert_eq!(date.ordinal(), 364);
        assert_eq!(date.ordinal0(), 363);
        assert_eq!(date.julian_day_number(), 19654466);
    }

    #[test]
    fn jdn_19654467_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(49098, Month::December, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 49098);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 19654467);
    }

    #[test]
    fn jdn_19654467_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(49098, 365).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 49098);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 19654467);
    }

    #[test]
    fn jdn_19654468_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(49099, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 49099);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 19654468);
    }

    #[test]
    fn jdn_19654468_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(49099, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 49099);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 19654468);
    }

    #[test]
    fn jdn_2147439589_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(5874657, Month::March, 3).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 5874657);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 3);
        assert_eq!(date.day_ordinal(), 3);
        assert_eq!(date.day_ordinal0(), 2);
        assert_eq!(date.ordinal(), 62);
        assert_eq!(date.ordinal0(), 61);
        assert_eq!(date.julian_day_number(), 2147439589);
    }

    #[test]
    fn jdn_2147439589_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(5874657, 62).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 5874657);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 3);
        assert_eq!(date.day_ordinal(), 3);
        assert_eq!(date.day_ordinal0(), 2);
        assert_eq!(date.ordinal(), 62);
        assert_eq!(date.ordinal0(), 61);
        assert_eq!(date.julian_day_number(), 2147439589);
    }

    #[test]
    fn jdn_2147483647_ymd() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ymd(5874777, Month::October, 17).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 5874777);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 17);
        assert_eq!(date.day_ordinal(), 17);
        assert_eq!(date.day_ordinal0(), 16);
        assert_eq!(date.ordinal(), 290);
        assert_eq!(date.ordinal0(), 289);
        assert_eq!(date.julian_day_number(), 2147483647);
    }

    #[test]
    fn jdn_2147483647_ordinal() {
        let cal = Calendar::JULIAN;
        let date = cal.at_ordinal_date(5874777, 290).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(date.is_julian());
        assert!(!date.is_gregorian());
        assert_eq!(date.year(), 5874777);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 17);
        assert_eq!(date.day_ordinal(), 17);
        assert_eq!(date.day_ordinal0(), 16);
        assert_eq!(date.ordinal(), 290);
        assert_eq!(date.ordinal0(), 289);
        assert_eq!(date.julian_day_number(), 2147483647);
    }
}

mod jdn_to_gregorian {
    use crate::{Calendar, Month};

    #[test]
    fn jdn_neg_2147483648() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(-2147483648);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -5884323);
        assert_eq!(date.month(), Month::May);
        assert_eq!(date.day(), 15);
        assert_eq!(date.day_ordinal(), 15);
        assert_eq!(date.day_ordinal0(), 14);
        assert_eq!(date.ordinal(), 135);
        assert_eq!(date.ordinal0(), 134);
        assert_eq!(date.julian_day_number(), -2147483648);
    }

    #[test]
    fn jdn_neg_214725() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(-214725);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -5300);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -214725);
    }

    #[test]
    fn jdn_neg_178201() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(-178201);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -5200);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -178201);
    }

    #[test]
    fn jdn_neg_141676() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(-141676);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -5100);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -141676);
    }

    #[test]
    fn jdn_neg_105152() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(-105152);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -5000);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -105152);
    }

    #[test]
    fn jdn_neg_68628() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(-68628);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4900);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -68628);
    }

    #[test]
    fn jdn_neg_32104() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(-32104);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4800);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -32104);
    }

    #[test]
    fn jdn_neg_31738() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(-31738);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4799);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -31738);
    }

    #[test]
    fn jdn_neg_1402() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(-1402);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4716);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 22);
        assert_eq!(date.day_ordinal(), 22);
        assert_eq!(date.day_ordinal0(), 21);
        assert_eq!(date.ordinal(), 22);
        assert_eq!(date.ordinal0(), 21);
        assert_eq!(date.julian_day_number(), -1402);
    }

    #[test]
    fn jdn_neg_1095() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(-1095);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4716);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 24);
        assert_eq!(date.day_ordinal(), 24);
        assert_eq!(date.day_ordinal0(), 23);
        assert_eq!(date.ordinal(), 329);
        assert_eq!(date.ordinal0(), 328);
        assert_eq!(date.julian_day_number(), -1095);
    }

    #[test]
    fn jdn_neg_730() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(-730);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4715);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 24);
        assert_eq!(date.day_ordinal(), 24);
        assert_eq!(date.day_ordinal0(), 23);
        assert_eq!(date.ordinal(), 328);
        assert_eq!(date.ordinal0(), 327);
        assert_eq!(date.julian_day_number(), -730);
    }

    #[test]
    fn jdn_neg_692() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(-692);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4714);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -692);
    }

    #[test]
    fn jdn_neg_365() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(-365);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4714);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 24);
        assert_eq!(date.day_ordinal(), 24);
        assert_eq!(date.day_ordinal0(), 23);
        assert_eq!(date.ordinal(), 328);
        assert_eq!(date.ordinal0(), 327);
        assert_eq!(date.julian_day_number(), -365);
    }

    #[test]
    fn jdn_neg_328() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(-328);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4714);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), -328);
    }

    #[test]
    fn jdn_neg_327() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(-327);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4713);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -327);
    }

    #[test]
    fn jdn_neg_1() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(-1);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4713);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 23);
        assert_eq!(date.day_ordinal(), 23);
        assert_eq!(date.day_ordinal0(), 22);
        assert_eq!(date.ordinal(), 327);
        assert_eq!(date.ordinal0(), 326);
        assert_eq!(date.julian_day_number(), -1);
    }

    #[test]
    fn jdn_0() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(0);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4713);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 24);
        assert_eq!(date.day_ordinal(), 24);
        assert_eq!(date.day_ordinal0(), 23);
        assert_eq!(date.ordinal(), 328);
        assert_eq!(date.ordinal0(), 327);
        assert_eq!(date.julian_day_number(), 0);
    }

    #[test]
    fn jdn_1() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(1);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4713);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 25);
        assert_eq!(date.day_ordinal(), 25);
        assert_eq!(date.day_ordinal0(), 24);
        assert_eq!(date.ordinal(), 329);
        assert_eq!(date.ordinal0(), 328);
        assert_eq!(date.julian_day_number(), 1);
    }

    #[test]
    fn jdn_37() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(37);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4713);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 37);
    }

    #[test]
    fn jdn_38() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(38);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4712);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 38);
    }

    #[test]
    fn jdn_365() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(365);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4712);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 23);
        assert_eq!(date.day_ordinal(), 23);
        assert_eq!(date.day_ordinal0(), 22);
        assert_eq!(date.ordinal(), 328);
        assert_eq!(date.ordinal0(), 327);
        assert_eq!(date.julian_day_number(), 365);
    }

    #[test]
    fn jdn_366() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(366);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4712);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 24);
        assert_eq!(date.day_ordinal(), 24);
        assert_eq!(date.day_ordinal0(), 23);
        assert_eq!(date.ordinal(), 329);
        assert_eq!(date.ordinal0(), 328);
        assert_eq!(date.julian_day_number(), 366);
    }

    #[test]
    fn jdn_404() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(404);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4711);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 404);
    }

    #[test]
    fn jdn_730() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(730);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4711);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 23);
        assert_eq!(date.day_ordinal(), 23);
        assert_eq!(date.day_ordinal0(), 22);
        assert_eq!(date.ordinal(), 327);
        assert_eq!(date.ordinal0(), 326);
        assert_eq!(date.julian_day_number(), 730);
    }

    #[test]
    fn jdn_731() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(731);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4711);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 24);
        assert_eq!(date.day_ordinal(), 24);
        assert_eq!(date.day_ordinal0(), 23);
        assert_eq!(date.ordinal(), 328);
        assert_eq!(date.ordinal0(), 327);
        assert_eq!(date.julian_day_number(), 731);
    }

    #[test]
    fn jdn_768() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(768);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4711);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 768);
    }

    #[test]
    fn jdn_769() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(769);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4710);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 769);
    }

    #[test]
    fn jdn_1095() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(1095);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4710);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 23);
        assert_eq!(date.day_ordinal(), 23);
        assert_eq!(date.day_ordinal0(), 22);
        assert_eq!(date.ordinal(), 327);
        assert_eq!(date.ordinal0(), 326);
        assert_eq!(date.julian_day_number(), 1095);
    }

    #[test]
    fn jdn_1096() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(1096);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4710);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 24);
        assert_eq!(date.day_ordinal(), 24);
        assert_eq!(date.day_ordinal0(), 23);
        assert_eq!(date.ordinal(), 328);
        assert_eq!(date.ordinal0(), 327);
        assert_eq!(date.julian_day_number(), 1096);
    }

    #[test]
    fn jdn_1133() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(1133);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4710);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 1133);
    }

    #[test]
    fn jdn_1134() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(1134);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4709);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 1134);
    }

    #[test]
    fn jdn_1460() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(1460);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4709);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 23);
        assert_eq!(date.day_ordinal(), 23);
        assert_eq!(date.day_ordinal0(), 22);
        assert_eq!(date.ordinal(), 327);
        assert_eq!(date.ordinal0(), 326);
        assert_eq!(date.julian_day_number(), 1460);
    }

    #[test]
    fn jdn_1461() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(1461);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4709);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 24);
        assert_eq!(date.day_ordinal(), 24);
        assert_eq!(date.day_ordinal0(), 23);
        assert_eq!(date.ordinal(), 328);
        assert_eq!(date.ordinal0(), 327);
        assert_eq!(date.julian_day_number(), 1461);
    }

    #[test]
    fn jdn_1498() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(1498);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4709);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 1498);
    }

    #[test]
    fn jdn_1499() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(1499);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4708);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 1499);
    }

    #[test]
    fn jdn_1826() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(1826);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4708);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 23);
        assert_eq!(date.day_ordinal(), 23);
        assert_eq!(date.day_ordinal0(), 22);
        assert_eq!(date.ordinal(), 328);
        assert_eq!(date.ordinal0(), 327);
        assert_eq!(date.julian_day_number(), 1826);
    }

    #[test]
    fn jdn_1827() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(1827);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4708);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 24);
        assert_eq!(date.day_ordinal(), 24);
        assert_eq!(date.day_ordinal0(), 23);
        assert_eq!(date.ordinal(), 329);
        assert_eq!(date.ordinal0(), 328);
        assert_eq!(date.julian_day_number(), 1827);
    }

    #[test]
    fn jdn_1864() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(1864);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4708);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 366);
        assert_eq!(date.ordinal0(), 365);
        assert_eq!(date.julian_day_number(), 1864);
    }

    #[test]
    fn jdn_1865() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(1865);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4707);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 1865);
    }

    #[test]
    fn jdn_4420() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(4420);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4701);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 4420);
    }

    #[test]
    fn jdn_4421() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(4421);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4700);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 4421);
    }

    #[test]
    fn jdn_4785() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(4785);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4700);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 4785);
    }

    #[test]
    fn jdn_4786() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(4786);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4699);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 4786);
    }

    #[test]
    fn jdn_40945() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(40945);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4600);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 40945);
    }

    #[test]
    fn jdn_77469() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(77469);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4500);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 77469);
    }

    #[test]
    fn jdn_113992() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(113992);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4401);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 113992);
    }

    #[test]
    fn jdn_113993() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(113993);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4400);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 113993);
    }

    #[test]
    fn jdn_150518() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(150518);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4300);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 150518);
    }

    #[test]
    fn jdn_1719656() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(1719656);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 27);
        assert_eq!(date.day_ordinal(), 27);
        assert_eq!(date.day_ordinal0(), 26);
        assert_eq!(date.ordinal(), 58);
        assert_eq!(date.ordinal0(), 57);
        assert_eq!(date.julian_day_number(), 1719656);
    }

    #[test]
    fn jdn_1721424() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(1721424);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 0);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 30);
        assert_eq!(date.day_ordinal(), 30);
        assert_eq!(date.day_ordinal0(), 29);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 1721424);
    }

    #[test]
    fn jdn_1721425() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(1721425);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 0);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 366);
        assert_eq!(date.ordinal0(), 365);
        assert_eq!(date.julian_day_number(), 1721425);
    }

    #[test]
    fn jdn_1721426() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(1721426);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 1721426);
    }

    #[test]
    fn jdn_1794166() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(1794166);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 200);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 27);
        assert_eq!(date.day_ordinal(), 27);
        assert_eq!(date.day_ordinal0(), 26);
        assert_eq!(date.ordinal(), 58);
        assert_eq!(date.ordinal0(), 57);
        assert_eq!(date.julian_day_number(), 1794166);
    }

    #[test]
    fn jdn_1794167() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(1794167);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 200);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 28);
        assert_eq!(date.day_ordinal(), 28);
        assert_eq!(date.day_ordinal0(), 27);
        assert_eq!(date.ordinal(), 59);
        assert_eq!(date.ordinal0(), 58);
        assert_eq!(date.julian_day_number(), 1794167);
    }

    #[test]
    fn jdn_1830691() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(1830691);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 300);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 28);
        assert_eq!(date.day_ordinal(), 28);
        assert_eq!(date.day_ordinal0(), 27);
        assert_eq!(date.ordinal(), 59);
        assert_eq!(date.ordinal0(), 58);
        assert_eq!(date.julian_day_number(), 1830691);
    }

    #[test]
    fn jdn_1830692() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(1830692);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 300);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 1830692);
    }

    #[test]
    fn jdn_1830997() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(1830997);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 300);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 1830997);
    }

    #[test]
    fn jdn_1830998() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(1830998);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 301);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 1830998);
    }

    #[test]
    fn jdn_1830999() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(1830999);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 301);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 2);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 2);
        assert_eq!(date.ordinal0(), 1);
        assert_eq!(date.julian_day_number(), 1830999);
    }

    #[test]
    fn jdn_2159358() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(2159358);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1200);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 8);
        assert_eq!(date.ordinal0(), 7);
        assert_eq!(date.julian_day_number(), 2159358);
    }

    #[test]
    fn jdn_2195883() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(2195883);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1300);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 8);
        assert_eq!(date.ordinal0(), 7);
        assert_eq!(date.julian_day_number(), 2195883);
    }

    #[test]
    fn jdn_2232408() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(2232408);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1400);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 9);
        assert_eq!(date.day_ordinal(), 9);
        assert_eq!(date.day_ordinal0(), 8);
        assert_eq!(date.ordinal(), 9);
        assert_eq!(date.ordinal0(), 8);
        assert_eq!(date.julian_day_number(), 2232408);
    }

    #[test]
    fn jdn_2268933() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(2268933);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1500);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 10);
        assert_eq!(date.day_ordinal(), 10);
        assert_eq!(date.day_ordinal0(), 9);
        assert_eq!(date.ordinal(), 10);
        assert_eq!(date.ordinal0(), 9);
        assert_eq!(date.julian_day_number(), 2268933);
    }

    #[test]
    fn jdn_2269298() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(2269298);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1501);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 10);
        assert_eq!(date.day_ordinal(), 10);
        assert_eq!(date.day_ordinal0(), 9);
        assert_eq!(date.ordinal(), 10);
        assert_eq!(date.ordinal0(), 9);
        assert_eq!(date.julian_day_number(), 2269298);
    }

    #[test]
    fn jdn_2298883() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(2298883);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 10);
        assert_eq!(date.day_ordinal(), 10);
        assert_eq!(date.day_ordinal0(), 9);
        assert_eq!(date.ordinal(), 10);
        assert_eq!(date.ordinal0(), 9);
        assert_eq!(date.julian_day_number(), 2298883);
    }

    #[test]
    fn jdn_2298884() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(2298884);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 11);
        assert_eq!(date.day_ordinal(), 11);
        assert_eq!(date.day_ordinal0(), 10);
        assert_eq!(date.ordinal(), 11);
        assert_eq!(date.ordinal0(), 10);
        assert_eq!(date.julian_day_number(), 2298884);
    }

    #[test]
    fn jdn_2299160() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(2299160);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 14);
        assert_eq!(date.day_ordinal(), 14);
        assert_eq!(date.day_ordinal0(), 13);
        assert_eq!(date.ordinal(), 287);
        assert_eq!(date.ordinal0(), 286);
        assert_eq!(date.julian_day_number(), 2299160);
    }

    #[test]
    fn jdn_2299161() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(2299161);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 15);
        assert_eq!(date.day_ordinal(), 15);
        assert_eq!(date.day_ordinal0(), 14);
        assert_eq!(date.ordinal(), 288);
        assert_eq!(date.ordinal0(), 287);
        assert_eq!(date.julian_day_number(), 2299161);
    }

    #[test]
    fn jdn_2299162() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(2299162);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 16);
        assert_eq!(date.day_ordinal(), 16);
        assert_eq!(date.day_ordinal0(), 15);
        assert_eq!(date.ordinal(), 289);
        assert_eq!(date.ordinal0(), 288);
        assert_eq!(date.julian_day_number(), 2299162);
    }

    #[test]
    fn jdn_2299238() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(2299238);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 2299238);
    }

    #[test]
    fn jdn_2299239() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(2299239);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1583);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2299239);
    }

    #[test]
    fn jdn_2305448() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(2305448);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1600);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2305448);
    }

    #[test]
    fn jdn_2305517() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(2305517);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1600);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 10);
        assert_eq!(date.day_ordinal(), 10);
        assert_eq!(date.day_ordinal0(), 9);
        assert_eq!(date.ordinal(), 70);
        assert_eq!(date.ordinal0(), 69);
        assert_eq!(date.julian_day_number(), 2305517);
    }

    #[test]
    fn jdn_2341973() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(2341973);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2341973);
    }

    #[test]
    fn jdn_2342032() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(2342032);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 2342032);
    }

    #[test]
    fn jdn_2342337() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(2342337);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 2342337);
    }

    #[test]
    fn jdn_2342338() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(2342338);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1701);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2342338);
    }

    #[test]
    fn jdn_2378496() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(2378496);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1799);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 2378496);
    }

    #[test]
    fn jdn_2378497() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(2378497);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1800);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2378497);
    }

    #[test]
    fn jdn_2378861() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(2378861);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1800);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 2378861);
    }

    #[test]
    fn jdn_2378862() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(2378862);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1801);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2378862);
    }

    #[test]
    fn jdn_2415020() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(2415020);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1899);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 2415020);
    }

    #[test]
    fn jdn_2415021() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(2415021);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1900);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2415021);
    }

    #[test]
    fn jdn_2415385() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(2415385);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1900);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 2415385);
    }

    #[test]
    fn jdn_2415386() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(2415386);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1901);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2415386);
    }

    #[test]
    fn jdn_2451605() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(2451605);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 2000);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 61);
        assert_eq!(date.ordinal0(), 60);
        assert_eq!(date.julian_day_number(), 2451605);
    }

    #[test]
    fn jdn_2451910() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(2451910);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 2000);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 366);
        assert_eq!(date.ordinal0(), 365);
        assert_eq!(date.julian_day_number(), 2451910);
    }

    #[test]
    fn jdn_2451911() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(2451911);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 2001);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2451911);
    }

    #[test]
    fn jdn_3145930() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(3145930);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 3901);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 3145930);
    }

    #[test]
    fn jdn_19508368() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(19508368);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 48699);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 30);
        assert_eq!(date.day_ordinal(), 30);
        assert_eq!(date.day_ordinal0(), 29);
        assert_eq!(date.ordinal(), 364);
        assert_eq!(date.ordinal0(), 363);
        assert_eq!(date.julian_day_number(), 19508368);
    }

    #[test]
    fn jdn_19581784() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(19581784);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 48901);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 19581784);
    }

    #[test]
    fn jdn_19581842() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(19581842);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 48901);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 28);
        assert_eq!(date.day_ordinal(), 28);
        assert_eq!(date.day_ordinal0(), 27);
        assert_eq!(date.ordinal(), 59);
        assert_eq!(date.ordinal0(), 58);
        assert_eq!(date.julian_day_number(), 19581842);
    }

    #[test]
    fn jdn_19581843() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(19581843);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 48901);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 19581843);
    }

    #[test]
    fn jdn_19582148() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(19582148);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 48901);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 19582148);
    }

    #[test]
    fn jdn_19582149() {
        let cal = Calendar::GREGORIAN;
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
    fn jdn_19617943() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(19617943);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 49000);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 19617943);
    }

    #[test]
    fn jdn_19654466() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(19654466);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 49099);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 19654466);
    }

    #[test]
    fn jdn_19654467() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(19654467);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 49100);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 19654467);
    }

    #[test]
    fn jdn_19654468() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(19654468);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 49100);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 2);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 2);
        assert_eq!(date.ordinal0(), 1);
        assert_eq!(date.julian_day_number(), 19654468);
    }

    #[test]
    fn jdn_2147439589() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(2147439589);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 5874777);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 18);
        assert_eq!(date.day_ordinal(), 18);
        assert_eq!(date.day_ordinal0(), 17);
        assert_eq!(date.ordinal(), 291);
        assert_eq!(date.ordinal0(), 290);
        assert_eq!(date.julian_day_number(), 2147439589);
    }

    #[test]
    fn jdn_2147483647() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_jdn(2147483647);
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 5874898);
        assert_eq!(date.month(), Month::June);
        assert_eq!(date.day(), 3);
        assert_eq!(date.day_ordinal(), 3);
        assert_eq!(date.day_ordinal0(), 2);
        assert_eq!(date.ordinal(), 154);
        assert_eq!(date.ordinal0(), 153);
        assert_eq!(date.julian_day_number(), 2147483647);
    }
}

mod gregorian_to_jdn {
    use crate::{Calendar, Month};

    #[test]
    fn jdn_neg_2147483648_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-5884323, Month::May, 15).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -5884323);
        assert_eq!(date.month(), Month::May);
        assert_eq!(date.day(), 15);
        assert_eq!(date.day_ordinal(), 15);
        assert_eq!(date.day_ordinal0(), 14);
        assert_eq!(date.ordinal(), 135);
        assert_eq!(date.ordinal0(), 134);
        assert_eq!(date.julian_day_number(), -2147483648);
    }

    #[test]
    fn jdn_neg_2147483648_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-5884323, 135).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -5884323);
        assert_eq!(date.month(), Month::May);
        assert_eq!(date.day(), 15);
        assert_eq!(date.day_ordinal(), 15);
        assert_eq!(date.day_ordinal0(), 14);
        assert_eq!(date.ordinal(), 135);
        assert_eq!(date.ordinal0(), 134);
        assert_eq!(date.julian_day_number(), -2147483648);
    }

    #[test]
    fn jdn_neg_214725_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-5300, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -5300);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -214725);
    }

    #[test]
    fn jdn_neg_214725_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-5300, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -5300);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -214725);
    }

    #[test]
    fn jdn_neg_178201_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-5200, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -5200);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -178201);
    }

    #[test]
    fn jdn_neg_178201_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-5200, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -5200);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -178201);
    }

    #[test]
    fn jdn_neg_141676_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-5100, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -5100);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -141676);
    }

    #[test]
    fn jdn_neg_141676_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-5100, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -5100);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -141676);
    }

    #[test]
    fn jdn_neg_105152_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-5000, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -5000);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -105152);
    }

    #[test]
    fn jdn_neg_105152_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-5000, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -5000);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -105152);
    }

    #[test]
    fn jdn_neg_68628_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4900, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4900);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -68628);
    }

    #[test]
    fn jdn_neg_68628_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4900, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4900);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -68628);
    }

    #[test]
    fn jdn_neg_32104_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4800, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4800);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -32104);
    }

    #[test]
    fn jdn_neg_32104_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4800, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4800);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -32104);
    }

    #[test]
    fn jdn_neg_31738_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4799, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4799);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -31738);
    }

    #[test]
    fn jdn_neg_31738_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4799, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4799);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -31738);
    }

    #[test]
    fn jdn_neg_1402_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4716, Month::January, 22).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4716);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 22);
        assert_eq!(date.day_ordinal(), 22);
        assert_eq!(date.day_ordinal0(), 21);
        assert_eq!(date.ordinal(), 22);
        assert_eq!(date.ordinal0(), 21);
        assert_eq!(date.julian_day_number(), -1402);
    }

    #[test]
    fn jdn_neg_1402_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4716, 22).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4716);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 22);
        assert_eq!(date.day_ordinal(), 22);
        assert_eq!(date.day_ordinal0(), 21);
        assert_eq!(date.ordinal(), 22);
        assert_eq!(date.ordinal0(), 21);
        assert_eq!(date.julian_day_number(), -1402);
    }

    #[test]
    fn jdn_neg_1095_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4716, Month::November, 24).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4716);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 24);
        assert_eq!(date.day_ordinal(), 24);
        assert_eq!(date.day_ordinal0(), 23);
        assert_eq!(date.ordinal(), 329);
        assert_eq!(date.ordinal0(), 328);
        assert_eq!(date.julian_day_number(), -1095);
    }

    #[test]
    fn jdn_neg_1095_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4716, 329).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4716);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 24);
        assert_eq!(date.day_ordinal(), 24);
        assert_eq!(date.day_ordinal0(), 23);
        assert_eq!(date.ordinal(), 329);
        assert_eq!(date.ordinal0(), 328);
        assert_eq!(date.julian_day_number(), -1095);
    }

    #[test]
    fn jdn_neg_730_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4715, Month::November, 24).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4715);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 24);
        assert_eq!(date.day_ordinal(), 24);
        assert_eq!(date.day_ordinal0(), 23);
        assert_eq!(date.ordinal(), 328);
        assert_eq!(date.ordinal0(), 327);
        assert_eq!(date.julian_day_number(), -730);
    }

    #[test]
    fn jdn_neg_730_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4715, 328).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4715);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 24);
        assert_eq!(date.day_ordinal(), 24);
        assert_eq!(date.day_ordinal0(), 23);
        assert_eq!(date.ordinal(), 328);
        assert_eq!(date.ordinal0(), 327);
        assert_eq!(date.julian_day_number(), -730);
    }

    #[test]
    fn jdn_neg_692_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4714, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4714);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -692);
    }

    #[test]
    fn jdn_neg_692_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4714, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4714);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -692);
    }

    #[test]
    fn jdn_neg_365_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4714, Month::November, 24).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4714);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 24);
        assert_eq!(date.day_ordinal(), 24);
        assert_eq!(date.day_ordinal0(), 23);
        assert_eq!(date.ordinal(), 328);
        assert_eq!(date.ordinal0(), 327);
        assert_eq!(date.julian_day_number(), -365);
    }

    #[test]
    fn jdn_neg_365_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4714, 328).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4714);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 24);
        assert_eq!(date.day_ordinal(), 24);
        assert_eq!(date.day_ordinal0(), 23);
        assert_eq!(date.ordinal(), 328);
        assert_eq!(date.ordinal0(), 327);
        assert_eq!(date.julian_day_number(), -365);
    }

    #[test]
    fn jdn_neg_328_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4714, Month::December, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4714);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), -328);
    }

    #[test]
    fn jdn_neg_328_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4714, 365).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4714);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), -328);
    }

    #[test]
    fn jdn_neg_327_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4713, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4713);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -327);
    }

    #[test]
    fn jdn_neg_327_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4713, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4713);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), -327);
    }

    #[test]
    fn jdn_neg_1_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4713, Month::November, 23).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4713);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 23);
        assert_eq!(date.day_ordinal(), 23);
        assert_eq!(date.day_ordinal0(), 22);
        assert_eq!(date.ordinal(), 327);
        assert_eq!(date.ordinal0(), 326);
        assert_eq!(date.julian_day_number(), -1);
    }

    #[test]
    fn jdn_neg_1_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4713, 327).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4713);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 23);
        assert_eq!(date.day_ordinal(), 23);
        assert_eq!(date.day_ordinal0(), 22);
        assert_eq!(date.ordinal(), 327);
        assert_eq!(date.ordinal0(), 326);
        assert_eq!(date.julian_day_number(), -1);
    }

    #[test]
    fn jdn_0_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4713, Month::November, 24).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4713);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 24);
        assert_eq!(date.day_ordinal(), 24);
        assert_eq!(date.day_ordinal0(), 23);
        assert_eq!(date.ordinal(), 328);
        assert_eq!(date.ordinal0(), 327);
        assert_eq!(date.julian_day_number(), 0);
    }

    #[test]
    fn jdn_0_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4713, 328).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4713);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 24);
        assert_eq!(date.day_ordinal(), 24);
        assert_eq!(date.day_ordinal0(), 23);
        assert_eq!(date.ordinal(), 328);
        assert_eq!(date.ordinal0(), 327);
        assert_eq!(date.julian_day_number(), 0);
    }

    #[test]
    fn jdn_1_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4713, Month::November, 25).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4713);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 25);
        assert_eq!(date.day_ordinal(), 25);
        assert_eq!(date.day_ordinal0(), 24);
        assert_eq!(date.ordinal(), 329);
        assert_eq!(date.ordinal0(), 328);
        assert_eq!(date.julian_day_number(), 1);
    }

    #[test]
    fn jdn_1_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4713, 329).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4713);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 25);
        assert_eq!(date.day_ordinal(), 25);
        assert_eq!(date.day_ordinal0(), 24);
        assert_eq!(date.ordinal(), 329);
        assert_eq!(date.ordinal0(), 328);
        assert_eq!(date.julian_day_number(), 1);
    }

    #[test]
    fn jdn_37_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4713, Month::December, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4713);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 37);
    }

    #[test]
    fn jdn_37_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4713, 365).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4713);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 37);
    }

    #[test]
    fn jdn_38_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4712, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4712);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 38);
    }

    #[test]
    fn jdn_38_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4712, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4712);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 38);
    }

    #[test]
    fn jdn_365_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4712, Month::November, 23).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4712);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 23);
        assert_eq!(date.day_ordinal(), 23);
        assert_eq!(date.day_ordinal0(), 22);
        assert_eq!(date.ordinal(), 328);
        assert_eq!(date.ordinal0(), 327);
        assert_eq!(date.julian_day_number(), 365);
    }

    #[test]
    fn jdn_365_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4712, 328).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4712);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 23);
        assert_eq!(date.day_ordinal(), 23);
        assert_eq!(date.day_ordinal0(), 22);
        assert_eq!(date.ordinal(), 328);
        assert_eq!(date.ordinal0(), 327);
        assert_eq!(date.julian_day_number(), 365);
    }

    #[test]
    fn jdn_366_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4712, Month::November, 24).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4712);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 24);
        assert_eq!(date.day_ordinal(), 24);
        assert_eq!(date.day_ordinal0(), 23);
        assert_eq!(date.ordinal(), 329);
        assert_eq!(date.ordinal0(), 328);
        assert_eq!(date.julian_day_number(), 366);
    }

    #[test]
    fn jdn_366_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4712, 329).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4712);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 24);
        assert_eq!(date.day_ordinal(), 24);
        assert_eq!(date.day_ordinal0(), 23);
        assert_eq!(date.ordinal(), 329);
        assert_eq!(date.ordinal0(), 328);
        assert_eq!(date.julian_day_number(), 366);
    }

    #[test]
    fn jdn_404_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4711, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4711);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 404);
    }

    #[test]
    fn jdn_404_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4711, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4711);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 404);
    }

    #[test]
    fn jdn_730_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4711, Month::November, 23).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4711);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 23);
        assert_eq!(date.day_ordinal(), 23);
        assert_eq!(date.day_ordinal0(), 22);
        assert_eq!(date.ordinal(), 327);
        assert_eq!(date.ordinal0(), 326);
        assert_eq!(date.julian_day_number(), 730);
    }

    #[test]
    fn jdn_730_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4711, 327).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4711);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 23);
        assert_eq!(date.day_ordinal(), 23);
        assert_eq!(date.day_ordinal0(), 22);
        assert_eq!(date.ordinal(), 327);
        assert_eq!(date.ordinal0(), 326);
        assert_eq!(date.julian_day_number(), 730);
    }

    #[test]
    fn jdn_731_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4711, Month::November, 24).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4711);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 24);
        assert_eq!(date.day_ordinal(), 24);
        assert_eq!(date.day_ordinal0(), 23);
        assert_eq!(date.ordinal(), 328);
        assert_eq!(date.ordinal0(), 327);
        assert_eq!(date.julian_day_number(), 731);
    }

    #[test]
    fn jdn_731_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4711, 328).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4711);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 24);
        assert_eq!(date.day_ordinal(), 24);
        assert_eq!(date.day_ordinal0(), 23);
        assert_eq!(date.ordinal(), 328);
        assert_eq!(date.ordinal0(), 327);
        assert_eq!(date.julian_day_number(), 731);
    }

    #[test]
    fn jdn_768_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4711, Month::December, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4711);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 768);
    }

    #[test]
    fn jdn_768_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4711, 365).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4711);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 768);
    }

    #[test]
    fn jdn_769_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4710, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4710);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 769);
    }

    #[test]
    fn jdn_769_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4710, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4710);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 769);
    }

    #[test]
    fn jdn_1095_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4710, Month::November, 23).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4710);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 23);
        assert_eq!(date.day_ordinal(), 23);
        assert_eq!(date.day_ordinal0(), 22);
        assert_eq!(date.ordinal(), 327);
        assert_eq!(date.ordinal0(), 326);
        assert_eq!(date.julian_day_number(), 1095);
    }

    #[test]
    fn jdn_1095_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4710, 327).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4710);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 23);
        assert_eq!(date.day_ordinal(), 23);
        assert_eq!(date.day_ordinal0(), 22);
        assert_eq!(date.ordinal(), 327);
        assert_eq!(date.ordinal0(), 326);
        assert_eq!(date.julian_day_number(), 1095);
    }

    #[test]
    fn jdn_1096_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4710, Month::November, 24).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4710);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 24);
        assert_eq!(date.day_ordinal(), 24);
        assert_eq!(date.day_ordinal0(), 23);
        assert_eq!(date.ordinal(), 328);
        assert_eq!(date.ordinal0(), 327);
        assert_eq!(date.julian_day_number(), 1096);
    }

    #[test]
    fn jdn_1096_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4710, 328).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4710);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 24);
        assert_eq!(date.day_ordinal(), 24);
        assert_eq!(date.day_ordinal0(), 23);
        assert_eq!(date.ordinal(), 328);
        assert_eq!(date.ordinal0(), 327);
        assert_eq!(date.julian_day_number(), 1096);
    }

    #[test]
    fn jdn_1133_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4710, Month::December, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4710);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 1133);
    }

    #[test]
    fn jdn_1133_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4710, 365).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4710);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 1133);
    }

    #[test]
    fn jdn_1134_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4709, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4709);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 1134);
    }

    #[test]
    fn jdn_1134_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4709, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4709);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 1134);
    }

    #[test]
    fn jdn_1460_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4709, Month::November, 23).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4709);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 23);
        assert_eq!(date.day_ordinal(), 23);
        assert_eq!(date.day_ordinal0(), 22);
        assert_eq!(date.ordinal(), 327);
        assert_eq!(date.ordinal0(), 326);
        assert_eq!(date.julian_day_number(), 1460);
    }

    #[test]
    fn jdn_1460_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4709, 327).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4709);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 23);
        assert_eq!(date.day_ordinal(), 23);
        assert_eq!(date.day_ordinal0(), 22);
        assert_eq!(date.ordinal(), 327);
        assert_eq!(date.ordinal0(), 326);
        assert_eq!(date.julian_day_number(), 1460);
    }

    #[test]
    fn jdn_1461_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4709, Month::November, 24).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4709);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 24);
        assert_eq!(date.day_ordinal(), 24);
        assert_eq!(date.day_ordinal0(), 23);
        assert_eq!(date.ordinal(), 328);
        assert_eq!(date.ordinal0(), 327);
        assert_eq!(date.julian_day_number(), 1461);
    }

    #[test]
    fn jdn_1461_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4709, 328).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4709);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 24);
        assert_eq!(date.day_ordinal(), 24);
        assert_eq!(date.day_ordinal0(), 23);
        assert_eq!(date.ordinal(), 328);
        assert_eq!(date.ordinal0(), 327);
        assert_eq!(date.julian_day_number(), 1461);
    }

    #[test]
    fn jdn_1498_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4709, Month::December, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4709);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 1498);
    }

    #[test]
    fn jdn_1498_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4709, 365).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4709);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 1498);
    }

    #[test]
    fn jdn_1499_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4708, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4708);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 1499);
    }

    #[test]
    fn jdn_1499_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4708, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4708);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 1499);
    }

    #[test]
    fn jdn_1826_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4708, Month::November, 23).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4708);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 23);
        assert_eq!(date.day_ordinal(), 23);
        assert_eq!(date.day_ordinal0(), 22);
        assert_eq!(date.ordinal(), 328);
        assert_eq!(date.ordinal0(), 327);
        assert_eq!(date.julian_day_number(), 1826);
    }

    #[test]
    fn jdn_1826_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4708, 328).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4708);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 23);
        assert_eq!(date.day_ordinal(), 23);
        assert_eq!(date.day_ordinal0(), 22);
        assert_eq!(date.ordinal(), 328);
        assert_eq!(date.ordinal0(), 327);
        assert_eq!(date.julian_day_number(), 1826);
    }

    #[test]
    fn jdn_1827_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4708, Month::November, 24).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4708);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 24);
        assert_eq!(date.day_ordinal(), 24);
        assert_eq!(date.day_ordinal0(), 23);
        assert_eq!(date.ordinal(), 329);
        assert_eq!(date.ordinal0(), 328);
        assert_eq!(date.julian_day_number(), 1827);
    }

    #[test]
    fn jdn_1827_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4708, 329).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4708);
        assert_eq!(date.month(), Month::November);
        assert_eq!(date.day(), 24);
        assert_eq!(date.day_ordinal(), 24);
        assert_eq!(date.day_ordinal0(), 23);
        assert_eq!(date.ordinal(), 329);
        assert_eq!(date.ordinal0(), 328);
        assert_eq!(date.julian_day_number(), 1827);
    }

    #[test]
    fn jdn_1864_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4708, Month::December, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4708);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 366);
        assert_eq!(date.ordinal0(), 365);
        assert_eq!(date.julian_day_number(), 1864);
    }

    #[test]
    fn jdn_1864_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4708, 366).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4708);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 366);
        assert_eq!(date.ordinal0(), 365);
        assert_eq!(date.julian_day_number(), 1864);
    }

    #[test]
    fn jdn_1865_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4707, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4707);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 1865);
    }

    #[test]
    fn jdn_1865_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4707, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4707);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 1865);
    }

    #[test]
    fn jdn_4420_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4701, Month::December, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4701);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 4420);
    }

    #[test]
    fn jdn_4420_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4701, 365).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4701);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 4420);
    }

    #[test]
    fn jdn_4421_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4700, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4700);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 4421);
    }

    #[test]
    fn jdn_4421_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4700, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4700);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 4421);
    }

    #[test]
    fn jdn_4785_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4700, Month::December, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4700);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 4785);
    }

    #[test]
    fn jdn_4785_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4700, 365).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4700);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 4785);
    }

    #[test]
    fn jdn_4786_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4699, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4699);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 4786);
    }

    #[test]
    fn jdn_4786_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4699, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4699);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 4786);
    }

    #[test]
    fn jdn_40945_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4600, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4600);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 40945);
    }

    #[test]
    fn jdn_40945_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4600, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4600);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 40945);
    }

    #[test]
    fn jdn_77469_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4500, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4500);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 77469);
    }

    #[test]
    fn jdn_77469_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4500, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4500);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 77469);
    }

    #[test]
    fn jdn_113992_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4401, Month::December, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4401);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 113992);
    }

    #[test]
    fn jdn_113992_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4401, 365).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4401);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 113992);
    }

    #[test]
    fn jdn_113993_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4400, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4400);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 113993);
    }

    #[test]
    fn jdn_113993_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4400, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4400);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 113993);
    }

    #[test]
    fn jdn_150518_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4300, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4300);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 150518);
    }

    #[test]
    fn jdn_150518_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4300, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4300);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 150518);
    }

    #[test]
    fn jdn_1719656_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(-4, Month::February, 27).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 27);
        assert_eq!(date.day_ordinal(), 27);
        assert_eq!(date.day_ordinal0(), 26);
        assert_eq!(date.ordinal(), 58);
        assert_eq!(date.ordinal0(), 57);
        assert_eq!(date.julian_day_number(), 1719656);
    }

    #[test]
    fn jdn_1719656_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(-4, 58).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), -4);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 27);
        assert_eq!(date.day_ordinal(), 27);
        assert_eq!(date.day_ordinal0(), 26);
        assert_eq!(date.ordinal(), 58);
        assert_eq!(date.ordinal0(), 57);
        assert_eq!(date.julian_day_number(), 1719656);
    }

    #[test]
    fn jdn_1721424_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(0, Month::December, 30).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 0);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 30);
        assert_eq!(date.day_ordinal(), 30);
        assert_eq!(date.day_ordinal0(), 29);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 1721424);
    }

    #[test]
    fn jdn_1721424_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(0, 365).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 0);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 30);
        assert_eq!(date.day_ordinal(), 30);
        assert_eq!(date.day_ordinal0(), 29);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 1721424);
    }

    #[test]
    fn jdn_1721425_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(0, Month::December, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 0);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 366);
        assert_eq!(date.ordinal0(), 365);
        assert_eq!(date.julian_day_number(), 1721425);
    }

    #[test]
    fn jdn_1721425_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(0, 366).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 0);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 366);
        assert_eq!(date.ordinal0(), 365);
        assert_eq!(date.julian_day_number(), 1721425);
    }

    #[test]
    fn jdn_1721426_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(1, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 1721426);
    }

    #[test]
    fn jdn_1721426_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(1, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 1721426);
    }

    #[test]
    fn jdn_1794166_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(200, Month::February, 27).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 200);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 27);
        assert_eq!(date.day_ordinal(), 27);
        assert_eq!(date.day_ordinal0(), 26);
        assert_eq!(date.ordinal(), 58);
        assert_eq!(date.ordinal0(), 57);
        assert_eq!(date.julian_day_number(), 1794166);
    }

    #[test]
    fn jdn_1794166_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(200, 58).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 200);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 27);
        assert_eq!(date.day_ordinal(), 27);
        assert_eq!(date.day_ordinal0(), 26);
        assert_eq!(date.ordinal(), 58);
        assert_eq!(date.ordinal0(), 57);
        assert_eq!(date.julian_day_number(), 1794166);
    }

    #[test]
    fn jdn_1794167_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(200, Month::February, 28).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 200);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 28);
        assert_eq!(date.day_ordinal(), 28);
        assert_eq!(date.day_ordinal0(), 27);
        assert_eq!(date.ordinal(), 59);
        assert_eq!(date.ordinal0(), 58);
        assert_eq!(date.julian_day_number(), 1794167);
    }

    #[test]
    fn jdn_1794167_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(200, 59).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 200);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 28);
        assert_eq!(date.day_ordinal(), 28);
        assert_eq!(date.day_ordinal0(), 27);
        assert_eq!(date.ordinal(), 59);
        assert_eq!(date.ordinal0(), 58);
        assert_eq!(date.julian_day_number(), 1794167);
    }

    #[test]
    fn jdn_1830691_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(300, Month::February, 28).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 300);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 28);
        assert_eq!(date.day_ordinal(), 28);
        assert_eq!(date.day_ordinal0(), 27);
        assert_eq!(date.ordinal(), 59);
        assert_eq!(date.ordinal0(), 58);
        assert_eq!(date.julian_day_number(), 1830691);
    }

    #[test]
    fn jdn_1830691_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(300, 59).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 300);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 28);
        assert_eq!(date.day_ordinal(), 28);
        assert_eq!(date.day_ordinal0(), 27);
        assert_eq!(date.ordinal(), 59);
        assert_eq!(date.ordinal0(), 58);
        assert_eq!(date.julian_day_number(), 1830691);
    }

    #[test]
    fn jdn_1830692_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(300, Month::March, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 300);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 1830692);
    }

    #[test]
    fn jdn_1830692_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(300, 60).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 300);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 1830692);
    }

    #[test]
    fn jdn_1830997_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(300, Month::December, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 300);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 1830997);
    }

    #[test]
    fn jdn_1830997_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(300, 365).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 300);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 1830997);
    }

    #[test]
    fn jdn_1830998_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(301, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 301);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 1830998);
    }

    #[test]
    fn jdn_1830998_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(301, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 301);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 1830998);
    }

    #[test]
    fn jdn_1830999_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(301, Month::January, 2).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 301);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 2);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 2);
        assert_eq!(date.ordinal0(), 1);
        assert_eq!(date.julian_day_number(), 1830999);
    }

    #[test]
    fn jdn_1830999_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(301, 2).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 301);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 2);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 2);
        assert_eq!(date.ordinal0(), 1);
        assert_eq!(date.julian_day_number(), 1830999);
    }

    #[test]
    fn jdn_2159358_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(1200, Month::January, 8).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1200);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 8);
        assert_eq!(date.ordinal0(), 7);
        assert_eq!(date.julian_day_number(), 2159358);
    }

    #[test]
    fn jdn_2159358_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(1200, 8).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1200);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 8);
        assert_eq!(date.ordinal0(), 7);
        assert_eq!(date.julian_day_number(), 2159358);
    }

    #[test]
    fn jdn_2195883_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(1300, Month::January, 8).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1300);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 8);
        assert_eq!(date.ordinal0(), 7);
        assert_eq!(date.julian_day_number(), 2195883);
    }

    #[test]
    fn jdn_2195883_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(1300, 8).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1300);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 8);
        assert_eq!(date.day_ordinal(), 8);
        assert_eq!(date.day_ordinal0(), 7);
        assert_eq!(date.ordinal(), 8);
        assert_eq!(date.ordinal0(), 7);
        assert_eq!(date.julian_day_number(), 2195883);
    }

    #[test]
    fn jdn_2232408_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(1400, Month::January, 9).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1400);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 9);
        assert_eq!(date.day_ordinal(), 9);
        assert_eq!(date.day_ordinal0(), 8);
        assert_eq!(date.ordinal(), 9);
        assert_eq!(date.ordinal0(), 8);
        assert_eq!(date.julian_day_number(), 2232408);
    }

    #[test]
    fn jdn_2232408_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(1400, 9).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1400);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 9);
        assert_eq!(date.day_ordinal(), 9);
        assert_eq!(date.day_ordinal0(), 8);
        assert_eq!(date.ordinal(), 9);
        assert_eq!(date.ordinal0(), 8);
        assert_eq!(date.julian_day_number(), 2232408);
    }

    #[test]
    fn jdn_2268933_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(1500, Month::January, 10).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1500);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 10);
        assert_eq!(date.day_ordinal(), 10);
        assert_eq!(date.day_ordinal0(), 9);
        assert_eq!(date.ordinal(), 10);
        assert_eq!(date.ordinal0(), 9);
        assert_eq!(date.julian_day_number(), 2268933);
    }

    #[test]
    fn jdn_2268933_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(1500, 10).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1500);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 10);
        assert_eq!(date.day_ordinal(), 10);
        assert_eq!(date.day_ordinal0(), 9);
        assert_eq!(date.ordinal(), 10);
        assert_eq!(date.ordinal0(), 9);
        assert_eq!(date.julian_day_number(), 2268933);
    }

    #[test]
    fn jdn_2269298_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(1501, Month::January, 10).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1501);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 10);
        assert_eq!(date.day_ordinal(), 10);
        assert_eq!(date.day_ordinal0(), 9);
        assert_eq!(date.ordinal(), 10);
        assert_eq!(date.ordinal0(), 9);
        assert_eq!(date.julian_day_number(), 2269298);
    }

    #[test]
    fn jdn_2269298_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(1501, 10).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1501);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 10);
        assert_eq!(date.day_ordinal(), 10);
        assert_eq!(date.day_ordinal0(), 9);
        assert_eq!(date.ordinal(), 10);
        assert_eq!(date.ordinal0(), 9);
        assert_eq!(date.julian_day_number(), 2269298);
    }

    #[test]
    fn jdn_2298883_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(1582, Month::January, 10).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 10);
        assert_eq!(date.day_ordinal(), 10);
        assert_eq!(date.day_ordinal0(), 9);
        assert_eq!(date.ordinal(), 10);
        assert_eq!(date.ordinal0(), 9);
        assert_eq!(date.julian_day_number(), 2298883);
    }

    #[test]
    fn jdn_2298883_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(1582, 10).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 10);
        assert_eq!(date.day_ordinal(), 10);
        assert_eq!(date.day_ordinal0(), 9);
        assert_eq!(date.ordinal(), 10);
        assert_eq!(date.ordinal0(), 9);
        assert_eq!(date.julian_day_number(), 2298883);
    }

    #[test]
    fn jdn_2298884_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(1582, Month::January, 11).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 11);
        assert_eq!(date.day_ordinal(), 11);
        assert_eq!(date.day_ordinal0(), 10);
        assert_eq!(date.ordinal(), 11);
        assert_eq!(date.ordinal0(), 10);
        assert_eq!(date.julian_day_number(), 2298884);
    }

    #[test]
    fn jdn_2298884_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(1582, 11).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 11);
        assert_eq!(date.day_ordinal(), 11);
        assert_eq!(date.day_ordinal0(), 10);
        assert_eq!(date.ordinal(), 11);
        assert_eq!(date.ordinal0(), 10);
        assert_eq!(date.julian_day_number(), 2298884);
    }

    #[test]
    fn jdn_2299160_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(1582, Month::October, 14).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 14);
        assert_eq!(date.day_ordinal(), 14);
        assert_eq!(date.day_ordinal0(), 13);
        assert_eq!(date.ordinal(), 287);
        assert_eq!(date.ordinal0(), 286);
        assert_eq!(date.julian_day_number(), 2299160);
    }

    #[test]
    fn jdn_2299160_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(1582, 287).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 14);
        assert_eq!(date.day_ordinal(), 14);
        assert_eq!(date.day_ordinal0(), 13);
        assert_eq!(date.ordinal(), 287);
        assert_eq!(date.ordinal0(), 286);
        assert_eq!(date.julian_day_number(), 2299160);
    }

    #[test]
    fn jdn_2299161_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(1582, Month::October, 15).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 15);
        assert_eq!(date.day_ordinal(), 15);
        assert_eq!(date.day_ordinal0(), 14);
        assert_eq!(date.ordinal(), 288);
        assert_eq!(date.ordinal0(), 287);
        assert_eq!(date.julian_day_number(), 2299161);
    }

    #[test]
    fn jdn_2299161_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(1582, 288).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 15);
        assert_eq!(date.day_ordinal(), 15);
        assert_eq!(date.day_ordinal0(), 14);
        assert_eq!(date.ordinal(), 288);
        assert_eq!(date.ordinal0(), 287);
        assert_eq!(date.julian_day_number(), 2299161);
    }

    #[test]
    fn jdn_2299162_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(1582, Month::October, 16).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 16);
        assert_eq!(date.day_ordinal(), 16);
        assert_eq!(date.day_ordinal0(), 15);
        assert_eq!(date.ordinal(), 289);
        assert_eq!(date.ordinal0(), 288);
        assert_eq!(date.julian_day_number(), 2299162);
    }

    #[test]
    fn jdn_2299162_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(1582, 289).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 16);
        assert_eq!(date.day_ordinal(), 16);
        assert_eq!(date.day_ordinal0(), 15);
        assert_eq!(date.ordinal(), 289);
        assert_eq!(date.ordinal0(), 288);
        assert_eq!(date.julian_day_number(), 2299162);
    }

    #[test]
    fn jdn_2299238_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(1582, Month::December, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 2299238);
    }

    #[test]
    fn jdn_2299238_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(1582, 365).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1582);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 2299238);
    }

    #[test]
    fn jdn_2299239_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(1583, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1583);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2299239);
    }

    #[test]
    fn jdn_2299239_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(1583, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1583);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2299239);
    }

    #[test]
    fn jdn_2305448_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(1600, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1600);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2305448);
    }

    #[test]
    fn jdn_2305448_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(1600, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1600);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2305448);
    }

    #[test]
    fn jdn_2305517_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(1600, Month::March, 10).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1600);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 10);
        assert_eq!(date.day_ordinal(), 10);
        assert_eq!(date.day_ordinal0(), 9);
        assert_eq!(date.ordinal(), 70);
        assert_eq!(date.ordinal0(), 69);
        assert_eq!(date.julian_day_number(), 2305517);
    }

    #[test]
    fn jdn_2305517_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(1600, 70).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1600);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 10);
        assert_eq!(date.day_ordinal(), 10);
        assert_eq!(date.day_ordinal0(), 9);
        assert_eq!(date.ordinal(), 70);
        assert_eq!(date.ordinal0(), 69);
        assert_eq!(date.julian_day_number(), 2305517);
    }

    #[test]
    fn jdn_2341973_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(1700, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2341973);
    }

    #[test]
    fn jdn_2341973_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(1700, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2341973);
    }

    #[test]
    fn jdn_2342032_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(1700, Month::March, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 2342032);
    }

    #[test]
    fn jdn_2342032_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(1700, 60).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 2342032);
    }

    #[test]
    fn jdn_2342337_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(1700, Month::December, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 2342337);
    }

    #[test]
    fn jdn_2342337_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(1700, 365).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1700);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 2342337);
    }

    #[test]
    fn jdn_2342338_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(1701, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1701);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2342338);
    }

    #[test]
    fn jdn_2342338_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(1701, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1701);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2342338);
    }

    #[test]
    fn jdn_2378496_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(1799, Month::December, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1799);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 2378496);
    }

    #[test]
    fn jdn_2378496_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(1799, 365).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1799);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 2378496);
    }

    #[test]
    fn jdn_2378497_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(1800, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1800);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2378497);
    }

    #[test]
    fn jdn_2378497_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(1800, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1800);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2378497);
    }

    #[test]
    fn jdn_2378861_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(1800, Month::December, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1800);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 2378861);
    }

    #[test]
    fn jdn_2378861_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(1800, 365).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1800);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 2378861);
    }

    #[test]
    fn jdn_2378862_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(1801, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1801);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2378862);
    }

    #[test]
    fn jdn_2378862_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(1801, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1801);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2378862);
    }

    #[test]
    fn jdn_2415020_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(1899, Month::December, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1899);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 2415020);
    }

    #[test]
    fn jdn_2415020_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(1899, 365).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1899);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 2415020);
    }

    #[test]
    fn jdn_2415021_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(1900, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1900);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2415021);
    }

    #[test]
    fn jdn_2415021_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(1900, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1900);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2415021);
    }

    #[test]
    fn jdn_2415385_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(1900, Month::December, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1900);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 2415385);
    }

    #[test]
    fn jdn_2415385_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(1900, 365).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1900);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 2415385);
    }

    #[test]
    fn jdn_2415386_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(1901, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1901);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2415386);
    }

    #[test]
    fn jdn_2415386_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(1901, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 1901);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2415386);
    }

    #[test]
    fn jdn_2451605_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(2000, Month::March, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 2000);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 61);
        assert_eq!(date.ordinal0(), 60);
        assert_eq!(date.julian_day_number(), 2451605);
    }

    #[test]
    fn jdn_2451605_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(2000, 61).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 2000);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 61);
        assert_eq!(date.ordinal0(), 60);
        assert_eq!(date.julian_day_number(), 2451605);
    }

    #[test]
    fn jdn_2451910_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(2000, Month::December, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 2000);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 366);
        assert_eq!(date.ordinal0(), 365);
        assert_eq!(date.julian_day_number(), 2451910);
    }

    #[test]
    fn jdn_2451910_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(2000, 366).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 2000);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 366);
        assert_eq!(date.ordinal0(), 365);
        assert_eq!(date.julian_day_number(), 2451910);
    }

    #[test]
    fn jdn_2451911_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(2001, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 2001);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2451911);
    }

    #[test]
    fn jdn_2451911_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(2001, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 2001);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 2451911);
    }

    #[test]
    fn jdn_3145930_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(3901, Month::March, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 3901);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 3145930);
    }

    #[test]
    fn jdn_3145930_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(3901, 60).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 3901);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 3145930);
    }

    #[test]
    fn jdn_19508368_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(48699, Month::December, 30).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 48699);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 30);
        assert_eq!(date.day_ordinal(), 30);
        assert_eq!(date.day_ordinal0(), 29);
        assert_eq!(date.ordinal(), 364);
        assert_eq!(date.ordinal0(), 363);
        assert_eq!(date.julian_day_number(), 19508368);
    }

    #[test]
    fn jdn_19508368_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(48699, 364).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 48699);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 30);
        assert_eq!(date.day_ordinal(), 30);
        assert_eq!(date.day_ordinal0(), 29);
        assert_eq!(date.ordinal(), 364);
        assert_eq!(date.ordinal0(), 363);
        assert_eq!(date.julian_day_number(), 19508368);
    }

    #[test]
    fn jdn_19581784_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(48901, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 48901);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 19581784);
    }

    #[test]
    fn jdn_19581784_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(48901, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 48901);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 19581784);
    }

    #[test]
    fn jdn_19581842_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(48901, Month::February, 28).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 48901);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 28);
        assert_eq!(date.day_ordinal(), 28);
        assert_eq!(date.day_ordinal0(), 27);
        assert_eq!(date.ordinal(), 59);
        assert_eq!(date.ordinal0(), 58);
        assert_eq!(date.julian_day_number(), 19581842);
    }

    #[test]
    fn jdn_19581842_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(48901, 59).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 48901);
        assert_eq!(date.month(), Month::February);
        assert_eq!(date.day(), 28);
        assert_eq!(date.day_ordinal(), 28);
        assert_eq!(date.day_ordinal0(), 27);
        assert_eq!(date.ordinal(), 59);
        assert_eq!(date.ordinal0(), 58);
        assert_eq!(date.julian_day_number(), 19581842);
    }

    #[test]
    fn jdn_19581843_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(48901, Month::March, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 48901);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 19581843);
    }

    #[test]
    fn jdn_19581843_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(48901, 60).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 48901);
        assert_eq!(date.month(), Month::March);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 60);
        assert_eq!(date.ordinal0(), 59);
        assert_eq!(date.julian_day_number(), 19581843);
    }

    #[test]
    fn jdn_19582148_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(48901, Month::December, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 48901);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 19582148);
    }

    #[test]
    fn jdn_19582148_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(48901, 365).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 48901);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 19582148);
    }

    #[test]
    fn jdn_19582149_ymd() {
        let cal = Calendar::GREGORIAN;
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
    fn jdn_19582149_ordinal() {
        let cal = Calendar::GREGORIAN;
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
    fn jdn_19617943_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(49000, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 49000);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 19617943);
    }

    #[test]
    fn jdn_19617943_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(49000, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 49000);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 19617943);
    }

    #[test]
    fn jdn_19654466_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(49099, Month::December, 31).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 49099);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 19654466);
    }

    #[test]
    fn jdn_19654466_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(49099, 365).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 49099);
        assert_eq!(date.month(), Month::December);
        assert_eq!(date.day(), 31);
        assert_eq!(date.day_ordinal(), 31);
        assert_eq!(date.day_ordinal0(), 30);
        assert_eq!(date.ordinal(), 365);
        assert_eq!(date.ordinal0(), 364);
        assert_eq!(date.julian_day_number(), 19654466);
    }

    #[test]
    fn jdn_19654467_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(49100, Month::January, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 49100);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 19654467);
    }

    #[test]
    fn jdn_19654467_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(49100, 1).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 49100);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 1);
        assert_eq!(date.day_ordinal(), 1);
        assert_eq!(date.day_ordinal0(), 0);
        assert_eq!(date.ordinal(), 1);
        assert_eq!(date.ordinal0(), 0);
        assert_eq!(date.julian_day_number(), 19654467);
    }

    #[test]
    fn jdn_19654468_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(49100, Month::January, 2).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 49100);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 2);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 2);
        assert_eq!(date.ordinal0(), 1);
        assert_eq!(date.julian_day_number(), 19654468);
    }

    #[test]
    fn jdn_19654468_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(49100, 2).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 49100);
        assert_eq!(date.month(), Month::January);
        assert_eq!(date.day(), 2);
        assert_eq!(date.day_ordinal(), 2);
        assert_eq!(date.day_ordinal0(), 1);
        assert_eq!(date.ordinal(), 2);
        assert_eq!(date.ordinal0(), 1);
        assert_eq!(date.julian_day_number(), 19654468);
    }

    #[test]
    fn jdn_2147439589_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(5874777, Month::October, 18).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 5874777);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 18);
        assert_eq!(date.day_ordinal(), 18);
        assert_eq!(date.day_ordinal0(), 17);
        assert_eq!(date.ordinal(), 291);
        assert_eq!(date.ordinal0(), 290);
        assert_eq!(date.julian_day_number(), 2147439589);
    }

    #[test]
    fn jdn_2147439589_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(5874777, 291).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 5874777);
        assert_eq!(date.month(), Month::October);
        assert_eq!(date.day(), 18);
        assert_eq!(date.day_ordinal(), 18);
        assert_eq!(date.day_ordinal0(), 17);
        assert_eq!(date.ordinal(), 291);
        assert_eq!(date.ordinal0(), 290);
        assert_eq!(date.julian_day_number(), 2147439589);
    }

    #[test]
    fn jdn_2147483647_ymd() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ymd(5874898, Month::June, 3).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 5874898);
        assert_eq!(date.month(), Month::June);
        assert_eq!(date.day(), 3);
        assert_eq!(date.day_ordinal(), 3);
        assert_eq!(date.day_ordinal0(), 2);
        assert_eq!(date.ordinal(), 154);
        assert_eq!(date.ordinal0(), 153);
        assert_eq!(date.julian_day_number(), 2147483647);
    }

    #[test]
    fn jdn_2147483647_ordinal() {
        let cal = Calendar::GREGORIAN;
        let date = cal.at_ordinal_date(5874898, 154).unwrap();
        assert_eq!(date.calendar(), cal);
        assert!(!date.is_julian());
        assert!(date.is_gregorian());
        assert_eq!(date.year(), 5874898);
        assert_eq!(date.month(), Month::June);
        assert_eq!(date.day(), 3);
        assert_eq!(date.day_ordinal(), 3);
        assert_eq!(date.day_ordinal0(), 2);
        assert_eq!(date.ordinal(), 154);
        assert_eq!(date.ordinal0(), 153);
        assert_eq!(date.julian_day_number(), 2147483647);
    }
}
