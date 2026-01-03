FROM alpine AS config-builder

RUN addgroup -g 10001 otel && adduser -u 10001 -G otel -s /bin/sh -D otel

RUN mkdir -p /tmp/certs /tmp/config
RUN chown -R 10001:10001 /tmp/certs /tmp/config

COPY weather-aggregator-geocoding-service-otel-agent-config.yaml /tmp/config/
COPY certs /tmp/certs


# STAGE 2
FROM otel/opentelemetry-collector-contrib:0.139.0 AS final-agent

COPY --from=config-builder /tmp/certs /certs
COPY --from=config-builder /tmp/config /config


EXPOSE 4317
ENV APP_ENV=dev

ENTRYPOINT ["/otelcol-contrib"]