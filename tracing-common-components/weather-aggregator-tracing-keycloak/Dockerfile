FROM keycloak/keycloak:26.5

COPY /certs /opt/keycloak/conf
COPY /realms /opt/keycloak/data/import/

CMD ["start-dev", "--import-realm"]

EXPOSE 8080
