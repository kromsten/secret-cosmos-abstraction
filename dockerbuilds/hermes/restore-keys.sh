#!/bin/bash
set -e

# Load shell variables
. ./network/hermes/variables.sh

### Sleep is needed otherwise the relayer crashes when trying to init
sleep 1

### Restore Keys
$HERMES_BINARY --config $CONFIG_DIR keys add --key-name testkey_1 --chain test-1 --mnemonic-file <(echo "chair love bleak wonder skirt permit say assist aunt credit roast size obtain minute throw sand usual age smart exact enough room shadow charge")
sleep 5


$HERMES_BINARY --config $CONFIG_DIR keys add --key-name testkey_2 --chain test-2 --mnemonic-file <(echo "word twist toast cloth movie predict advance crumble escape whale sail such angry muffin balcony keen move employ cook valve hurt glimpse breeze brick")
sleep 8


export ADDRESS=$(cat /root/.hermes/keys/test-1/keyring-test/testkey_1.json  | jq -r '.account')
curl "http://ls-1:5000/faucet?address=${ADDRESS}"

sleep 2

export ADDRESS=$(cat /root/.hermes/keys/test-2/keyring-test/testkey_2.json  | jq -r '.account')
curl "http://ls-2:5000/faucet?address=${ADDRESS}"
