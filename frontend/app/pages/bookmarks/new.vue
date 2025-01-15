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
        <div class="space-y-2">
          <UCheckbox
            label="Make It Public"
            v-model="makeDataEntryPublic"
            class="flex-grow-0"
            :disabled="!secretNetworkClient || transactionStatusStore.transactionInProgress"
          />
          <UAlert
            v-if="makeDataEntryPublic"
            icon="i-carbon-warning-alt"
            title="Warning"
          >
            <template #description>
              Currently public entries cannot be updated to be private except the last public entry to prevent expensive index rewrite
            </template>
          </UAlert>
        </div>
      </div>
      <div class="flex items-center justify-center space-x-2">
        <UButton
         color="black"
         :label="`Bookmark Number (${number})${makeDataEntryPublic ? ' (Public)' : ''}`"
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
        v-if="lastTxResponse.code === 0"
        icon="i-carbon-checkmark"
        title="Done!"
      >
        <template #description>
          Success: {{ lastTxResponse.timestamp }}
        </template>
      </UAlert>
      <UAlert
        v-else
        icon="i-carbon-warning"
        title="Oops"
      >
        <template #description>
          Fail: rawLog={{ lastTxResponse.rawLog }}
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

const route = useRoute()

const connectedWalletEventListener = useConnectedWalletEventListener()
const connectedWalletAndClientStore = useConnectedWalletAndClientStore()
const { secretNetworkClient } = storeToRefs(connectedWalletAndClientStore)

const secretClientProxy = useSecretClientProxy()

const transactionStatusStore = useTransactionStatusStore()

const numberFromQuery = typeof route.query.number === "string" && parseInt(route.query.number) || NaN
const number = ref(isNaN(numberFromQuery) ? 1 : numberFromQuery)
const memo = ref('')
const makeDataEntryPublic = ref(false)
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
        mark_entry_as_public: makeDataEntryPublic.value
      },
    },
    onSuccess: (res) => { lastTxResponse.value = res }
  })
}
</script>
