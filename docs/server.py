#!/usr/bin/env python3
"""
Simple HTTP Server for zkEVM Benchmark Website

Serves the static website files and analytics JSON data.
"""

import http.server
import socketserver
import webbrowser
import os
from pathlib import Path

class CustomHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    """Custom handler to serve files with proper CORS headers."""
    
    def end_headers(self):
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        super().end_headers()

def main():
    """Start the HTTP server for the website."""
    
    # Get the directory containing this script
    website_dir = Path(__file__).parent
    
    # Change to the parent directory (python-analytics) to serve both website/ and analytics_output/
    parent_dir = website_dir.parent
    os.chdir(parent_dir)
    
    # Check if analytics files exist
    analytics_dir = parent_dir / "analytics_output"
    if not analytics_dir.exists():
        print("❌ Analytics directory not found!")
        print(f"   Please run 'python3 generate_analytics.py' first to create analytics files.")
        print(f"   Looking for: {analytics_dir}")
        return
    
    print("🚀 Starting zkEVM Benchmark Website Server")
    print("=" * 50)
    print(f"📁 Serving from: {parent_dir}")
    print(f"📊 Analytics from: {analytics_dir}")
    print(f"🌐 Website files: {website_dir}")
    
    PORT = 8080
    
    try:
        with socketserver.TCPServer(("", PORT), CustomHTTPRequestHandler) as httpd:
            print(f"🌐 Server running at: http://localhost:{PORT}")
            print(f"📈 Dashboard URL:    http://localhost:{PORT}/website/index.html")
            print()
            print("Press Ctrl+C to stop the server")
            print("-" * 50)
            
            # Try to open the browser automatically
            try:
                webbrowser.open(f"http://localhost:{PORT}/website/index.html")
                print("🌍 Opening browser automatically...")
            except:
                print("💡 Open your browser and navigate to the URL above")
            
            print()
            httpd.serve_forever()
            
    except KeyboardInterrupt:
        print("\n👋 Server stopped by user")
    except OSError as e:
        if e.errno == 48:  # Address already in use
            print(f"❌ Port {PORT} is already in use!")
            print("   Try stopping other servers or use a different port")
        else:
            print(f"❌ Server error: {e}")

if __name__ == "__main__":
    main() 