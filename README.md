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
    println!("PgUint types can be converted to and from BigDecimals, and are storable in an sqlx::Postgres database.");
    println!("If you load a PgUint from a database successfully, you can be sure that it's a valid fixed-size unsigned integer.");
}
```
