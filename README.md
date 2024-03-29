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
      - [Build docker compose](#build-docker-compose)
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
    - [Admin: `/api/admin`](#admin-apiadmin)
    - [Pagination](#pagination)
  - [Response](#response)
    - [Message response](#message-response)
    - [Data Response](#data-response)
    - [Error response](#error-response)
  - [Docs](#docs)
    - [Naming conventions](#naming-conventions)
      - [Controller](#controller)
      - [Service](#service)
      - [Repository](#repository)
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

Compiling rust in docker is a really slow process. There is no way around it. Although, final image only weights around 110 Mb, reaching that point the first time can take up to 1000s.

With `cargo chef` speed is a lot better for incremental builds. As long as there are no changes in `Cargo.toml`, dependencies are cached, thus speeding whole process.

#### Build docker image

```bash
docker build -t nighty_night -f ./docker/Dockerfile .
```

Optional, run container:

```bash
docker run --env-file .env -d -p 3000:3000 --name rs nighty_night
```

#### Build docker compose

Create network with app and databases; initialize containers:

```bash
docker compose --env-file ./key/docker.env -f ./docker/compose.yaml up -d
```

Stop containers:

```bash
docker compose -f ./docker/compose.yaml stop
```

Delete containers:

```bash
docker compose -f ./docker/compose.yaml down
```

Delete dangling images and volumes:

```bash
docker image prune && docker volume prune
```

To remove all images which are not used by existing containers, use the `-a` flag:

```bash
docker image prune -a
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

-p = maps container ports to host ports

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
  POSTGRES_PORT: "5432"
  LOGGER_LEVEL: debug
  ADDRESS: 0.0.0.0
  PORT: "3000"
  REDIS_ADDRESS: redis-service.default.svc.cluster.local
  REDIS_PORT: "6379"
  SESSION_DURATION: "600"
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

| Route     | Method   | Function                    | Parameters | Arguments                                  |
| --------- | -------- | --------------------------- | ---------- | ------------------------------------------ |
| /register | `post`   | Create a new user           | Body: Json | {username, password, email, name, surname} |
| /user     | `post`   | find user by username       | Body: Json | {username}                                 |
| /session  | `get`    | Get current user in session |            |                                            |
| /session  | `post`   | login user                  | Body: Json | {username, password}                       |
| /session  | `delete` | logout user                 |            |                                            |
| /profile  | `get`    | Get user profile            |            |                                            |
| /profile  | `patch`  | Update user profile         | Body: Json | {name, surname, email, }                   |
| /profile  | `delete` | Deactivate user             |            |                                            |

### Baby: `/api/baby`

| Route                             | Method   | Function                                     | Parameters                     | Arguments         |
| --------------------------------- | -------- | -------------------------------------------- | ------------------------------ | ----------------- |
| /                                 | `get`    | Get all babies for current user              |                                |                   |
| /                                 | `post`   | Add new baby                                 | Body: Json                     | {name, birthdate} |
| /:baby_id                         | `patch`  | Update baby info by id                       | Path: Uuid \| Body: Json       | {name, birthdate} |
| /:baby_id                         | `delete` | Delete baby and all records associated to it | Path: Uuid                     |                   |
| /:baby_id/share?username=username | `post`   | Associate current baby to another username   | Path: Uuid \| username: String |                   |
| /:baby_id/transfer                | `patch`  | Transfer current baby to another username    | Path: Uuid \| Body: Json       | {username}        |

### Meals: `/api/baby/:baby_id`

| Route                                        | Method   | Function                                     | Parameters                   | Arguments                 |
| -------------------------------------------- | -------- | -------------------------------------------- | ---------------------------- | ------------------------- |
| /meals?all=true                              | `get`    | Get all meals associated to a baby           | all: boolean                 |                           |
| /meals?date=YYYY-mm-dd                       | `get`    | Get all meals in a given date                | date: String                 |                           |
| /meals?from=YYYY-mm-dd&to=YYYY-mm-dd         | `get`    | Get all meals in a given range               | {from: String \| to: String} |                           |
| /meals?last_days=X                           | `get`    | Get all meals from last X days, default to 7 | last_days: integer           |                           |
| /meals                                       | `post`   | Add new meals to an associated baby          | Body: Json                   | {date, quantity, elapsed} |
| /meals/:record                               | `patch`  | Update a meal record with any new values     | Path: Integer \| Body: Json  | {date, quantity, elapsed} |
| /meals/:record                               | `delete` | Delete entry X from DB                       | Path: Integer                |                           |
| /meals/:record                               | `get`    | Get an individual record                     | Path: Integer                |                           |
| /meals/summary?all=bool                      | `get`    | Get all summaries                            | all: Boolean                 |                           |
| /meals/summary?date=YYYY-mm-dd               | `get`    | Get a summary from one day's data            | date: String                 |                           |
| /meals/summary?date=today                    | `get`    | Get a summary from today's data              |                              |                           |
| /meals/summary?last_days=X                   | `get`    | Get a summary from last X days, default to 7 | days: Integer                |                           |
| /meals/summary?from=YYYY-mm-dd&to=YYYY-mm-dd | `get`    | Get a summary from date X up to date Y       | {from: String \| to: String} |                           |

### Dreams: `/api/baby/:baby_id`

| Route                                         | Method   | Function                                      | Parameters                   | Arguments             |
| --------------------------------------------- | -------- | --------------------------------------------- | ---------------------------- | --------------------- |
| /dreams?all=true                              | `get`    | Get all dreams associated to a baby           | all: boolean                 |                       |
| /dreams?date=YYYY-mm-dd                       | `get`    | Get all dreams in a given date                | date: String                 |                       |
| /dreams?from=YYYY-mm-dd&to=YYYY-mm-dd         | `get`    | Get all dreams in a given range               | {from: String \| to: String} |                       |
| /dreams?last_days=X                           | `get`    | Get all dreams from last X days, default to 7 | last_days: integer           |                       |
| /dreams                                       | `post`   | Add new dreams to an associated baby          | Body: Json                   | {from_date, to_date } |
| /dreams/:record                               | `patch`  | Update a dream record with any new values     | Path: Integer \| Body: Json  | {from_date, to_date } |
| /dreams/:record                               | `delete` | Delete entry X from DB                        | Path: Integer                |                       |
| /dreams/:record                               | `get`    | Get an individual record                      | Path: Integer                |                       |
| /dreams/summary?all=bool                      | `get`    | Get all summaries                             | all: Boolean                 |                       |
| /dreams/summary?date=YYYY-mm-dd               | `get`    | Get a summary from one day's data             | date: String                 |                       |
| /dreams/summary?date=today                    | `get`    | Get a summary from today's data               |                              |                       |
| /dreams/summary?days=X                        | `get`    | Get a summary from last X days, default to 7  | days: Integer                |                       |
| /dreams/summary?from=YYYY-mm-dd&to=YYYY-mm-dd | `get`    | Get a summary from date X up to date Y        | {from: String \| to: String} |                       |

### Weights: `/api/baby/:baby_id`

| Route                                  | Method   | Function                                     | Parameters                   | Arguments      |
| -------------------------------------- | -------- | -------------------------------------------- | ---------------------------- | -------------- |
| /weights?all=true                      | `get`    | Get all weight measures associated to a baby | all: boolean                 |                |
| /weights?date=YYYY-mm-dd               | `get`    | Get weight in a given date                   | date: String                 |                |
| /weights?from=YYYY-mm-dd&to=YYYY-mm-dd | `get`    | Get weights in a given range                 | {from: String \| to: String} |                |
| /weights?last_days=X                   | `get`    | Get weights from last X days, default to 30  | last_days: Integer           |                |
| /weights                               | `post`   | Add new weight measure to an associated baby | Body: Json                   | {date, value } |
| /weights/:record                       | `patch`  | Update a measure with any new values         | Path: Integer \| Body: Json  | {date, value } |
| /weights/:record                       | `delete` | Delete entry X from DB                       | Path: Integer                |                |
| /weights/:record                       | `get`    | Get an individual record                     | Path: Integer                |                |

### Admin: `/api/admin`

| Route                 | Method   | Function                            | Parameters     | Arguments        |
| --------------------- | -------- | ----------------------------------- | -------------- | ---------------- |
| /user                 | `get`    | Get all users in db                 |                |                  |
| /user                 | `patch`  | Activate user by id                 | entry: Integer |                  |
| /user                 | `delete` | Delete all inactive users           |                |                  |
| /user?entry=X         | `delete` | Delete a user by id                 | entry: Integer |                  |
| /baby                 | `get`    | Get all babies in db                |                |                  |
| /baby/baby_id?entry=X | `get`    | Get baby info by id                 | entry: Integer |                  |
| /stats                | `get`    | Get number of records & statistics  |                |                  |
| /roles                | `get`    | Get roles and associated statistics |                |                  |
| /roles                | `put`    | Add role to user                    | Body: Json     | {username, role} |
| /roles                | `delete` | Delete role from user               | Body: Json     | {username, role} |

### Pagination

It's implemented on methods that requests several hundred records from database. It needs two (2) parameters, `page` and `per_page`. `page` is page number requested, `per_page` are records per page requested. `page` is always requested and `per_page` could be omitted. By default, `page=1&per_page=100`.

Pagination is implemented by default in `get` requests for:

1. `/api/admin/user`
2. `/api/admin/baby`
3. `/api/baby/:baby_id/dreams`
4. `/api/baby/:baby_id/dreams/summary`
5. `/api/baby/:baby_id/meals`
6. `/api/baby/:baby_id/meals/summary`
7. `/api/baby/:baby_id/weights`

## Response

Response is in json format. It always has `data` field. It may contain an additional key `page_info` when appropriate.

### Message response

Simple response is like:

```json
{
  "message": {
    "status": 201,
    "title": "Created",
    "detail": "New record added."
  }
}
```

### Data Response

Objects contain info separated in two levels. Top level contains `id`, `attributes` and `type`

```json
{
  "data": {
    "attributes": {
      "date": "2023-07-04",
      "elapsed": "00:00",
      "quantity": 145,
      "time": "08:25"
    },
    "id": 169,
    "type": "meal"
  }
}
```

Response, with pagination, is like:

```json
{
  "data": [
    {
      "attributes": {
        "added_on": "2023-08-28T10:14:45.898688",
        "belongs_to": 2,
        "name": "BabyOne"
      },
      "id": 1,
      "type": "Baby"
    },
    {
      "attributes": {
        "added_on": "2023-08-28T10:32:31.408920",
        "belongs_to": 2,
        "name": "BabyTwo"
      },
      "id": 2,
      "type": "Baby"
    }
  ],
  "page_info": {
    "current": 1,
    "total_pages": 1
  }
}
```

### Error response

Error message:

```json
{
  "errors": {
    "status": 404,
    "title": "Not found",
    "detail": "No user found."
  }
}
```

## Docs

[Axum](https://docs.rs/axum/latest/axum/)

[Diesel](https://docs.rs/diesel/latest/diesel/)

[Redis](https://redis.io/)

[PostgreSQL](https://www.postgresql.org/)

### Naming conventions

#### Controller

Start with http request method

- get
- post
- put
- patch
- delete

#### Service

If the function comes from the controller, should follow the pattern:

`<http-request><controller-function-name><service>`

`get_user` >> `get_user_service`

`post_new_baby` >> `post_new_baby_service`

#### Repository

Start with SQL action

- select
- insert
- delete
- update

## APP ROADMAP

Proposed layout.

- [x] Implement tracing system.
- [x] Update Cargo.toml and license.
- [x] Authentication and session.
- [x] Migrate database to PostgreSQL.
- [x] Time and dates.
- [x] Implement meals and dreams tables.
- [x] Set up associations.
- [x] User profile.
- [x] Admin panel.
- [x] Logout user.
- [x] Add co-parenting.
- [x] Update fields.
- [x] Delete entries.
- [ ] Add entries by batch.
- [x] Elapsed times.
- [ ] Recovery system.
- [x] Docker.
- [x] Kubernetes.

This layout is not set in stone. It can, and possibly will, change, neither they're in order.

## License

[Apache-2.0](http://www.apache.org/licenses/LICENSE-2.0)

Copyright 2023 Jaime Alvarez Fernandez
