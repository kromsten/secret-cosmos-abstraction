import * as env from '../src/env';
import { expect, test, describe, it } from 'vitest';
import { codeConfigExists, contractConfigExists, ibcConfigExists, loadCodeConfig, loadContractConfig } from '../src/config';
import { secretClient } from '../src/clients';


describe('Env, IBC and Contract setup', () => {

    test('Checking environment variables', async () => {
        Object.entries(env).forEach(([key, value]) => {
            expect(value, key + ` in .env file must be specified`).not.toBeUndefined();
            expect(typeof value).toBe('string');
        });
    });

    describe("IBC setup", async () => {
        it("should be configured", async () => {
            expect(ibcConfigExists()).toBe(true);
        })
    })


    describe("Contracts", async () => {   
        expect(codeConfigExists()).toBe(true)
        const codeConfig  = loadCodeConfig();
        

        it("should be deployed", async () => {
            const codes = (await secretClient.query.compute.codes({})).code_infos!;
            expect(codes.length).toBeGreaterThan(0);
            const registryCode = codes.find(c => 
                Number(c.code_id!) == codeConfig.registry?.code_id &&
                c.code_hash == codeConfig.registry.code_hash 
            );
            const accountCode = codes.find(c =>
                Number(c.code_id!) == codeConfig.account?.code_id &&
                c.code_hash == codeConfig.account.code_hash
            );
            expect(registryCode).toBeDefined();
            expect(accountCode).toBeDefined();
        })

        it("should have addresses", async () => {
            expect(contractConfigExists()).toBe(true);
            const codeConfig  = loadCodeConfig();
            const contracts = loadContractConfig();
            expect(contracts.registry?.address).toBeDefined();

            const registryContractInfos = await secretClient.query.compute.contractsByCodeId(
                { code_id: codeConfig.registry!.code_id.toString() }
            )
            const registryContract = registryContractInfos.contract_infos?.find(
                c => c.contract_address == contracts.registry?.address
            );
            expect(registryContract).toBeDefined();
        })
    })

});
