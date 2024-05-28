import { readFileSync, writeFileSync, existsSync } from "fs";
import { CodeConfig, ContractConfig, IbcConfig } from "./types";

export const CONFIG_DIR = './configs';
export const CODE_FILE = 'codes.json';
export const CONTRACTS_FILE = 'contracts.json';
export const IBC_FILE = 'ibc.json';

export const CODE_CONFIG_PATH = `${CONFIG_DIR}/${CODE_FILE}`;
export const CONTRACT_CONFIG_PATH = `${CONFIG_DIR}/${CONTRACTS_FILE}`;
export const IBC_CONFIG_PATH = `${CONFIG_DIR}/${IBC_FILE}`;


export const loadCodeConfig = () : CodeConfig => {
    return JSON.parse(readFileSync(CODE_CONFIG_PATH, 'utf8'));
}

export const loadContractConfig = () : ContractConfig => {
    return JSON.parse(readFileSync(CONTRACT_CONFIG_PATH, 'utf8'));
}

export const loadIbcConfig = () : IbcConfig => {
    return JSON.parse(readFileSync(IBC_CONFIG_PATH, 'utf8'));
}

export const saveCodeConfig = (config : CodeConfig) : void => {
    const json = JSON.stringify(config, null, 4);
    writeFileSync(CODE_CONFIG_PATH, json, 'utf8');
}

export const saveContractConfig = (config : ContractConfig) : void => {
    const json = JSON.stringify(config, null, 4);
    writeFileSync(CONTRACT_CONFIG_PATH, json, 'utf8');
}

export const saveIbcConfig = (config : IbcConfig) : void => {
    const json = JSON.stringify(config, null, 4);
    writeFileSync(IBC_CONFIG_PATH, json, 'utf8');
}

export const codeConfigFileExists = () : boolean => {
    return existsSync(CODE_CONFIG_PATH);
}

export const codeConfigExists = () : boolean => {
    if (!existsSync(CODE_CONFIG_PATH)) return false;
    const config = loadCodeConfig();
    return Boolean(config.registry) && Boolean(config.account) && Boolean(config.snip20);
}

export const contractConfigFileExists = () : boolean => {
    return existsSync(CONTRACT_CONFIG_PATH);
}


export const contractConfigExists = () : boolean => {
    if (!existsSync(CONTRACT_CONFIG_PATH)) return false;
    const config = loadContractConfig();
    return Boolean(config.registry) && Boolean(config.sscrt);
}


export const ibcConfigExists = () : boolean => {
    return existsSync(IBC_CONFIG_PATH);
}