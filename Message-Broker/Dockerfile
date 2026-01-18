FROM rabbitmq:4.1.5-management-alpine

COPY /config/ /tmp/config/

#config is always present
COPY /config/rabbitmq-default.conf /etc/rabbitmq/rabbitmq.conf
COPY /scripts /usr/local/scripts/
COPY /certs /usr/local/certs/

ENTRYPOINT ["/usr/local/scripts/rabbitmq-init.sh"]
