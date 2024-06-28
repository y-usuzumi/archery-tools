use std::borrow::Cow;

use chrono::{DateTime, Local};

// The reason why Score is not a simple integer is because
// of special scores like X.
#[derive(PartialEq, Clone, Debug)]
struct Score<'a> {
    value: i32,
    display: Cow<'a, str>,
}

static X: Score = Score {
    value: 10,
    display: Cow::Borrowed("X"),
};

impl From<i32> for Score<'_> {
    fn from(value: i32) -> Self {
        Score {
            value,
            display: Cow::Owned(value.to_string()),
        }
    }
}

pub struct User {}

// A ScoreSheet is a record of a full session
#[derive(Clone, Debug)]
pub struct ScoreSheet<'a> {
    time_created: DateTime<Local>,
    time_finished: DateTime<Local>,
    scores: Vec<ScoreRoundGroup<'a>>,
}

// A ScoreSheet may be broken into multiple ScoreRoundGroup's.
// For example, you can have a group of morning and a group of
// afternoon. Or, you can also have different groups with different
// distances. This may be helpful in statistics in the future.
#[derive(Clone, Debug)]
pub struct ScoreRoundGroup<'a> {
    score_rounds: Vec<ScoreRound<'a>>,
    metadata: ScoreRoundGroupMetadata,
}

#[derive(Clone, Debug)]
pub struct ScoreRoundGroupMetadata {
    description: String,
    distance: i32,
}

#[derive(Clone, Debug)]
pub struct ScoreRound<'a> {
    scores: Vec<Score<'a>>,
}

#[cfg(test)]
mod tests {
    use super::{ScoreRound, ScoreRoundGroup, ScoreRoundGroupMetadata, ScoreSheet, X};
    use chrono::{Duration, Local};
    use std::vec;

    #[test]
    fn test_complete_score_sheet() {
        let score_sheet = ScoreSheet {
            time_created: Local::now() - Duration::hours(1),
            time_finished: Local::now(),
            scores: vec![
                ScoreRoundGroup {
                    metadata: ScoreRoundGroupMetadata {
                        description: "Morning".to_string(),
                        distance: 70,
                    },
                    score_rounds: vec![
                        ScoreRound {
                            scores: vec![10.into(), 9.into(), X.clone()],
                        },
                        ScoreRound {
                            scores: vec![6.into(), 9.into(), 8.into()],
                        },
                    ],
                },
                ScoreRoundGroup {
                    metadata: ScoreRoundGroupMetadata {
                        description: "Morning".to_string(),
                        distance: 30,
                    },
                    score_rounds: vec![
                        ScoreRound {
                            scores: vec![X.clone(), X.clone(), 8.into()],
                        },
                        ScoreRound {
                            scores: vec![6.into(), 6.into(), 7.into()],
                        },
                    ],
                },
            ],
        };
        println!("{:?}", score_sheet);
    }
}
