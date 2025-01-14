<template>
  <UCard class="mt-4">
    <div class="space-y-4">
      <UButton
        v-if="shownEntriesTotalCount === 0"
        color="black"
        label="Query Global Count Update History"
        icon="i-carbon-chart-column"
        block
        :disabled="!secretNetworkClient"
        @click="() => fetch(page, pageSize)"
      />
      <template v-if="secretNetworkClient">
        <UDivider />
        <UAccordion
          :key="queryEntriesResultLastUpdatedAt"
          :items="queryEntriesResultItems"
        >
          <template #result>
            <div v-if="entries == null">
            </div>
            <div v-else-if="entries?.length === 0">
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
                <template v-for="(e, index) in entries">
                  <EntryRow
                    class="p-2"
                    :entry="e"
                    :marked-as-public-at-visible="true"
                    :owner-address-visible="true"
                  />
                  <UDivider v-if="index < entries.length - 1" />
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
        </UAccordion>
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


const entries: Ref<null | BookmarkedNumberEntry[]> = ref(null)
const queryEntriesError = ref(null) as Ref<String | null>
const queryEntriesResultLastUpdatedAt = ref("")


const shownEntries: Ref<BookmarkedNumberEntry[]> = ref([])
const shownEntriesTotalCount = ref(0)
const page = ref(1)
const pageSize = ref(10)
function resetState(): void {
  entries.value = null
  queryEntriesError.value = null
  queryEntriesResultLastUpdatedAt.value = ""

  shownEntries.value = []
  shownEntriesTotalCount.value = 0
  page.value = 1
  pageSize.value = 10
}
useConnectedWalletEventListener().onWalletDisconnected(resetState)
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
      queryEntriesResultLastUpdatedAt.value = Date.now().toString()
      return
    }

    entries.value = queryResult.bookmarked_number_entries.entries
    shownEntriesTotalCount.value = queryResult.bookmarked_number_entries.total_count
    queryEntriesError.value = null
    queryEntriesResultLastUpdatedAt.value = Date.now().toString()
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
const queryEntriesResultItems: ComputedRef<Array<any>> = computed(() => {
  return [
    {
      label: "Result",
      slot: "result",
      defaultOpen: entries.value !== null,
      disabled: entries.value === null,
    },
    {
      label: "Error",
      content: queryEntriesError.value ? queryEntriesError.value : "",
      defaultOpen: queryEntriesError.value !== null,
      disabled: queryEntriesError.value === null,
    },
  ]
})

</script>
