SELECT username, hashpassword, salt, loa1level, loa2level, amid, amstate, amlocktime,
       name , emailaddress , typeuser , firstname , lastname , usertechnicalid
FROM
    userinfo($1, $2, $3);