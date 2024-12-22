import { MsgExecuteContract, type TxResponse } from "secretjs"

enum ErrorHandlingStrategy {
  DefaultNotification = "DefaultNotification",
  Return = "Return",
}
type ExecuteContractonSuccess = (res: TxResponse) => void

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

  async function executeContract({msg, onSuccess, errorHandlingStrategy = ErrorHandlingStrategy.DefaultNotification}: {msg: object, onSuccess?: ExecuteContractonSuccess, errorHandlingStrategy?: ErrorHandlingStrategy}): Promise<TxResponse | string> {
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

  return {
    queryContract,
    executeContract,
  }
}
