# Nighty night

## Objective

---
It is an application for recording and monitoring a newborn's sleeping and eating patterns.

Users can register in the application and add entries as they see fit.

It is a backend written in Rust with the help of the Axum framework.
It has a session service provided by Redis. The main database is SQLite.

## How to run it

---

### CLI

Launch application

`cargo run`

Endpoint => <http://127.0.0.0:3000>

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

## Docs

[axum](https://docs.rs/axum/latest/axum/)

[diesel](https://docs.rs/diesel/latest/diesel/)

[redis](https://redis.io/)

[SQlite3](https://sqlite.org/index.html)
