#!/bin/bash
if [[ "$(docker images -q ivenza-auth-migrator:latest 2> /dev/null)" == "" ]]; then
    echo "Docker image not found, building..."
    docker build -f Dockerfile . -t ivenza-auth-migrator
else
    echo "Docker image already found"
fi

docker run --rm --network unicon-ivenza --env-file .env ivenza-auth-migrator:latest
