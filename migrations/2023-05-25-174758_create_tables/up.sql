-- Your SQL goes here
-- create table users
CREATE TABLE
    IF NOT EXISTS users (
        "id" SERIAL PRIMARY KEY not NULL,
        "username" VARCHAR(64) not NULL,
        "password" VARCHAR(128) not null,
        "name" VARCHAR(64),
        "surname" VARCHAR(64),
        "email" VARCHAR,
        "active" BOOLEAN not null DEFAULT TRUE,
        "created_at" TIMESTAMP not NULL,
        "updated_at" TIMESTAMP,
        UNIQUE (username)
    );

-- create table roles
CREATE TABLE
    IF NOT EXISTS roles (
        "id" smallint PRIMARY KEY not null,
        "name" VARCHAR(16) not NULL
    );

-- create table babies
CREATE TABLE
    IF NOT EXISTS babies (
        "id" SERIAL PRIMARY KEY not NULL,
        "name" VARCHAR(64) not NULL,
        "birthdate" DATE not NULL
    );

-- create table dreams
CREATE TABLE
    IF NOT EXISTS dreams (
        "id" SERIAL PRIMARY KEY not NULL,
        "baby_id" INTEGER not null,
        "from_date" TIMESTAMP not null,
        "to_date" TIMESTAMP,
        "elapsed" INTERVAL,
        CONSTRAINT fk_baby_dreams FOREIGN KEY (baby_id) REFERENCES babies (id) ON DELETE CASCADE ON UPDATE CASCADE
    );

-- create tables meals
CREATE TABLE
    IF NOT EXISTS meals (
        "id" SERIAL PRIMARY KEY not NULL,
        "baby_id" INTEGER not null,
        "date" TIMESTAMP not null,
        "quantity" smallint,
        "elapsed" smallint,
        CONSTRAINT fk_baby_meals FOREIGN KEY (baby_id) REFERENCES babies (id) ON DELETE CASCADE ON UPDATE CASCADE
    );

-- create table weights
CREATE TABLE
    IF NOT EXISTS weights (
        "id" SERIAL PRIMARY KEY not NULL,
        "baby_id" INTEGER not null,
        "date" DATE not null,
        "value" REAL not null,
        CONSTRAINT fk_baby_weights FOREIGN KEY (baby_id) REFERENCES babies (id) ON DELETE CASCADE ON UPDATE CASCADE
    );

-- create intermediate table roles-users
CREATE TABLE
    IF NOT EXISTS users_roles (
        "id" SERIAL PRIMARY KEY not null,
        "rol_id" smallint not null,
        "user_id" INTEGER not null,
        CONSTRAINT fk_rol_users_roles FOREIGN KEY (rol_id) REFERENCES roles (id),
        CONSTRAINT fk_user_users_roles FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE ON UPDATE CASCADE
    );

-- create intermediate table baby-user
CREATE TABLE
    IF NOT EXISTS users_babies (
        "id" SERIAL PRIMARY KEY not null,
        "baby_id" INTEGER not null,
        "user_id" INTEGER not null,
        CONSTRAINT fk_baby_users_babies FOREIGN KEY (baby_id) REFERENCES babies (id) ON DELETE CASCADE ON UPDATE CASCADE,
        CONSTRAINT fk_user_users_babies FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE ON UPDATE CASCADE
    );

-- insert roles
insert into
    roles (id, name)
VALUES
    (0, 'admin');

insert into
    roles (id, name)
VALUES
    (1, 'user');

insert into
    roles (id, name)
VALUES
    (2, 'anonymous');

-- Insert anonymous user into users
insert into
    users (
        username,
        password,
        name,
        surname,
        email,
        active,
        created_at
    )
values
    (
        'guest',
        '$2b$12$Cn6h/UOHVMbZrkNzGSX7lulLhg9/zH6stl38C5RUmPMP7Gy.ZYPC2',
        'Test',
        'User',
        'admin@a.a',
        true,
        CURRENT_TIMESTAMP AT TIME ZONE 'Europe/Berlin'
    );

-- Insert admin user into users
insert into
    users (
        username,
        password,
        name,
        surname,
        email,
        active,
        created_at
    )
values
    (
        'admin',
        '$2b$12$3X1BP9hIp.NpRbqSi5EZ3e.oH0qs53M0Tj7IGYnohpLbWgJvcmebK',
        'Test',
        'User',
        'admin@a.a',
        true,
        CURRENT_TIMESTAMP AT TIME ZONE 'Europe/Berlin'
    );

-- associate admin user and roles
-- Give admin status
insert into
    users_roles (rol_id, user_id)
VALUES
    (0, 2);

-- Give user status
insert into
    users_roles (rol_id, user_id)
VALUES
    (1, 2);

-- associate anonymous user
-- Give anonymous status
insert into
    users_roles (rol_id, user_id)
VALUES
    (2, 1);