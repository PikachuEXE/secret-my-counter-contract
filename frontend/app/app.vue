<template>
  <UContainer class="pt-10">
    <NuxtPage />
  </UContainer>
  <UNotificationStaticContainer>
    <UNotificationWithProgress
      v-if="transactionStatusStore.transactionInProgress"
      id="notification-transactionInProgress"
      title="Transaction in Progress"
      icon="i-carbon-fetch-upload-cloud"
      :timeout="0"
      :close-button="false"
    />
  </UNotificationStaticContainer>
  <UNotifications />
</template>

<script setup lang="ts">
const transactionStatusStore = useTransactionStatusStore()
</script>

<style scoped>
/* Don't fire event on disabled whatever */
.disabled {
  pointer-events: none;
}
</style>
