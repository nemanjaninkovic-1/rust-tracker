#!/bin/bash
# Quick script to serve the frontend locally for development

set -e

echo "Starting RustTracker Frontend Development Server"
echo "================================================"

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "ERROR: Please run this script from the frontend directory"
    exit 1
fi

# Install npm dependencies if needed
if [ ! -d "node_modules" ]; then
    echo "Installing npm dependencies..."
    npm install
fi

# Build CSS
echo "Building Tailwind CSS..."
npm run build-css

# Start trunk serve
echo "Starting Trunk development server..."
echo "Frontend will be available at: http://localhost:8080"
echo "Hot reload enabled - changes will be reflected automatically"
echo ""
echo "Press Ctrl+C to stop the server"
echo ""

trunk serve
