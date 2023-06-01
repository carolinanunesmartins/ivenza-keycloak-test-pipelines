#!/bin/bash
if [[ "$(docker images -q ivenza-auth-migrator:latest 2> /dev/null)" == "" ]]; then
    echo "Docker image not found, building..."
    # added --platform linux/amd64 to docker build command in ./run-migrator.sh so that 
    # x86 images are pulled and zlib1g-dev and libmariadb3 installs end up in the expected place 
    # (where they are copied from)
    docker build --platform linux/amd64 -f Dockerfile . -t ivenza-auth-migrator
else
    echo "Docker image already found"
fi

docker run --rm --network host --env-file .env ivenza-auth-migrator:latest
