const path = require("path");
const webpack = require("webpack");
var HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
  entry: "./src/index.tsx",
  mode: "development",
  module: {
    rules: [
      {
        test: /\.ts(x?)$/,
        exclude: /(node_modules|bower_components)/,
        loader: "ts-loader"
      }
    ]
  },
  resolve: { extensions: [".ts", ".tsx", ".js"] },
  output: {
    path: path.resolve(__dirname, "build/"),
    publicPath: "/build/",
    filename: "bundle.js"
  },
  devtool: "inline-source-map",
  devServer: {
    contentBase: path.join(__dirname, "public/"),
    port: 3000,
    publicPath: "http://localhost:3000/build/",
    hotOnly: true
  },
  plugins: [
      new webpack.HotModuleReplacementPlugin(),
      new HtmlWebpackPlugin()
    ]
};