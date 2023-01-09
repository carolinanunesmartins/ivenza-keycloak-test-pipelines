# Setup keycloak environment variables.

## Create a .env file:
Create a `.env` in this folder, which looks like the following.

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
