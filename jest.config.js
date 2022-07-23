module.exports = {
  testMatch: ['<rootDir>/src/**/?(*.)+(spec|test).[jt]s?(x)'],
  clearMocks: true,
  transform: {
    '^.+\\.tsx?$': 'esbuild-jest',
  },
  testEnvironment: 'jsdom',
};
