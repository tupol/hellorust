docker stop postgres-perf
docker rm postgres-perf
docker image rm db-postgres-perf --force
rm -rf var
