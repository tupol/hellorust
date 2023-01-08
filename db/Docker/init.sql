create database datagen;
create user maverick with encrypted password 'maverick';
grant all privileges on database datagen to maverick;
CREATE EXTENSION IF NOT EXISTS pgcrypto;
