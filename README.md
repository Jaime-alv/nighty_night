# Nighty night

## Index

- [Nighty night](#nighty-night)
  - [Index](#index)
  - [UNDER CONSTRUCTION](#under-construction)
  - [Objective](#objective)
  - [How to run it](#how-to-run-it)
    - [Manual installation](#manual-installation)
      - [DB](#db)
      - [.ENV](#env)
      - [Diesel-cli](#diesel-cli)
      - [CLI](#cli)
    - [Docker](#docker)
      - [Build docker image](#build-docker-image)
      - [Run compose image](#run-compose-image)
      - [Stop docker compose](#stop-docker-compose)
      - [Delete docker compose](#delete-docker-compose)
      - [.env file](#env-file)
      - [Docker flags](#docker-flags)
    - [Kubernetes](#kubernetes)
      - [Config map](#config-map)
      - [Secrets](#secrets)
  - [Default users](#default-users)
  - [Endpoints](#endpoints)
    - [Users: `/api/auth`](#users-apiauth)
    - [Baby: `/api/baby`](#baby-apibaby)
    - [Meals: `/api/baby/:baby_id`](#meals-apibabybaby_id)
    - [Dreams: `/api/baby/:baby_id`](#dreams-apibabybaby_id)
    - [Weights: `/api/baby/:baby_id`](#weights-apibabybaby_id)
  - [Docs](#docs)
  - [APP ROADMAP](#app-roadmap)
  - [License](#license)

## UNDER CONSTRUCTION

![Sign](docs/img/work-in-progress-yellow.png)

Project is not ready. Work in progress!

## Objective

It is an application for recording and monitoring a newborn's sleeping and eating patterns.

Users can register in the application and add entries as they see fit.

It is a backend written in Rust with the help of the Axum framework.
It has a session service provided by Redis. The main database is in PostgreSQL.

## How to run it

### Manual installation

#### DB

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

#### .ENV

Build an .env file, with name `local.env` inside `./key` folder with these environments variables:

`local.env`

```.env
BRANCH=local
POSTGRES_PASSWORD=password
POSTGRES_USER=username
POSTGRES_DB=nighty_night_db
POSTGRES_HOST=localhost
POSTGRES_PORT=5432
LOGGER_LEVEL=debug
ADDRESS=127.0.0.1
PORT=3000
REDIS_ADDRESS=127.0.0.1
REDIS_PORT=6379
SESSION_DURATION=600
```

#### Diesel-cli

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

#### CLI

Launch application

`cargo build --release`

`exec ./target/release/nighty_night`

Test Endpoint => <http://127.0.0.0:3000/api/auth>

### Docker

#### Build docker image

```bash
docker build -t nighty_night:latest -f ./docker/Dockerfile .
```

Optional:

```bash
docker run --env-file .env -d -p 3000:3000 --name rs nighty_night:latest
```

#### Run compose image

```bash
docker compose --env-file ./key/docker.env -f ./docker/compose.yaml up -d
```

#### Stop docker compose

```bash
docker compose -f ./docker/compose.yaml stop
```

#### Delete docker compose

```bash
docker compose -f ./docker/compose.yaml down
```

#### .env file

```.env
BRANCH=branch_name
POSTGRES_PASSWORD=password
POSTGRES_USER=user
POSTGRES_DB=app_db
POSTGRES_HOST=host.docker.internal
POSTGRES_PORT=8080
LOGGER_LEVEL=debug
<!-- Leave ADDRESS to 0.0.0.0 in docker -->
ADDRESS=0.0.0.0
PORT=3000
REDIS_ADDRESS=host.docker.internal
REDIS_PORT=8081
SESSION_DURATION=600
```

Modify ports accordingly. This is an example with default ports. Docker compose file runs on ports 8080 for postgreSQL and 8081 for redis.

#### Docker flags

-e = environment variable

-d = container runs as a background application

-p =  maps container ports to host ports

--rm = will delete container after stopping the app

--name = image name

--env-file = path to .env file

### Kubernetes

Create secrets and config maps files. Examples below.

If running in local:

`minikube start`

Tell docker where to build the image:

`eval $(minikube docker-env)`

Create nighty night image inside kubernetes:

`docker build -t nighty_night:latest -f docker/Dockerfile .`

Load secrets and config map in kubernetes' cluster:

`kubectl create -f kubernetes/local/secrets.yaml`

`kubectl apply -f kubernetes/local/configmap.yaml`

Apply all deployments and services:

```bash
kubectl apply -f kubernetes/00-postgreSQL-configmap.yaml
kubectl apply -f kubernetes/01-postgreSQL-service.yaml
kubectl apply -f kubernetes/02-postgreSQL-pvc.yaml
kubectl apply -f kubernetes/03-postgreSQL-deployment.yaml
kubectl apply -f kubernetes/04-redis-deployment.yaml
kubectl apply -f kubernetes/05-redis-service.yaml
kubectl apply -f kubernetes/06-nighty-night-deployment.yaml
kubectl apply -f kubernetes/07-nighty-night-service.yaml
```

Port forward deployment pod to desired port:

`kubectl port-forward nighty-night-deployment from:to`

#### Config map

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: nighty-night-config-map
  labels:
    group: nighty.night.app
    app: nighty-night
    branch: minikube
data:
  # APP
  BRANCH: K8s
  POSTGRES_DB: nighty_night_db
  POSTGRES_HOST: psql-service.default.svc.cluster.local
  POSTGRES_PORT: '5432'
  LOGGER_LEVEL: debug
  ADDRESS: 0.0.0.0
  PORT: '3000'
  REDIS_ADDRESS: redis-service.default.svc.cluster.local
  REDIS_PORT: '6379'
  SESSION_DURATION: '600'  
```

#### Secrets

Get the values for each secret with:

`echo -n "value" | base64`

```yaml
apiVersion: v1
kind: Secret
metadata:
  name: nighty-night-secrets
  labels:
    group: nighty.night.app
    app: nighty-night-API
    branch: minikube
type: Opaque
data:
  POSTGRES_USER: ZGJh 
  POSTGRES_PASSWORD: MTIzNA==
```

## Default users

| Rol   | Username | Password  |
| ----- | -------- | --------- |
| Guest | guest    | anonymous |
| Admin | admin    | admin     |

## Endpoints

### Users: `/api/auth`

| Route     | Method  | Function              | Parameters | Arguments                                  |
| --------- | ------- | --------------------- | ---------- | ------------------------------------------ |
| /         | `get`   | Endpoint test         |            |                                            |
| /register | `post`  | Create a new user     | Body: Json | {username, password, email, name, surname} |
| /all      | `get`   | Get all users         |            |                                            |
| /user     | `post`  | find user by username | Body: Json | {username}                                 |
| /login    | `post`  | login user            | Body: Json | {username, password}                       |
| /profile  | `get`   | Get user profile      |            |                                            |
| /profile  | `patch` | Update user profile   | Body: Json | {name, surname, email, }                   |

### Baby: `/api/baby`

| Route     | Method  | Function                 | Parameters              | Arguments         |
| --------- | ------- | ------------------------ | ----------------------- | ----------------- |
| /new      | `post`  | Add new baby             | Body: Json              | {name, birthdate} |
| /:baby_id | `get`   | Get baby info by id      | Path: i32               |                   |
| /:baby_id | `patch` | Update baby info by id   | Path: i32 \| Body: Json | {name, birthdate} |
| /all      | `get`   | Get all babies in system |                         |                   |

### Meals: `/api/baby/:baby_id`

| Route                                              | Method  | Function                                 | Parameters         | Arguments                 |
| -------------------------------------------------- | ------- | ---------------------------------------- | ------------------ | ------------------------- |
| /meals                                             | `get`   | Get all meals associated to a baby       |                    |                           |
| /meals                                             | `post`  | Add new meals to an associated baby      | Body: Json         | {date, quantity, elapsed} |
| /meals                                             | `patch` | Update a meal record with any new values | Body: Json         | {date, quantity, elapsed} |
| /meals?date=YYYY-mm-dd                             | `get`   | Get all meals in a given date            |                    |                           |
| /meals/summary?date=YYYY-mm-dd                     | `get`   | Get a summary from one day's data        | String             |                           |
| /meals/summary/today                               | `get`   | Get a summary from today's data          |                    |                           |
| /meals/summary/last?days=X                         | `get`   | Get a summary from last X days           | int                |                           |
| /meals/summary/range?from=YYYY-mm-dd&to=YYYY-mm-dd | `get`   | Get a summary from date X up to date Y   | {String \| String} |                           |

### Dreams: `/api/baby/:baby_id`

| Route                                               | Method  | Function                                  | Parameters         | Arguments             |
| --------------------------------------------------- | ------- | ----------------------------------------- | ------------------ | --------------------- |
| /dreams                                             | `get`   | Get all dreams associated to a baby       |                    |                       |
| /dreams                                             | `post`  | Add new dreams to an associated baby      | Body: Json         | {from_date, to_date } |
| /dreams                                             | `patch` | Update a dream record with any new values | Body: Json         | {from_date, to_date } |
| /dreams?date=YYYY-mm-dd                             | `get`   | Get all dreams in a given date            |                    |                       |
| /dreams/summary?date=YYYY-mm-dd                     | `get`   | Get a summary from one day's data         | String             |                       |
| /dreams/summary/today                               | `get`   | Get a summary from today's data           |                    |                       |
| /dreams/summary/last?days=X                         | `get`   | Get a summary from last X days            | int                |                       |
| /dreams/summary/range?from=YYYY-mm-dd&to=YYYY-mm-dd | `get`   | Get a summary from date X up to date Y    | {String \| String} |                       |

### Weights: `/api/baby/:baby_id`

| Route    | Method  | Function                                     | Parameters | Arguments      |
| -------- | ------- | -------------------------------------------- | ---------- | -------------- |
| /weights | `get`   | Get all weight measures associated to a baby |            |                |
| /weights | `post`  | Add new weight measure to an associated baby | Body: Json | {date, value } |
| /weights | `patch` | Update a measure with any new values         | Body: Json | {date, value } |

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
- [X] Migrate database to PostgreSQL.
- [X] Time and dates.
- [X] Implement meals and dreams tables.
- [X] Set up associations.
- [X] User profile.
- [ ] Admin panel.
- [ ] Logout user.
- [ ] Add co-parenting.
- [X] Update fields.
- [ ] Delete entries.
- [ ] Add entries by batch.
- [X] Elapsed times.
- [ ] Recovery system.
- [X] Docker.
- [X] Kubernetes.

This layout is not set in stone. It can, and possibly will, change, neither they're in order.

## License

[Apache-2.0](http://www.apache.org/licenses/LICENSE-2.0)

Copyright 2023 Jaime Alvarez Fernandez
