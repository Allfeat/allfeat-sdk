import { exit } from "process";
import { AllfeatClient } from "../../pkg/allfeat_sdk.js";

console.log("Creating our client instance...");
let client = await AllfeatClient.createClient();

let balance = await client.getBalanceOf(
    "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
);
console.log("💰 Balance: " + balance);

exit();
