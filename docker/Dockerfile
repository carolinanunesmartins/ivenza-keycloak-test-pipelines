# This is the keycloak version to use.
# when overridden at build time by passing KEYCLOAK_VERSION as an argument, the given value will be used in stead of this version
ARG KEYCLOAK_VERSION=20.0
FROM quay.io/keycloak/keycloak:${KEYCLOAK_VERSION}

COPY ./themes/ivenza /opt/keycloak/themes/ivenza
COPY ./config/delihome-realm.json /tmp/delihome-realm.json

CMD ["start-dev","-Dkeycloak.import=/tmp/delihome-realm.json", "-Dkeycloak.migration.strategy=SKIP"]