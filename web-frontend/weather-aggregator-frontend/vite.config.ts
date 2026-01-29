import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    proxy: {
      // Service 1: Weather
      '/api/weather': {
        target: 'https://localhost:9000',
        changeOrigin: true,
        secure: false, // Ignores the "Invalid Cert" error from before
        rewrite: (path) => path.replace(/^\/api\/weather/, '/api/v1')
      },
      // Service 2: Users
      '/api/auth': {
        target: 'https://localhost:9010',
        changeOrigin: true,
        secure: false,
        rewrite: (path) => path.replace(/^\/api\/auth/, '/api/v1')
      },
      // Service 3: Settings
      '/api/user': {
        target: 'https://localhost:9011',
        changeOrigin: true,
        secure: false,
        rewrite: (path) => path.replace(/^\/api\/user/, '/api/v1')
      }
    }
  }
})
