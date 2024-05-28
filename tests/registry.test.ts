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
         
        await createAccount(
            public_acc_creds,
            "user_id",
            true
        ) 

        it("should be possible to query an account publicly", async () => {
            let res : any = await getAccountInfo({ account_id: "0" })
            expect(res.contract_address).toBeDefined();

            res = await getAccountInfo({ account_id: "0", extension: { full_info: true} })
            expect(res as string).toContain("not found")

            res = await getAccountInfo({ credential_id: public_acc_creds[0].pubkey })
            expect(res.contract_address).toBeDefined();

            res = await getAccountInfo({ extension: { user_id: "user_id" } })
            expect(res.contract_address).toBeDefined();
        });

        it("should be possible to query full info with auth", async () => {

            const res : any  = await getAccountInfoWithAuth(
                { account_id: "0", extension: { full_info: true }},
                public_acc_creds
            )
            console.log("Account info: ", res);
            expect(res.info.secret_tia_address).toBeDefined();
            expect(res.info.user_address).toBe(public_acc_signer)

        });

    });

 

});
