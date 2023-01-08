docker-compose -f docker-compose.yml up -d
docker cp ./create.sql postgres:/tmp/create.sql
sleep 1;
docker exec -u postgres postgres psql datagen maverick -f /tmp/create.sql
