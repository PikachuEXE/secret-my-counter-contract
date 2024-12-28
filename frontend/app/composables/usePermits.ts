import type { Permit } from "secretjs"

export const usePermits = () => {
  async function getOwnerPermit(onSuccess: (permit: Permit) => void) {
    const secretClientProxy = useSecretClientProxy()
    const { CONTRACT_ADDRESS } = useAppRuntimeConfig()

    return await secretClientProxy.getPermit({
      permitName: "owner",
      allowedContracts: [CONTRACT_ADDRESS],
      permissions: ["owner"],
      onSuccess: onSuccess,
    })
  }

  return {
    getOwnerPermit,
  }
}
