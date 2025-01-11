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

    <div v-if="queryEntryInProgress">
      <p>
        Loading...
      </p>
      <UProgress
        animation="carousel"
      />
    </div>
    <div v-else-if="remoteEntry" class="space-y-4">
      <div class="space-y-2">
        <p>
          Editing:
          <UBadge>
            {{ route.params.entry_id }}
          </UBadge>
        </p>
        <p>
          Number: {{ remoteEntry.number }}
        </p>
        <UInput
          color="primary"
          variant="outline"
          placeholder="Memo..."
          v-model="memo"
        />
      </div>
      <div class="flex items-center justify-center space-x-2">
        <UButton
          color="black"
          :label="`Update Entry ${route.params.entry_id}`"
          icon="i-carbon-edit"
          class="flex-grow-1 justify-center"
          :disabled="!secretNetworkClient || transactionStatusStore.transactionInProgress || queryEntryInProgress"
          @click="broadcastExecuteMsg"
        />
      </div>
    </div>
    <div v-else>
      Entry Query Failed: {{ queryEntryError }}
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
import { type BookmarkedNumberEntry } from "../types"

const route = useRoute()

const connectedWalletEventListener = useConnectedWalletEventListener()
const connectedWalletAndClientStore = useConnectedWalletAndClientStore()
const { secretNetworkClient } = storeToRefs(connectedWalletAndClientStore)
const secretClientProxy = useSecretClientProxy()
const transactionStatusStore = useTransactionStatusStore()
const permits = usePermits()

const remoteEntry = ref(null as null | BookmarkedNumberEntry)
const queryEntryError = ref(null) as Ref<String | null>
const queryEntryInProgress = computed(() => {
  if (remoteEntry.value != null) { return false }
  if (queryEntryError.value != null) { return false }

  return true
})
connectedWalletEventListener.onWalletConnected(async () => {
  await permits.getOwnerPermit(async (permit) => {
    const queryResult = await secretClientProxy.queryContract({
      with_permit: {
        query: {
          one_owned_bookmarked_number_entry: {
            entry_id: route.params.entry_id,
          },
        },
        permit: permit,
      },
    }) as {
      one_bookmarked_number_entry: {
        entry: BookmarkedNumberEntry,
      },
    } | string
    if (typeof queryResult === "string") {
      remoteEntry.value = null
      queryEntryError.value = queryResult
      return
    }

    remoteEntry.value = queryResult.one_bookmarked_number_entry.entry
    memo.value = queryResult.one_bookmarked_number_entry.entry.memo_text
    queryEntryError.value = null
  })
})

const memo = ref('')
const lastTxResponse: Ref<null | TxResponse> = ref(null)
connectedWalletEventListener.onWalletDisconnected(() => {
  lastTxResponse.value = null
})
async function broadcastExecuteMsg() {
  await secretClientProxy.executeContract({
    msg: {
      update_bookmarked_number: {
        entry_id: route.params.entry_id,
        memo_text: memo.value,
      },
    },
    onSuccess: (res) => { lastTxResponse.value = res }
  })
}
</script>
