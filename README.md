First experiments with a Rust webservice.

```bash
curl http://localhost:3031/token -X POST -d '{"username":"NPA","password":"usr001.."}' -H 'Content-Type: application/json'
```

Load test
```bash
wrk -s post-token.lua -d60 -t50 -c50 http://localhost:8780/token
```