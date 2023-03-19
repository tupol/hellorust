First experiments with a Rust webservice.

### Database & docker network
Prepare docker network
```bash
docker network create pg-perf
```

Start the database:
```bash
cd db
./run_db.sh
```
It will start a container called `postgres-perf` containing one table called `users`.

In case you need to rebuild the postgres container run:
```bash
./db/rm_docker.sh
```
Manual entry to `postgres-perf`:
```bash
docker exec -it postgres-perf bash
psql --u maverick datagen
\d
select * from users;
```

### Actix
```bash
cd actix-web
docker-compose up -d
```

Invoke /token endpoint
```bash
curl http://localhost:8780/token -X POST -d '{"username":"NPA","password":"usr001.."}' -H 'Content-Type: application/json'
```

Load test
```bash
wrk -s post-token.lua -d60 -t50 -c50 http://localhost:8780/token
```
