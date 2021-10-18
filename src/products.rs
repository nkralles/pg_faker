use pgx::*;

extern crate rand_regex;
extern crate regex_syntax;

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;


#[pg_extern]
// faker_sku_number will generate a random SKU number XXXX-XXX-XXXX
fn faker_sku_number() -> String {
    let mut rng = rand::thread_rng();
    let mut parser = regex_syntax::ParserBuilder::new().unicode(false).build();
    let hir = parser.parse(r"\d{4}-\d{3}-\d{4}").unwrap();
    let gen = rand_regex::Regex::with_hir(hir, 100).unwrap();
    rng.sample(&gen)
}

/// ```funcname
/// faker_sku_number
/// ```
#[pg_extern]
// faker_sku_number will generate a random SKU number XXXX-XXX-XXXX
fn faker_sku_number_with_seed(seed: i64) -> String {
    let mut rng = StdRng::seed_from_u64(seed as u64);
    let mut parser = regex_syntax::ParserBuilder::new().unicode(false).build();
    let hir = parser.parse(r"\d{4}-\d{3}-\d{4}").unwrap();
    let gen = rand_regex::Regex::with_hir(hir, 100).unwrap();
    rng.sample(&gen)
}

/// ```funcname
/// faker_sku_number
/// ```
#[pg_extern]
// faker_sku_number will generate a random SKU number XXXX-XXX-XXXX
fn faker_sku_number_with_regex(regex: String) -> String {
    let mut rng = rand::thread_rng();
    let mut parser = regex_syntax::ParserBuilder::new().unicode(false).build();
    let hir = parser.parse(regex.as_str()).unwrap();
    let gen = rand_regex::Regex::with_hir(hir, 100).unwrap();
    rng.sample(&gen)
}

/// ```funcname
/// faker_sku_number
/// ```
#[pg_extern]
// faker_sku_number will generate a random SKU number XXXX-XXX-XXXX
fn faker_sku_number_with_regex_and_seed(regex: String, seed: i64) -> String {
    let mut rng = StdRng::seed_from_u64(seed as u64);
    let mut parser = regex_syntax::ParserBuilder::new().unicode(false).build();
    let hir = parser.parse(regex.as_str()).unwrap();
    let gen = rand_regex::Regex::with_hir(hir, 100).unwrap();
    rng.sample(&gen)
}

/// ```funcname
/// faker_sku_numbers
/// ```
#[pg_extern]
fn faker_sku_numbers_iter(
    num_rows: i64,
) -> impl std::iter::Iterator<Item=(name!(index, i64), name!(sku_number, String))> {
    let mut rng = rand::thread_rng();
    let mut parser = regex_syntax::ParserBuilder::new().unicode(false).build();
    let hir = parser.parse(r"\d{4}-\d{3}-\d{4}").unwrap();
    let gen = rand_regex::Regex::with_hir(hir, 100).unwrap();
    (1..=num_rows).map(move |i| (i,  rng.sample(&gen)))
}

/// ```funcname
/// faker_sku_numbers
/// ```
#[pg_extern]
fn faker_sku_numbers_regex_iter(
    num_rows: i64,
    regex: String
) -> impl std::iter::Iterator<Item=(name!(index, i64), name!(sku_number, String))> {
    let mut rng = rand::thread_rng();
    let mut parser = regex_syntax::ParserBuilder::new().unicode(false).build();
    let hir = parser.parse(regex.as_str()).unwrap();
    let gen = rand_regex::Regex::with_hir(hir, 100).unwrap();
    (1..=num_rows).map(move |i| (i,  rng.sample(&gen)))
}


/// ```funcname
/// faker_sku_numbers
/// ```
#[pg_extern]
fn faker_sku_numbers_seed_iter(
    num_rows: i64,
    seed: i64
) -> impl std::iter::Iterator<Item=(name!(index, i64), name!(sku_number, String))> {
    let mut rng = StdRng::seed_from_u64(seed as u64);
    let mut parser = regex_syntax::ParserBuilder::new().unicode(false).build();
    let hir = parser.parse(r"\d{4}-\d{3}-\d{4}").unwrap();
    let gen = rand_regex::Regex::with_hir(hir, 100).unwrap();
    (1..=num_rows).map(move |i| (i,  rng.sample(&gen)))
}

/// ```funcname
/// faker_sku_numbers
/// ```
#[pg_extern]
fn faker_sku_numbers_regex_seed_iter(
    num_rows: i64,
    regex: String,
    seed: i64
) -> impl std::iter::Iterator<Item=(name!(index, i64), name!(sku_number, String))> {
    let mut rng = StdRng::seed_from_u64(seed as u64);
    let mut parser = regex_syntax::ParserBuilder::new().unicode(false).build();
    let hir = parser.parse(regex.as_str()).unwrap();
    let gen = rand_regex::Regex::with_hir(hir, 100).unwrap();
    (1..=num_rows).map(move |i| (i,  rng.sample(&gen)))
}