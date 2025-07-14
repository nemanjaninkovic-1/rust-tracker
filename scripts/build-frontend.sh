#!/bin/bash
# Frontend development build script

set -e

echo "🎨 Building Tailwind CSS..."
cd frontend
npm run build-css-prod

echo "🦀 Building Rust WASM..."
trunk build --release

echo "✅ Frontend build complete!"
echo "📁 Files ready in frontend/dist/"
