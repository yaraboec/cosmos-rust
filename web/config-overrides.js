module.exports = function override(config, env) {
  config.resolve.fallback = {
    "path": false,
    "crypto": false
  };

  return config;
}