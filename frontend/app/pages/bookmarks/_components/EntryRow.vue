<template>
  <div>
    <p v-if="props.ownerAddressVisible">
      Owner: {{ props.entry.owner_addr }}
    </p>
    <p>
      Number: {{ props.entry.number }}
    </p>
    <p v-if="props.entry.memo_text">
      Memo: {{ props.entry.memo_text }}
    </p>
    <p>
      Created:
      <NuxtTime :datetime="props.entry.created_at_in_ms" relative /> ({{ new Date(props.entry.created_at_in_ms).toISOString() }})
    </p>
    <p>
      Updated:
      <NuxtTime :datetime="props.entry.updated_at_in_ms" relative /> ({{ new Date(props.entry.updated_at_in_ms).toISOString() }})
    </p>
    <div v-if="props.editButtonVisible" class="mt-2">
      <UButton
        label="Edit"
        color="primary"
        variant="outline"
        size="sm"
        icon="i-carbon-edit"
        :to="`/bookmarks/${props.entry.entry_id}/edit`"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { type BookmarkedNumberEntry } from "../types"

const props = defineProps({
  entry: {
    type: Object as PropType<BookmarkedNumberEntry>,
    required: true,
  },
  ownerAddressVisible: {
    type: Boolean,
    required: true,
  },
  editButtonVisible: {
    type: Boolean,
    default: false,
  },
})
</script>
