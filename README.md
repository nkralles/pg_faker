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

## faker products

#### sku numbers
```sql
-- pseudo product sku numbers  
select * from faker_sku_number();
 faker_sku_number 
------------------
 2745-504-0516
(1 row)

-- custom regex expression, seed optional
select faker_sku_number(
               seed := 8675319,
               regex := '97[89]-[0-9]{2,5}-[0-9]{1,7}-[0-9]{1,6}-[0-9]'
           ) as internal_number;
internal_number
-------------------
978-4582-1-0980-1
(1 row)

-- as a record

select *
from faker_sku_numbers(
             num_rows := 4,
             seed := 8675319,
             regex := '97[89]-[0-9]{2,5}-[0-9]{1,7}-[0-9]{1,6}-[0-9]'
         );
index |      sku_number      
-------+----------------------
     1 | 978-4582-1-0980-1
     2 | 979-224-32054-6-0
     3 | 979-8115-7956291-1-3
     4 | 978-20142-0199-891-7
(4 rows)

```

