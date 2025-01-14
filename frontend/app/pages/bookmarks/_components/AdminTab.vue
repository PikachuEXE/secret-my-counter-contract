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
      <template v-if="shownEntries != null">
        <UDivider />
        <p v-if="queryEntriesError != null">
          {{ queryEntriesError }}
        </p>
        <div v-else-if="shownEntries?.length === 0">
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
            <template v-for="(e, index) in shownEntries">
              <EntryRow
                class="p-2"
                :entry="e"
                :marked-as-public-at-visible="true"
                :owner-address-visible="true"
              />
              <UDivider v-if="index < shownEntries.length - 1" />
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

import { type BookmarkedNumberEntry } from "../types"
import EntryRow from "./EntryRow.vue"

const connectedWalletStore = useConnectedWalletStore()
const connectedWalletAndClientStore = useConnectedWalletAndClientStore()
const { secretNetworkClient } = storeToRefs(connectedWalletAndClientStore)

const secretClientProxy = useSecretClientProxy()

const permits = usePermits()


const shownEntries: Ref<null | BookmarkedNumberEntry[]> = ref(null)
const shownEntriesTotalCount = ref(0)
const queryEntriesError = ref(null) as Ref<String | null>
const page = ref(1)
const pageSize = ref(10)
function resetState(): void {
  shownEntries.value = null
  shownEntriesTotalCount.value = 0
  queryEntriesError.value = null
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
          global_bookmarked_number_entries: {
            reverse_order: true,
            page: page,
            page_size: pageSize,
          },
        },
        permit: permit,
      },
    }) as {
      bookmarked_number_entries: {
        entries: BookmarkedNumberEntry[],
        total_count: number,
      },
    } | string
    if (typeof queryResult === "string") {
      shownEntries.value = []
      shownEntriesTotalCount.value = Number.POSITIVE_INFINITY
      queryEntriesError.value = queryResult
      return
    }

    shownEntries.value = queryResult.bookmarked_number_entries.entries
    shownEntriesTotalCount.value = queryResult.bookmarked_number_entries.total_count
    queryEntriesError.value = null
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
  total: shownEntriesTotalCount,
  page: 1,
  pageSize,
  onPageChange: fetchData,
  onPageSizeChange: fetchData,
})

</script>
