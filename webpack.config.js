 
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
      hot: true,
      open: false,
      historyApiFallback: true,
    },
    
    entry: './app/index.ts',
    
    output: {
      path: distPath,
      filename: isProd ? '[name].[contenthash:8].js' : 'index.js',
      webassemblyModuleFilename: isProd ? '[hash].wasm' : 'index.wasm',
      clean: true,
      publicPath: '/',
    },
    
    devtool: isProd ? 'source-map' : 'eval-source-map',
    
    module: {
      rules: [
        {
          test: /\.s[ac]ss$/i,
          use: [
            'style-loader',
            {
              loader: 'css-loader',
              options: {
                sourceMap: !isProd,
              },
            },
            {
              loader: 'sass-loader',
              options: {
                sourceMap: !isProd,
              },
            },
          ],
        },
        {
          test: /\.css$/i,
          use: [
            'style-loader',
            {
              loader: 'css-loader',
              options: {
                sourceMap: !isProd,
              },
            },
          ],
        },
        {
          test: /\.(ts|tsx)?$/,
          use: {
            loader: 'swc-loader',
            options: {
              jsc: {
                parser: {
                  syntax: 'typescript',
                  tsx: true,
                },
                target: 'es2020',
                minify: isProd ? {
                  compress: true,
                  mangle: true,
                } : undefined,
              },
            },
          },
          exclude: /node_modules/,
        },
        {
          test: /\.js$/,
          use: {
            loader: 'swc-loader',
            options: {
              jsc: {
                parser: {
                  syntax: 'ecmascript',
                },
                target: 'es2020',
              },
            },
          },
          exclude: /node_modules/,
        },
      ],
    },
    
    plugins: [
      new CopyWebpackPlugin({
        patterns: [{ from: './public', to: distPath }],
      }),
      new WasmPackPlugin({
        crateDirectory: path.join(__dirname, 'rust'),
        outDir: path.join(__dirname, 'rust/pkg'),
        forceMode: isProd ? 'production' : 'development',
        extraArgs: '--target web',
      }),
    ],
    
    resolve: {
      extensions: ['.ts', '.tsx', '.js', '.jsx', '.mjs', '.wasm'],
      alias: {
        '@': path.resolve(__dirname, 'app'),
        '@rust': path.resolve(__dirname, 'rust/pkg'),
      },
    },
    
    experiments: {
      asyncWebAssembly: true,
      syncWebAssembly: true,
    },
    
    optimization: {
      minimize: isProd,
      usedExports: true,
      sideEffects: true,
      splitChunks: isProd ? {
        chunks: 'all',
        cacheGroups: {
          vendor: {
            test: /[\\/]node_modules[\\/]/,
            name: 'vendors',
            priority: 10,
          },
          wasm: {
            test: /\.wasm$/,
            name: 'wasm',
            priority: 20,
          },
        },
      } : false,
    },
    
    performance: {
      hints: isProd ? 'warning' : false,
      maxEntrypointSize: 512000,
      maxAssetSize: 512000,
    },
  };
};
