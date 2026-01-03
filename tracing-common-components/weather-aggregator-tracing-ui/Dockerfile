FROM jaegertracing/jaeger:2.11.0

USER root
RUN apk add --no-cache curl

COPY /certs /usr/local/bin/jaeger/certs

COPY weather-aggregator-tracing-ui-config.yaml weather-aggregator-tracing-ui-config.yaml

USER root

COPY weather-aggregator-tracing-ui-init.sh /data/weather-aggregator-tracing-ui-init.sh

RUN chmod +x /data/weather-aggregator-tracing-ui-init.sh

ENTRYPOINT ["/data/weather-aggregator-tracing-ui-init.sh"]