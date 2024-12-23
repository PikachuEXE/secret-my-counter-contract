import { MsgExecuteContract, type Permit, type Permission, type TxResponse } from "secretjs"

enum ErrorHandlingStrategy {
  DefaultNotification = "DefaultNotification",
  Return = "Return",
}

export const useSecretClientProxy = () => {
  async function queryContract<R extends any>(query: object) {
    const connectedWalletAndClientStore = useConnectedWalletAndClientStore()
    const { secretNetworkClient } = connectedWalletAndClientStore
    if (!secretNetworkClient) {
      return "App Error: queryContract called when secretNetworkClient unavailable"
    }

    const { CONTRACT_ADDRESS } = useAppRuntimeConfig()
    return await secretNetworkClient.query.compute.queryContract({
      contract_address: CONTRACT_ADDRESS,
      query: query,
    }) as R
  }

  async function executeContract({msg, onSuccess, errorHandlingStrategy = ErrorHandlingStrategy.DefaultNotification}: {msg: object, onSuccess?: (res: TxResponse) => void, errorHandlingStrategy?: ErrorHandlingStrategy}): Promise<TxResponse | string> {
    const connectedWalletAndClientStore = useConnectedWalletAndClientStore()
    const { secretNetworkClient, keplrAccount } = connectedWalletAndClientStore
    const toast = useToast()
    if (!secretNetworkClient || !keplrAccount) {
      const message = "App Error: executeContract called when secretNetworkClient unavailable"
      if (errorHandlingStrategy === ErrorHandlingStrategy.DefaultNotification) {
        toast.add({
          title: message,
          icon: 'i-mdi-alert',
          color: 'yellow',
          timeout: 5000,
        })
      }
      // Return error message regardless
      return message
    }

    const { CONTRACT_ADDRESS } = useAppRuntimeConfig()
    const transactionStatusStore = useTransactionStatusStore()
    const result = await transactionStatusStore.runTransactionWithLock(async () => {
      const executeMsg = new MsgExecuteContract({
        sender: keplrAccount.address,
        contract_address: CONTRACT_ADDRESS,
        msg: msg,
        sent_funds: [],
      })

      return await secretNetworkClient.tx.broadcast([executeMsg], {
        gasLimit: 200_000,
      })
    })

    if (typeof result === "string") {
      if (errorHandlingStrategy === ErrorHandlingStrategy.DefaultNotification) {
        toast.add({
          title: result,
          icon: 'i-mdi-alert',
          color: 'yellow',
          timeout: 5000,
        })
      }
      return result
    }

    if (onSuccess != null) {
      onSuccess(result)
    }
    return result
  }

  async function getPermit({permitName, allowedContracts, permissions, onSuccess, errorHandlingStrategy = ErrorHandlingStrategy.DefaultNotification}: {permitName: string, allowedContracts?: string[], permissions?: Permission[], onSuccess?: (res: Permit) => void, errorHandlingStrategy?: ErrorHandlingStrategy}): Promise<string | Permit> {
    const permitStore = usePermitStore()
    const { SECRET_CHAIN_ID, CONTRACT_ADDRESS, PERMIT_VALID_START_TIME_UNIX_MS } = useAppRuntimeConfig()
    permitStore.clearAllInvalidPermits(PERMIT_VALID_START_TIME_UNIX_MS)
    const permit = permitStore.getPermit(permitName)
    if (permit != null) {
      if (onSuccess != null) { onSuccess(permit) }
      return permit
    }

    const connectedWalletStore = useConnectedWalletStore()
    const connectedWalletAndClientStore = useConnectedWalletAndClientStore()
    const { secretNetworkClient, keplrAccount } = connectedWalletAndClientStore
    const toast = useToast()
    if (!secretNetworkClient || !keplrAccount) {
      const message = "App Error: getPermit called when secretNetworkClient unavailable"
      if (errorHandlingStrategy === ErrorHandlingStrategy.DefaultNotification) {
        toast.add({
          title: message,
          icon: 'i-mdi-alert',
          color: 'yellow',
          timeout: 5000,
        })
      }
      // Return error message regardless
      return message
    }

    const newPermit = await secretNetworkClient.utils.accessControl.permit.sign(
      keplrAccount.address,
      SECRET_CHAIN_ID,
      permitName,
      allowedContracts ?? [CONTRACT_ADDRESS],
      permissions ?? [],
      connectedWalletStore.connectedWalletIsKeplr,
    )
    permitStore.storePermit(permitName, newPermit)
    if (onSuccess != null) { onSuccess(newPermit) }
    return newPermit
  }

  return {
    queryContract,
    executeContract,
    getPermit,
  }
}
