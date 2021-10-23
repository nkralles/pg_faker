use pgx::*;

extern crate rand_regex;
extern crate regex_syntax;

#[path = "data/products.rs"]
mod products;

use rand::{Rng, RngCore, SeedableRng};
use rand::rngs::StdRng;
use rand_seeder::{Seeder};
use rand_pcg::Pcg64;
use serde::*;
use uuid::Uuid;


use crate::products::products::{RANDOM_ADJECTIVES, RANDOM_COLORS, RANDOM_THINGS};


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
    (1..=num_rows).map(move |i| (i, rng.sample(&gen)))
}

/// ```funcname
/// faker_sku_numbers
/// ```
#[pg_extern]
fn faker_sku_numbers_regex_iter(
    num_rows: i64,
    regex: String,
) -> impl std::iter::Iterator<Item=(name!(index, i64), name!(sku_number, String))> {
    let mut rng = rand::thread_rng();
    let mut parser = regex_syntax::ParserBuilder::new().unicode(false).build();
    let hir = parser.parse(regex.as_str()).unwrap();
    let gen = rand_regex::Regex::with_hir(hir, 100).unwrap();
    (1..=num_rows).map(move |i| (i, rng.sample(&gen)))
}


/// ```funcname
/// faker_sku_numbers
/// ```
#[pg_extern]
fn faker_sku_numbers_seed_iter(
    num_rows: i64,
    seed: i64,
) -> impl std::iter::Iterator<Item=(name!(index, i64), name!(sku_number, String))> {
    let mut rng = StdRng::seed_from_u64(seed as u64);
    let mut parser = regex_syntax::ParserBuilder::new().unicode(false).build();
    let hir = parser.parse(r"\d{4}-\d{3}-\d{4}").unwrap();
    let gen = rand_regex::Regex::with_hir(hir, 100).unwrap();
    (1..=num_rows).map(move |i| (i, rng.sample(&gen)))
}

/// ```funcname
/// faker_sku_numbers
/// ```
#[pg_extern]
fn faker_sku_numbers_regex_seed_iter(
    num_rows: i64,
    regex: String,
    seed: i64,
) -> impl std::iter::Iterator<Item=(name!(index, i64), name!(sku_number, String))> {
    let mut rng = StdRng::seed_from_u64(seed as u64);
    let mut parser = regex_syntax::ParserBuilder::new().unicode(false).build();
    let hir = parser.parse(regex.as_str()).unwrap();
    let gen = rand_regex::Regex::with_hir(hir, 100).unwrap();
    (1..=num_rows).map(move |i| (i, rng.sample(&gen)))
}


/// ```funcname
/// faker_random_product
/// ```
#[pg_extern]
fn random_product() -> &'static str {
    RANDOM_THINGS[rand::thread_rng().gen_range(0..RANDOM_THINGS.len() - 1)]
}


#[derive(PostgresType, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ProductListing {
    thing: String,
    sku: String,
}

/// ```funcname
/// faker_random_product_listing
/// ```
#[pg_extern]
fn random_product_listing() -> impl std::iter::Iterator<Item=(name!(sku, String), name!(product, String))> {
    let prod = format!("{} {} {}",
                       RANDOM_ADJECTIVES[rand::thread_rng().gen_range(0..RANDOM_ADJECTIVES.len() - 1)],
                       RANDOM_COLORS[rand::thread_rng().gen_range(0..RANDOM_COLORS.len() - 1)],
                       RANDOM_THINGS[rand::thread_rng().gen_range(0..RANDOM_THINGS.len() - 1)]);
    let mut rng: Pcg64 = Seeder::from(prod.as_str()).make_rng();
    (1..=1).map(move |i| (gen_seeded_sku(&mut rng), prod.to_string()))
}

/// ```funcname
/// faker_random_product_listings
/// ```
#[pg_extern]
fn random_product_listings(num_rows: i64) -> impl std::iter::Iterator<Item=(name!(sku, String), name!(product, String), name!(price, f64))> {
    (1..=num_rows).map(move |i| (make_random_product()))
}

fn make_random_product() -> (String, String, f64) {
    let prod = format!("{} {} {}",
                       RANDOM_ADJECTIVES[rand::thread_rng().gen_range(0..RANDOM_ADJECTIVES.len() - 1)],
                       RANDOM_COLORS[rand::thread_rng().gen_range(0..RANDOM_COLORS.len() - 1)],
                       RANDOM_THINGS[rand::thread_rng().gen_range(0..RANDOM_THINGS.len() - 1)]);
    let mut rng: Pcg64 = Seeder::from(prod.as_str()).make_rng();

    let mut parser = regex_syntax::ParserBuilder::new().unicode(false).build();
    let hir = parser.parse(r"[\d]{1,5}\.[\d]{2}").unwrap();
    let gen = rand_regex::Regex::with_hir(hir, 100).unwrap();
    let pstr: String = rng.sample(&gen);
    (gen_seeded_sku(&mut rng), prod, pstr.parse::<f64>().unwrap())
}

/// ```funcname
/// faker_transactions
/// ```
#[pg_extern]
fn random_transactions(
    entity_pool_size: i64,
    num_transactions: i64,
    max_transaction_amount: f64,
    allow_circular: bool) -> impl std::iter::Iterator<Item=(name!(transaction_id, String),
                                                            name!(to, String),
                                                            name!(from, String),
                                                            name!(transcation_amount, f64))> {
    if entity_pool_size <= 1 {
        error!("{}", "entity pool size must be > 1")
    }
    // let pool: Vec<String>=(1..entity_pool_size).map(Uuid::new_v4().to_string()).collect();
    // (0..=entity_pool_size-1).map(move |i| pool[i],pool[i])
    let mut pool: Vec<String> = Vec::with_capacity(entity_pool_size as usize);
    for _ in 0..entity_pool_size {
        pool.push(Uuid::new_v4().to_string());
    }
    let mut rng = rand::thread_rng();

    (1..=num_transactions).map(move |i| (
        Uuid::new_v4().to_string(),
        gen_random_transaction_pair(&pool, &mut rng, allow_circular),
        f64::trunc(rng.gen_range(0.0..max_transaction_amount) * 100.0) / 100.0).into_flattened())
}

trait DumpStringTuple {
    type Output;
    fn into_flattened(self) -> Self::Output;
}

impl<A, B, C, D> DumpStringTuple for (A, (B, C), D) {
    type Output = (A, B, C, D);
    fn into_flattened(self) -> Self::Output {
        (self.0, (self.1).0, (self.1).1, self.2)
    }
}

fn gen_random_transaction_pair(pool: &Vec<String>, rng: &mut dyn RngCore, allow_circular: bool) -> (String, String) {
    let to = &pool[rng.gen_range(0..pool.len())];
    let mut from = &pool[rng.gen_range(0..pool.len())];
    if !allow_circular {
        while from == to {
            from = &pool[rng.gen_range(0..pool.len())];
        }
    }
    (to.to_string(), from.to_string())
}


fn gen_seeded_sku(seed: &mut dyn RngCore) -> String {
    let mut parser = regex_syntax::ParserBuilder::new().unicode(false).build();
    let hir = parser.parse(r"\d{4}-\d{3}-\d{4}").unwrap();
    let gen = rand_regex::Regex::with_hir(hir, 100).unwrap();
    seed.sample(&gen)
}

fn get_seeded_value(seed: &mut dyn RngCore, val: &'static [&'static str]) -> &'static str {
    val[(*seed).gen_range(0..val.len() - 1)]
}