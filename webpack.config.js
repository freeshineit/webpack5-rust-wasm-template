const path = require("path");
const CopyWebpackPlugin = require("copy-webpack-plugin");

module.exports = (env, argv) => {
  const isProd = argv.mode === "production";
  const distPath = path.resolve(__dirname, isProd ? "docs" : "dist");
  return {
    devServer: {
      static: {
        directory: distPath
      },
      compress: isProd,
      port: 8000
    },
    entry: "./app/index.ts",
    output: {
      path: distPath,
      filename: "index.js"
    },
    module: {
      rules: [
        {
          test: /\.s[ac]ss$/i,
          use: ["style-loader", "css-loader", "sass-loader"]
        },
        {
          test: /\.(ts|js)?$/,
          use: "ts-loader",
          exclude: /node_modules/
        }
      ]
    },
    plugins: [
      new CopyWebpackPlugin({
        patterns: [{ from: "./public", to: distPath }]
      })
    ],
    resolve: {
      // A little overkill for our tutorial but useful.
      extensions: [".ts", ".tsx", ".js", ".jsx", ".mts", ".mjs"]
    }
  };
};
