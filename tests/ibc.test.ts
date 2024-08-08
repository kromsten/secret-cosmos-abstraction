import { expect, describe, test, beforeAll } from 'vitest';
import { getGatewayEncryptionKey, queryGatewayAuth } from '../src/gateway';
import { getConsumerClient, getConsumerWallet } from '../src/clients';
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

        const signingWallet = await getConsumerWallet();
        const signingClient = await getConsumerClient(signingWallet);
        const signerAddress = (await signingWallet.getAccounts())[0].address;
        const consumerQCFirst = await getArb36Credential(signingWallet!, "foo")
        const consumerQCSecond = await getArb36Credential(signingWallet!, "bar")

        const ibcConfig = loadIbcConfig();
        const secretGateway = loadContractConfig().gateway!;


        test('if can see simple ibc-hook msg', async () => {

            const new_text = "new_text_" + Math.random().toString(36).substring(7);

            const response = await sendIBCToken(
                signingClient,
                signerAddress,
                secretGateway.address,
                CONSUMER_TOKEN!,
                "1",
                ibcConfig.consumer_channel_id,
                gatewayHookMemo(
                    { extension: { msg: { store_secret: { text: new_text } } }},
                    secretGateway
                )
            )
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
                signingClient,
                signerAddress,
                secretGateway.address,
                CONSUMER_TOKEN!,
                "1",
                ibcConfig.consumer_channel_id,
                await gatewayChachaHookMemo(
                    signingWallet!,
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
