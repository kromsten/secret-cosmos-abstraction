import { expect, describe, it, beforeAll } from 'vitest';
import { executeGatewayEncrypted, getGatewayEncryptionKey, queryGatewayAuth } from '../src/gateway';
import { getConsumerWallet, secretWallet } from '../src/clients';
import { getArb36Credential } from '../src/crypto';


let gatewayKey : string | undefined;

beforeAll(async () => {
    gatewayKey = await getGatewayEncryptionKey();
});


describe('Gateway contract interaction', () => {


    describe('setting secret encrypted text', async () => {
        // simply signing a 036 message withour encryption
        // only for queries (no replay-attack protection)
        const signerWallet = await getConsumerWallet();

        const consumerQueryCredential = await getArb36Credential(signerWallet, "data")

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
                signerWallet,
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


});
