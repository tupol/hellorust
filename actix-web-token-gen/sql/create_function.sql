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
