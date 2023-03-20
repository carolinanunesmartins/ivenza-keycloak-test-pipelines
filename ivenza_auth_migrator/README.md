# Running the migrator using Docker
In order to run the migrator using Docker you need to create a .env file first in this folder:

```
DATABASE_URL=[DATABASE URL TO CONNECT TO MARIADB] # example: mysql://ivenza:ivenza@db:3306/ivenza
AUTHORITY=[KEYCLOAK MASTER REALM ON THE KEYCLOAK INSTANCE] # example: http://keycloak:8080/realms/master
KEYCLOAK_CLIENT_ID=admin-cli
KEYCLOAK_ADMIN_USERNAME=[KEYCLOAK ADMIN USERNAME] # example: keycloak
KEYCLOAK_ADMIN_PASSWORD=[KEYCLOAK ADMIN PASSWORD] # example: keycloak
ADMIN_BASE_URL=[THE ADMIN BASE URL OF THE REALM WHERE THE CLIENT RESIDES WHERE YOU WANT TO SYNC THE PERMISSIONS] # example: http://keycloak:8080/admin/realms/delihome
CLIENT_ID=[CLIENT ID OF THE CLIENT TO ADD THE PERMISSIONS TO] # example: 17873d79-f666-487a-ab9f-239fabbeb24d
SKIP_INTERNAL_USERS=true/false # if set to true, users with uniconcreation.com and delihome.com e-mail addresses will not be imported into Keycloak.
```

If this is done, you could simply run the [run-migrator.sh file](./run-migrator.sh). It will check whether an image was build for the migration tool. If not, a docker image will be build.
After this, the docker image will be ran automatically with the configured environment variables, and removed after being finished.

## Running the migrator manually

To build the migration tool docker image:

```bash 
docker build -f Dockerfile . -t ivenza-auth-migrator
```

To run the migration tool using the `.env` file

```bash 
docker run --rm --network unicon-ivenza --env-file .env ivenza-auth-migrator:latest
```

Or to run without the .env file and specifically specify environment variables (example)
```bash
docker run --rm -it --network unicon-ivenza \
    -e DATABASE_URL=mysql://ivenza:ivenza@db:3306/ivenza \
    -e AUTHORITY=http://keycloak:8080/realms/master \
    -e KEYCLOAK_CLIENT_ID=admin-cli \
    -e KEYCLOAK_ADMIN_USERNAME=keycloak \
    -e KEYCLOAK_ADMIN_PASSWORD=keycloak \
    -e ADMIN_BASE_URL=http://keycloak:8080/admin/realms/delihome \
    -e CLIENT_ID=17873d79-f666-487a-ab9f-239fabbeb24d \
    -e SKIP_INTERNAL_USERS=true\
    ivenza-auth-migrator:latest
```
