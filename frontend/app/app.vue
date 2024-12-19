<template>
  <UContainer class="pt-10">
    <UCard>
      <template #header>
        <div class="flex justify-between">
          <h1>Welcome to Nuxt UI Starter</h1>
          <ColorScheme><USelect v-model="$colorMode.preference" :options="['system', 'light', 'dark']" /></ColorScheme>
        </div>
      </template>
    </UCard>
    <UCard class="mt-4">
      <span>Wallet: </span>
      <UButton
        v-if="!secretNetworkClient"
        class="ml-2 flex-grow-0"
        icon="i-carbon-link"
        color="violet"
        variant="solid"
        @click="connectKeplr"
      >
        Keplr
      </UButton>
      <template v-else>
        <span>{{ keplrAccount?.address }}</span>
        <UButton
          class="ml-2 flex-grow-0"
          icon="i-carbon-unlink"
          color="violet"
          variant="solid"
          @click="disconnectKeplr"
          :disabled="transactionStatusStore.transactionInProgress"
        >
          Disconnect
        </UButton>
      </template>
    </UCard>
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
  </UContainer>
  <UNotifications />
</template>

<script setup lang="ts">
import {
  MsgExecuteContract,
  SecretNetworkClient,
  type TxResponse,
  type Permit,
} from "secretjs"
import type { ComputedRef, Ref } from "@vue/reactivity"
import type {
  AccountData,
  ChainInfo,
  Keplr,
  OfflineDirectSigner,
} from "@keplr-wallet/types"

const runtimeConfig = useRuntimeConfig()
const CONTRACT_ADDRESS = runtimeConfig.public.contractAddress
const SECRET_NODE_RPC = runtimeConfig.public.secretNodeRpc
const SECRET_NODE_REST = runtimeConfig.public.secretNodeRest
const SECRET_CHAIN_ID = runtimeConfig.public.secretChainId
// Should be dev/testnet only
const SHOULD_SUGGEST_CUSTOM_CHAIN = runtimeConfig.public.shouldSuggestCustomChain.toString() === 'true'
// Just for display
const SECRET_CHAIN_NAME = runtimeConfig.public.secretChainName

const secretNetworkClient: ComputedRef<undefined | SecretNetworkClient> = computed(() => {
  if (!keplrOfflineSigner.value || !keplrAccount.value) { return }
  if (!window.keplr) { return }

  return new SecretNetworkClient({
    url: SECRET_NODE_REST,
    chainId: SECRET_CHAIN_ID,
    wallet: keplrOfflineSigner.value,
    walletAddress: keplrAccount.value.address,
    encryptionUtils: window.keplr!.getEnigmaUtils(SECRET_CHAIN_ID),
  })
})

const transactionStatusStore = useTransactionStatusStore()

// Clear all permits stored before this time
const PERMIT_VALID_START_TIME_UNIX_MS = 0
const permitStore = usePermitStore()

const toast = useToast()

const connectedWalletStore = useConnectedWalletStore()


declare global {
  interface Window {
    keplr: undefined | Keplr,
  }
}
const keplrOfflineSigner: Ref<undefined | OfflineDirectSigner> = ref(undefined)
const keplrAccount: Ref<undefined | AccountData> = ref(undefined)
const localChainInfoForKeplr: ChainInfo = {
  chainId: SECRET_CHAIN_ID,
  chainName: SECRET_CHAIN_NAME,
  rpc: SECRET_NODE_RPC,
  rest: SECRET_NODE_REST,
  bip44: {
    coinType: 529,
  },
  bech32Config: {
    bech32PrefixAccAddr: "secret",
    bech32PrefixAccPub: "secretpub",
    bech32PrefixValAddr: "secretvaloper",
    bech32PrefixValPub: "secretvaloperpub",
    bech32PrefixConsAddr: "secretvalcons",
    bech32PrefixConsPub: "secretvalconspub",
  },
  currencies: [
    {
      coinDenom: "SCRT",
      coinMinimalDenom: "uscrt",
      coinDecimals: 6,
      coinGeckoId: "secret",
    },
  ],
  feeCurrencies: [
    {
      coinDenom: "SCRT",
      coinMinimalDenom: "uscrt",
      coinDecimals: 6,
      coinGeckoId: "secret",
      gasPriceStep: {
        low: 0.1,
        average: 0.25,
        high: 1,
      },
    },
  ],
  stakeCurrency: {
    coinDenom: "SCRT",
    coinMinimalDenom: "uscrt",
    coinDecimals: 6,
    coinGeckoId: "secret",
  },
  features: ["secretwasm", "stargate", "ibc-transfer", "ibc-go"],
}
async function getKeplr(): Promise<Keplr | undefined> {
  if (window.keplr) {
    return window.keplr
  }

  if (document.readyState === "complete") {
    return window.keplr
  }

  return new Promise((resolve) => {
    const documentStateChange = (event: Event) => {
      if (
        event.target &&
        (event.target as Document).readyState === "complete"
      ) {
        resolve(window.keplr)
        document.removeEventListener("readystatechange", documentStateChange)
      }
    }

    document.addEventListener("readystatechange", documentStateChange)
  })
}
async function connectKeplr() {
  const keplr = await getKeplr()
  if (!keplr) {
    toast.add({
      title: 'Keplr not detected',
      icon: 'i-mdi-alert',
      color: 'yellow',
      timeout: 5000,
    })
    return
  }

  if (SHOULD_SUGGEST_CUSTOM_CHAIN) {
    await keplr!.experimentalSuggestChain(localChainInfoForKeplr)
  }
  await keplr!.enable(SECRET_CHAIN_ID)

  const offlineSigner = keplr!.getOfflineSigner(SECRET_CHAIN_ID)
  const accounts = await offlineSigner.getAccounts()

  keplrOfflineSigner.value = offlineSigner
  keplrAccount.value = accounts[0]
  connectedWalletStore.setConnectedWalletTypeAsKeplr()
}
function disconnectKeplr() {
  keplrOfflineSigner.value = undefined
  keplrAccount.value = undefined
  permitStore.clearAll()
  connectedWalletStore.resetConnectedWalletType()
}
function autoConnectKeplrSometimes() {
  if (connectedWalletStore.connectedWalletIsKeplr) {
    connectKeplr()
  }
}
if (import.meta.client) {
  setTimeout(async () => {
    if (window.keplr || document.readyState === "complete") {
      autoConnectKeplrSometimes()
      return
    }

    await new Promise((resolve) => {
      const documentStateChange = (event: Event) => {
        if (
          event.target &&
          (event.target as Document).readyState === "complete"
        ) {
          resolve(window.keplr)
          document.removeEventListener("readystatechange", documentStateChange)
        }
      }

      document.addEventListener("readystatechange", documentStateChange)
    })
    autoConnectKeplrSometimes()
  }, 0)
}


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

async function queryCount() {
  const queryResult = await secretNetworkClient.value!.query.compute.queryContract({
    contract_address: CONTRACT_ADDRESS,
    // code_hash: CONTRACT_CODE_HASH,
    query: {
      get_count: {},
    },
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
async function increaseCount() {
  await transactionStatusStore.runTransactionWithLock(async () => {
    const msg = new MsgExecuteContract({
      sender: keplrAccount.value!.address,
      contract_address: CONTRACT_ADDRESS,
      // code_hash: CONTRACT_CODE_HASH,
      msg: { increment: { count: countIncreaseAmount.value } },
      sent_funds: [],
    })

    lastCountIncreaseTxResponse.value = await secretNetworkClient.value!.tx.broadcast([msg], {
      gasLimit: 200_000,
    })
  })

  // Might as well
  await queryCount()
}


async function getActivePermitWithFallbackPersisted(permitName: string, fallbackFunc: () => Promise<Permit>): Promise<Permit> {
  permitStore.clearAllInvalidPermits(PERMIT_VALID_START_TIME_UNIX_MS)
  const permit = permitStore.getPermit(permitName)
  if (permit != null) { return permit }

  const newPermit = await fallbackFunc()
  permitStore.storePermit(permitName, newPermit)
  return newPermit
}
async function getOwnerPermit() {
  // Rename this later maybe
  const permitName = "owner"
  return await getActivePermitWithFallbackPersisted(permitName, async () => await secretNetworkClient.value!.utils.accessControl.permit.sign(
    keplrAccount.value!.address,
    SECRET_CHAIN_ID,
    permitName,
    [CONTRACT_ADDRESS],
    ["owner"],
    true,
  ))
}


type PersonalStats = {
  count_increment_count: number
}
const personalStats: Ref<null | PersonalStats> = ref(null)
const queryPersonalStatsError = ref(null) as Ref<String | null>
const queryPersonalStatsResultLastUpdatedAt = ref("")
async function queryPersonalStats() {
  const permit = await getOwnerPermit()

  let queryResult = await secretNetworkClient.value!.query.compute.queryContract({
    contract_address: CONTRACT_ADDRESS,
    // code_hash: CONTRACT_CODE_HASH,
    query: {
      with_permit: {
        query: {
          user_statistic_data: {},
        },
        permit: permit,
      },
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
async function queryGlobalStats() {
  const permit = await getOwnerPermit()

  let queryResult = await secretNetworkClient.value!.query.compute.queryContract({
    contract_address: CONTRACT_ADDRESS,
    // code_hash: CONTRACT_CODE_HASH,
    query: {
      with_permit: {
        query: {
          global_statistic_data: {},
        },
        permit: permit,
      },
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
async function resetCount() {
  await transactionStatusStore.runTransactionWithLock(async () => {
    const msg = new MsgExecuteContract({
      sender: keplrAccount.value!.address,
      contract_address: CONTRACT_ADDRESS,
      // code_hash: CONTRACT_CODE_HASH,
      msg: { reset: { count: countResetValue.value } },
      sent_funds: [],
    })

    lastCountResetTxResponse.value = await secretNetworkClient.value!.tx.broadcast([msg], {
      gasLimit: 200_000,
    })
  })

  // Might as well
  await queryCount()
}
</script>

<style scoped>
/* Don't fire event on disabled whatever */
.disabled {
  pointer-events: none;
}
</style>
