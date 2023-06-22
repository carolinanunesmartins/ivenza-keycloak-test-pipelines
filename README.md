# Setup Keycloak variables.

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

You can now spin up your Keycloak instance using docker-compose. Navigate to the `docker` folder and run `docker-compose up -d && docker-compose logs -f`. This will start the docker compose file 'detached' and tail the logging for as long as you like. You can quit the log tail by pressing `CTRL+V`.


# Running the migration tool.
Before running the migration tool, you need to have both a running Keycloak environment, as well as a ivenza environment with access to it's database.
Once these a up and running, please consult the [migrator readme file](./ivenza_auth_migrator/README.md) to run the migration tool using Docker

## Keycloak realms.
Keep in mind, we can only bind one Ivenza instance to a single Keycloak realm.
The Keycloak realms are build up as followed.

* Delihome/Organization/something, we currently have a delihome realm, but it
  doesn't really make sense.
* Customer???
    * We can split customers per realm. Gamma users only have access to gamma
      environments for example. Although we already check for the domain within
      ivenza before letting anybody through.
* DeploymentType??
    * Timber, Doors, Storage could all be separate realms. This is more in line
      with the notion before that we can only bind a single Ivenza instance to a
      single OIDC authority (realm). A 'Doors' deployment should be linked to
      the /realms/doors realm. I think this is also in line with the current
      database setup, as all related users are also imported from the single
      underlying database to the same single realm.
    * I would expect the realms to be available:
        * Accept
            * doors
            * timber
        * Staging
            * demo
            * doors
            * istare
            * jwr
            * kastopmaat (should we switch to english?)
            * mobile
            * swk
            * timber
        * production
            * demo
            * doors
            * houtshop
            * istare
            * jwr
            * kastopmaat (should we switch to english?)
            * mobile
            * swk
            * timber

* Then there is the question if separating user information per realm is
  sufficient. Timber realm users cannot login to doors, and vice versa.
  Out of the box, it is not possible to have multiple databases for a single
  Keycloak instance. If we want to really separate users into multiple databases
  we should instantiate more Keycloak instances. Keep in mind that these all add
  a initial memory footprint to the Kubernetes clusters. With every instance,
  around 650MB of memory is added to the cluster workload. If we have 9
  instances of Keycloak running, this would add up to 5.85 GB of memory, which
  is quite a lot.
