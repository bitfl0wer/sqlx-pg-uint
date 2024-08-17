# sqlx-pg-uint

`SQLx` extension to support working with Rust unsigned integers in PostgreSQL.

---

This crate provides types with `sqlx::{Encode, Decode, Type}` implemented for them, which allow you
to work with fixed-size unsigned integers in PostgreSQL.
