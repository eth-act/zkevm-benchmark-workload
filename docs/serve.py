#!/usr/bin/env python3
"""
Simple HTTP server for serving the static zkEVM benchmark website.
This server is designed to serve the website directory for GitHub Pages testing.
"""

import http.server
import socketserver
import os
import sys

def main():
    PORT = 8080
    
    # Ensure we're serving from the website directory
    web_dir = os.path.dirname(os.path.abspath(__file__))
    os.chdir(web_dir)
    
    print(f"Serving static website from: {web_dir}")
    print(f"Available at: http://localhost:{PORT}")
    print("Press Ctrl+C to stop the server\n")
    
    class CORSHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
        def end_headers(self):
            self.send_header('Access-Control-Allow-Origin', '*')
            self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
            self.send_header('Access-Control-Allow-Headers', 'Content-Type')
            super().end_headers()
        
        def log_message(self, format, *args):
            # Log requests
            print(f"[{self.log_date_time_string()}] {format % args}")
    
    try:
        with socketserver.TCPServer(("", PORT), CORSHTTPRequestHandler) as httpd:
            print(f"Website is now running at http://localhost:{PORT}")
            print(f"Main page: http://localhost:{PORT}/index.html")
            print(f"RISC0 details: http://localhost:{PORT}/zkvm.html?zkvm=risc0")
            print(f"SP1 details: http://localhost:{PORT}/zkvm.html?zkvm=sp1")
            httpd.serve_forever()
    except KeyboardInterrupt:
        print("\nServer stopped.")
    except OSError as e:
        if e.errno == 48:  # Address already in use
            print(f"Error: Port {PORT} is already in use. Please stop any existing server or choose a different port.")
        else:
            print(f"Error starting server: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main() 