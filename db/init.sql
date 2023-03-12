create database datagen;
create user maverick with encrypted password 'maverick';
grant all privileges on database datagen to maverick;
-- CREATE EXTENSION IF NOT EXISTS pgcrypto; // must be run as maverick
GRANT EXECUTE ON ALL FUNCTIONS IN SCHEMA public TO maverick;
ALTER ROLE maverick SUPERUSER;