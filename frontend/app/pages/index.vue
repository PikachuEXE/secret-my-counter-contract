<template>
  <UTabs :items="funcTabsItems" class="mt-4">
    <template #main>
      <UCard class="mt-4">
        <div class="space-y-4">
          <UButton
            color="black"
            label="Query Count"
            icon="i-carbon-query"
            block
            :disabled="!secretNetworkClient"
            @click="queryCount"
          />
          <template v-if="secretNetworkClient">
            <UDivider />
            <UAccordion
              :key="queryResultLastUpdatedAt"
              :items="queryResultItems"
            />
          </template>
        </div>
      </UCard>
      <UCard class="mt-4">
        <div class="space-y-4">
          <URange
            v-model="countIncreaseAmount"
            :min="1" :max="100"
            :disabled="!secretNetworkClient"
          />
          <UButton
            color="black"
            :label="`Increase Count (${countIncreaseAmount})`"
            icon="i-carbon-arrow-up"
            block
            :disabled="!secretNetworkClient || transactionStatusStore.transactionInProgress"
            @click="increaseCount"
          />
          <UAlert
            v-if="!transactionStatusStore.transactionInProgress && lastCountIncreaseTxResponse"
            icon="i-carbon-checkmark"
            title="Done!"
            :close-button="{ icon: 'i-heroicons-x-mark-20-solid', color: 'gray', variant: 'link', padded: false }"
          >
            <template #description>
          <span v-if="lastCountIncreaseTxResponse.code === 0">
            Success: {{ lastCountIncreaseTxResponse.timestamp }}
          </span>
              <span v-else>
            Fail: rawLog={{ lastCountIncreaseTxResponse.rawLog }}
          </span>
            </template>
          </UAlert>
        </div>
      </UCard>
      <UCard class="mt-4">
        <div class="space-y-4">
          <UButton
            color="black"
            label="Query Personal Stats"
            icon="i-carbon-chart-column"
            block
            :disabled="!secretNetworkClient"
            @click="queryPersonalStats"
          />
          <template v-if="secretNetworkClient">
            <UDivider />
            <UAccordion
              :key="queryPersonalStatsResultLastUpdatedAt"
              :items="queryPersonalStatsResultItems"
            />
          </template>
        </div>
      </UCard>
    </template>
    <template #admin>
      <UCard class="mt-4">
        <div class="space-y-4">
          <UButton
            color="black"
            label="Query Global Stats"
            icon="i-carbon-chart-column"
            block
            :disabled="!secretNetworkClient"
            @click="queryGlobalStats"
          />
          <template v-if="secretNetworkClient">
            <UDivider />
            <UAccordion
              :key="queryGlobalStatsResultLastUpdatedAt"
              :items="queryGlobalStatsResultItems"
            />
          </template>
        </div>
      </UCard>
      <UCard class="mt-4">
        <div class="space-y-4">
          <NumberFieldRoot
            class="text-sm text-white flex items-center"
            :required="true"
            :min="0" :max="10000"
            :model-value="countResetValue"
            @update:modelValue="(v) => countResetValue = v"
            :disabled="!secretNetworkClient"
          >
            <label for="countResetValueTextInput">Reset Count To:</label>
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
          <URange
            v-model="countResetValue"
            :min="0" :max="10000"
            :disabled="!secretNetworkClient"
          />
          <UButton
            color="black"
            :label="`Reset Count (${countResetValue})`"
            icon="i-carbon-arrow-up"
            block
            :disabled="!secretNetworkClient || transactionStatusStore.transactionInProgress"
            @click="resetCount"
          />
          <UAlert
            v-if="!transactionStatusStore.transactionInProgress && lastCountResetTxResponse"
            icon="i-carbon-checkmark"
            title="Done!"
            :close-button="{ icon: 'i-heroicons-x-mark-20-solid', color: 'gray', variant: 'link', padded: false }"
          >
            <template #description>
          <span v-if="lastCountResetTxResponse.code === 0">
            Success: {{ lastCountResetTxResponse.timestamp }}
          </span>
              <span v-else>
            Fail: rawLog={{ lastCountResetTxResponse.rawLog }}
          </span>
            </template>
          </UAlert>
        </div>
      </UCard>
    </template>
  </UTabs>
</template>

<script setup lang="ts">
import {
  type TxResponse,
} from "secretjs"
import type { ComputedRef, Ref } from "@vue/reactivity"

const connectedWalletEventListener = useConnectedWalletEventListener()
const connectedWalletAndClientStore = useConnectedWalletAndClientStore()
const { secretNetworkClient } = storeToRefs(connectedWalletAndClientStore)

const secretClientProxy = useSecretClientProxy()

const transactionStatusStore = useTransactionStatusStore()
const permits = usePermits()


const funcTabsItems = [{
  slot: 'main',
  label: 'Main',
  icon: 'i-heroicons-information-circle',
}, {
  slot: 'admin',
  label: 'Admin',
  icon: 'i-carbon-operations-field',
}]


const count = ref(null) as Ref<number | null>
const queryCountError = ref(null) as Ref<String | null>
const queryResultLastUpdatedAt = ref("")
connectedWalletEventListener.onWalletDisconnected(() => {
  count.value = null
  queryCountError.value = null
  queryResultLastUpdatedAt.value = ""
})

async function queryCount() {
  const queryResult = await secretClientProxy.queryContract({
    get_count: {},
  }) as {count: number} | string
  if (typeof queryResult === "string") {
    count.value = null
    queryCountError.value = queryResult
    queryResultLastUpdatedAt.value = Date.now().toString()
    return
  }

  count.value = queryResult.count
  queryCountError.value = null
  queryResultLastUpdatedAt.value = Date.now().toString()
}

const queryResultItems: ComputedRef<Array<any>> = computed(() => {
  return [
    {
      label: "Result",
      content: count.value != null ? count.value : "",
      defaultOpen: count.value !== null,
      disabled: count.value === null,
    },
    {
      label: "Error",
      content: queryCountError.value ? queryCountError.value : "",
      defaultOpen: queryCountError.value !== null,
      disabled: queryCountError.value === null,
    },
  ]
})


const countIncreaseAmount = ref(1)
const lastCountIncreaseTxResponse: Ref<null | TxResponse> = ref(null)
connectedWalletEventListener.onWalletDisconnected(() => {
  countIncreaseAmount.value = 1
  lastCountIncreaseTxResponse.value = null
})
async function increaseCount() {
  await secretClientProxy.executeContract({
    msg: { increment: { count: countIncreaseAmount.value } },
    onSuccess: (res) => { lastCountIncreaseTxResponse.value = res }
  })

  // Might as well
  await queryCount()
}


type PersonalStats = {
  count_increment_count: number
}
const personalStats: Ref<null | PersonalStats> = ref(null)
const queryPersonalStatsError = ref(null) as Ref<String | null>
const queryPersonalStatsResultLastUpdatedAt = ref("")
connectedWalletEventListener.onWalletDisconnected(() => {
  personalStats.value = null
  queryPersonalStatsError.value = null
  queryPersonalStatsResultLastUpdatedAt.value = ""
})
async function queryPersonalStats() {
  await permits.getOwnerPermit(async (permit) => {
    const queryResult = await secretClientProxy.queryContract({
      with_permit: {
        query: {
          user_statistic_data: {},
        },
        permit: permit,
      },
    }) as {user_statistic_data: PersonalStats} | string
    if (typeof queryResult === "string") {
      personalStats.value = null
      queryPersonalStatsError.value = queryResult
      queryPersonalStatsResultLastUpdatedAt.value = Date.now().toString()
      return
    }

    personalStats.value = queryResult.user_statistic_data
    queryPersonalStatsError.value = null
    queryPersonalStatsResultLastUpdatedAt.value = Date.now().toString()
  })
}
const queryPersonalStatsResultItems: ComputedRef<Array<any>> = computed(() => {
  return [
    {
      label: "Result",
      content: personalStats.value ? `count_increment_count: ${personalStats.value.count_increment_count}` : "",
      defaultOpen: personalStats.value !== null,
      disabled: personalStats.value === null,
    },
    {
      label: "Error",
      content: queryPersonalStatsError.value ? queryPersonalStatsError.value : "",
      defaultOpen: queryPersonalStatsError.value !== null,
      disabled: queryPersonalStatsError.value === null,
    },
  ]
})


type GlobalStats = {
  count_increment_count: number
  count_reset_count: number
}
const globalStats: Ref<null | GlobalStats> = ref(null)
const queryGlobalStatsError = ref(null) as Ref<String | null>
const queryGlobalStatsResultLastUpdatedAt = ref("")
connectedWalletEventListener.onWalletDisconnected(() => {
  globalStats.value = null
  queryGlobalStatsError.value = null
  queryGlobalStatsResultLastUpdatedAt.value = ""
})
async function queryGlobalStats() {
  await permits.getOwnerPermit(async (permit) => {
    const queryResult = await secretClientProxy.queryContract({
      with_permit: {
        query: {
          global_statistic_data: {},
        },
        permit: permit,
      },
    }) as {global_statistic_data: GlobalStats} | string
    if (typeof queryResult === "string") {
      globalStats.value = null
      queryGlobalStatsError.value = queryResult
      queryGlobalStatsResultLastUpdatedAt.value = Date.now().toString()
      return
    }

    globalStats.value = queryResult.global_statistic_data
    queryGlobalStatsError.value = null
    queryGlobalStatsResultLastUpdatedAt.value = Date.now().toString()
  })
}
const queryGlobalStatsResultItems: ComputedRef<Array<any>> = computed(() => {
  return [
    {
      label: "Result",
      content: globalStats.value ? `count_increment_count: ${globalStats.value.count_increment_count}, count_reset_count: ${globalStats.value.count_reset_count}` : "",
      defaultOpen: globalStats.value !== null,
      disabled: globalStats.value === null,
    },
    {
      label: "Error",
      content: queryGlobalStatsError.value ? queryGlobalStatsError.value : "",
      defaultOpen: queryGlobalStatsError.value !== null,
      disabled: queryGlobalStatsError.value === null,
    },
  ]
})


const countResetValue = ref(0)
const lastCountResetTxResponse: Ref<null | TxResponse> = ref(null)
connectedWalletEventListener.onWalletDisconnected(() => {
  countResetValue.value = 0
  lastCountResetTxResponse.value = null
})
async function resetCount() {
  await secretClientProxy.executeContract({
    msg: { reset: { count: countResetValue.value } },
    onSuccess: (res) => { lastCountResetTxResponse.value = res }
  })

  // Might as well
  await queryCount()
}
</script>
