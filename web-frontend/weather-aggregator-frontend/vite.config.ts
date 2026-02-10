import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    host: true,
    proxy: {
      // Service 1: Weather
      '/api/weather': {
        target: 'http://localhost:9000',
        changeOrigin: true,
        secure: false, // Ignores the "Invalid Cert" error from before
        rewrite: (path) => path.replace(/^\/api\/weather/, '/api/v1')
      },
      // Service 2: Users
      '/api/auth': {
        target: 'http://localhost:9010',
        changeOrigin: true,
        secure: false,
        rewrite: (path) => path.replace(/^\/api\/auth/, '/api/v1/auth'),
        configure: (proxy, _options) => {
        proxy.on('proxyReq', (proxyReq, req, _res) => {
          // You can debug here to see if headers are present
          console.log('Sending Request to Backend:', req.headers.cookie);
        });
      },
      },
      // Service 3: Settings
      '/api/user': {
        target: 'http://localhost:9011',
        changeOrigin: true,
        secure: false,
        rewrite: (path) => path.replace(/^\/api\/user/, '/api/v1/user')
      }
    }
  }
})
