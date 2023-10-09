module.exports = {
  configureWebpack: {
    resolve: {
      extensions: ['.ts', '.tsx'], // Add '.ts' and '.tsx' to the extensions
    },
    module: {
      rules: [
        {
          test: /\.tsx?$/, // Match TypeScript files
          exclude: /node_modules/,
          use: [
            {
              loader: 'ts-loader',
              options: {
                appendTsSuffixTo: [/\.vue$/], // Append .ts to .vue files
                transpileOnly: true, // Faster builds (optional, but recommended)
              },
            },
          ],
        },
      ],
    },
  },
};
