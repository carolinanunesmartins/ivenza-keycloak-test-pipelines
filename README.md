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

## Setup Microsoft as an external IDP
You can use Microsoft Active Directory as an external Identify Provider. To do
so, you will need to configure the Identify provider accordingly:

* Create a new OIDC Identity Provider (Do not use Microsoft, as we are connecting with a specific tennant)
  ![image](https://github.com/Unicon-Creation/ivenza-keycloak/assets/35781348/8184f397-8453-472d-ae18-28f49562f332)
* Make sure to set the following settings:
  ![image](https://github.com/Unicon-Creation/ivenza-keycloak/assets/35781348/159e7e11-2186-431d-8218-d19ab6899869)
  * Replace the obfuscated UUID of the Authorization URL and Token URL, with the TenantID, provided by Delihome IT.
    * https://login.microsoftonline.com/[TENANT_ID]/oauth2/v2.0/authorize
    * https://login.microsoftonline.com/[TENANT_ID]/oauth2/v2.0/token
  * Replace the obfuscated UUID of the Client ID with the ClientId, provided by Delihome IT
  * Replace the obfuscated Client Secret with the ClientSercret, provided by Delihome IT
  * Make sure the redirect URL is also registered at the Azure AD client, by Delihome IT (case sensitive)
    * http(s)://[BASE_URL]/realms/[REALM]/broker/[PROVIDER_ALIAS]/endpoint
* Under 'advanced', make sure to set the following scopes. These allow us to read user information (firstname, lastname, email and username) from Azure AD
  * scopes: `openid profile email`
    * ![image](https://github.com/Unicon-Creation/ivenza-keycloak/assets/35781348/19758d00-cf45-4c4d-b5cd-0effb70e4e8a)



* To make sure the user information from Azure Active Directory is automatically assigned to a new user account, go to `Mappers`.
  *  Create 4 mappings like so:
![image](https://github.com/Unicon-Creation/ivenza-keycloak/assets/35781348/aa1a50aa-75f1-4378-bd5e-8cee1d0a351d)
    *  Firstname from ADD > claim: given_name, attribute: firstName
    *  LastName from ADD > claim: family_name, attribute: lastName
    *  UserName from ADD > claim: name, attribute: username
    *  Email from ADD > claim unique_name, attribute: email

If a new user logs in now via Azure AD, all it's information will be available within keycloak automatically


