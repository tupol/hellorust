docker stop postgres-perf
docker rm postgres-perf
docker image rm db_postgres-perf --force
rm -rf var
