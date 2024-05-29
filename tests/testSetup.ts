import { contractConfigExists, ibcConfigExists } from "../src/config";
import { setup  } from "../src/setup";


if (!ibcConfigExists() || !contractConfigExists()) {
    console.log("Running test setup...");
    await setup();
}