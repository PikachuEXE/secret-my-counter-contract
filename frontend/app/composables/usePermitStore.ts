import { defineStore } from "pinia"
import { type Permit } from "secretjs"

interface PermitEntry {
  permit: Permit
  createdAtUnixMs: number
}

export const usePermitStore = defineStore("permits", {
  state: () => ({
    entriesPerName: {} as {[key: string]: PermitEntry},
  }),
  getters: {
    getPermit: (state) => {
      return (name: string) => state.entriesPerName[name]?.permit
    },
  },
  actions: {
    storePermit(name: string, permit: Permit) {
      if (this.entriesPerName[name] != null) {
        throw new Error(`Permit with name <${name}> exists!`)
      }
      this.entriesPerName[name] = {
        permit,
        createdAtUnixMs: Date.now(),
      }
    },
    clearAllInvalidPermits(validStartTimeUnixMs: number) {
      const newEntriesPerName = {} as {[key: string]: PermitEntry}
      for (const [key, entry] of Object.entries(this.entriesPerName)) {
        if (entry.createdAtUnixMs > validStartTimeUnixMs) {
          newEntriesPerName[key] = entry
        }
      }
      this.entriesPerName = newEntriesPerName
    },
    clearAll() {
      this.entriesPerName = {}
    },
  },
  persist: true,
})
