
export const useConnectedWalletPrivileges = createSharedComposable(() => {
  const isContractManager = ref(false)

  const connectedWalletEventListener = useConnectedWalletEventListener()
  connectedWalletEventListener.onWalletConnected(async () => {
    const secretClientProxy = useSecretClientProxy()
    const connectedWalletAndClientStore = useConnectedWalletAndClientStore()
    const queryResult = await secretClientProxy.queryContract({
      get_privileges: {
        wallet_address: connectedWalletAndClientStore.keplrAccount!.address,
      },
    }) as {
      is_contract_manager: boolean,
    } | string


    if (typeof queryResult === "string") {
      console.error('queryResult', queryResult)
      isContractManager.value = false
      return
    }

    isContractManager.value = queryResult.is_contract_manager
  })
  connectedWalletEventListener.onWalletDisconnected(() => {
    isContractManager.value = false
  })

  return {
    isContractManager,
  }
})
