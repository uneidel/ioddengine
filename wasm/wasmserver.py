#!/usr/bin/env python3

import http.server
import socketserver

PORT = 8000



from http.server import HTTPServer, SimpleHTTPRequestHandler

Handler = http.server.SimpleHTTPRequestHandler
Handler.extensions_map.update({
    '.wasm': 'application/wasm',
})

class CORSRequestHandler(SimpleHTTPRequestHandler):
    
    def end_headers(self):
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', '*')
        self.send_header('Access-Control-Allow-Headers', '*')
        self.send_header('Cache-Control', 'no-store, no-cache, must-revalidate')
        return super(CORSRequestHandler, self).end_headers()

    def do_OPTIONS(self):
        self.send_response(200)
        self.end_headers()


httpd = HTTPServer(('localhost', 8000), CORSRequestHandler)
httpd.serve_forever()