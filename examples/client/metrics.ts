import { exit } from "process";
import { AllfeatClient } from "../../client/pkg/allfeat_client.js";

console.log("Creating our client instance...");
let client = await AllfeatClient.createClient();

let active_wallets = await client.getActiveWalletsCount();
let created_midds = await client.getAllMiddsCreatedCount();
console.log("Active Wallets count: " + active_wallets);
console.log("Total Created MIDDS: " + created_midds);

exit();
