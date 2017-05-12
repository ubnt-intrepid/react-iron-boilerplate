'use strict';
var path = require('path');

module.exports = {
  entry: {
    index: './src/javascript/index.jsx',
  },
  
  output: {
    path: path.join(__dirname, 'assets/js'),
    filename: '[name].bundle.js',
  },

  devtool: 'inline-source-map',

  module: {
    loaders: [
      {
        test: /\.jsx$/,
        exclude: /node_modules/,
        loader: 'babel-loader',
      },
      {
        test: /\.css$/,
        exclude: /node_modules/,
        loader: 'css-loader',
      }
    ]
  },

  resolve: {
    extensions: [ '.js', '.jsx' ],
  }
};
