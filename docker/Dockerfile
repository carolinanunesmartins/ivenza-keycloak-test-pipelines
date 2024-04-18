# This is the keycloak version to use.
# when overridden at build time by passing KEYCLOAK_VERSION as an argument, the given value will be used in stead of this version
ARG KEYCLOAK_VERSION=21.0
FROM quay.io/keycloak/keycloak:${KEYCLOAK_VERSION}

COPY ./themes/ivenza /opt/keycloak/themes/ivenza
COPY ./themes/skantrae /opt/keycloak/themes/skantrae
COPY ./config/delihome-realm.json /opt/keycloak/data/import/delihome-realm.json

# Temporary
# Azure only supports SHA1 for mysql flexible server, which is no longer support with the latest JDK, unless permitted.
# We'll allow it for now, but we should remove this, once Microsoft updates it's policy.
# https://learn.microsoft.com/en-us/answers/questions/1199915/certificates-do-not-conform-to-algorithm
# https://github.com/keycloak/keycloak/issues/17320
USER root
RUN ["sed", "-i", "s/SHA1, //g", "/usr/share/crypto-policies/DEFAULT/java.txt"]
USER 1000

# We want to access keycloak using the /auth context
# https://www.keycloak.org/migration/migrating-to-quarkus
# By default, the new Quarkus distribution removes /auth from the context-path. To re-introduce the /auth use the http-relative-path build option. For example:
# bin/kc.[sh|bat] start-dev --http-relative-path /auth
# Notice that relative paths don't seem to be working well:
# https://groups.google.com/g/keycloak-user/c/wtCpDiFD70U
# TL 13-3-2023 : Discussed with Peer not to use a relative path. If this is a requirement down the line, enable this relative path parameter
# CMD ["start-dev","--http-relative-path /auth -Dkeycloak.import=/tmp/delihome-realm.json", "-Dkeycloak.migration.strategy=SKIP"]
CMD ["start"]
