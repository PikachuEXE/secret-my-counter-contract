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
  ],


  vite: {
    vue: {
      optionsAPI: false,
    },
  },

  compatibilityDate: "2024-12-11",

  future: {
    compatibilityVersion: 4,
  },
})