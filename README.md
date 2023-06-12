# Nighty night

## UNDER CONSTRUCTION

![Sign](docs/img/work-in-progress-yellow.png)

Project is not ready. Work in progress!

## Objective

It is an application for recording and monitoring a newborn's sleeping and eating patterns.

Users can register in the application and add entries as they see fit.

It is a backend written in Rust with the help of the Axum framework.
It has a session service provided by Redis. The main database is in PostgreSQL.

## How to run it

### DB

Set up a redis and postgreSQL servers.

If installed locally:

```bash
sudo service redis-server start
```

```bash
sudo service postgresql start 
```

Or run both commands at same time with:

```bash
sudo service redis-server start && sudo service postgresql start
```

### .ENV

Build an .env file in the root folder with these environments variables:

```.env
BRANCH="local"
DATABASE_URL=postgres://{username}:{password}@localhost/{db_name}
LOGGER_LEVEL=debug
ADDRESS=127.0.0.0
PORT=3000
REDIS_ADDRESS=127.0.0.1
REDIS_PORT=6379
SESSION_DURATION=600
```

### Diesel-cli

Install [libpq](https://www.postgresql.org/docs/current/libpq.html)

```bash
sudo apt-get update
sudo apt-get install libpq-dev
```

Install [diesel cli](https://diesel.rs/guides/getting-started):

```bash
cargo install diesel_cli --no-default-features --features postgres
```

run migrations:

```bash
diesel migration run
```

### CLI

Launch application

`cargo build --release`

`exec ./target/release/nighty_night`

Test Endpoint => <http://127.0.0.0:3000/api/auth>

## Endpoints

### Users: `/api/auth`

| Route     | Method | Function              | Parameters | Arguments                                  |
| --------- | ------ | --------------------- | ---------- | ------------------------------------------ |
| /         | `get`  | Endpoint test         |            |                                            |
| /register | `post` | Create a new user     | Body: Json | {username, password, email, name, surname} |
| /all      | `get`  | Get all users         |
| /user     | `post` | find user by username | Body: Json | {username}                                 |
| /login    | `post` | login user            | Body: Json | {username, password}                       |

### Baby: `/api/baby`

| Route             | Method | Function                                     | Parameters              | Arguments                 |
| ----------------- | ------ | -------------------------------------------- | ----------------------- | ------------------------- |
| /new              | `post` | Add new baby                                 | Body: Json              | {name, birthdate}         |
| /:baby_id         | `get`  | Get baby info by id                          | Path: i32               |                           |
| /:baby_id/meals   | `get`  | Get all meals associated to a baby           | Path: i32               |                           |
| /:baby_id/meals   | `post` | Add new meals to an associated baby          | Path: i32 \| Body: Json | {date, quantity, elapsed} |
| /:baby_id/dreams  | `get`  | Get all sleep records associated to a baby   | Path: i32               |                           |
| /:baby_id/dreams  | `post` | Add new sleep patterns to an associated baby | Path: i32 \| Body: Json | {from_date, to_date }     |
| /:baby_id/weights | `get`  | Get all weight measures associated to a baby | Path: i32               |                           |
| /:baby_id/weights | `post` | Add new weight measure to an associated baby | Path: i32 \| Body: Json | {date, value }            |
| /all              | `get`  | Get all babies in system                     |                         |                           |

## Docs

[Axum](https://docs.rs/axum/latest/axum/)

[Diesel](https://docs.rs/diesel/latest/diesel/)

[Redis](https://redis.io/)

[PostgreSQL](https://www.postgresql.org/)

## APP ROADMAP

Proposed layout.

- [X] Implement tracing system.
- [X] Update Cargo.toml and license.
- [X] Authentication and session.
- [X] Migrate database to postgress.
- [X] Time and dates.
- [X] Implement meals and dreams tables.
- [X] Set up associations.
- [ ] User profile.
- [ ] Update fields.
- [ ] Elapsed times.
- [ ] Recovery system.
- [ ] Docker.
- [ ] Kubernetes.

This layout is not set in stone. It can, and possibly will, change, neither they're in order.

## License

[Apache-2.0](http://www.apache.org/licenses/LICENSE-2.0)

Copyright 2023 Jaime Alvarez Fernandez
