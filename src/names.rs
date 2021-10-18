use pgx::*;
use rand::{Rng, SeedableRng, RngCore};

#[path = "data/names.rs"]
mod names;

#[path = "util/validate.rs"]
mod validate;


// use crate::faker::names::*;
use rand::rngs::StdRng;
use crate::names::names::*;
use crate::names::validate::check_valid_probability;

#[pg_extern]
fn faker_first_name() -> &'static str {
    if rand::thread_rng().gen_bool(0.5) {
        return faker_mens_name();
    }
    return faker_women_name();
}

#[pg_extern]
fn faker_first_name_with_seed(seed: i64) -> &'static str {
    let mut r = StdRng::seed_from_u64(seed as u64);
    if r.gen_bool(0.5) {
        return faker_mens_name_with_seed(seed);
    }
    return faker_women_name_with_seed(seed);
}


/// ```funcname
/// faker_first_name
/// ```
#[pg_extern]
fn faker_first_name_with_seed_iter(
    num_rows: i32,
    seed: i64,
) -> impl std::iter::Iterator<Item=(name!(index, i32), name!(first_name, &'static str))> {
    let mut r = StdRng::seed_from_u64(seed as u64);
    (1..=num_rows).map(move |i| (i, random_seeded_first_name_gendered(&mut r, 0.5)))
}

/// ```funcname
/// faker_first_name
/// ```
#[pg_extern]
fn faker_first_name_iter_with_probability(
    num_rows: i32,
    probability_male: f64,
) -> impl std::iter::Iterator<Item=(name!(index, i32), name!(first_name, &'static str))> {
    check_valid_probability(probability_male);
    let mut r = rand::thread_rng();
    (1..=num_rows).map(move |i| (i, random_seeded_first_name_gendered(&mut r, probability_male)))
}


/// ```funcname
/// faker_first_name
/// ```
#[pg_extern]
fn faker_first_name_iter(
    num_rows: i32,
) -> impl std::iter::Iterator<Item=(name!(index, i32), name!(first_name, &'static str))> {
    let mut r = rand::thread_rng();
    (1..=num_rows).map(move |i| (i, random_seeded_first_name_gendered(&mut r, 0.5)))
}


fn random_seeded_first_name_gendered(seed: &mut dyn RngCore, prob_male: f64) -> &'static str {
    if prob_male > 1. {
        error!("invalid (male) probability. must between 0..1");
    }
    if seed.gen_bool(prob_male) {
        return get_seeded_value(seed, MENS_NAMES);
    }
    return get_seeded_value(seed, WOMEN_NAMES);
}

fn get_seeded_value(seed: &mut dyn RngCore, val: &'static [&'static str]) -> &'static str {
    val[(*seed).gen_range(0..val.len() - 1)]
}

#[pg_extern]
fn faker_full_name() -> String {
    format!("{} {}", faker_first_name(), faker_last_name()).to_string()
}

#[pg_extern]
fn faker_full_name_with_seed(seed: i64) -> String {
    format!("{} {}", faker_first_name_with_seed(seed), faker_last_name_with_seed(seed)).to_string()
}

/// ```funcname
/// faker_full_name
/// ```
#[pg_extern]
fn faker_full_name_with_seed_iter(
    num_rows: i32,
    seed: i64,
) -> impl std::iter::Iterator<Item=(name!(index, i32), name!(full_name, String))> {
    let mut r = StdRng::seed_from_u64(seed as u64);
    (1..=num_rows).map(move |i| (i, random_seeded_full_name(&mut r)))
}

/// ```funcname
/// faker_full_name
/// ```
#[pg_extern]
fn faker_full_name_iter(
    num_rows: i32,
) -> impl std::iter::Iterator<Item=(name!(index, i32), name!(full_name, String))> {
    let mut r = rand::thread_rng();
    (1..=num_rows).map(move |i| (i, random_seeded_full_name(&mut r)))
}


fn random_seeded_full_name(seed: &mut dyn RngCore) -> String {
    if seed.gen_bool(0.50) {
        return format!("{} {}", get_seeded_value(seed, MENS_NAMES), get_seeded_value(seed, LAST_NAMES));
    }
    return format!("{} {}", get_seeded_value(seed, WOMEN_NAMES), get_seeded_value(seed, LAST_NAMES));
}

#[pg_extern]
fn faker_mens_name() -> &'static str {
    MENS_NAMES[rand::thread_rng().gen_range(0..MENS_NAMES.len() - 1)]
}

/// ```funcname
/// faker_mens_name_with_seed
/// ```
#[pg_extern]
fn faker_mens_name_with_seed(seed: i64) -> &'static str {
    MENS_NAMES[StdRng::seed_from_u64(seed as u64).gen_range(0..MENS_NAMES.len() - 1)]
}

/// ```funcname
/// faker_mens_name
/// ```
#[pg_extern]
fn faker_mens_name_with_seed_iter(
    num_rows: i32,
    seed: i64,
) -> impl std::iter::Iterator<Item=(name!(index, i32), name!(mens_name, &'static str))> {
    let mut r = StdRng::seed_from_u64(seed as u64);
    (1..=num_rows).map(move |i| (i, MENS_NAMES[r.gen_range(0..MENS_NAMES.len() - 1)]))
}

/// ```funcname
/// faker_mens_name
/// ```
#[pg_extern]
fn faker_mens_name_iter(
    num_rows: i32,
) -> impl std::iter::Iterator<Item=(name!(index, i32), name!(mens_name, &'static str))> {
    let mut r = rand::thread_rng();
    (1..=num_rows).map(move |i| (i, MENS_NAMES[r.gen_range(0..MENS_NAMES.len() - 1)]))
}


#[pg_extern]
fn faker_women_name() -> &'static str {
    WOMEN_NAMES[rand::thread_rng().gen_range(0..WOMEN_NAMES.len() - 1)]
}

/// ```funcname
/// faker_mens_name_with_seed
/// ```
#[pg_extern]
fn faker_women_name_with_seed(seed: i64) -> &'static str {
    WOMEN_NAMES[StdRng::seed_from_u64(seed as u64).gen_range(0..WOMEN_NAMES.len() - 1)]
}

/// ```funcname
/// faker_women_name
/// ```
#[pg_extern]
fn faker_women_name_with_seed_iter(
    num_rows: i32,
    seed: i64,
) -> impl std::iter::Iterator<Item=(name!(index, i32), name!(mens_name, &'static str))> {
    let mut r = StdRng::seed_from_u64(seed as u64);
    (1..=num_rows).map(move |i| (i, WOMEN_NAMES[r.gen_range(0..WOMEN_NAMES.len() - 1)]))
}

/// ```funcname
/// faker_women_name
/// ```
#[pg_extern]
fn faker_women_name_iter(
    num_rows: i32,
) -> impl std::iter::Iterator<Item=(name!(index, i32), name!(mens_name, &'static str))> {
    let mut r = rand::thread_rng();
    (1..=num_rows).map(move |i| (i, WOMEN_NAMES[r.gen_range(0..WOMEN_NAMES.len() - 1)]))
}

#[pg_extern]
fn faker_last_name() -> &'static str {
    LAST_NAMES[rand::thread_rng().gen_range(0..LAST_NAMES.len() - 1)]
}


/// ```funcname
/// faker_last_name_with_seed
/// ```
#[pg_extern]
fn faker_last_name_with_seed(seed: i64) -> &'static str {
    LAST_NAMES[StdRng::seed_from_u64(seed as u64).gen_range(0..LAST_NAMES.len() - 1)]
}


/// ```funcname
/// faker_last_name
/// ```
#[pg_extern]
fn faker_last_name_with_seed_iter(
    num_rows: i32,
    seed: i64,
) -> impl std::iter::Iterator<Item=(name!(index, i32), name!(mens_name, &'static str))> {
    let mut r = StdRng::seed_from_u64(seed as u64);
    (1..=num_rows).map(move |i| (i, LAST_NAMES[r.gen_range(0..LAST_NAMES.len() - 1)]))
}

/// ```funcname
/// faker_last_name
/// ```
#[pg_extern]
fn faker_last_name_iter(
    num_rows: i32,
) -> impl std::iter::Iterator<Item=(name!(index, i32), name!(women_name, &'static str))> {
    let mut r = rand::thread_rng();
    (1..=num_rows).map(move |i| (i, LAST_NAMES[r.gen_range(0..LAST_NAMES.len() - 1)]))
}


/// ```funcname
/// faker_first_and_last_name
/// ```
#[pg_extern]
fn faker_first_and_last_name_with_seed_iter(
    num_rows: i32,
    seed: i64,
) -> impl std::iter::Iterator<Item=(name!(index, i32), name!(first_name, &'static str), name!(last_name, &'static str))> {
    let mut r = StdRng::seed_from_u64(seed as u64);
    (1..=num_rows).map(move |i| (i, random_seeded_first_name_gendered(&mut r, 0.5), get_seeded_value(&mut r, LAST_NAMES)))
}

/// ```funcname
/// faker_first_and_last_name
/// ```
#[pg_extern]
fn faker_first_and_last_name_iter(
    num_rows: i32,
) -> impl std::iter::Iterator<Item=(name!(index, i32), name!(first_name, &'static str), name!(last_name, &'static str))> {
    let mut r = rand::thread_rng();
    (1..=num_rows).map(move |i| (i, random_seeded_first_name_gendered(&mut r, 0.5), get_seeded_value(&mut r, LAST_NAMES)))
}


