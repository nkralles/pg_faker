use pgx::*;

mod names;
mod products;

pg_module_magic!();

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_hello_pg_faker() {
        assert_eq!("Hello, pg_faker", crate::hello_pg_faker());
    }

}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
