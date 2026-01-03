#!/bin/sh
set -e

# Defaults
: "${CONFIG_FILE=/data/weather-aggregator-tracing-collector-config.yaml}"
: "${JAEGER_BIN:=/cmd/jaeger/jaeger-linux}"
: "${RECEIVER_OTLP_GRPC_PORT:=4317}"
: "${RECEIVER_OTLP_GRPC_TLS_CERT:=/certs/weather-aggregator-tracing-collector.crt}"
: "${RECEIVER_OTLP_GRPC_TLS_KEY:=/certs/weather-aggregator-tracing-collector.key}"
: "${RECEIVER_OTLP_GRPC_TLS_CA:=/weather-aggregator-ca.crt}"


enable_otlp_receiver_mtls() {
  awk '
  BEGIN {found=0}
  {
    print $0
    if ($0 ~ /grpc:/) {
      print "        tls:"
      print "          cert_file: /certs/weather-aggregator-tracing-collector.crt"
      print "          key_file: /certs/weather-aggregator-tracing-collector.key"
      print "          client_ca_file: /certs/weather-aggregator-ca.crt"
      found=1
    }
  }' "$CONFIG_FILE" > "${CONFIG_FILE}.tmp"
  mv -f "${CONFIG_FILE}.tmp" "$CONFIG_FILE"
}

enable_otlp_receiver_tls() {
  awk '
  BEGIN {found=0}
  {
    print $0
    if ($0 ~ /grpc:/) {
      print "        tls:"
      print "          cert_file: /certs/weather-aggregator-tracing-collector.crt"
      print "          key_file: /certs/weather-aggregator-tracing-collector.key"
      found=1
    }
  }' "$CONFIG_FILE" > "${CONFIG_FILE}.tmp"
  mv -f "${CONFIG_FILE}.tmp" "$CONFIG_FILE"
}

disable_otlp_receiver_tls() {
  grep -v "tls:" "$CONFIG_FILE" | \
  grep -v "cert_file:" | \
  grep -v "key_file:" | \
  grep -v "client_ca_file:" > "${CONFIG_FILE}.tmp"
  mv -f "${CONFIG_FILE}.tmp" "$CONFIG_FILE"
}


if [ "$MTLS_GRPC_ENABLED" = "true" ]; then
  echo "Enabling mutual TLS"
  enable_otlp_receiver_mtls
elif [ "$RECEIVER_OTLP_GRPC_TLS_ENABLED" = "true" ]; then
  echo "Enabling server-only TLS"
  enable_otlp_receiver_tls
else
  echo "Disabling TLS"
  disable_otlp_receiver_tls
fi


# Check if both credentials environment variables are set
if [[ -n "$ELASTIC_USERNAME" ]] && [[ -n "$ELASTIC_PASSWORD" ]]; then
    echo "Found Elasticsearch credentials. Dynamically configuring basic auth..."

    # Use sed to find the 'server_urls:' line and append the basic auth block after it.
    # The indentation is critical (8 spaces for 'auth:', 10 for 'basic:', 12 for username/password).
    # NOTE: The target line must be unique in the file!
    sed -i "/server_urls/a\\
          auth:\\
            basic:\\
              username: \"$ELASTIC_USERNAME\"\\
              password: \"$ELASTIC_PASSWORD\"" "$CONFIG_FILE"

    echo "Basic authentication fields successfully added to the configuration."
else
    echo "Elasticsearch credentials not found in environment variables. Exiting."
	exit 1
fi


cat "${CONFIG_FILE}"

ELASTIC_HEALTH_URL="https://weather-aggregator-elasticsearch-tracing-storage:9200/_cluster/health?wait_for_status=yellow"

echo "Waiting for Elasticsearch..."

until response=$(curl --cacert /certs/weather-aggregator-ca.crt \
                      -u "${ELASTIC_USERNAME}:${ELASTIC_PASSWORD}" \
                      -s https://weather-aggregator-elasticsearch-tracing-storage:9200/_cluster/health?wait_for_status=yellow 2>&1)
do
  echo "Elasticsearch not ready yet..."
  echo "$response"
  sleep 2
done

echo "Elasticsearch is ready:"

	
CMD="$JAEGER_BIN --config=${CONFIG_FILE}"

echo "Starting Jaeger with command:"
echo "$CMD"

exec $CMD
