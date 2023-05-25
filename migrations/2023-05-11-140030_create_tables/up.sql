-- Copyright 2023 Jaime Alvarez Fernandez

-- Licensed under the Apache License, Version 2.0 (the "License");
-- you may not use this file except in compliance with the License.
-- You may obtain a copy of the License at

--     http://www.apache.org/licenses/LICENSE-2.0

-- Unless required by applicable law or agreed to in writing, software
-- distributed under the License is distributed on an "AS IS" BASIS,
-- WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
-- See the License for the specific language governing permissions and
-- limitations under the License.

-- create table users
CREATE TABLE IF NOT EXISTS
    users (
        id INTEGER PRIMARY KEY not NULL,
        username TEXT not NULL,
        password TEXT not null,
        name TEXT,
        surname TEXT,
        email TEXT not null,
        active BOOLEAN not null
    );

-- create table roles
CREATE TABLE IF NOT EXISTS
    roles (
        id INTEGER PRIMARY KEY not null,
        name TEXT not NULL
    );

-- create table babies
CREATE TABLE IF NOT EXISTS
    babies (
        id INTEGER PRIMARY KEY not NULL,
        name TEXT not NULL
    );

-- create table dreams
CREATE TABLE IF NOT EXISTS
    dreams (
        id INTEGER PRIMARY KEY not NULL,
        baby_id INTEGER not null references babies (id),
        from_date TIMESTAMP not null,
        from_time TIMESTAMP not null,
        to_date TIMESTAMP,
        to_time TIMESTAMP
    );

-- create tables meals
CREATE TABLE IF NOT EXISTS
    meals (
        id INTEGER PRIMARY KEY not NULL,
        baby_id INTEGER not null references babies (id),
        date TIMESTAMP not null,
        quantity INTEGER,
        elapsed INTEGER
    );

-- create intermediate table roles-users
CREATE TABLE IF NOT EXISTS
    users_roles (
        id INTEGER PRIMARY KEY not null,
        rol_id INTEGER not null references roles (id),
        user_id INTEGER not null references users (id) ON DELETE CASCADE ON UPDATE CASCADE
    );

-- create intermediate table baby-user
CREATE TABLE IF NOT EXISTS
    users_babies (
        id INTEGER PRIMARY KEY not null,
        baby_id INTEGER not null references babies(id),
        user_id INTEGER not null references users(id) ON DELETE CASCADE ON UPDATE CASCADE
    );


-- insert roles
insert into
    roles
VALUES
    (0, "admin");

insert into
    roles
VALUES
    (1, "user");

insert into
    roles
VALUES
    (2, "anonymous");