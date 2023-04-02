echo "Stopping running container"
docker stop actix-web-token-gen-server-1
echo "Removing container"
docker container rm actix-web-token-gen_server-1
echo "Removing image"
docker image rm actix-web-token-gen-server
