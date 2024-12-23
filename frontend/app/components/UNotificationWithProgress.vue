<template>
  <Transition appear v-bind="ui.transition">
    <div
      :class="wrapperClass"
      role="status"
      v-bind="attrs"
    >
      <div :class="[ui.container, ui.rounded, ui.ring]">
        <div class="flex" :class="[ui.padding, ui.gap, { 'items-start': description || $slots.description, 'items-center': !description && !$slots.description }]">
          <UIcon v-if="icon" :name="icon" :class="iconClass" />
          <UAvatar v-if="avatar" v-bind="{ size: ui.avatar.size, ...avatar }" :class="ui.avatar.base" />

          <div :class="ui.inner">
            <p v-if="(title || $slots.title)" :class="ui.title">
              <slot name="title" :title="title">
                {{ title }}
              </slot>
            </p>
            <div v-if="(description || $slots.description)" :class="twMerge(ui.description, !title && !$slots.title && 'mt-0 leading-5')">
              <slot name="description" :description="description">
                {{ description }}
              </slot>
            </div>

            <div v-if="(description || $slots.description) && actions.length" :class="ui.actions">
              <UButton v-for="(action, index) of actions" :key="index" v-bind="{ ...(ui.default.actionButton || {}), ...action }" @click.stop="onAction(action)" />
            </div>
          </div>
          <div v-if="closeButton || (!description && !$slots.description && actions.length)" :class="twMerge(ui.actions, 'mt-0')">
            <template v-if="!description && !$slots.description && actions.length">
              <UButton v-for="(action, index) of actions" :key="index" v-bind="{ ...(ui.default.actionButton || {}), ...action }" @click.stop="onAction(action)" />
            </template>

            <UButton v-if="closeButton" aria-label="Close" v-bind="{ ...(ui.default.closeButton || {}), ...closeButton }" @click.stop="onClose" />
          </div>
        </div>
        <UProgress
          :class="progressClass"
          :animation="progressAnimation"
          :size="progressSize"
          :color="progressColor"
        />
      </div>
    </div>
  </Transition>
</template>

<script lang="ts">
import { computed, toRef, defineComponent } from 'vue'
import type { PropType } from 'vue'
import { twMerge, twJoin } from 'tailwind-merge'
import UIcon from '#ui/components/elements/Icon.vue'
import UAvatar from '#ui/components/elements/Avatar.vue'
import UButton from '#ui/components/elements/Button.vue'
import UProgress from '#ui/components/elements/Progress.vue'
import { useUI } from '#ui/composables/useUI'
import { mergeConfig } from '#ui/utils'
import type {
  Avatar,
  Button,
  NotificationColor,
  NotificationAction,
  Strategy,
  ProgressAnimation,
  ProgressSize,
  ProgressColor,
  DeepPartial,
} from '#ui/types'
// @ts-expect-error
import appConfig from '#build/app.config'
import { notification } from '#ui/ui.config'
import { progress } from '#ui/ui.config'

const config = mergeConfig<typeof notification>(appConfig.ui.strategy, appConfig.ui.notification, notification)
const progressConfig = mergeConfig<typeof progress>(appConfig.ui.strategy, appConfig.ui.progress, progress)

export default defineComponent({
  components: {
    UIcon,
    UAvatar,
    UButton,
    UProgress,
  },
  inheritAttrs: false,
  props: {
    id: {
      type: [String, Number],
      required: true
    },
    title: {
      type: String,
      default: null
    },
    description: {
      type: String,
      default: null
    },
    icon: {
      type: String,
      default: () => config.default.icon
    },
    avatar: {
      type: Object as PropType<Avatar>,
      default: null
    },
    closeButton: {
      type: [Boolean, Object] as boolean | PropType<Button>,
      default: () => config.default.closeButton as Button
    },
    actions: {
      type: Array as PropType<NotificationAction[]>,
      default: () => []
    },
    callback: {
      type: Function,
      default: null
    },
    color: {
      type: String as PropType<NotificationColor>,
      default: () => config.default.color,
      validator(value: string) {
        return ['gray', ...appConfig.ui.colors].includes(value)
      }
    },
    class: {
      type: [String, Object, Array] as PropType<any>,
      default: () => ''
    },
    ui: {
      type: Object as PropType<DeepPartial<typeof config> & { strategy?: Strategy }>,
      default: () => ({})
    },
    progressAnimation: {
      type: String as PropType<ProgressAnimation>,
      default: () => progressConfig.default.animation,
      validator(value: string) {
        return Object.keys(progressConfig.animation).includes(value)
      }
    },
    progressSize: {
      type: String as PropType<ProgressSize>,
      // default: () => progressConfig.default.size,
      // Default progress size on notification seems too fat
      default: () => 'sm',
      validator(value: string) {
        return Object.keys(progressConfig.progress.size).includes(value)
      }
    },
    progressColor: {
      type: String as PropType<ProgressColor>,
      default: (rawProps) => {
        const propColor = rawProps.color as PropType<NotificationColor>
        // Default same color as notification not progress color
        // Except when invalid value passed
        return appConfig.ui.colors.includes(propColor) ? propColor : progressConfig.default.color
      },
      validator(value: string) {
        return appConfig.ui.colors.includes(value)
      }
    },
  },
  emits: ['close'],
  setup(props, { emit }) {
    const { ui, attrs } = useUI('notification', toRef(props, 'ui'), config)

    const wrapperClass = computed(() => {
      return twMerge(twJoin(
        ui.value.wrapper,
        ui.value.background?.replaceAll('{color}', props.color),
        ui.value.rounded,
        ui.value.shadow,
        ui.value.ring?.replaceAll('{color}', props.color)
      ), props.class)
    })

    const progressClass = computed(() => {
      return twJoin(
        ui.value.progress.base,
        ui.value.progress.background?.replaceAll('{color}', props.color),
        // Unlike `UNotifications`, progress bar height is not set by default (let it be controlled by prop size)
        "h-[unset]",
      )
    })

    const iconClass = computed(() => {
      return twJoin(
        ui.value.icon.base,
        ui.value.icon.color?.replaceAll('{color}', props.color)
      )
    })

    function onClose() {
      if (props.callback) {
        props.callback()
      }

      emit('close')
    }

    function onAction(action: NotificationAction) {
      if (action.click) {
        action.click()
      }

      emit('close')
    }

    return {
      // eslint-disable-next-line vue/no-dupe-keys
      ui,
      attrs,
      wrapperClass,
      progressClass,
      iconClass,
      onClose,
      onAction,
      twMerge
    }
  }
})
</script>
