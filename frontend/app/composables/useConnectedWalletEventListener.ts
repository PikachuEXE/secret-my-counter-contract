
type callbackFunc = () => void

export const useConnectedWalletEventListener = createSharedComposable(() => {
  const walletDisconnectFallbacks: callbackFunc[] = []

  const connectedWalletAndClientStore = useConnectedWalletAndClientStore()
  const { secretNetworkClient } = storeToRefs(connectedWalletAndClientStore)

  function runWalletDisconnectFallbacks() {
    walletDisconnectFallbacks.forEach((f) => f())
  }

  watch(secretNetworkClient, (newValue) => {
    if (newValue == null) {
      // Disconnected
      runWalletDisconnectFallbacks()
    }
  })

  if (import.meta.client) {
    window.addEventListener("keplr_keystorechange", async () => {
      // Still got to clean stuff
      runWalletDisconnectFallbacks()
      await connectedWalletAndClientStore.connectKeplr(true)
    })
  }

  function onWalletDisconnected(func: callbackFunc): void {
    walletDisconnectFallbacks.push(func)
  }

  return {
    onWalletDisconnected,
  }
})
