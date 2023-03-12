First experiments with a Rust webservice.

```bash
cd actix-oefen
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