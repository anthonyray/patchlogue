const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin([
      "index.html",
      "style.css",
      "favicon.ico",
      { from: "assets/help.svg", to: "assets/help.svg" },
      { from: "assets/github.svg", to: "assets/github.svg" },
    ]),
  ],
};
