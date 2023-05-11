-- Your SQL goes here
-- Insert admin user into users
insert into
    users
values
    (0, "admin", "$2b$12$3X1BP9hIp.NpRbqSi5EZ3e.oH0qs53M0Tj7IGYnohpLbWgJvcmebK", "Test", "User", "admin@a.a");

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

-- associate admin user and roles
insert into
    users_roles
VALUES
    (0, 0, 0);

insert into
    users_roles
VALUES
    (1, 1, 0);