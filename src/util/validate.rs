
use pgx::*;


pub(crate) fn check_valid_probability(p: f64) {
    if p > 1. || p < 0. {
        error!("{}", "invalid probability of > 1 or < 1");
    }
}