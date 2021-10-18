# pg_faker
postgres faker data to produce usable test data inside postgres

## Installation 
- TBD

## Usaged
### faker names
```sql
-- random gendered 
select faker_first_name(),faker_last_name();
faker_first_name | faker_last_name 
------------------+-----------------
 Leonardo         | Bryant
(1 row)

-- random full name
select faker_full_name();
faker_full_name 
-----------------
 Chase Schmidt
(1 row)

-- random full name
select faker_full_name(867531);
faker_full_name 
-----------------
 Chase Schmidt
(1 row)

-- male probability
select faker_first_name(probability_male := .75, seed := 8675314);
faker_first_name 
------------------
 Tommy
(1 row)


-- random full names as a record
select * from faker_full_names(4);
index |   full_name   
-------+---------------
     1 | Kabir Douglas
     2 | Hamza Foley
     3 | Alani Mason
     4 | Fox Brady
(4 rows)

-- random full name record with bigint as a seed
select * from faker_full_names(3,867531);
index |    full_name     
-------+------------------
     1 | Rivka Delacruz
     2 | Danielle Gardner
     3 | Yareli French
(3 rows)

```

