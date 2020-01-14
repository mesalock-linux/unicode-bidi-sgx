use std::prelude::v1::*;
use unicode_bidi::*;
use unicode_bidi::BidiClass::*;

//#[test]
pub fn test_level_runs() {
    assert_eq!(level_runs(&Level::vec(&[]), &[]), &[]);
    assert_eq!(
        level_runs(&Level::vec(&[0, 0, 0, 1, 1, 2, 0, 0]), &[L; 8]),
        &[0..3, 3..5, 5..6, 6..8]
    );
}

// From <http://www.unicode.org/reports/tr9/#BD13>
//#[cfg_attr(rustfmt, rustfmt_skip)]
//#[test]
pub fn test_isolating_run_sequences() {

    // == Example 1 ==
    // text1·RLE·text2·PDF·RLE·text3·PDF·text4
    // index        0    1  2    3    4  5    6  7
    let classes = &[L, RLE, L, PDF, RLE, L, PDF, L];
    let levels =  &[0,   1, 1,   1,   1, 1,   1, 0];
    let para_level = Level::ltr();
    let mut sequences = isolating_run_sequences(para_level, classes, &Level::vec(levels));
    sequences.sort_by(|a, b| a.runs[0].clone().cmp(b.runs[0].clone()));
    assert_eq!(
        sequences.iter().map(|s| s.runs.clone()).collect::<Vec<_>>(),
        vec![vec![0..2], vec![2..7], vec![7..8]]
    );

    // == Example 2 ==
    // text1·RLI·text2·PDI·RLI·text3·PDI·text4
    // index        0    1  2    3    4  5    6  7
    let classes = &[L, RLI, L, PDI, RLI, L, PDI, L];
    let levels =  &[0,   0, 1,   0,   0, 1,   0, 0];
    let para_level = Level::ltr();
    let mut sequences = isolating_run_sequences(para_level, classes, &Level::vec(levels));
    sequences.sort_by(|a, b| a.runs[0].clone().cmp(b.runs[0].clone()));
    assert_eq!(
        sequences.iter().map(|s| s.runs.clone()).collect::<Vec<_>>(),
        vec![vec![0..2, 3..5, 6..8], vec![2..3], vec![5..6]]
    );

    // == Example 3 ==
    // text1·RLI·text2·LRI·text3·RLE·text4·PDF·text5·PDI·text6·PDI·text7
    // index        0    1  2    3  4    5  6    7  8    9  10  11  12
    let classes = &[L, RLI, L, LRI, L, RLE, L, PDF, L, PDI, L, PDI,  L];
    let levels =  &[0,   0, 1,   1, 2,   3, 3,   3, 2,   1, 1,   0,  0];
    let para_level = Level::ltr();
    let mut sequences = isolating_run_sequences(para_level, classes, &Level::vec(levels));
    sequences.sort_by(|a, b| a.runs[0].clone().cmp(b.runs[0].clone()));
    assert_eq!(
        sequences.iter().map(|s| s.runs.clone()).collect::<Vec<_>>(),
        vec![vec![0..2, 11..13], vec![2..4, 9..11], vec![4..6], vec![6..8], vec![8..9]]
    );
}

// From <http://www.unicode.org/reports/tr9/#X10>
#[cfg_attr(rustfmt, rustfmt_skip)]
//#[test]
pub fn test_isolating_run_sequences_sos_and_eos() {

    // == Example 1 ==
    // text1·RLE·text2·LRE·text3·PDF·text4·PDF·RLE·text5·PDF·text6
    // index        0    1  2    3  4    5  6    7    8  9   10  11
    let classes = &[L, RLE, L, LRE, L, PDF, L, PDF, RLE, L, PDF,  L];
    let levels =  &[0,   1, 1,   2, 2,   2, 1,   1,   1, 1,   1,  0];
    let para_level = Level::ltr();
    let mut sequences = isolating_run_sequences(para_level, classes, &Level::vec(levels));
    sequences.sort_by(|a, b| a.runs[0].clone().cmp(b.runs[0].clone()));

    // text1
    assert_eq!(
        &sequences[0],
        &IsolatingRunSequence {
            runs: vec![0..2],
            sos: L,
            eos: R,
        }
    );

    // text2
    assert_eq!(
        &sequences[1],
        &IsolatingRunSequence {
            runs: vec![2..4],
            sos: R,
            eos: L,
        }
    );

    // text3
    assert_eq!(
        &sequences[2],
        &IsolatingRunSequence {
            runs: vec![4..6],
            sos: L,
            eos: L,
        }
    );

    // text4 text5
    assert_eq!(
        &sequences[3],
        &IsolatingRunSequence {
            runs: vec![6..11],
            sos: L,
            eos: R,
        }
    );

    // text6
    assert_eq!(
        &sequences[4],
        &IsolatingRunSequence {
            runs: vec![11..12],
            sos: R,
            eos: L,
        }
    );

    // == Example 2 ==
    // text1·RLI·text2·LRI·text3·PDI·text4·PDI·RLI·text5·PDI·text6
    // index        0    1  2    3  4    5  6    7    8  9   10  11
    let classes = &[L, RLI, L, LRI, L, PDI, L, PDI, RLI, L, PDI,  L];
    let levels =  &[0,   0, 1,   1, 2,   1, 1,   0,   0, 1,   0,  0];
    let para_level = Level::ltr();
    let mut sequences = isolating_run_sequences(para_level, classes, &Level::vec(levels));
    sequences.sort_by(|a, b| a.runs[0].clone().cmp(b.runs[0].clone()));

    // text1·RLI·PDI·RLI·PDI·text6
    assert_eq!(
        &sequences[0],
        &IsolatingRunSequence {
            runs: vec![0..2, 7..9, 10..12],
            sos: L,
            eos: L,
        }
    );

    // text2·LRI·PDI·text4
    assert_eq!(
        &sequences[1],
        &IsolatingRunSequence {
            runs: vec![2..4, 5..7],
            sos: R,
            eos: R,
        }
    );

    // text3
    assert_eq!(
        &sequences[2],
        &IsolatingRunSequence {
            runs: vec![4..5],
            sos: L,
            eos: L,
        }
    );

    // text5
    assert_eq!(
        &sequences[3],
        &IsolatingRunSequence {
            runs: vec![9..10],
            sos: R,
            eos: R,
        }
    );
}

//#[test]
pub fn test_removed_by_x9() {
    let rem_classes = &[RLE, LRE, RLO, LRO, PDF, BN];
    let not_classes = &[L, RLI, AL, LRI, PDI];
    for x in rem_classes {
        assert_eq!(removed_by_x9(*x), true);
    }
    for x in not_classes {
        assert_eq!(removed_by_x9(*x), false);
    }
}

//#[test]
pub fn test_not_removed_by_x9() {
    let non_x9_classes = &[L, R, AL, EN, ES, ET, AN, CS, NSM, B, S, WS, ON, LRI, RLI, FSI, PDI];
    for x in non_x9_classes {
        assert_eq!(not_removed_by_x9(&x), true);
    }
}
