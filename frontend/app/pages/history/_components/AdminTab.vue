<template>
  <UCard class="mt-4">
    <div class="space-y-4">
      <UButton
        color="black"
        label="Query Global Count Update History"
        icon="i-carbon-chart-column"
        block
        :disabled="!secretNetworkClient"
        @click="refreshInitPage"
      />
      <template v-if="userCountUpdateHistoryEntries != null">
        <UDivider />
        <p v-if="queryUserCountUpdateHistoryEntriesError">
          {{ queryUserCountUpdateHistoryEntriesError }}
        </p>
        <div v-else-if="userCountUpdateHistoryEntries?.length === 0">
          Empty
        </div>
        <div class="space-y-4" v-else>
          <div class="flex justify-center gap-x-2">
            <UButton
              label="prev"
              color="primary"
              variant="ghost"
              size="xs"
              icon="i-mdi-arrow-left"
              :disabled="isFirstPage"
              @click="prev"
            />
            <UBadge
              :label="`${currentPage} / ${pageCount}`"
              color="primary"
              variant="solid"
              size="xs"
            />
            <UButton
              label="next"
              color="primary"
              variant="ghost"
              size="xs"
              icon="i-mdi-arrow-right"
              :trailing="true"
              :disabled="isLastPage"
              @click="next"
            />
          </div>

          <div class="entry-list">
            <template v-for="(e, index) in userCountUpdateHistoryEntries">
              <div
                class="p-2"
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
              <UDivider v-if="index < userCountUpdateHistoryEntries.length - 1" />
            </template>
          </div>

          <div class="flex justify-center gap-x-2">
            <UButton
              label="prev"
              color="primary"
              variant="ghost"
              size="xs"
              icon="i-mdi-arrow-left"
              :disabled="isFirstPage"
              @click="prev"
            />
            <UBadge
              :label="`${currentPage} / ${pageCount}`"
              color="primary"
              variant="solid"
              size="xs"
            />
            <UButton
              label="next"
              color="primary"
              variant="ghost"
              size="xs"
              icon="i-mdi-arrow-right"
              :trailing="true"
              :disabled="isLastPage"
              @click="next"
            />
          </div>
        </div>
      </template>
    </div>
  </UCard>
</template>

<script setup lang="ts">
import { useOffsetPagination } from "@vueuse/core"

import type { UserCountUpdateHistoryEntry } from "../types.d.ts"

const connectedWalletStore = useConnectedWalletStore()
const connectedWalletAndClientStore = useConnectedWalletAndClientStore()
const { secretNetworkClient } = storeToRefs(connectedWalletAndClientStore)

const secretClientProxy = useSecretClientProxy()

const permits = usePermits()


const userCountUpdateHistoryEntries: Ref<null | UserCountUpdateHistoryEntry[]> = ref(null)
const queryUserCountUpdateHistoryEntriesError = ref(null) as Ref<String | null>


const shownUserCountUpdateHistoryEntries: Ref<UserCountUpdateHistoryEntry[]> = ref([])
const shownUserCountUpdateHistoryEntriesTotalCount = ref(0)
const page = ref(1)
const pageSize = ref(10)
function resetState(): void {
  userCountUpdateHistoryEntries.value = null
  queryUserCountUpdateHistoryEntriesError.value = null

  shownUserCountUpdateHistoryEntries.value = []
  shownUserCountUpdateHistoryEntriesTotalCount.value = 0
  page.value = 1
  pageSize.value = 10
}
useConnectedWalletEventListener().onWalletDisconnected(resetState)
async function refreshInitPage() {
  page.value = 1
  pageSize.value = 10
  await fetch(page.value, pageSize.value)
}
function fetchData({ currentPage, currentPageSize }: { currentPage: number, currentPageSize: number }) {
  if (!connectedWalletStore.isWalletConnected) { return }

  fetch(currentPage, currentPageSize)
}
async function fetch(page: number, pageSize: number) {
  await permits.getOwnerPermit(async (permit) => {
    const queryResult = await secretClientProxy.queryContract({
      with_permit: {
        query: {
          global_user_count_update_history_entries: {
            reverse_order: true,
            page: page,
            page_size: pageSize,
          },
        },
        permit: permit,
      },
    }) as {
      user_count_update_history_entries: {
        entries: UserCountUpdateHistoryEntry[],
        total_count: number,
      },
    } | string
    if (typeof queryResult === "string") {
      shownUserCountUpdateHistoryEntries.value = []
      shownUserCountUpdateHistoryEntriesTotalCount.value = Number.POSITIVE_INFINITY
      queryUserCountUpdateHistoryEntriesError.value = queryResult
      return
    }

    userCountUpdateHistoryEntries.value = queryResult.user_count_update_history_entries.entries
    shownUserCountUpdateHistoryEntriesTotalCount.value = queryResult.user_count_update_history_entries.total_count
    queryUserCountUpdateHistoryEntriesError.value = null
  })
}

const {
  currentPage,
  pageCount,
  isFirstPage,
  isLastPage,
  prev,
  next,
} = useOffsetPagination({
  total: shownUserCountUpdateHistoryEntriesTotalCount,
  page: 1,
  pageSize,
  onPageChange: fetchData,
  onPageSizeChange: fetchData,
})

</script>
