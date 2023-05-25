-- Your SQL goes here
-- Insert admin user into users
insert into
    users
values
    (2, "admin", "$2b$12$3X1BP9hIp.NpRbqSi5EZ3e.oH0qs53M0Tj7IGYnohpLbWgJvcmebK", "Test", "User", "admin@a.a", true);

insert into
    users
values
    (1, "guest", "$2b$12$Cn6h/UOHVMbZrkNzGSX7lulLhg9/zH6stl38C5RUmPMP7Gy.ZYPC2", "Test", "User", "admin@a.a", true);


-- associate admin user and roles
insert into
    users_roles
VALUES
    (0, 0, 2);

insert into
    users_roles
VALUES
    (1, 1, 2);

-- associate anonymous user
insert into
    users_roles
VALUES
    (2, 2, 1);