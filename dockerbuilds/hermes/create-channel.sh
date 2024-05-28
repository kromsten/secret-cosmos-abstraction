#!/bin/bash
set -e

# Load shell variables
. ./network/hermes/variables.sh

### Configure the clients and connection
echo "Initiating channel on transfer ports..."
while ! $HERMES_BINARY --config $CONFIG_DIR create channel --a-chain test-1 --a-connection connection-0 --a-port transfer --b-port transfer; do
  sleep 1
done
sleep 2
