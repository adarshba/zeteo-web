# Project Structure Guide

## Monorepo vs Separate Repositories

### ✅ Recommended: Single Repository (Current Setup)

This project uses a **monorepo structure**, which is ideal for this use case. Here's why:

#### Advantages of Monorepo

1. **Unified Version Control**
   - Single source of truth for all components
   - Easier to track changes across frontend and backend
   - Atomic commits that span multiple components

2. **Simplified Setup**
   - One `git clone` gets everything
   - All documentation in one place
   - Easier for new contributors

3. **Better Developer Experience**
   - Jump between frontend and backend easily
   - See the full picture in one place
   - Shared tooling and scripts

4. **Consistent Releases**
   - Backend and frontend versions always match
   - No version compatibility issues
   - Single changelog

5. **Easier Testing**
   - Test integration between components
   - CI/CD is simpler
   - End-to-end tests in one place

6. **Documentation Co-location**
   - Setup guides reference both components
   - Examples can show full stack
   - No duplicate documentation

#### When to Use Separate Repositories

Consider splitting into separate repos if:

- **Different teams** own backend and frontend with independent release cycles
- **Different programming languages** with teams that don't overlap
- **Very large codebases** (>100k LOC each) causing performance issues
- **Different deployment schedules** - frontend and backend deploy independently
- **Open source considerations** - you want to open source only one part
- **Different security requirements** - stricter access control needed for one component

### Current Structure

```
zeteo/                                  # 📦 Monorepo root
│
├── 🦀 backend/                     # Backend service
│   ├── src/
│   │   ├── main.rs                     # API server
│   │   ├── ai.rs                       # AI integration
│   │   ├── elasticsearch.rs            # ES client
│   │   ├── openobserve.rs              # OO client
│   │   └── models.rs                   # Data models
│   ├── Cargo.toml                      # Dependencies
│   ├── .env.example                    # Config template
│   └── .gitignore
│
├── 🎨 frontend/                         # Web UI
│   ├── src/
│   │   ├── routes/
│   │   │   └── +page.svelte            # Main page
│   │   └── lib/
│   │       ├── Settings.svelte         # Settings modal
│   │       ├── QueryInterface.svelte   # Query UI
│   │       └── LogResults.svelte       # Results display
│   ├── package.json
│   ├── vite.config.ts
│   ├── svelte.config.js
│   └── .gitignore
│
├── 📚 Documentation (root level)
│   ├── README.md                       # Main overview
│   ├── AI-BOT-SETUP.md                 # Web app setup
│   ├── QUICKSTART.md                   # 5-min start
│   ├── otel-mcp-server-setup.md        # MCP guide
│   ├── NEUROLINK-INTEGRATION.md        # SDK guide
│   ├── config-examples.md              # Configs
│   ├── query-examples.md               # Query help
│   ├── COMPARISON.md                   # Tool comparison
│   └── PROJECT-STRUCTURE.md            # This file
│
├── 💡 examples/ (coming soon)
│   ├── nodejs-bot/                     # Node.js bot example
│   ├── python-analyzer/                # Python analyzer
│   └── slack-integration/              # Slack bot
│
├── 🧪 tests/ (coming soon)
│   ├── integration/                    # E2E tests
│   └── fixtures/                       # Test data
│
└── 🚀 deployment/
    ├── docker-compose.yml              # Docker setup
    ├── kubernetes/                     # K8s manifests
    └── nginx.conf                      # nginx config
```

## Development Workflow

### For Monorepo Structure

```bash
# 1. Clone once
git clone https://github.com/adarshba/logs-explorer.git
cd logs-explorer

# 2. Setup backend
cd backend
cp .env.example .env
# Edit .env with your API keys
cargo run

# 3. Setup frontend (new terminal)
cd ../frontend
pnpm install
pnpm run dev

# 4. Commit changes to both
cd ..
git add backend/ frontend/
git commit -m "feat: add new feature"
git push
```

### Branching Strategy

```
main                    # Production-ready code
├── develop             # Development branch
├── feature/query-ui    # Feature branches
├── feature/rust-api
├── fix/auth-bug        # Bug fixes
└── docs/setup-guide    # Documentation
```

## CI/CD Pipeline

### GitHub Actions Example

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  backend:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Build backend
        run: |
          cd backend
          cargo build --release
          cargo test

  frontend:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: pnpm/action-setup@v2
      - uses: actions/setup-node@v3
        with:
          node-version: 18
      - name: Build frontend
        run: |
          cd frontend
          pnpm install
          pnpm run build
```

## Deployment Strategies

### 1. Single Docker Compose (Recommended)

```yaml
# docker-compose.yml
version: '3.8'

services:
  backend:
    build: ./backend
    ports:
      - "3001:3001"
    environment:
      - OPENAI_API_KEY=${OPENAI_API_KEY}

  frontend:
    build: ./frontend
    ports:
      - "80:80"
    depends_on:
      - backend
```

### 2. Kubernetes

```yaml
# deployment/kubernetes/deployment.yml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: zeteo
spec:
  replicas: 2
  template:
    spec:
      containers:
      - name: backend
        image: zeteo-backend:latest
      - name: frontend
        image: zeteo-frontend:latest
```

### 3. Separate Deployments

If you need independent deployments:

```bash
# Deploy backend only
cd backend
cargo build --release
./target/release/zeteo-backend

# Deploy frontend only
cd frontend
pnpm run build
# Copy build/ to web server
```

## Migration Path: Monorepo → Separate Repos

If you later decide to split:

### Step 1: Extract Backend

```bash
# Create new backend repo
git clone logs-explorer logs-explorer-backend
cd zeteo-backend
git filter-branch --subdirectory-filter backend -- --all
# Now backend/ becomes root
```

### Step 2: Extract Frontend

```bash
# Create new frontend repo
git clone logs-explorer logs-explorer-frontend
cd zeteo-frontend
git filter-branch --subdirectory-filter frontend -- --all
# Now frontend/ becomes root
```

### Step 3: Update Documentation

Each repo gets its own README:
- Backend: API documentation, deployment
- Frontend: UI guide, configuration
- Docs: Separate docs repo or website

## Recommended Tools for Monorepo

### Build Tools
- **Rust**: Cargo (built-in)
- **Frontend**: pnpm (better for monorepos than npm)
- **Scripts**: Make or Just for common tasks

### Monorepo Tools (If Scaling)
- **Turborepo**: Fast builds, caching
- **Nx**: Advanced monorepo management
- **Bazel**: For very large repos

### Current Setup is Fine!

For this project size, you **don't need** special monorepo tools. The simple structure we have is perfect.

## Best Practices

### ✅ DO

- Keep root-level docs (README, guides)
- Use workspace for pnpm if adding more Node.js packages
- Share common configs (prettier, eslint) at root
- Use path aliases in imports
- Have clear separation between backend and frontend
- Document the structure (like this file!)

### ❌ DON'T

- Mix backend and frontend code in same directories
- Create circular dependencies
- Duplicate configuration files
- Ignore .gitignore files
- Forget to update both when making breaking changes

## Summary

### Current Choice: ✅ Monorepo

**Perfect for this project because:**
- Small to medium size
- Tightly coupled frontend/backend
- Single team/developer
- Easier to onboard contributors
- Better for documentation
- Simpler CI/CD

**Stay with monorepo unless:**
- Project grows to >200k LOC
- Multiple teams with different schedules
- Need to open source only parts
- Independent deployment is critical

---

**Decision: Keep the monorepo structure!** 🎉

It's the right choice for this project and will serve you well as it grows.
