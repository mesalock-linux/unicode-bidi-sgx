use unicode_bidi::BidiClass::*;
use unicode_bidi::*;
use std::prelude::v1::*;

//#[test]
pub fn test_ascii() {
    assert_eq!(bidi_class('\u{0000}'), BN);
    assert_eq!(bidi_class('\u{0040}'), ON);
    assert_eq!(bidi_class('\u{0041}'), L);
    assert_eq!(bidi_class('\u{0062}'), L);
    assert_eq!(bidi_class('\u{007F}'), BN);
}

//#[test]
pub fn test_bmp() {
    // Hebrew
    assert_eq!(bidi_class('\u{0590}'), R);
    assert_eq!(bidi_class('\u{05D0}'), R);
    assert_eq!(bidi_class('\u{05D1}'), R);
    assert_eq!(bidi_class('\u{05FF}'), R);

    // Arabic
    assert_eq!(bidi_class('\u{0600}'), AN);
    assert_eq!(bidi_class('\u{0627}'), AL);
    assert_eq!(bidi_class('\u{07BF}'), AL);

    // Default R + Arabic Extras
    assert_eq!(bidi_class('\u{07C0}'), R);
    assert_eq!(bidi_class('\u{085F}'), R);
    assert_eq!(bidi_class('\u{0860}'), AL);
    assert_eq!(bidi_class('\u{0870}'), R);
    assert_eq!(bidi_class('\u{089F}'), R);
    assert_eq!(bidi_class('\u{08A0}'), AL);
    assert_eq!(bidi_class('\u{089F}'), R);
    assert_eq!(bidi_class('\u{08FF}'), NSM);

    // Default ET
    assert_eq!(bidi_class('\u{20A0}'), ET);
    assert_eq!(bidi_class('\u{20CF}'), ET);

    // Arabic Presentation Forms
    assert_eq!(bidi_class('\u{FB1D}'), R);
    assert_eq!(bidi_class('\u{FB4F}'), R);
    assert_eq!(bidi_class('\u{FB50}'), AL);
    assert_eq!(bidi_class('\u{FDCF}'), AL);
    assert_eq!(bidi_class('\u{FDF0}'), AL);
    assert_eq!(bidi_class('\u{FDFF}'), AL);
    assert_eq!(bidi_class('\u{FE70}'), AL);
    assert_eq!(bidi_class('\u{FEFE}'), AL);
    assert_eq!(bidi_class('\u{FEFF}'), BN);

    // noncharacters
    assert_eq!(bidi_class('\u{FDD0}'), L);
    assert_eq!(bidi_class('\u{FDD1}'), L);
    assert_eq!(bidi_class('\u{FDEE}'), L);
    assert_eq!(bidi_class('\u{FDEF}'), L);
    assert_eq!(bidi_class('\u{FFFE}'), L);
    assert_eq!(bidi_class('\u{FFFF}'), L);
}

//#[test]
pub fn test_smp() {
    // Default AL + R
    assert_eq!(bidi_class('\u{10800}'), R);
    assert_eq!(bidi_class('\u{10FFF}'), R);
    assert_eq!(bidi_class('\u{1E800}'), R);
    assert_eq!(bidi_class('\u{1EDFF}'), R);
    assert_eq!(bidi_class('\u{1EE00}'), AL);
    assert_eq!(bidi_class('\u{1EEFF}'), AL);
    assert_eq!(bidi_class('\u{1EF00}'), R);
    assert_eq!(bidi_class('\u{1EFFF}'), R);
}

//#[test]
pub fn test_unassigned_planes() {
    assert_eq!(bidi_class('\u{30000}'), L);
    assert_eq!(bidi_class('\u{40000}'), L);
    assert_eq!(bidi_class('\u{50000}'), L);
    assert_eq!(bidi_class('\u{60000}'), L);
    assert_eq!(bidi_class('\u{70000}'), L);
    assert_eq!(bidi_class('\u{80000}'), L);
    assert_eq!(bidi_class('\u{90000}'), L);
    assert_eq!(bidi_class('\u{a0000}'), L);
}
