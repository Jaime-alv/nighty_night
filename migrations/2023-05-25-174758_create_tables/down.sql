-- This file should undo anything in `up.sql`
drop TABLE users CASCADE;
drop TABLE roles CASCADE;
drop TABLE babies CASCADE;
drop TABLE dreams;
drop TABLE meals;
drop TABLE weights;
drop TABLE users_roles;
drop TABLE users_babies;