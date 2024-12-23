// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },

  modules: [
    "@nuxt/ui",
    "@unocss/nuxt",
    "radix-vue/nuxt",
    "@pinia/nuxt",
    "pinia-plugin-persistedstate/nuxt",
    "@nuxt/icon",
    "@vueuse/nuxt",
  ],


  vite: {
    vue: {
      optionsAPI: false,
    },
    assetsInclude: [
      /\.svg/,
    ],
    css: {
      preprocessorOptions: {
        scss: {
          api: "modern-compiler",
        },
      },
    },
  },

  compatibilityDate: "2024-12-11",

  future: {
    compatibilityVersion: 4,
  },

  runtimeConfig: {
    public: {
      // NUXT_PUBLIC_CONTRACT_ADDRESS
      contractAddress: '',
      // NUXT_PUBLIC_SECRET_NODE_RPC
      secretNodeRpc: '',
      // NUXT_PUBLIC_SECRET_NODE_REST
      secretNodeRest: '',
      // NUXT_PUBLIC_SECRET_CHAIN_ID
      secretChainId: '',
      // NUXT_PUBLIC_SECRET_SHOULD_SUGGEST_CUSTOM_CHAIN
      // Mainly for dev/testnet
      shouldSuggestCustomChain: 'false',
      // NUXT_PUBLIC_SECRET_CHAIN_NAME
      // Mainly for dev/testnet, purely for display
      secretChainName: 'Unknown Secret Chain',

      // NUXT_PUBLIC_PERMIT_VALID_START_TIME_UNIX_MS
      // Permit before this time will be considered as invalid
      permitValidStartTimeUnixMs: 0,
    }
  },
})
