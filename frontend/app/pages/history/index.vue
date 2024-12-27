<template>
  <UTabs :items="funcTabsItems" class="mt-4">
    <template #main>
      <MainTab />
    </template>
    <template #admin>
      <AdminTab />
    </template>
  </UTabs>
</template>

<script setup lang="ts">
import MainTab from "./_components/MainTab.vue"
import AdminTab from "./_components/AdminTab.vue"

const funcTabsItems = [{
  slot: 'main',
  label: 'Main',
  icon: 'i-heroicons-information-circle',
}, {
  slot: 'admin',
  label: 'Admin',
  icon: 'i-carbon-operations-field',
}]

</script>
