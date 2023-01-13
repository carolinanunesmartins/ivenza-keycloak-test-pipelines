#!/bin/bash
if [[ "$(docker images -q ivenza-auth-migrator:latest 2> /dev/null)" == "" ]]; then
    echo "Docker image not found, building..."
    docker build -f Dockerfile . -t ivenza-auth-migrator
else
    echo "Docker image already found"
fi

docker run --rm -it --network unicon-ivenza -e DATABASE_URL=mysql://ivenza:ivenza@db:3306/ivenza -e AUTHORITY=http://keycloak:8080/realms/master -e KEYCLOAK_CLIENT_ID=admin-cli -e KEYCLOAK_ADMIN_USERNAME=keycloak -e KEYCLOAK_ADMIN_PASSWORD=keycloak -e ADMIN_BASE_URL=http://keycloak:8080/admin/realms/delihome -e CLIENT_ID=17873d79-f666-487a-ab9f-239fabbeb24d ivenza-auth-migrator:latest
