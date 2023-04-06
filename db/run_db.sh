docker-compose -f docker-compose.yml up -d
docker cp ./init.sql postgres-perf:/tmp/init.sql
docker cp ./create.sql postgres-perf:/tmp/create.sql
echo "Wait 10 seconds for postgres to startup"
sleep 10;
docker exec -it postgres-perf psql --u postgres -f /tmp/init.sql
docker exec -it postgres-perf psql --u maverick datagen -f /tmp/create.sql
echo "Postgres is ready and contains one table users"
