<template>
  <AppDesktopNav />
  <main>
    <UContainer class="pt-10">
      <NuxtPage />
    </UContainer>
  </main>
  <UNotificationStaticContainer v-if="transactionStatusStore.transactionInProgress">
    <UNotificationWithProgress
      v-if="transactionStatusStore.transactionInProgress"
      id="notification-transactionInProgress"
      title="Transaction in Progress"
      icon="i-carbon-fetch-upload-cloud"
      :timeout="0"
      :close-button="false"
    />
  </UNotificationStaticContainer>
  <UNotifications class="sm:w-sm" />
</template>

<script setup lang="ts">
const runtimeConfig = useRuntimeConfig()

useHead({
  title: `Just a Counter (${runtimeConfig.public.secretChainName})`,
})

import { storeToRefs } from 'pinia'
const transactionStatusStore = useTransactionStatusStore()
const { latestTransactionError } = storeToRefs(transactionStatusStore)
const toast = useToast()

watch(latestTransactionError, () => {
  const error = latestTransactionError.value
  if (error == null) { return }
  if (error.message.includes("Request rejected")) {
    toast.add({
      title: 'Request rejected',
      icon: 'i-carbon-stop-outline-filled',
      color: 'yellow',
      timeout: 5000,
    })
  }
})

const connectedWalletAndClientStore = useConnectedWalletAndClientStore()
// This should be called once per page load only, so placed in `app.vue`
connectedWalletAndClientStore.tryAutoConnectKeplrSometimes()
</script>

<style scoped>
/* Don't fire event on disabled whatever */
.disabled {
  pointer-events: none;
}

main {
  margin-left: 250px;
  width: calc(100% - 250px);
}
</style>
