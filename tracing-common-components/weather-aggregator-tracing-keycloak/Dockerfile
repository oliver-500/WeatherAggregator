FROM keycloak/keycloak:26.5

COPY /certs /opt/keycloak/conf

CMD ["start-dev", "--import-realm"]

EXPOSE 8080
