module.exports = {
  testMatch: ['<rootDir>/src/**/?(*.)+(spec|test).ts?(x)'],
  clearMocks: true,
  transform: {
    '^.+\\.tsx?$': 'esbuild-jest',
  },
  testEnvironment: 'jsdom',
  setupFilesAfterEnv: ['<rootDir>/jest.setup.ts'],
};
