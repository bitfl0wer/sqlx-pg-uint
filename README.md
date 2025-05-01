# sqlx-pg-uint

`SQLx` extension to support working with Rust unsigned integers in PostgreSQL.

---

This crate provides types with `sqlx::{Encode, Decode, Type}` implemented for them, which allow you
to work with fixed-size unsigned integers in PostgreSQL.

```rs
use sqlx_pg_uint::PgU64;

fn main() {
    let a_u64_number = 2937854645u64;
    let pg_u_64 = PgU64::from(a_u64_number);
    println!("PgU64: {}", pg_u_64);
    let back_to_u64: u64 = pg_u_64.to_uint();
    println!("Back to u64: {}", back_to_u64);
    println!(
        "Maths work the same way as you'd expect: {}",
        PgU64::from(67) + PgU64::from(2) * PgU64::from(3) / PgU64::from(3)
    );
    println!(
        "Interact with the underlying BigDecimal type directly: {}",
        pg_u_64.as_big_decimal()
    );
    println!("You can also convert an Option<PgUInt> to a Option<uint> easily.")
    let somepguint: Option<PgU32> = Some(PgU32::from(123u32));
    let someuint: Option<u32> = somepguint.to_option_uint();
    assert_eq!(someuint, Some(123u32));
    println!("PgUint types can be converted to and from BigDecimals, and are storable in an sqlx::Postgres database.");
    println!("If you load a PgUint from a database successfully, you can be sure that it's a valid fixed-size unsigned integer.");
}
```

When defining a column for a PostgreSQL table, which should store a fixed-size unsigned integer,
you should use the `NUMERIC` type.

| Rust Type | PostgreSQL Type  |
| --------- | ---------------- |
| `PgU8`    | `NUMERIC(3, 0)`  |
| `PgU16`   | `NUMERIC(5, 0)`  |
| `PgU32`   | `NUMERIC(10, 0)` |
| `PgU64`   | `NUMERIC(20, 0)` |
| `PgU128`  | `NUMERIC(39, 0)` |

Additionally, you are advised to use `constraints` to ensure that the value stored in the column is
a valid fixed-size unsigned integer, guaranteed to be in range for the type.

```sql
CREATE TABLE my_table (
    -- `id` is an unsigned 64-bit integer in the corresponding Rust struct
    id numeric(20, 0) not null constraint chk_id_range check (id >= 0 AND id <= 18446744073709551615),
);
```

> Constraining columns (or Rust types) to only store valid values is not a recommendation specific to
> this crate, but a general best practice to avoid faulty states in your application.

## serde

This crate also provides serde de-/serialization, if the `serde` feature is enabled.

With the `serde` feature enabled, you can use the `PgUint` types in structs that you want to serialize and deserialize.

Types are serialized as their respective unsigned integer values, and deserialized using the underlying `Deserialize` trait implemented for `BigDecimal`.

## MSRV

1.74.1
