use serde_test::{Token, assert_tokens};
use unicode_bidi::*;
use unicode_bidi::format_chars as chars;
use std::prelude::v1::*;
use unicode_bidi::BidiClass::*;
use std::borrow::Cow;

//#[test]
pub fn test_initial_text_info() {
    let text = "a1";
    assert_eq!(
        InitialInfo::new(text, None),
        InitialInfo {
            text,
            original_classes: vec![L, EN],
            paragraphs: vec![
                ParagraphInfo {
                    range: 0..2,
                    level: LTR_LEVEL,
                },
            ],
        }
    );

    let text = "غ א";
    assert_eq!(
        InitialInfo::new(text, None),
        InitialInfo {
            text,
            original_classes: vec![AL, AL, WS, R, R],
            paragraphs: vec![
                ParagraphInfo {
                    range: 0..5,
                    level: RTL_LEVEL,
                },
            ],
        }
    );

    let text = "a\u{2029}b";
    assert_eq!(
        InitialInfo::new(text, None),
        InitialInfo {
            text,
            original_classes: vec![L, B, B, B, L],
            paragraphs: vec![
                ParagraphInfo {
                    range: 0..4,
                    level: LTR_LEVEL,
                },
                ParagraphInfo {
                    range: 4..5,
                    level: LTR_LEVEL,
                },
            ],
        }
    );

    let text = format!("{}א{}a", chars::FSI, chars::PDI);
    assert_eq!(
        InitialInfo::new(&text, None),
        InitialInfo {
            text: &text,
            original_classes: vec![RLI, RLI, RLI, R, R, PDI, PDI, PDI, L],
            paragraphs: vec![
                ParagraphInfo {
                    range: 0..9,
                    level: LTR_LEVEL,
                },
            ],
        }
    );
}

//#[test]
pub fn test_process_text() {
    let text = "abc123";
    assert_eq!(
        BidiInfo::new(text, Some(LTR_LEVEL)),
        BidiInfo {
            text,
            levels: Level::vec(&[0, 0, 0, 0, 0, 0]),
            original_classes: vec![L, L, L, EN, EN, EN],
            paragraphs: vec![
                ParagraphInfo {
                    range: 0..6,
                    level: LTR_LEVEL,
                },
            ],
        }
    );

    let text = "abc אבג";
    assert_eq!(
        BidiInfo::new(text, Some(LTR_LEVEL)),
        BidiInfo {
            text,
            levels: Level::vec(&[0, 0, 0, 0, 1, 1, 1, 1, 1, 1]),
            original_classes: vec![L, L, L, WS, R, R, R, R, R, R],
            paragraphs: vec![
                ParagraphInfo {
                    range: 0..10,
                    level: LTR_LEVEL,
                },
            ],
        }
    );
    assert_eq!(
        BidiInfo::new(text, Some(RTL_LEVEL)),
        BidiInfo {
            text,
            levels: Level::vec(&[2, 2, 2, 1, 1, 1, 1, 1, 1, 1]),
            original_classes: vec![L, L, L, WS, R, R, R, R, R, R],
            paragraphs: vec![
                ParagraphInfo {
                    range: 0..10,
                    level: RTL_LEVEL,
                },
            ],
        }
    );

    let text = "אבג abc";
    assert_eq!(
        BidiInfo::new(text, Some(LTR_LEVEL)),
        BidiInfo {
            text,
            levels: Level::vec(&[1, 1, 1, 1, 1, 1, 0, 0, 0, 0]),
            original_classes: vec![R, R, R, R, R, R, WS, L, L, L],
            paragraphs: vec![
                ParagraphInfo {
                    range: 0..10,
                    level: LTR_LEVEL,
                },
            ],
        }
    );
    assert_eq!(
        BidiInfo::new(text, None),
        BidiInfo {
            text,
            levels: Level::vec(&[1, 1, 1, 1, 1, 1, 1, 2, 2, 2]),
            original_classes: vec![R, R, R, R, R, R, WS, L, L, L],
            paragraphs: vec![
                ParagraphInfo {
                    range: 0..10,
                    level: RTL_LEVEL,
                },
            ],
        }
    );

    let text = "غ2ظ א2ג";
    assert_eq!(
        BidiInfo::new(text, Some(LTR_LEVEL)),
        BidiInfo {
            text,
            levels: Level::vec(&[1, 1, 2, 1, 1, 1, 1, 1, 2, 1, 1]),
            original_classes: vec![AL, AL, EN, AL, AL, WS, R, R, EN, R, R],
            paragraphs: vec![
                ParagraphInfo {
                    range: 0..11,
                    level: LTR_LEVEL,
                },
            ],
        }
    );

    let text = "a א.\nג";
    assert_eq!(
        BidiInfo::new(text, None),
        BidiInfo {
            text,
            original_classes: vec![L, WS, R, R, CS, B, R, R],
            levels: Level::vec(&[0, 0, 1, 1, 0, 0, 1, 1]),
            paragraphs: vec![
                ParagraphInfo {
                    range: 0..6,
                    level: LTR_LEVEL,
                },
                ParagraphInfo {
                    range: 6..8,
                    level: RTL_LEVEL,
                },
            ],
        }
    );

    // BidiTest:69635 (AL ET EN)
    let bidi_info = BidiInfo::new("\u{060B}\u{20CF}\u{06F9}", None);
    assert_eq!(bidi_info.original_classes, vec![AL, AL, ET, ET, ET, EN, EN]);
}

//#[test]
pub fn test_bidi_info_has_rtl() {
    // ASCII only
    assert_eq!(BidiInfo::new("123", None).has_rtl(), false);
    assert_eq!(BidiInfo::new("123", Some(LTR_LEVEL)).has_rtl(), false);
    assert_eq!(BidiInfo::new("123", Some(RTL_LEVEL)).has_rtl(), false);
    assert_eq!(BidiInfo::new("abc", None).has_rtl(), false);
    assert_eq!(BidiInfo::new("abc", Some(LTR_LEVEL)).has_rtl(), false);
    assert_eq!(BidiInfo::new("abc", Some(RTL_LEVEL)).has_rtl(), false);
    assert_eq!(BidiInfo::new("abc 123", None).has_rtl(), false);
    assert_eq!(BidiInfo::new("abc\n123", None).has_rtl(), false);

    // With Hebrew
    assert_eq!(BidiInfo::new("אבּג", None).has_rtl(), true);
    assert_eq!(BidiInfo::new("אבּג", Some(LTR_LEVEL)).has_rtl(), true);
    assert_eq!(BidiInfo::new("אבּג", Some(RTL_LEVEL)).has_rtl(), true);
    assert_eq!(BidiInfo::new("abc אבּג", None).has_rtl(), true);
    assert_eq!(BidiInfo::new("abc\nאבּג", None).has_rtl(), true);
    assert_eq!(BidiInfo::new("אבּג abc", None).has_rtl(), true);
    assert_eq!(BidiInfo::new("אבּג\nabc", None).has_rtl(), true);
    assert_eq!(BidiInfo::new("אבּג 123", None).has_rtl(), true);
    assert_eq!(BidiInfo::new("אבּג\n123", None).has_rtl(), true);
}

fn reorder_paras(text: &str) -> Vec<Cow<str>> {
    let bidi_info = BidiInfo::new(text, None);
    bidi_info
        .paragraphs
        .iter()
        .map(|para| bidi_info.reorder_line(para, para.range.clone()))
        .collect()
}

//#[test]
pub fn test_reorder_line() {
    // Bidi_Class: L L L B L L L B L L L
    assert_eq!(
        reorder_paras("abc\ndef\nghi"),
        vec!["abc\n", "def\n", "ghi"]
    );

    // Bidi_Class: L L EN B L L EN B L L EN
    assert_eq!(
        reorder_paras("ab1\nde2\ngh3"),
        vec!["ab1\n", "de2\n", "gh3"]
    );

    // Bidi_Class: L L L B AL AL AL
    assert_eq!(reorder_paras("abc\nابج"), vec!["abc\n", "جبا"]);

    // Bidi_Class: AL AL AL B L L L
    assert_eq!(reorder_paras("ابج\nabc"), vec!["\nجبا", "abc"]);

    assert_eq!(reorder_paras("1.-2"), vec!["1.-2"]);
    assert_eq!(reorder_paras("1-.2"), vec!["1-.2"]);
    assert_eq!(reorder_paras("abc אבג"), vec!["abc גבא"]);

    // Numbers being weak LTR characters, cannot reorder strong RTL
    assert_eq!(reorder_paras("123 אבג"), vec!["גבא 123"]);

    assert_eq!(reorder_paras("abc\u{202A}def"), vec!["abc\u{202A}def"]);

    assert_eq!(
        reorder_paras("abc\u{202A}def\u{202C}ghi"),
        vec!["abc\u{202A}def\u{202C}ghi"]
    );

    assert_eq!(
        reorder_paras("abc\u{2066}def\u{2069}ghi"),
        vec!["abc\u{2066}def\u{2069}ghi"]
    );

    // Testing for RLE Character
    assert_eq!(
        reorder_paras("\u{202B}abc אבג\u{202C}"),
        vec!["\u{202B}\u{202C}גבא abc"]
    );

    // Testing neutral characters
    assert_eq!(reorder_paras("אבג? אבג"), vec!["גבא ?גבא"]);

    // Testing neutral characters with special case
    assert_eq!(reorder_paras("A אבג?"), vec!["A גבא?"]);

    // Testing neutral characters with Implicit RTL Marker
    assert_eq!(
        reorder_paras("A אבג?\u{200F}"),
        vec!["A \u{200F}?גבא"]
    );
    assert_eq!(reorder_paras("אבג abc"), vec!["abc גבא"]);
    assert_eq!(
        reorder_paras("abc\u{2067}.-\u{2069}ghi"),
        vec!["abc\u{2067}-.\u{2069}ghi"]
    );

    assert_eq!(
        reorder_paras("Hello, \u{2068}\u{202E}world\u{202C}\u{2069}!"),
        vec!["Hello, \u{2068}\u{202E}\u{202C}dlrow\u{2069}!"]
    );

    // With mirrorable characters in RTL run
    assert_eq!(reorder_paras("א(ב)ג."), vec![".ג)ב(א"]);

    // With mirrorable characters on level boundry
    assert_eq!(
        reorder_paras("אב(גד[&ef].)gh"),
        vec!["ef].)gh&[דג(בא"]
    );
}

fn reordered_levels_for_paras(text: &str) -> Vec<Vec<Level>> {
    let bidi_info = BidiInfo::new(text, None);
    bidi_info
        .paragraphs
        .iter()
        .map(|para| bidi_info.reordered_levels(para, para.range.clone()))
        .collect()
}

fn reordered_levels_per_char_for_paras(text: &str) -> Vec<Vec<Level>> {
    let bidi_info = BidiInfo::new(text, None);
    bidi_info
        .paragraphs
        .iter()
        .map(|para| {
            bidi_info.reordered_levels_per_char(para, para.range.clone())
        })
        .collect()
}

//#[test]
pub fn test_reordered_levels() {

    // BidiTest:946 (LRI PDI)
    let text = "\u{2067}\u{2069}";
    assert_eq!(
        reordered_levels_for_paras(text),
        vec![Level::vec(&[0, 0, 0, 0, 0, 0])]
    );
    assert_eq!(
        reordered_levels_per_char_for_paras(text),
        vec![Level::vec(&[0, 0])]
    );

    /* TODO
    /// BidiTest:69635 (AL ET EN)
    let text = "\u{060B}\u{20CF}\u{06F9}";
    assert_eq!(
        reordered_levels_for_paras(text),
        vec![Level::vec(&[1, 1, 1, 1, 1, 2, 2])]
    );
    assert_eq!(
        reordered_levels_per_char_for_paras(text),
        vec![Level::vec(&[1, 1, 2])]
    );
     */

    /* TODO
    // BidiTest:291284 (AN RLI PDF R)
    assert_eq!(
        reordered_levels_per_char_for_paras("\u{0605}\u{2067}\u{202C}\u{0590}"),
        vec![&["2", "0", "x", "1"]]
    );
     */
}


//#[test]
pub fn test_levels() {
    let text = "abc אבג";
    let bidi_info = BidiInfo::new(text, None);
    let levels = bidi_info.levels;
    assert_eq!(text.as_bytes().len(), 10);
    assert_eq!(levels.len(), 10);
    assert_tokens(
        &levels,
        &[
            Token::Seq { len: Some(10) },
            Token::NewtypeStruct { name: "Level" },
            Token::U8(0),
            Token::NewtypeStruct { name: "Level" },
            Token::U8(0),
            Token::NewtypeStruct { name: "Level" },
            Token::U8(0),
            Token::NewtypeStruct { name: "Level" },
            Token::U8(0),
            Token::NewtypeStruct { name: "Level" },
            Token::U8(1),
            Token::NewtypeStruct { name: "Level" },
            Token::U8(1),
            Token::NewtypeStruct { name: "Level" },
            Token::U8(1),
            Token::NewtypeStruct { name: "Level" },
            Token::U8(1),
            Token::NewtypeStruct { name: "Level" },
            Token::U8(1),
            Token::NewtypeStruct { name: "Level" },
            Token::U8(1),
            Token::SeqEnd,
        ],
    );
}
