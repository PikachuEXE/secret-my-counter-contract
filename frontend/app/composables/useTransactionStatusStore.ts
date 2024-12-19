import { defineStore } from "pinia"

export const useTransactionStatusStore = defineStore("transactionStatus", {
  state: () => ({
    transactionInProgress: false,
  }),
  actions: {
    async runTransactionWithLock(callback: () => Promise<void>) {
      // Fallback, frontend should be "locked" separately
      if (this.transactionInProgress) { return }

      this.transactionInProgress = true
      try {
        await callback()
      }
      finally {
        this.transactionInProgress = false
      }
    },
  },
})
