import { defineStore } from "pinia"
import {
  SecretNetworkClient,
} from "secretjs"
import type {
  AccountData,
  Keplr,
  OfflineDirectSigner,
} from "@keplr-wallet/types"
import { useAppRuntimeConfig } from "~/composables/useAppRuntimeConfig";

declare global {
  interface Window {
    keplr: undefined | Keplr,
  }
}

export const useConnectedWalletAndClientStore = defineStore("connectedWalletAndClientStore", {
  state: () => ({
    keplrOfflineSigner: undefined as OfflineDirectSigner | undefined,
    keplrAccount: undefined as AccountData | undefined,
  }),
  getters: {
    getKeplr: async function(): Promise<Keplr | undefined> {
      if (window.keplr) {
        return window.keplr
      }

      if (document.readyState === "complete") {
        return window.keplr
      }

      return new Promise((resolve) => {
        const documentStateChange = (event: Event) => {
          if (
            event.target &&
            (event.target as Document).readyState === "complete"
          ) {
            resolve(window.keplr)
            document.removeEventListener("readystatechange", documentStateChange)
          }
        }

        document.addEventListener("readystatechange", documentStateChange)
      })
    },

    secretNetworkClient(state) {
      if (!state.keplrOfflineSigner || !state.keplrAccount) { return }
      if (!window.keplr) { return }

      const appRuntimeConfig = useAppRuntimeConfig()
      return new SecretNetworkClient({
        url: appRuntimeConfig.SECRET_NODE_REST,
        chainId: appRuntimeConfig.SECRET_CHAIN_ID,
        wallet: state.keplrOfflineSigner,
        walletAddress: state.keplrAccount.address,
        encryptionUtils: window.keplr!.getEnigmaUtils(appRuntimeConfig.SECRET_CHAIN_ID),
      })
    },

    truncatedAddress(state) {
      if (!state.keplrAccount) { return "" }

      // `secret1` = 7, plus 4 chars
      const address = state.keplrAccount.address
      const start = address.substring(0, 7 + 4 - 1)
      const end = address.substring(address.length - 1 - 4, address.length - 1)
      return `${start}...${end}`
    },
  },
  actions: {
    async connectKeplr() {
      const runtimeConfig = useRuntimeConfig()
      // Should be dev/testnet only
      const SHOULD_SUGGEST_CUSTOM_CHAIN = runtimeConfig.public.shouldSuggestCustomChain.toString() === 'true'
      const toast = useToast()
      const keplr = await this.getKeplr
      if (!keplr) {
        toast.add({
          title: 'Keplr not detected',
          icon: 'i-mdi-alert',
          color: 'yellow',
          timeout: 5000,
        })
        return
      }

      const appRuntimeConfig = useAppRuntimeConfig()
      if (SHOULD_SUGGEST_CUSTOM_CHAIN) {
        await keplr!.experimentalSuggestChain({
          chainId: appRuntimeConfig.SECRET_CHAIN_ID,
          chainName: appRuntimeConfig.SECRET_CHAIN_NAME,
          rpc: appRuntimeConfig.SECRET_NODE_RPC,
          rest: appRuntimeConfig.SECRET_NODE_REST,
          bip44: {
            coinType: 529,
          },
          bech32Config: {
            bech32PrefixAccAddr: "secret",
            bech32PrefixAccPub: "secretpub",
            bech32PrefixValAddr: "secretvaloper",
            bech32PrefixValPub: "secretvaloperpub",
            bech32PrefixConsAddr: "secretvalcons",
            bech32PrefixConsPub: "secretvalconspub",
          },
          currencies: [
            {
              coinDenom: "SCRT",
              coinMinimalDenom: "uscrt",
              coinDecimals: 6,
              coinGeckoId: "secret",
            },
          ],
          feeCurrencies: [
            {
              coinDenom: "SCRT",
              coinMinimalDenom: "uscrt",
              coinDecimals: 6,
              coinGeckoId: "secret",
              gasPriceStep: {
                low: 0.1,
                average: 0.25,
                high: 1,
              },
            },
          ],
          stakeCurrency: {
            coinDenom: "SCRT",
            coinMinimalDenom: "uscrt",
            coinDecimals: 6,
            coinGeckoId: "secret",
          },
          features: ["secretwasm", "stargate", "ibc-transfer", "ibc-go"],
        })
      }
      await keplr!.enable(appRuntimeConfig.SECRET_CHAIN_ID)

      const offlineSigner = keplr!.getOfflineSigner(appRuntimeConfig.SECRET_CHAIN_ID)
      const accounts = await offlineSigner.getAccounts()

      this.keplrOfflineSigner = offlineSigner
      this.keplrAccount = accounts[0]
      const connectedWalletStore = useConnectedWalletStore()
      connectedWalletStore.setConnectedWalletTypeAsKeplr()
    },
    disconnectKeplr() {
      const connectedWalletStore = useConnectedWalletStore()
      const permitStore = usePermitStore()

      this.keplrOfflineSigner = undefined
      this.keplrAccount = undefined
      permitStore.clearAll()
      connectedWalletStore.resetConnectedWalletType()
    },
    async autoConnectKeplrSometimes() {
      const connectedWalletStore = useConnectedWalletStore()
      if (connectedWalletStore.connectedWalletIsKeplr) {
        await this.connectKeplr()
      }
    },
    async tryAutoConnectKeplrSometimes() {
      if (import.meta.client) {
        setTimeout(async () => {
          if (window.keplr || document.readyState === "complete") {
            await this.autoConnectKeplrSometimes()
            return
          }

          await new Promise((resolve) => {
            const documentStateChange = (event: Event) => {
              if (
                event.target &&
                (event.target as Document).readyState === "complete"
              ) {
                resolve(window.keplr)
                document.removeEventListener("readystatechange", documentStateChange)
              }
            }

            document.addEventListener("readystatechange", documentStateChange)
          })
          await this.autoConnectKeplrSometimes()
        }, 0)
      }
    }
  },
})
