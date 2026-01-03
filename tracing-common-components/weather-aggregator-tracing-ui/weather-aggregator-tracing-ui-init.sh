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

	
CMD="$JAEGER_BIN --config=${CONFIG_FILE}"

echo "Starting Jaeger with command:"
echo "$CMD"

exec $CMD