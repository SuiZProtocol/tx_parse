# GitHub Actions Setup

## Auto-publishing Workflow

The `publish.yml` workflow automatically publishes the package to npm when a version tag is pushed to the main branch.

### Setup Requirements

1. **NPM Token**: You need to add your npm access token as a GitHub secret:
   - Go to https://www.npmjs.com/settings/[your-username]/tokens
   - Create a new access token with "Automation" type
   - Add it to your GitHub repository: Settings → Secrets and variables → Actions → New repository secret
   - Name: `NPM_TOKEN`
   - Value: Your npm token

2. **SUI RPC URL** (Optional): If your tests require a specific Sui RPC endpoint:
   - Add as GitHub secret: `SUI_RPC_URL`
   - If not set, tests will use the default mainnet URL

### How to Trigger

1. Create and push a version tag:
   ```bash
   # For patch version (0.0.x)
   git tag v0.0.2
   git push origin v0.0.2

   # For minor version (0.x.0)
   git tag v0.1.0
   git push origin v0.1.0

   # For major version (x.0.0)
   git tag v1.0.0
   git push origin v1.0.0
   ```

2. The workflow will automatically:
   - Install dependencies
   - Run tests
   - Build the project
   - Publish to npm (only the dist folder)
   - Create a GitHub release

### What Gets Published

Only the following files are included in the npm package:
- `dist/` folder (compiled JavaScript and TypeScript definitions)
- `README.md`
- `package.json`

Source code, tests, and development files are excluded.