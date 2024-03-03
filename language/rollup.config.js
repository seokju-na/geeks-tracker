import { nodeResolve } from '@rollup/plugin-node-resolve';

export default {
  input: './src/parser.js',
  output: [
    {
      format: 'cjs',
      file: './index.cjs',
    },
    {
      format: 'es',
      file: './index.js',
    },
  ],
  external(id) {
    return !/^[.\/]/.test(id);
  },
  plugins: [nodeResolve()],
};
