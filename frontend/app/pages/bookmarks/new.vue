<template>
  <UCard>
    <template #header>
      <UButton
        color="primary"
        variant="outline"
        label="Back"
        icon="i-carbon-chevron-left"
        to="/bookmarks"
      />
    </template>

    <div class="space-y-4">
      <div class="space-y-2">
        <NumberFieldRoot
          class="text-sm text-white flex items-center"
          :required="true"
          :min="0" :max="Number.MAX_SAFE_INTEGER"
          :model-value="number"
          @update:modelValue="(v) => number = v"
          :disabled="!secretNetworkClient"
        >
          <label for="countResetValueTextInput">Number to bookmark:</label>
          <div class="ml-2 flex items-center border bg-black border-black rounded-md">
            <NumberFieldDecrement class="p-2 disabled-opacity-20 flex items-center">
              <Icon name="carbon:subtract-large" />
            </NumberFieldDecrement>
            <NumberFieldInput id="countResetValueTextInput" class="bg-transparent w-20 tabular-nums focus:outline-0 p-1" />
            <NumberFieldIncrement class="p-2 disabled-opacity-20 flex items-center">
              <Icon name="carbon:add-large" />
            </NumberFieldIncrement>
          </div>
        </NumberFieldRoot>
        <UInput
          color="primary"
          variant="outline"
          placeholder="Memo..."
          v-model="memo"
        />
        <UCheckbox
          label="Make It Public"
          v-model="makeNewDataEntryPublic"
          class="flex-grow-0"
          :disabled="!secretNetworkClient || transactionStatusStore.transactionInProgress"
        />
      </div>
      <div class="flex items-center justify-center space-x-2">
        <UButton
         color="black"
         :label="`Bookmark Number (${number})${makeNewDataEntryPublic ? ' (Public)' : ''}`"
         icon="i-carbon-bookmark-add"
         class="flex-grow-1 justify-center"
         :disabled="!secretNetworkClient || transactionStatusStore.transactionInProgress"
         @click="broadcastExecuteMsg"
        />
      </div>
    </div>

    <template
      #footer
     v-if="!transactionStatusStore.transactionInProgress && lastTxResponse"
    >
      <UAlert
       icon="i-carbon-checkmark"
       title="Done!"
       :close-button="{ icon: 'i-heroicons-x-mark-20-solid', color: 'gray', variant: 'link', padded: false }"
      >
        <template #description>
          <span v-if="lastTxResponse.code === 0">
            Success: {{ lastTxResponse.timestamp }}
          </span>
          <span v-else>
            Fail: rawLog={{ lastTxResponse.rawLog }}
          </span>
        </template>
      </UAlert>
    </template>
  </UCard>
</template>

<script setup lang="ts">
import {
  type TxResponse,
} from "secretjs"
import type { Ref } from "@vue/reactivity"

const connectedWalletEventListener = useConnectedWalletEventListener()
const connectedWalletAndClientStore = useConnectedWalletAndClientStore()
const { secretNetworkClient } = storeToRefs(connectedWalletAndClientStore)

const secretClientProxy = useSecretClientProxy()

const transactionStatusStore = useTransactionStatusStore()

const number = ref(1)
const memo = ref('')
const makeNewDataEntryPublic = ref(false)
const lastTxResponse: Ref<null | TxResponse> = ref(null)
connectedWalletEventListener.onWalletDisconnected(() => {
  lastTxResponse.value = null
})
async function broadcastExecuteMsg() {
  await secretClientProxy.executeContract({
    msg: {
      add_bookmark_number: {
        number: number.value,
        memo_text: memo.value,
        mark_entry_as_public: makeNewDataEntryPublic.value
      },
    },
    onSuccess: (res) => { lastTxResponse.value = res }
  })
}
</script>
