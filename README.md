# Zeteo Web

Marketing and documentation website for [Zeteo CLI](https://github.com/adarshba/zeteo-cli).

## Overview

Zeteo is an AI-powered terminal assistant for developers. This repository contains the source code for the official Zeteo website, built with SvelteKit.

## Development

### Requirements

- Node.js 18 or later
- npm

### Setup

Clone the repository and install dependencies:

```bash
git clone https://github.com/adarshba/zeteo-web.git
cd zeteo-web
npm install
```

### Running Locally

Start the development server:

```bash
npm run dev
```

Open [http://localhost:5173](http://localhost:5173) in your browser.

### Building

Generate a production build:

```bash
npm run build
```

Preview the production build:

```bash
npm run preview
```

## Project Structure

```
src/
├── app.css              Global styles
├── app.html             HTML template
├── lib/
│   └── components/      Reusable Svelte components
└── routes/
    └── +page.svelte     Home page
```

## Deployment

This is a static site built with `@sveltejs/adapter-static`. The build output is generated in the `build/` directory and can be deployed to any static hosting service.

## Related

- [Zeteo CLI](https://github.com/adarshba/zeteo-cli) — The command-line application

## License

MIT
