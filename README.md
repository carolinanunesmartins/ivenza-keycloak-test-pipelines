# Setup keycloak variables.

## Create a .env file:
Create a `.env` in the `docker` folder, which looks like the following.

Please set all placeholders with your own credentials:
```env
COMPOSE_PROJECT_NAME=ivenza-keycloak
 
POSTGRES_VERSION=15.0-alpine
 
KEYCLOAK_VERSION=20.0
KEYCLOAK_USER=[KEYCLOAK-USER]
KEYCLOAK_PASSWORD=[KEYCLOAK-PASSWORD]
 
KEYCLOAK_DATABASE_NAME=keycloak
KEYCLOAK_DATABASE_USER=[KEYCLOAK-DB-USER]
KEYCLOAK_DATABASE_PASSWORD=[KEYCLOAK-DB-USER-PASSWORD]
KEYCLOAK_DATABASE_HOST=keycloakdb
KEYCLOAK_DATABASE_VENDOR=postgres
```

You can now spin up your keycloak instance using docker-compose. Navigate to the `docker` folder and run `docker-compose up -d && docker-compose logs -f`. This will start the docker compose file 'detached' and tail the logging for as long as you like. You can quit the log tail by pressing `CTRL+V`.


# Running the migration tool.
Before running the migration tool, you need to have both a running keycloak environment, as well as a ivenza environment with access to it's database.
Once these a up and running, please consult the [migrator readme file](./ivenza_auth_migrator/README.md) to run the migration tool using Docker

