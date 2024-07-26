#!/bin/bash

if [ $SSL_ENABLED == "true" ]; then
  curl -f -k https://localhost:$BIND_PORT/health || exit 1
elif [ $SSL_ENABLED == "false" ]; then
  curl -f -k http://localhost:$BIND_PORT/health || exit 1
else
  echo "SSL_ENABLED must be true or false"
  exit 1
fi
