use pgx::*;
use rand::Rng;

#[path = "data/names.rs"] mod names;
use crate::faker::names::*;

#[pg_extern]
fn rand_first_name() ->  &'static str {
    let idx = rand::thread_rng().gen_range(0..(MENS_NAMES.len()+WOMEN_NAMES.len() -1));
    if idx < MENS_NAMES.len(){
        return MENS_NAMES[idx];
    }
   return  WOMEN_NAMES[idx - MENS_NAMES.len()];
}

#[pg_extern]
fn rand_mens_name() ->  &'static str {
    MENS_NAMES[rand::thread_rng().gen_range(0..MENS_NAMES.len()-1)]
}

#[pg_extern]
fn rand_women_name() ->  &'static str {
    WOMEN_NAMES[rand::thread_rng().gen_range(0..WOMEN_NAMES.len()-1)]
}
