-- Your SQL goes here
-- create table users
CREATE TABLE
    users (
        id INTEGER PRIMARY KEY not NULL,
        username TEXT not NULL,
        password TEXT not null,
        name TEXT,
        surname TEXT,
        email TEXT not null
    );

-- create table roles
CREATE TABLE
    roles (
        id INTEGER PRIMARY KEY not null,
        name TEXT not NULL
    );

-- create table babies
CREATE TABLE
    babies (
        id INTEGER PRIMARY KEY not NULL,
        name TEXT not NULL
    );

-- create table dreams
CREATE TABLE
    dreams (
        id INTEGER PRIMARY KEY not NULL,
        baby_id INTEGER not null references babies (id),
        from_date TIMESTAMP not null,
        from_time TIMESTAMP not null,
        to_date TIMESTAMP,
        to_time TIMESTAMP
    );

-- create tables meals
CREATE TABLE
    meals (
        id INTEGER PRIMARY KEY not NULL,
        baby_id INTEGER not null references babies (id),
        date TIMESTAMP not null,
        quantity INTEGER,
        elapsed INTEGER
    );

-- create intermediate table roles-users
CREATE TABLE
    users_roles (
        id INTEGER PRIMARY KEY not null,
        rol_id INTEGER not null references roles (id),
        user_id INTEGER not null references users (id) ON DELETE CASCADE ON UPDATE CASCADE
    );

-- create intermediate table baby-user
CREATE TABLE
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