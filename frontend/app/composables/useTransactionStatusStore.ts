import { defineStore } from "pinia"

export const useTransactionStatusStore = defineStore("transactionStatus", {
  state: () => ({
    transactionInProgress: false,
    latestTransactionError: null as Error | null,
  }),
  actions: {
    async runTransactionWithLock<T>(callback: () => Promise<T>) {
      // Fallback, frontend should be "locked" separately
      if (this.transactionInProgress) { return "Error: Transaction In Progress" }

      this.transactionInProgress = true
      try {
        return await callback()
      }
      catch (error: any) {
        this.latestTransactionError = error
        throw error
      }
      finally {
        this.transactionInProgress = false
      }
    },
  },
})
