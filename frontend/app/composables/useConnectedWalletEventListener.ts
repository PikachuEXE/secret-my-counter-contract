
type callbackFunc = () => void

export const useConnectedWalletEventListener = createSharedComposable(() => {
  const walletConnectFallbacks: callbackFunc[] = []
  const walletDisconnectFallbacks: callbackFunc[] = []

  const connectedWalletAndClientStore = useConnectedWalletAndClientStore()
  const { secretNetworkClient } = storeToRefs(connectedWalletAndClientStore)

  function runWalletConnectFallbacks() {
    walletConnectFallbacks.forEach((f) => f())
  }
  function runWalletDisconnectFallbacks() {
    walletDisconnectFallbacks.forEach((f) => f())
  }

  watch(secretNetworkClient, (newValue) => {
    if (newValue == null) {
      // Disconnected
      runWalletDisconnectFallbacks()
    }
    else {
      runWalletConnectFallbacks()
    }
  })

  if (import.meta.client) {
    window.addEventListener("keplr_keystorechange", async () => {
      // Still got to clean stuff
      runWalletDisconnectFallbacks()
      await connectedWalletAndClientStore.connectKeplr(true)
    })
  }

  function onWalletConnected(func: callbackFunc): void {
    walletConnectFallbacks.push(func)
  }
  function onWalletDisconnected(func: callbackFunc): void {
    walletDisconnectFallbacks.push(func)
  }

  return {
    onWalletConnected,
    onWalletDisconnected,
  }
})
