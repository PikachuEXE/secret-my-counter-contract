
<template>
  <nav :class="ui.wrapper" v-bind="attrs">
    <ul v-for="(section, sectionIndex) of sections" :key="`section${sectionIndex}`">
      <li v-for="(link, index) of section" :key="`section${sectionIndex}-${index}`">
        <ULink
          v-slot="{ isActive }"
          v-bind="getULinkProps(link)"
          :class="[ui.base, ui.padding, ui.width, ui.ring, ui.rounded, ui.font, ui.size]"
          :active-class="ui.active"
          :inactive-class="ui.inactive"
          @click="link.click"
          @keyup.enter="$event.target.blur()"
        >
          <slot name="avatar" :link="link" :is-active="isActive">
            <UAvatar
              v-if="link.avatar"
              v-bind="{ size: ui.avatar.size, ...link.avatar }"
              :class="[ui.avatar.base]"
            />
          </slot>
          <slot name="icon" :link="link" :is-active="isActive">
            <UIcon
              v-if="link.icon"
              :name="link.icon"
              :class="twMerge(twJoin(ui.icon.base, isActive ? ui.icon.active : ui.icon.inactive), link.iconClass)"
            />
          </slot>
          <slot :link="link" :is-active="isActive">
            <span v-if="link.label" :class="twMerge(ui.label, link.labelClass)">
              <span v-if="isActive" class="sr-only">
                Current page:
              </span>
              {{ link.label }}
            </span>
          </slot>
          <slot name="badge" :link="link" :is-active="isActive">
            <UBadge
              v-if="link.badge"
              v-bind="{
                size: ui.badge.size,
                color: ui.badge.color,
                variant: ui.badge.variant,
                ...((typeof link.badge === 'string' || typeof link.badge === 'number') ? { label: link.badge } : link.badge)
              }"
              :class="ui.badge.base"
            />
          </slot>
        </ULink>
      </li>
      <UDivider v-if="sectionIndex < sections.length - 1" :ui="ui.divider" />
    </ul>
    <div class="nav-footer">
      <div class="wallet">
        <UButton
          v-if="!secretNetworkClient"
          class="flex-grow-0 "
          block
          size="xl"
          icon="i-mdi-wallet"
          color="violet"
          variant="solid"
          @click="connectKeplr"
        >
          Connect Keplr
        </UButton>
        <template v-else>
          <UBadge
            :label="truncatedAddress"
            size="lg"
            color="gray"
            class="flex flex-items-center"
          >
            <template #leading>
              <UAvatar
                :src="keplrIconUrl"
                size="xs"
                class="mr-2"
              />
            </template>
          </UBadge>
          <UButton
            class="mt-2"
            icon="i-carbon-unlink"
            block
            size="xl"
            color="gray"
            variant="outline"
            @click="disconnectKeplr"
            :disabled="transactionStatusStore.transactionInProgress"
          >
            Disconnect
          </UButton>
        </template>
      </div>
    </div>
  </nav>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { twMerge, twJoin } from 'tailwind-merge'
import UIcon from '#ui/components/elements/Icon.vue'
import UAvatar from '#ui/components/elements/Avatar.vue'
import UBadge from '#ui/components/elements/Badge.vue'
import ULink from '#ui/components/elements/Link.vue'
import UDivider from '#ui/components/layout/Divider.vue'
import { useUI } from '#ui/components/../composables/useUI'
import { mergeConfig, getULinkProps } from '#ui/utils'
import type { VerticalNavigationLink, Strategy, DeepPartial } from '#ui/types'
// @ts-expect-error
import appConfig from '#build/app.config'
import { verticalNavigation } from '#ui/ui.config'
import { storeToRefs } from "pinia";

const config = mergeConfig<typeof verticalNavigation>(appConfig.ui.strategy, appConfig.ui.verticalNavigation, verticalNavigation)

const props = defineProps({
  links: {
    type: Array as PropType<VerticalNavigationLink[][] | VerticalNavigationLink[]>,
    default: () => []
  },
  class: {
    type: [String, Object, Array] as PropType<any>,
    default: () => ''
  },
  ui: {
    type: Object as PropType<DeepPartial<typeof config> & { strategy?: Strategy }>,
    default: () => ({})
  }
})

const { ui, attrs } = useUI('verticalNavigation', toRef(props, 'ui'), config, toRef(props, 'class'))

const sections = computed(() => (Array.isArray(links[0]) ? links : [links]) as VerticalNavigationLink[][])

const links = [
  {
    label: 'Home',
    icon: 'i-mdi-home',
    to: '/',
  },
]

const connectedWalletAndClientStore = useConnectedWalletAndClientStore()
const { secretNetworkClient, truncatedAddress } = storeToRefs(connectedWalletAndClientStore)
const { connectKeplr, disconnectKeplr } = connectedWalletAndClientStore

const transactionStatusStore = useTransactionStatusStore()

import keplrIconUrl from '~/assets/Keplr_icon_ver.1.3_2.svg'
</script>


<style scoped>
nav {
  border-right: 1.5px solid rgba(228,228,228,.1);
  padding: 25px;

  position: fixed;

  width: 100%;
  max-width: 250px;
  height: 100%;
}
.nav-footer {
  border-top: 1px solid rgba(228,228,228,.1);
  padding-top: 20px;
  padding-bottom: 20px;

  position: fixed;
  bottom: 0;

  width: 100%;
  max-width: 200px;

  display: flex;
  flex-direction: column;
}
.wallet {
  width: 100%;
}
</style>
