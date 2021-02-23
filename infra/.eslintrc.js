module.exports = {
  env: {
    browser: true,
    es2021: true,
  },
  extends: ['airbnb-typescript', 'prettier', 'prettier/@typescript-eslint'],
  parser: '@typescript-eslint/parser',
  parserOptions: {
    ecmaFeatures: {
      jsx: true,
    },
    ecmaVersion: 12,
    sourceType: 'module',
    project: 'tsconfig.json',
  },
  plugins: ['prettier', '@typescript-eslint'],
  settings: {
    'import/resolver': {
      node: {
        extensions: ['.js', '.ts'],
      },
      typescript: {},
    },
  },
  rules: {
    'prettier/prettier': [
      'error',
      { trailingComma: 'all', singleQuote: true, parser: 'typescript' },
    ],
    'import/prefer-default-export': 'off',
    '@typescript-eslint/no-use-before-define': ['error', { functions: false }],
    '@typescript-eslint/naming-convention': 'off',
    'no-new': 'off',
  },
};
