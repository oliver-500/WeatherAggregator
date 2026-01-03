FROM jaegertracing/jaeger:2.11.0


USER root
RUN apk add --no-cache curl
COPY /certs /certs



COPY weather-aggregator-tracing-collector-config.yaml /data/weather-aggregator-tracing-collector-config.yaml

RUN mkdir -p /data && chown -R 10001:10001 /data /certs


COPY weather-aggregator-tracing-collector-init.sh /data/weather-aggregator-tracing-collector-init.sh

RUN chmod +x /data/weather-aggregator-tracing-collector-init.sh
RUN chmod +x /cmd/jaeger

EXPOSE 4317

ENV APP_ENV=dev



ENTRYPOINT ["/data/weather-aggregator-tracing-collector-init.sh"]

