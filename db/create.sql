CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE IF NOT EXISTS users (
  id uuid PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL,
  password VARCHAR(255) NOT NULL,
  salt VARCHAR(255) NOT NULL
);

-- salt/password
INSERT INTO users (id, name, email, password, salt)
VALUES (gen_random_uuid(), 'John Doe', 'john@example.com', '13601bda4ea78e55a07b98866d2be6be0744e3866f13c00c811cab608a28f322', 'c2FsdA==');

-- create a function that returns a fixed hash/salt for any username
CREATE OR REPLACE FUNCTION userinfo(uname text, decrkey text, hashkey text)
RETURNS TABLE(
  username TEXT,
  hashpassword TEXT,
  salt TEXT,
  loa1level INTEGER,
  loa2level INTEGER,
  amid TEXT,
  amstate VARCHAR(100),
  amlocktime INTEGER,
  name TEXT,
  emailaddress TEXT,
  typeuser TEXT,
  firstname TEXT,
  lastname TEXT,
  usertechnicalid TEXT,
  pwCreated DATE
) AS $$

BEGIN
RETURN QUERY
SELECT
    uname AS username,
    '0mhcQnHQjB02bWs9J1u5WFD7e9qZnq32GWfsZjO/XlA=' AS hashpassword,
    'pSMrnnFNdJtanr5m+D2ZNHOpszE0sYFTAMWXkLfvR7F0euELeyEu1Q1AqwS7o3RTrHyo0UdYtwexDWe7N3gEyA==' AS salt,
    3 AS loa1level,
    4 AS loa2level,
    'am.id' AS amid,
    'am.state'::varchar(100) AS amstate,
        1000 AS amlocktime,
    'Dennis Bergkamp' AS name,
    uname AS emailaddress,
    'NPA' AS typeuser,
    'Dennis' AS firstname,
    'Bergkamp' AS lastname,
    '7a4c1918-af9a-4caa-bba0-6ca06ceeefd8' AS usertechnicalid,
    TO_DATE('20170103','YYYYMMDD') AS pwCreated;
END;
$$ LANGUAGE PLPGSQL;
