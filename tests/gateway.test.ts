import { expect, describe, it, test, beforeAll } from 'vitest';
import { executeGatewayEncrypted, getGatewayEncryptionKey, queryGatewayAuth } from '../src/gateway';
import { consumerClient, consumerWallet, secretWallet } from '../src/clients';
import { gatewayChachaHookMemo, gatewayHookMemo, sendIBCToken } from '../src/ibc';
import { loadContractConfig, loadIbcConfig } from '../src/config';
import { getArb36Credential } from '../src/crypto';
import { CONSUMER_TOKEN } from '../src/env';

describe('Gateway contract interaction', () => {

    let gatewayKey : string | undefined;

    beforeAll(async () => {
        gatewayKey = await getGatewayEncryptionKey();
        console.log("Gateway key:", gatewayKey)
    });
    

    describe('setting secret encrypted text', async () => {
        // simply signing a 036 message withour encryption
        // only for queries (no replay-attack protection)
        const consumerQueryCredential = await getArb36Credential(consumerWallet, "data")
        const secretQueryCredential = await getArb36Credential(secretWallet, "data")

        it('should be able to to set secret texts', async () => {
            const old_text = (
                await queryGatewayAuth(
                    { get_secret: {} }, 
                    [consumerQueryCredential]
                )
            ) as string;

            const new_text = "new_text_" + Math.random().toString(36).substring(7);
            expect(old_text).not.toEqual(new_text);
            
            // called with regular  authentication + encryption 
            // regular secret wallet relaying the message
            await executeGatewayEncrypted(
                { extension: { msg: { store_secret: { text: new_text } } } },
                consumerWallet,
                gatewayKey
            )
            
            // secret wallet is relaying the queries but
            // it can't read anyting by itself without passed credentials 
            const non_auth_text = (await queryGatewayAuth(
                { get_secret: {} },
                []
            )) as string;
            expect(non_auth_text.toLowerCase()).toContain("must not be empty");


            // the consumer can read the secret text
            const updated_text = (await queryGatewayAuth(
                { get_secret: {} },
                [consumerQueryCredential]
            )) as string;
            expect(updated_text).toEqual(new_text);


            // the secret wallet can pass it's own credentials
            // but can only access a secret text of it's own
            const secret_text = (await queryGatewayAuth(
                { get_secret: {} },
                [secretQueryCredential]
            )) as string;
            expect(secret_text).not.toEqual(new_text);
        });

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
            console.log("hook res:", response)
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
            console.log("hook res:", response)
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
