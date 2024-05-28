
import { readFileSync, readdirSync } from "fs"
import { sha256 } from "@noble/hashes/sha256";
import { toHex, MsgStoreCodeParams, TxResultCode } from "secretjs"
import { codeConfigFileExists, contractConfigFileExists, loadCodeConfig, loadContractConfig, loadIbcConfig, saveCodeConfig, saveContractConfig } from "./config";
import { secretClient } from "./clients";
import { CodeConfig, ContractConfig, TokenData } from "./types";
import { SECRET_TOKEN } from "./env";
import { instantiateRegistry } from "./registry";
import { instantiateSnip20 } from "./snip20";
import { sleep } from "./utils";


export const uploadContracts = async (
    wasmDirPath: string = "./artifacts"
) => {
    console.log("Uploading contracts...");

    const config : CodeConfig = codeConfigFileExists() ? loadCodeConfig() : {};

    for (const file of readdirSync(wasmDirPath).filter(f => f.includes(".wasm"))) {
        if (file.includes("registry") && Boolean(config.registry)) continue;
        if (file.includes("account") && Boolean(config.account)) continue;
        if (file.includes("snip20") && Boolean(config.snip20)) continue;

        console.log(`Uploading contract: ${file}`);

        const wasmPath = `${wasmDirPath}/${file}`;
        const wasm_byte_code = readFileSync(wasmPath) as Uint8Array;
        const codeHash = toHex(sha256(wasm_byte_code)); 

        const msg : MsgStoreCodeParams = {
            sender: secretClient.address,
            wasm_byte_code,
            source: "",
            builder: ""
        }

        const tx = await secretClient.tx.compute.storeCode(msg, { gasLimit: 8_000_000 });

        if (tx.code !==  TxResultCode.Success) {
            throw new Error(`Error while uploading contract: ${tx.rawLog}`);
        }

        const codeId = Number(tx.arrayLog!.find(x => x.key === "code_id")!.value);

        const contract = {
            code_id: codeId,
            code_hash: codeHash
        }

        if (file.includes("registry")) {
            config.registry = contract
        }
        
        if (file.includes("account") ) {
            config.account = contract
        }

        if (file.includes("snip20")) {
            config.snip20 = contract
        }
       
        saveCodeConfig(config);
    }
}



export const getTokenData = () : TokenData => {

    const ibc = loadIbcConfig();
    const contracts = loadContractConfig();

    return {
        sscrt_contract: {
            address: contracts.sscrt!.address,
            hash: contracts.sscrt!.hash
        },
    }
}



export const instantiateContracts = async () => {
    const config : ContractConfig = contractConfigFileExists() 
            ? loadContractConfig() 
            : { accounts: {} };
    
    if (!Boolean(config.sscrt)) {
        config.sscrt = await instantiateSnip20("Secret SCRT", "sSCRT", SECRET_TOKEN!);
        await sleep(1000);
        saveContractConfig(config);
    }

    config.registry = await instantiateRegistry();
    saveContractConfig(config);
}