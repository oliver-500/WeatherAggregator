#!/bin/sh
: "${CONFIG_FILE=weather-aggregator-tracing-ui-config.yaml}"
: "${JAEGER_BIN:=/cmd/jaeger/jaeger-linux}"


cat ${CONFIG_FILE}


echo ""
echo "Waiting for Elasticsearch..."

until response=$(curl --cacert /usr/local/bin/jaeger/certs/weather-aggregator-ca.crt \
                      -u "${ELASTIC_USERNAME}:${ELASTIC_PASSWORD}" \
                      -s https://weather-aggregator-elasticsearch-tracing-storage:9200/_cluster/health?wait_for_status=yellow 2>&1)
do
  echo "Elasticsearch not ready yet..."
  echo "$response"
  sleep 2
done

echo "Elasticsearch is ready:"


if [[ -n "$ELASTIC_USERNAME" ]] && [[ -n "$ELASTIC_PASSWORD" ]]; then
  echo "Found Elasticsearch credentials."

  if grep -q "auth:" "$CONFIG_FILE"; then
    echo "Auth already present, skipping injection"
  else
    sed -i "/server_urls/a\\
          auth:\\
            basic:\\
              username: \"$ELASTIC_USERNAME\"\\
              password: \"$ELASTIC_PASSWORD\"" "$CONFIG_FILE"
    echo "Basic authentication injected."
  fi
else
  echo "Elasticsearch credentials missing. Exiting."
  exit 1
fi

	
CMD="$JAEGER_BIN --config=${CONFIG_FILE}"

echo "Starting Jaeger with command:"
echo "$CMD"

exec $CMD