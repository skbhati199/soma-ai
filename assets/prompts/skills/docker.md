---
name: docker
triggers: [docker, container, image, compose, volume, dockerfile, registry]
---
DOCKER SKILL (docker CLI is called directly in PowerShell)
- Never use $(...). For ids use: docker ps -q | ForEach-Object { ... $_ ... }
- "all" with containers means: running = docker ps -q, every container = docker ps -aq

Request: list docker images
PowerShell Command:
docker images

Request: show running containers
PowerShell Command:
docker ps

Request: show all containers including stopped
PowerShell Command:
docker ps -a

Request: stop all running containers
PowerShell Command:
docker ps -q | ForEach-Object { docker stop $_ }

Request: remove all stopped containers
PowerShell Command:
docker container prune -f

Request: remove all containers
PowerShell Command:
docker ps -aq | ForEach-Object { docker rm -f $_ }

Request: remove dangling images
PowerShell Command:
docker image prune -f

Request: remove all unused images
PowerShell Command:
docker image prune -a -f

Request: remove image nginx
PowerShell Command:
docker rmi nginx

Request: follow logs of container api
PowerShell Command:
docker logs -f --tail 100 api

Request: open shell inside container api
PowerShell Command:
docker exec -it api sh

Request: run postgres on port 5432 with password secret
PowerShell Command:
docker run -d --name postgres -p 5432:5432 -e POSTGRES_PASSWORD=secret postgres:16

Request: run redis in background
PowerShell Command:
docker run -d --name redis -p 6379:6379 redis:7

Request: build image named myapp from current folder
PowerShell Command:
docker build -t myapp .

Request: restart container api
PowerShell Command:
docker restart api

Request: copy file from container api to current folder
PowerShell Command:
docker cp api:/app/app.log .

Request: copy file into container api
PowerShell Command:
docker cp .\config.json api:/app/config.json

Request: show container ip address of api
PowerShell Command:
docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' api

Request: show cpu and memory usage of containers
PowerShell Command:
docker stats --no-stream

Request: show docker disk usage
PowerShell Command:
docker system df

Request: clean everything unused in docker
PowerShell Command:
docker system prune -a --volumes -f

Request: list docker volumes
PowerShell Command:
docker volume ls

Request: list docker networks
PowerShell Command:
docker network ls

Request: start compose services in background
PowerShell Command:
docker compose up -d

Request: rebuild and start compose
PowerShell Command:
docker compose up -d --build

Request: stop compose and remove containers
PowerShell Command:
docker compose down

Request: stop compose and remove volumes too
PowerShell Command:
docker compose down -v

Request: show compose logs for service web
PowerShell Command:
docker compose logs -f web

Request: tag image myapp and push to registry
PowerShell Command:
docker tag myapp myuser/myapp:latest
docker push myuser/myapp:latest

Request: find which container uses port 8080
PowerShell Command:
docker ps --filter 'publish=8080'