use std::collections::HashMap;

use crate::models::scoresheet::ScoreSheet;

pub struct Stats {
    average: f64,
    sum: i64,
}

pub struct GroupedStats {
    stats: HashMap<i32, Stats>,
}
