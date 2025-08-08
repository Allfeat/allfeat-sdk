// Example using Allfeat SDK
// This is a basic template for using the Allfeat client
import { createClient } from "polkadot-api";
import { getSmProvider } from "polkadot-api/sm-provider";
import { start } from "polkadot-api/smoldot";
import { melodieDev as chainSpec } from "@allfeat/papi-providers";
async function main() {
    try {
        console.log("Allfeat SDK example");
        const smoldot = start();
        const chain = await smoldot.addChain({
            chainSpec: JSON.stringify(chainSpec),
        });
        const client = createClient(getSmProvider(chain));
        client.finalizedBlock$.subscribe((finalizedBlock) => console.log(finalizedBlock.number, finalizedBlock.hash));
    }
    catch (error) {
        console.error("Error:", error);
    }
}
main().catch(console.error);
//# sourceMappingURL=index.js.map