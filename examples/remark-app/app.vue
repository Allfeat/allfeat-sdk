<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { web3Accounts, web3Enable, web3FromSource } from '@polkadot/extension-dapp';
import init, { AllfeatClient } from '../../pkg/allfeat_sdk.js'

onBeforeMount(async () => {
    await web3Enable('wasm-sdk-remark-example');
    await init()
});

const sendRemark = async () => {
    const allAccounts = await web3Accounts();
    // We arbitrarily select the first account returned from the above snippet
    const account = allAccounts[0];
    const injector = await web3FromSource(account.meta.source);

    let client = await AllfeatClient.createClient();

    const tx = client.tx()
    const call = tx.system().remark("Test remark")

    const signed = await call.withSigner(injector?.signer, account.address).sign()
    await signed.submitAndWatch((status: any) => {
        console.log('Tx status:', status)
    })
}
</script>

<template>
    <div class="p-8">
        <NuxtRouteAnnouncer />

        <h1 class="text-xl font-bold mb-4">Send a Remark (via Allfeat Rust SDK WASM)</h1>
        <button @click="sendRemark" class="bg-blue-600 text-white px-4 py-2 rounded">
            Send Test Remark
        </button>
    </div>
</template>
