
type callbackFunc = () => void

export const useConnectedWalletEventListener = createSharedComposable(() => {
  const walletDisconnectFallbacks: callbackFunc[] = []

  const connectedWalletAndClientStore = useConnectedWalletAndClientStore()
  const { secretNetworkClient } = storeToRefs(connectedWalletAndClientStore)

  watch(secretNetworkClient, (newValue) => {
    if (newValue == null) {
      // Disconnected
      walletDisconnectFallbacks.forEach((f) => f())
    }
  })

  function onWalletDisconnected(func: callbackFunc): void {
    walletDisconnectFallbacks.push(func)
  }

  return {
    onWalletDisconnected,
  }
})
