# axum_demo

Rust backend with [axum](https://github.com/tokio-rs/axum) and [diesel](https://github.com/diesel-rs/diesel).

[SQlite3](https://sqlite.org/index.html) as Relational DataBase

## Objective

---

It is a backend written in Rust to understand how the Axum framework works.
Users can register themselves and add new babies to track their sleeping and eating patterns.

## How to run it

---

### CLI

Launch application

`cargo run`

Endpoint => <http://127.0.0.0:3000>

### Migrations

Create a new migration

`diesel migration generate X`

inside up.sql:

```sql
ALTER TABLE user_model
add COLUMN email TEXT;
```

Apply a migration

`diesel migration run`

Re apply a migration

`diesel migration redo`

`diesel migration redo --all`

Revert a migration

`diesel migration revert`

## Endpoints

 ---

### Users: `/api/auth`

| Route     | Method | Function              | Parameters | Arguments                                  |
| --------- | ------ | --------------------- | ---------- | ------------------------------------------ |
| /register | `post` | Create a new user     | Body: Json | {username, password, email, name, surname} |
| /all      | `get`  | Get all users         |
| /user     | `post` | find user by username | Body: Json | {username}                                 |
| /login    | `post` | login user            | Body: Json | {username, password}                       |

### Tasks: `/api/baby`

| Route      | Method | Function                 | Parameters                            | Arguments            |
| ---------- | ------ | ------------------------ | ------------------------------------- | -------------------- |
| /new       | `post` | Add new baby             | Path: i32 \| Body: Json               | user_id \| {name}    |
| /new       | `post` | Add new baby by username | Body: Json \| Query: <String, String> | {name} \| ?username= |
| /all       | `get`  | Get all babies in system |                                       |                      |

## Dependencies

- Axum:

    ```toml
    axum = "0.6.15"
    hyper = { version = "0.14.26", features = ["full"] }
    tokio = { version = "1.27.0", features = ["full"] }
    tower = "0.4.13"
    ```

- Serde:

    Necessary for serialisation and de-serialisation of structs.

    ```toml
    serde = { version = "1.0.160", features = ["derive"] }
    ```

- diesel:

    ORM and simpler queries

    ```toml
    diesel = {version = "*", features = ["sqlite", "returning_clauses_for_sqlite_3_35"]}
    ```

    features:
  - "sqlite" => support for SQlite database
  - "returning_clauses_for_sqlite_3_35" => support returning clauses

- Dotenvy:

    Reading `.env` files

    ```toml
    dotenvy = "*"
    ```

- Axum-macros:

    Better error comments, use `#[axum_macros::debug_handler]`

    ```toml
    axum-macros = "0.3.7"
    ```

- SQlite version: 3.37.2

## Docs

[axum](https://docs.rs/axum/latest/axum/)

[diesel](https://docs.rs/diesel/latest/diesel/)
