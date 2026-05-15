use rustc_hash::FxHashMap;

use crate::{
    Hints, Word,
    dict::{LETTER_KINDS, WORD_LEN},
    hints::Hint,
    letter::Letter,
};

#[derive(Debug)]
pub(super) struct ConstraintDelta {
    pub(super) positions: [Option<PositionDelta>; WORD_LEN],
    pub(super) letters: [Option<LetterDelta>; LETTER_KINDS],
}

impl ConstraintDelta {
    pub(super) fn from_feedback(guess: &Word, hints: &Hints) -> Self {
        let mut positions = [None; WORD_LEN];
        let mut letters = [None; LETTER_KINDS];

        let mut map = FxHashMap::<Letter, (u8, u8)>::default();

        for (index, (letter, hint)) in guess.iter().zip(hints.iter()).enumerate() {
            positions[index] = Some(PositionDelta { letter, hint });

            let (green_or_yellow, gray) = map.entry(letter).or_insert((0, 0));
            if hint == Hint::NotExists {
                *gray += 1;
            } else {
                *green_or_yellow += 1;
            }
        }

        for (letter, (green_or_yellow, gray)) in map {
            letters[letter.as_index()] = Some(LetterDelta {
                min_count: green_or_yellow,
                max_count: if gray == 0 {
                    None
                } else {
                    Some(green_or_yellow)
                },
            });
        }

        Self { positions, letters }
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct PositionDelta {
    pub(super) letter: Letter,
    pub(super) hint: Hint,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct LetterDelta {
    pub(super) min_count: u8,
    pub(super) max_count: Option<u8>,
}
