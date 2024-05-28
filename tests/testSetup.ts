import { setup, settingUp } from "../src/setup";


if (!settingUp) {
    console.log("Running test setup...");
    await setup();
}