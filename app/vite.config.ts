import { defineConfig } from 'vite'

// https://vitejs.dev/config/
export default defineConfig({
  build: {
    outDir: "../dist"
  },
  server: {
    "proxy": {
      "/api": "http://localhost:8080"
    }
  }
})
