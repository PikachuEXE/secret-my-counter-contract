<template>
  <Teleport to="body">
    <div :class="wrapperClass" role="region" v-bind="attrs">
      <div :class="ui.container">
        <slot />
      </div>
    </div>
  </Teleport>
</template>

<script lang="ts">
// Like UNotifications except useToast
import { computed, toRef, defineComponent } from 'vue'
import type { PropType } from 'vue'
import { twMerge, twJoin } from 'tailwind-merge'
import { useUI } from '#ui/composables/useUI'
import { mergeConfig } from '#ui/utils'
import type { DeepPartial, Strategy } from '#ui/types'
import UNotification from '#ui/components/overlays/Notification.vue'
// @ts-expect-error
import appConfig from '#build/app.config'
import { notifications } from '#ui/ui.config'

const config = mergeConfig<typeof notifications>(appConfig.ui.strategy, appConfig.ui.notifications, notifications)

export default defineComponent({
  components: {
    UNotification
  },
  inheritAttrs: false,
  props: {
    class: {
      type: [String, Object, Array] as PropType<any>,
      default: () => ''
    },
    ui: {
      type: Object as PropType<DeepPartial<typeof config> & { strategy?: Strategy }>,
      default: () => ({})
    },
  },
  setup(props) {
    const { ui, attrs } = useUI('notifications', toRef(props, 'ui'), config)

    const wrapperClass = computed(() => {
      return twMerge(twJoin(
        ui.value.wrapper,
        ui.value.position,
        ui.value.width
      ), props.class)
    })

    return {
      // eslint-disable-next-line vue/no-dupe-keys
      ui,
      attrs,
      wrapperClass
    }
  }
})
</script>
