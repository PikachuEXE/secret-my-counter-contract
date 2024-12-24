<template>
  <UTabs :items="funcTabsItems" class="mt-4">
    <template #main>
      <UCard class="mt-4">
        <div class="space-y-4">
          <UButton
            color="black"
            label="Query Personal Count Update History"
            icon="i-carbon-chart-column"
            block
            :disabled="!secretNetworkClient"
            @click="queryPersonalUserCountUpdateHistoryEntries"
          />
          <template v-if="secretNetworkClient">
            <UDivider />
            <UAccordion
              :key="queryPersonalUserCountUpdateHistoryEntriesResultLastUpdatedAt"
              :items="queryPersonalUserCountUpdateHistoryEntriesResultItems"
            >
              <template #result>
                <div v-if="personalUserCountUpdateHistoryEntries == null">
                </div>
                <div v-else-if="personalUserCountUpdateHistoryEntries?.length === 0">
                  Empty
                </div>
                <div class="space-y-4" v-else>
                  <template v-for="(e, index) in personalUserCountUpdateHistoryEntries">
                    <div
                      class="history-entry"
                    >
                      <p>
                        Count Change: {{ e.count_change }}
                      </p>
                      <p>
                        Time:
                        <NuxtTime :datetime="e.created_at_in_ms" relative /> ({{ new Date(e.created_at_in_ms).toISOString() }})
                      </p>
                    </div>
                    <UDivider v-if="index < personalUserCountUpdateHistoryEntries.length - 1" />
                  </template>
                </div>
              </template>
            </UAccordion>
          </template>
        </div>
      </UCard>
    </template>
    <template #admin>
      <UCard class="mt-4">
        <div class="space-y-4">
          <UButton
            color="black"
            label="Query Global Count Update History"
            icon="i-carbon-chart-column"
            block
            :disabled="!secretNetworkClient"
            @click="queryGlobalUserCountUpdateHistoryEntries"
          />
          <template v-if="secretNetworkClient">
            <UDivider />
            <UAccordion
              :key="queryGlobalUserCountUpdateHistoryEntriesResultLastUpdatedAt"
              :items="queryGlobalUserCountUpdateHistoryEntriesResultItems"
            >
              <template #result>
                <div v-if="globalUserCountUpdateHistoryEntries == null">
                </div>
                <div v-else-if="globalUserCountUpdateHistoryEntries?.length === 0">
                  Empty
                </div>
                <div class="space-y-4" v-else>
                  <template v-for="(e, index) in globalUserCountUpdateHistoryEntries">
                    <div
                      class="history-entry"
                    >
                      <p>
                        User: {{ e.user_addr }}
                      </p>
                      <p>
                        Count Change: {{ e.count_change }}
                      </p>
                      <p>
                        Time:
                        <NuxtTime :datetime="e.created_at_in_ms" relative /> ({{ new Date(e.created_at_in_ms).toISOString() }})
                      </p>
                    </div>
                    <UDivider v-if="index < globalUserCountUpdateHistoryEntries.length - 1" />
                  </template>
                </div>
              </template>
            </UAccordion>
          </template>
        </div>
      </UCard>
    </template>
  </UTabs>
</template>

<script setup lang="ts">
import type { ComputedRef, Ref } from "@vue/reactivity"

const connectedWalletAndClientStore = useConnectedWalletAndClientStore()
const { secretNetworkClient } = storeToRefs(connectedWalletAndClientStore)

const secretClientProxy = useSecretClientProxy()

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


type UserCountUpdateHistoryEntry = {
  user_addr: string
  count_change: number
  created_at_in_ms: number
}
const personalUserCountUpdateHistoryEntries: Ref<null | UserCountUpdateHistoryEntry[]> = ref(null)
const queryPersonalUserCountUpdateHistoryEntriesError = ref(null) as Ref<String | null>
const queryPersonalUserCountUpdateHistoryEntriesResultLastUpdatedAt = ref("")
async function queryPersonalUserCountUpdateHistoryEntries() {
  await permits.getOwnerPermit(async (permit) => {
    const queryResult = await secretClientProxy.queryContract({
      with_permit: {
        query: {
          user_count_update_history_entries: {
            reverse_order: true,
          },
        },
        permit: permit,
      },
    }) as {user_count_update_history_entries: {entries: UserCountUpdateHistoryEntry[]}} | string
    if (typeof queryResult === "string") {
      personalUserCountUpdateHistoryEntries.value = null
      queryPersonalUserCountUpdateHistoryEntriesError.value = queryResult
      queryPersonalUserCountUpdateHistoryEntriesResultLastUpdatedAt.value = Date.now().toString()
      return
    }

    personalUserCountUpdateHistoryEntries.value = queryResult.user_count_update_history_entries.entries
    queryPersonalUserCountUpdateHistoryEntriesError.value = null
    queryPersonalUserCountUpdateHistoryEntriesResultLastUpdatedAt.value = Date.now().toString()
  })
}
const queryPersonalUserCountUpdateHistoryEntriesResultItems: ComputedRef<Array<any>> = computed(() => {
  return [
    {
      label: "Result",
      slot: "result",
      defaultOpen: personalUserCountUpdateHistoryEntries.value !== null,
      disabled: personalUserCountUpdateHistoryEntries.value === null,
    },
    {
      label: "Error",
      content: queryPersonalUserCountUpdateHistoryEntriesError.value ? queryPersonalUserCountUpdateHistoryEntriesError.value : "",
      defaultOpen: queryPersonalUserCountUpdateHistoryEntriesError.value !== null,
      disabled: queryPersonalUserCountUpdateHistoryEntriesError.value === null,
    },
  ]
})


const globalUserCountUpdateHistoryEntries: Ref<null | UserCountUpdateHistoryEntry[]> = ref(null)
const queryGlobalUserCountUpdateHistoryEntriesError = ref(null) as Ref<String | null>
const queryGlobalUserCountUpdateHistoryEntriesResultLastUpdatedAt = ref("")
async function queryGlobalUserCountUpdateHistoryEntries() {
  await permits.getOwnerPermit(async (permit) => {
    const queryResult = await secretClientProxy.queryContract({
      with_permit: {
        query: {
          global_user_count_update_history_entries: {
            reverse_order: true,
          },
        },
        permit: permit,
      },
    }) as {user_count_update_history_entries: {entries: UserCountUpdateHistoryEntry[]}} | string
    if (typeof queryResult === "string") {
      globalUserCountUpdateHistoryEntries.value = null
      queryGlobalUserCountUpdateHistoryEntriesError.value = queryResult
      queryGlobalUserCountUpdateHistoryEntriesResultLastUpdatedAt.value = Date.now().toString()
      return
    }

    globalUserCountUpdateHistoryEntries.value = queryResult.user_count_update_history_entries.entries
    queryGlobalUserCountUpdateHistoryEntriesError.value = null
    queryGlobalUserCountUpdateHistoryEntriesResultLastUpdatedAt.value = Date.now().toString()
  })
}
const queryGlobalUserCountUpdateHistoryEntriesResultItems: ComputedRef<Array<any>> = computed(() => {
  return [
    {
      label: "Result",
      slot: "result",
      defaultOpen: globalUserCountUpdateHistoryEntries.value !== null,
      disabled: globalUserCountUpdateHistoryEntries.value === null,
    },
    {
      label: "Error",
      content: queryGlobalUserCountUpdateHistoryEntriesError.value ? queryGlobalUserCountUpdateHistoryEntriesError.value : "",
      defaultOpen: queryGlobalUserCountUpdateHistoryEntriesError.value !== null,
      disabled: queryGlobalUserCountUpdateHistoryEntriesError.value === null,
    },
  ]
})

</script>

<style scoped>
.history-entry {
  padding: 0.5em;
}
</style>
