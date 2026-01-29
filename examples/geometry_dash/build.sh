#!/bin/bash

echo "=== Geometry Dash Build Script ==="
echo "Using custom target directory to avoid /tmp space issues..."

# Set custom target directory
export CARGO_TARGET_DIR="/home/idealz/.cargo/target/geometry_dash"
export TMPDIR="/home/idealz/tmp"

# Create directories
mkdir -p "$CARGO_TARGET_DIR"
mkdir -p "$TMPDIR"

echo "Target directory: $CARGO_TARGET_DIR"
echo "Temp directory: $TMPDIR"

# Clean any previous builds
echo "Cleaning previous builds..."
cargo clean 2>/dev/null || true

# Build with minimal output
echo "Building Geometry Dash..."
cargo build --release --quiet

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    
    echo "Running tests..."
    cargo test --quiet
    
    if [ $? -eq 0 ]; then
        echo "✅ All tests passed!"
        
        # Show build output
        echo ""
        echo "=== Build Output ==="
        echo "Binary size: $(stat -c%s "$CARGO_TARGET_DIR/release/libgeometry_dash.so" 2>/dev/null || echo "N/A") bytes"
        
        # Create a simple WASM build script for later
        cat > build_wasm.sh << 'WASM_EOF'
#!/bin/bash
# Script to build WASM when dependencies are available
echo "To build for WASM when you have network access:"
echo ""
echo "1. Update Cargo.toml:"
echo '   [dependencies]'
echo '   soroban-sdk = "23.0.2"'
echo '   cougr-core = { git = "https://github.com/salazarsebas/Cougr.git", tag = "v0.0.1" }'
echo ""
echo "2. Add Soroban attributes to src/lib.rs:"
echo "   use soroban_sdk::{contract, contractimpl, contracttype};"
echo "   #[contract] pub struct GeometryDash;"
echo "   #[contractimpl] impl GeometryDash { ... }"
echo ""
echo "3. Build: cargo build --target wasm32-unknown-unknown --release"
echo ""
echo "4. Deploy: soroban contract deploy --wasm target/wasm32-unknown-unknown/release/geometry_dash.wasm"
WASM_EOF
        chmod +x build_wasm.sh
        
        echo ""
        echo "=== Project Summary ==="
        echo "📁 geometry_dash/"
        echo "├── 📄 Cargo.toml          - Package config"
        echo "├── 📁 src/"
        echo "│   └── 📄 lib.rs          - Complete game implementation"
        echo "├── 📄 build.sh            - This build script"
        echo "└── 📄 build_wasm.sh       - WASM build instructions"
        echo ""
        echo "🎮 Game Features:"
        echo "  • 7 different icon modes (Cube, Ship, Ball, UFO, Wave, Robot, Spider)"
        echo "  • Physics with gravity and jumping"
        echo "  • Collision detection with 6 obstacle types"
        echo "  • Score tracking and lives system"
        echo "  • Fixed-point arithmetic for precision"
        echo ""
        echo "✅ Ready for Soroban integration when network is available!"
        
    else
        echo "❌ Tests failed!"
        exit 1
    fi
else
    echo "❌ Build failed!"
    
    # Check for common issues
    echo ""
    echo "=== Troubleshooting ==="
    echo "1. Check Rust version: rustc --version"
    echo "2. Check disk space in home directory: df -h ~"
    echo "3. Check if /tmp has space: df -h /tmp"
    echo "4. Try clearing /tmp: sudo rm -rf /tmp/*"
    echo "5. Check file permissions"
    exit 1
fi