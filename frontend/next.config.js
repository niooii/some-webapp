/** @type {import('next').NextConfig} */
// next.config.js
module.exports = {
    webpack: (config, { dev }) => {
      // Check if in development mode
      if (dev) {
        // Set webpackDevMiddleware configuration
        config.devServer = {
          watchOptions: {
            poll: 1000, // Check for file changes every 1000ms (1 second)
            aggregateTimeout: 300, // Delay the rebuilt after the first change for 300ms
          },
        };
      }
      
    return config;
    },
};