import js from '@eslint/js';
import globals from 'globals';
import reactHooks from 'eslint-plugin-react-hooks';
import tseslint from 'typescript-eslint';

export default tseslint.config(
  { ignores: ['dist', 'node_modules', 'src-tauri/target', 'src-tauri/gen'] },
  js.configs.recommended,
  ...tseslint.configs.recommended,
  {
    files: ['**/*.{ts,tsx}'],
    languageOptions: { globals: { ...globals.browser } },
    plugins: { 'react-hooks': reactHooks },
    rules: {
      ...reactHooks.configs.recommended.rules,
      '@typescript-eslint/no-explicit-any': 'error',
      '@typescript-eslint/consistent-type-imports': 'error',
      // Interfaces sempre com prefixo "I" + PascalCase.
      '@typescript-eslint/naming-convention': [
        'error',
        { selector: 'interface', format: ['PascalCase'], custom: { regex: '^I[A-Z]', match: true } },
        { selector: 'typeAlias', format: ['PascalCase'], custom: { regex: '^I[A-Z]', match: false } },
      ],
      // Componentes/hooks apenas com `function`; sem classes nem React.FC.
      'no-restricted-syntax': [
        'error',
        {
          selector: "VariableDeclarator[id.name=/^[A-Z]/] > ArrowFunctionExpression",
          message: 'Declare componentes React com `function`, não com arrow functions.',
        },
        {
          selector: "VariableDeclarator[id.name=/^use[A-Z]/] > ArrowFunctionExpression",
          message: 'Declare hooks personalizados com `function`.',
        },
        {
          selector: "TSTypeReference[typeName.name='FC'], TSTypeReference[typeName.right.name='FC']",
          message: 'Não use React.FC / FC.',
        },
        {
          selector: "ClassDeclaration[superClass.name=/Component$/], ClassDeclaration[superClass.property.name=/Component$/]",
          message: 'Não use componentes de classe.',
        },
      ],
    },
  },
);
