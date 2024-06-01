/* eslint-disable @typescript-eslint/no-var-requires */
const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');

module.exports = (env, argv) => {
  const isProd = argv.mode === 'production';
  const distPath = path.resolve(__dirname, './dist');
  return {
    devServer: {
      static: {
        directory: distPath,
      },
      compress: isProd,
      port: 8000,
    },
    entry: './app/index.ts',
    output: {
      path: distPath,
      filename: 'index.js',
      webassemblyModuleFilename: 'index.wasm',
    },
    devtool: isProd ? false : 'cheap-module-source-map',
    module: {
      rules: [
        {
          test: /\.s[ac]ss$/i,
          use: ['style-loader', 'css-loader', 'sass-loader'],
        },
        {
          test: /\.css$/i,
          use: ['style-loader', 'css-loader'],
        },
        {
          test: /\.(ts|js)?$/,
          use: 'swc-loader',
          exclude: /node_modules/,
        },
      ],
    },
    plugins: [
      new CopyWebpackPlugin({
        patterns: [{ from: './public', to: distPath }],
      }),
      // We point our WasmPackPlugin to the location of the
      // the crates `Cargo.toml` file. Never the root file.
      new WasmPackPlugin({
        crateDirectory: path.join(__dirname, 'rust'),
        outDir: path.join(__dirname, 'rust/pkg'),
        // extraArgs: ""
      }),
    ],
    resolve: {
      // A little overkill for our tutorial but useful.
      extensions: ['.ts', '.tsx', '.js', '.jsx', '.mts', '.mjs'],
    },
    experiments: {
      asyncWebAssembly: true,
      syncWebAssembly: true,
    },
  };
};
