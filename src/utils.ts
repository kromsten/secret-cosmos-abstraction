import { SecretNetworkClient } from "secretjs";

export const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

export const getPermit = async (
    client: SecretNetworkClient, 
    contractAddress : string,
    chainId: string
) => {
    
    return await client.utils.accessControl.permit.sign(
        client.address,
        chainId,
        "my-permit",
        [contractAddress],
        ["owner"],
        false
    )

}