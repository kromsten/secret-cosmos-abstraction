import { expect, describe, it } from 'vitest';
import { createAccount, getAccountInfo, getAccountInfoWithAuth } from '../src/registry';

describe('Account Creation', () => {

    const public_acc_signer = "cosmos1pkptre7fdkl6gfrzlesjjvhxhlc3r4gmmk8rs6"

    const public_acc_creds = [{
        message: "SGVsbG8sIHdvcmxk",
        signature: "x9jjSFv8/n1F8gOSRjddakYDbvroQm8ZoDWht/Imc1t5xUW49+Xaq7gwcsE+LCpqYoTBxnaXLg/xgJjYymCWvw==",
        pubkey: "A08EGB7ro1ORuFhjOnZcSgwYlpe0DSFjVNUIkNNQxwKQ",
        hrp: "cosmos"
    }]

    describe('Creating a public account', async () => {
         
        /* await createAccount(
            public_acc_creds,
        ) */
        

        it("shouldn't be possible to query an account without creds", async () => {
            let res : any = await getAccountInfo({ account_id: "0" })
            expect(res as string).toContain("not found")
            
            res = await getAccountInfo({ credential_id: public_acc_creds[0].pubkey })
            expect(res as string).toContain("not found")
        });


        it("should be possible to query an account with auth data", async () => {

            const res : any  = await getAccountInfoWithAuth(
                { account_id: "0" },
                public_acc_creds
            )
            console.log("Account info: ", res);
        });

    });

 

});
