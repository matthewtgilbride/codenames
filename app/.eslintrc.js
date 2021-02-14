module.exports = {
  env: {
    browser: true,
    es2021: true,
  },
  extends: [
    'plugin:react/recommended',
    'airbnb-typescript',
    'prettier',
    'prettier/@typescript-eslint',
    'prettier/react',
  ],
  parser: '@typescript-eslint/parser',
  parserOptions: {
    ecmaFeatures: {
      jsx: true,
    },
    ecmaVersion: 12,
    sourceType: 'module',
    project: 'tsconfig.json',
  },
  plugins: ['prettier', 'react', 'react-hooks', '@typescript-eslint'],
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
    'react/prop-types': 'off',
    'react/jsx-props-no-spreading': 'off',
    'react/react-in-jsx-scope': 'off',
  },
};
