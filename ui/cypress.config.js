const { defineConfig } = require('cypress');

module.exports = defineConfig({
  e2e: {
    baseUrl: 'http://localhost:1420',
    setupNodeEvents(on, config) {
      // implement node event listeners here
    },
  },
  
  component: {
    devServer: {
      framework: 'react',
      bundler: 'vite',
    },
  },
  
  viewportWidth: 1280,
  viewportHeight: 800,
  
  // Configure retries
  retries: {
    runMode: 2,
    openMode: 0,
  },
  
  // Configure screenshots and videos
  screenshotOnRunFailure: true,
  video: true,
  
  // Environment variables
  env: {
    apiUrl: 'http://localhost:1420/api',
  },
});