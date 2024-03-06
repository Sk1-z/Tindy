from http.server import SimpleHTTPRequestHandler, HTTPServer
from http import HTTPStatus
import markdown
import webbrowser
from random import randint


class RequestHandler(SimpleHTTPRequestHandler):
    file_name: str = ""

    def wrap(self, html):
        head = f"""
        <div style="margin: 5vh 20vh; display: flex; justify-content: center; flex-direction: column;">
            <div style="display: flex; justify-content: center; border: 2px solid #000;">
             <h1>{self.file_name}</h1>
            </div>
            <div style="padding: 50px; border: 2px solid #000;">
        """
        tail = "</div></div>"
        return (head + html + tail).encode("utf-8")

    def log_message(self, format, *args):
        pass

    def do_GET(self):
        with open(RequestHandler.file_name, "r") as file:
            md_content = file.read()
            html_content = markdown.markdown(md_content)

        self.send_response(HTTPStatus.OK)
        self.send_header("Content-type", "text/html")
        self.end_headers()
        self.wfile.write(self.wrap(html_content))


def start_server(file_name):
    host = "localhost"
    port = randint(1024, 49151)
    server_address = (host, port)

    webbrowser.open(f"http://localhost:{port}")

    RequestHandler.file_name = file_name
    httpd = HTTPServer(server_address, RequestHandler)
    try:
        httpd.serve_forever()
    except:
        pass
