import { expect, describe, test, beforeAll } from 'vitest';
import { getGatewayEncryptionKey, queryGatewayAuth } from '../src/gateway';
import { consumerClient, consumerWallet } from '../src/clients';
import { getArb36Credential } from '../src/crypto';
import { gatewayChachaHookMemo, gatewayHookMemo, sendIBCToken } from '../src/ibc';
import { loadContractConfig, loadIbcConfig } from '../src/config';
import { CONSUMER_TOKEN } from '../src/env';

describe('Gateway contract interaction over IBC', () => {

    let gatewayKey : string | undefined;

    beforeAll(async () => {
        gatewayKey = await getGatewayEncryptionKey();
    });

    describe('sending a message over ibc', async () => {
        
        const consumerQCFirst = await getArb36Credential(consumerWallet, "foo")
        const consumerQCSecond = await getArb36Credential(consumerWallet, "bar")

        const ibcConfig = loadIbcConfig();
        const secretGateway = loadContractConfig().gateway!;


        test('if can see simple ibc-hook msg', async () => {

            const new_text = "new_text_" + Math.random().toString(36).substring(7);

            // non-authenticated & non-encrypted query
            // visible to everyone over IBC and the relayer will
            // be the one who stored the secret
            const response = await sendIBCToken(
                consumerClient,
                secretGateway.address,
                CONSUMER_TOKEN!,
                "1",
                ibcConfig.consumer_channel_id,
                gatewayHookMemo(
                    { extension: { msg: { store_secret: { text: new_text } } }},
                    secretGateway
                )
            )
            console.log("response", response)
            expect(response).toBeDefined();
            expect(response.code).toEqual(0);

            
            const non_updated_text = (await queryGatewayAuth(
                { get_secret: {} },
                [consumerQCFirst]
            )) as string;
            expect(non_updated_text).not.toEqual(new_text);
        })


        test('sending chacha20_poly1305_seal ibc-hook msg', async () => {

            const new_text = "new_text_" + Math.random().toString(36).substring(7);

            const response = await sendIBCToken(
                consumerClient,
                secretGateway.address,
                CONSUMER_TOKEN!,
                "1",
                ibcConfig.consumer_channel_id,
                await gatewayChachaHookMemo(
                    consumerWallet,
                    { extension: { msg: { store_secret: { text: new_text } } } },
                    secretGateway,
                    gatewayKey
                )
            )
            expect(response).toBeDefined();
            expect(response.code).toEqual(0);

            
            const updated_text = (await queryGatewayAuth(
                { get_secret: {} },
                [consumerQCSecond]
            )) as string;

            expect(updated_text).toEqual(new_text);

        });
    });
});
