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
            const gatewayCode = codes.find(c => 
                Number(c.code_id!) == codeConfig.gateway?.code_id &&
                c.code_hash == codeConfig.gateway.code_hash 
            );
            expect(gatewayCode).toBeDefined();
        })

        it("should have addresses", async () => {
            expect(codeConfigExists()).toBe(true);
            expect(contractConfigExists()).toBe(true);
            const codeConfig  = loadCodeConfig();
            const contracts = loadContractConfig();
            expect(contracts.gateway?.address).toBeDefined();

            const gatewayContractInfos = await secretClient.query.compute.contractsByCodeId(
                { code_id: codeConfig.gateway!.code_id.toString() }
            )
            const gatewayContract = gatewayContractInfos.contract_infos?.find(
                c => c.contract_address == contracts.gateway?.address
            );
            expect(gatewayContract).toBeDefined();
        })
    })

});
