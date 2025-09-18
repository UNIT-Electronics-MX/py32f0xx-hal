#!/bin/bash
#
# Build script for PY32F0xx HAL documentation
# 
# This script builds the mdBook documentation and optionally serves it locally

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if mdbook is installed
check_mdbook() {
    if ! command -v mdbook &> /dev/null; then
        print_error "mdbook is not installed"
        print_status "Installing mdbook..."
        cargo install mdbook
        print_success "mdbook installed"
    else
        print_success "mdbook found: $(mdbook --version)"
    fi
}

# Build documentation
build_docs() {
    print_status "Building documentation..."
    
    cd "$(dirname "$0")/../docs"
    
    # Clean previous build
    if [ -d "book" ]; then
        rm -rf book
        print_status "Cleaned previous build"
    fi
    
    # Build the book
    mdbook build
    
    print_success "Documentation built successfully"
    print_status "Output directory: $(pwd)/book"
}

# Serve documentation locally
serve_docs() {
    print_status "Starting local server..."
    print_status "Documentation will be available at: http://localhost:3000"
    print_status "Press Ctrl+C to stop the server"
    
    cd "$(dirname "$0")/../docs"
    mdbook serve --open
}

# Test documentation
test_docs() {
    print_status "Testing documentation..."
    
    cd "$(dirname "$0")/../docs"
    mdbook test
    
    print_success "Documentation tests passed"
}

# Main function
main() {
    print_status "PY32F0xx HAL Documentation Builder"
    print_status "================================="
    
    # Parse command line arguments
    case "${1:-build}" in
        "build")
            check_mdbook
            build_docs
            ;;
        "serve")
            check_mdbook
            build_docs
            serve_docs
            ;;
        "test")
            check_mdbook
            test_docs
            ;;
        "clean")
            print_status "Cleaning documentation build..."
            rm -rf "$(dirname "$0")/../docs/book"
            print_success "Build directory cleaned"
            ;;
        "help"|"--help")
            echo "Usage: $0 [command]"
            echo ""
            echo "Commands:"
            echo "  build    Build the documentation (default)"
            echo "  serve    Build and serve locally with live reload"
            echo "  test     Run documentation tests"
            echo "  clean    Clean build directory"
            echo "  help     Show this help"
            echo ""
            echo "Examples:"
            echo "  $0           # Build documentation"
            echo "  $0 serve     # Build and serve locally"
            echo "  $0 test      # Test documentation"
            ;;
        *)
            print_error "Unknown command: $1"
            print_status "Use '$0 help' for usage information"
            exit 1
            ;;
    esac
}

# Run main function with all arguments
main "$@"
