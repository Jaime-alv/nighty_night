# SQLite3 Queries

## Initial configuration

---

### Up

```sql
-- Your SQL goes here

-- create table user_model
CREATE TABLE
    user_model (
        "user_id" INTEGER PRIMARY KEY not NULL,
        "username" TEXT not NULL,
        "password" TEXT not null,
        "rol" INTEGER references roles (rol_id) not null,
        "task" INTEGER references task (task_id)
    );
-- Insert admin user into user_model
insert into user_model values(0, "admin", "admin", 0, null);

-- create table roles
CREATE TABLE
    roles (
        "rol_id" INTEGER PRIMARY KEY not null,
        "name" TEXT not NULL
    );

-- insert roles
insert into roles VALUES(0, "admin");
insert into roles VALUES(1, "user");
insert into roles VALUES(2, "anonymous");

-- create table tasks
CREATE TABLE
    task (
        "task_id" INTEGER PRIMARY KEY not NULL,
        "description" TEXT not NULL,
        "done" BOOLEAN not null
    );
```

### Down

```sql
-- This file should undo anything in `up.sql`
drop TABLE user_model;
drop TABLE roles;
drop TABLE task;
```
