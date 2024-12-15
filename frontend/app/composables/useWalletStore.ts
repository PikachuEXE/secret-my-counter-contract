import { defineStore } from "pinia"

enum CollectedWalletType {
  Keplr = "Keplr",
}

export const useConnectedWalletStore = defineStore("connectedWallet", {
  state: () => ({
    connectedWalletType: null as null | CollectedWalletType,
    connectedWalletTypeUpdatedAtUnixMs: 0,
  }),
  getters: {
    connectedWalletIsKeplr: (state) => {
      return state.connectedWalletType === CollectedWalletType.Keplr
    },
  },
  actions: {
    setConnectedWalletTypeAsKeplr() {
      this.connectedWalletType = CollectedWalletType.Keplr
      this.connectedWalletTypeUpdatedAtUnixMs = Date.now()
    },
    resetConnectedWalletType() {
      this.connectedWalletType = null
    }
  },
  persist: true,
})
