from http.server import BaseHTTPRequestHandler, HTTPServer
import time

class MyHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200)
        self.send_header('Content-type', 'text/plain')
        self.end_headers()
        with open('test_server.py', 'r') as f:
            for line in f:
                for char in line:
                    self.wfile.write(char.encode())
                    time.sleep(0.025)

if __name__ == '__main__':
    server = HTTPServer(('localhost', 4277), MyHandler)
    print('Starting server...')
    server.serve_forever()
