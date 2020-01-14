use serde_test::{Token, assert_tokens};
use unicode_bidi::*;
use unicode_bidi::level::*;
use std::prelude::v1::*;

//#[test]
pub fn test_new() {
    assert_eq!(Level::new(0), Ok(Level(0)));
    assert_eq!(Level::new(1), Ok(Level(1)));
    assert_eq!(Level::new(10), Ok(Level(10)));
    assert_eq!(Level::new(125), Ok(Level(125)));
    assert_eq!(Level::new(126), Ok(Level(126)));
    assert_eq!(Level::new(127), Err(Error::OutOfRangeNumber));
    assert_eq!(Level::new(255), Err(Error::OutOfRangeNumber));
}

//#[test]
pub fn test_new_explicit() {
    assert_eq!(Level::new_explicit(0), Ok(Level(0)));
    assert_eq!(Level::new_explicit(1), Ok(Level(1)));
    assert_eq!(Level::new_explicit(10), Ok(Level(10)));
    assert_eq!(Level::new_explicit(125), Ok(Level(125)));
    assert_eq!(Level::new_explicit(126), Err(Error::OutOfRangeNumber));
    assert_eq!(Level::new_explicit(255), Err(Error::OutOfRangeNumber));
}

//#[test]
pub fn test_is_ltr() {
    assert_eq!(Level(0).is_ltr(), true);
    assert_eq!(Level(1).is_ltr(), false);
    assert_eq!(Level(10).is_ltr(), true);
    assert_eq!(Level(11).is_ltr(), false);
    assert_eq!(Level(124).is_ltr(), true);
    assert_eq!(Level(125).is_ltr(), false);
}

//#[test]
pub fn test_is_rtl() {
    assert_eq!(Level(0).is_rtl(), false);
    assert_eq!(Level(1).is_rtl(), true);
    assert_eq!(Level(10).is_rtl(), false);
    assert_eq!(Level(11).is_rtl(), true);
    assert_eq!(Level(124).is_rtl(), false);
    assert_eq!(Level(125).is_rtl(), true);
}

//#[test]
pub fn test_raise() {
    let mut level = Level::ltr();
    assert_eq!(level.number(), 0);
    assert!(level.raise(100).is_ok());
    assert_eq!(level.number(), 100);
    assert!(level.raise(26).is_ok());
    assert_eq!(level.number(), 126);
    assert!(level.raise(1).is_err()); // invalid!
    assert!(level.raise(250).is_err()); // overflow!
    assert_eq!(level.number(), 126);
}

//#[test]
pub fn test_raise_explicit() {
    let mut level = Level::ltr();
    assert_eq!(level.number(), 0);
    assert!(level.raise_explicit(100).is_ok());
    assert_eq!(level.number(), 100);
    assert!(level.raise_explicit(25).is_ok());
    assert_eq!(level.number(), 125);
    assert!(level.raise_explicit(1).is_err()); // invalid!
    assert!(level.raise_explicit(250).is_err()); // overflow!
    assert_eq!(level.number(), 125);
}

//#[test]
pub fn test_lower() {
    let mut level = Level::rtl();
    assert_eq!(level.number(), 1);
    assert!(level.lower(1).is_ok());
    assert_eq!(level.number(), 0);
    assert!(level.lower(1).is_err()); // underflow!
    assert!(level.lower(250).is_err()); // underflow!
    assert_eq!(level.number(), 0);
}

//#[test]
pub fn test_has_rtl() {
    assert_eq!(has_rtl(&Level::vec(&[0, 0, 0])), false);
    assert_eq!(has_rtl(&Level::vec(&[0, 1, 0])), true);
    assert_eq!(has_rtl(&Level::vec(&[0, 2, 0])), false);
    assert_eq!(has_rtl(&Level::vec(&[0, 125, 0])), true);
    assert_eq!(has_rtl(&Level::vec(&[0, 126, 0])), false);
}

//#[test]
pub fn test_into() {
    let level = Level::rtl();
    assert_eq!(1u8, level.into());
}

//#[test]
pub fn test_vec() {
    assert_eq!(
        Level::vec(&[0, 1, 125]),
        vec![Level(0), Level(1), Level(125)]
    );
}

//#[test]
pub fn test_str_eq() {
    assert_eq!(Level::vec(&[0, 1, 4, 125]), vec!["0", "1", "x", "125"]);
    assert_ne!(Level::vec(&[0, 1, 4, 125]), vec!["0", "1", "5", "125"]);
}

//#[test]
pub fn test_string_eq() {
    assert_eq!(
        Level::vec(&[0, 1, 4, 125]),
        vec!["0".to_string(), "1".to_string(), "x".to_string(), "125".to_string()]
    );
}

//#[test]
pub fn test_statics() {
    assert_tokens(
        &Level::ltr(),
        &[Token::NewtypeStruct { name: "Level" }, Token::U8(0)],
    );
    assert_tokens(
        &Level::rtl(),
        &[Token::NewtypeStruct { name: "Level" }, Token::U8(1)],
    );
}

//#[test]
pub fn test_serde_new() {
    let level = Level::new(42).unwrap();
    assert_tokens(
        &level,
        &[Token::NewtypeStruct { name: "Level" }, Token::U8(42)],
    );
}
