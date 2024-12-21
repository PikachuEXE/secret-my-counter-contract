
export const useAppRuntimeConfig = () => {
  const runtimeConfig = useRuntimeConfig()
  return {
    SECRET_NODE_RPC: runtimeConfig.public.secretNodeRpc,
    SECRET_NODE_REST: runtimeConfig.public.secretNodeRest,
    SECRET_CHAIN_ID: runtimeConfig.public.secretChainId,
    SECRET_CHAIN_NAME: runtimeConfig.public.secretChainName,

    CONTRACT_ADDRESS: runtimeConfig.public.contractAddress,
  }
}
