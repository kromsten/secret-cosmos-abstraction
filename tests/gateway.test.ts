import { expect, describe, it } from 'vitest';
import { /* createAccount, queryGatewayRegistry, */ executeGatewayRegistry, queryGatewayRegistryInner } from '../src/registry';

describe('Account Creation', () => {

    // const signer = "cosmos1pkptre7fdkl6gfrzlesjjvhxhlc3r4gmmk8rs6"

    const message = "SGVsbG8sIHdvcmxk"
    const signature = "x9jjSFv8/n1F8gOSRjddakYDbvroQm8ZoDWht/Imc1t5xUW49+Xaq7gwcsE+LCpqYoTBxnaXLg/xgJjYymCWvw=="
    const pubkey = "A08EGB7ro1ORuFhjOnZcSgwYlpe0DSFjVNUIkNNQxwKQ"
    const hrp = "cosmos"

    const credentials = [{
        message,
        signature,
        pubkey,
        hrp
    }]



    describe('testing inner authorised methods', async () => {

        it('should be able to query simple test', async () => {
            const res = await queryGatewayRegistryInner({ test: {} }, credentials)
            expect(res as string).toContain("test success")
        });

        it('should be able to to set test text', async () => {
            const old_text = (await queryGatewayRegistryInner(
                { test_text: {} }, 
                credentials)
            ) as string;

            const new_text = "new_text_" + Math.random().toString(36).substring(7);
            expect(old_text).not.toEqual(new_text);

            await executeGatewayRegistry(
                { extension: { msg: { test: { text: new_text } } } },
            )
            
            const updated_text = (await queryGatewayRegistryInner(
                { test_text: {} }, 
                credentials)
            ) as string;

            expect(updated_text).toEqual(new_text);

        });

        
    });


    /* describe('Creating a proxy account', async () => {
         
        await createAccount(
            credentials,
        )
        
        it("shouldn't be possible to query an account without creds", async () => {
            let res : any = await queryGatewayRegistry({ account_id: "0" })
            expect(res as string).toContain("not found")
            
            res = await queryGatewayRegistry({ credential_id: credentials[0].pubkey })
            expect(res as string).toContain("not found")
        });


        it("should be possible to query an account with auth data", async () => {
            const res : any  = await queryGatewayRegistryInner(
                { account_id: "0" },
                credentials
            )
            console.log("Account info: ", res);
            expect(res as string).toContain("not found")
        });

    }); */

 

});
