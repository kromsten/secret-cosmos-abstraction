import { secretClient } from './clients';
import { codeConfigExists, contractConfigExists, ibcConfigExists, saveIbcConfig } from './config';
import { instantiateContracts, uploadContracts } from './contracts';
import { CONSUMER_CHAIN_ID, CONSUMER_TOKEN } from './env';
import { toHex, toUtf8 } from 'secretjs';
import { sha256 } from '@noble/hashes/sha256';
import { ClientState } from 'secretjs/src/protobuf/ibc/lightclients/tendermint/v1/tendermint';


const getIbcConnection = async () : Promise<string | undefined> => {
    const secretStates  = (await secretClient.query.ibc_client.clientStates({})).client_states!;
    for (const state of secretStates.reverse()) {
        try {
            const stateRes = state.client_state as ClientState;
            if (stateRes.chain_id == CONSUMER_CHAIN_ID
            ) {
                const clientConn = await secretClient.query.ibc_connection.clientConnections({
                    client_id: state.client_id
                })
                if (clientConn.connection_paths?.length == 1) {
                    const connection_id = clientConn.connection_paths[0];

                    const conn = await secretClient.query.ibc_connection.connection({
                        connection_id
                    });

                    if (conn.connection?.state == 'STATE_OPEN') {
                        return connection_id
                    }

                }

            }
        } catch (error) {}
    }
}

export const getIbcChannel = async ( connection? : string ) => {
    const channels = (await secretClient.query.ibc_channel.connectionChannels({ connection })).channels!
    const channel = channels.reverse().find(ch => ch.state == "STATE_OPEN")
    return channel
}




const setupIbc = async () => {
    const connection = await getIbcConnection();
    const channel = await getIbcChannel(connection);
    if (!channel) return false;

    const ibc_denom = "ibc/" + toHex(sha256(toUtf8(
        channel.channel_id! + '/transfer/' + CONSUMER_TOKEN
    ))).toUpperCase();


    saveIbcConfig({
        secret_channel_id: channel.channel_id!,
        consumer_channel_id: channel.counterparty!.channel_id!,
        ibc_denom
    })
    return true;
}




export const setup = async () => {

    if (!ibcConfigExists()) {
        if (!(await setupIbc())) {
            return
        }
    }
    if (!codeConfigExists()) {
        await uploadContracts();
    }

    if (!contractConfigExists()) {
        await instantiateContracts();
    }
}



