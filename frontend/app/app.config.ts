export default defineAppConfig({
  title: "Pika My Counter Contract Frontend",
  ui: {
    primary: 'lime',
    gray: 'neutral',

    notifications: {
      // Show toasts at the top left of the screen
      position: 'top-0 bottom-[unset] start-0 end-[unset]',
      // `sm:w-sm` needed for unocss
      // Also setting the value here only won't work due to class name not detected by unocss
      // Must have at least 1 presence in template (e.g. on `UNotifications`)
      width: 'sm:w-sm',
    },
  }
})
