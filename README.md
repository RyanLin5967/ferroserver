# HTTP Server
An HTTP/1.1 server built from scratch in Rust (no frameworks). I built it to better understand how HTTP requests work and to learn Rust. 

## Features
Request parsing, static file serving, keep-alive connection, gzip compression, reverse proxy, conccurrent connections using tokio (async runtime for Rust).

## What each file does
main.rs: creates router, registers routes, starts server
server.rs: Tcp listener, accept loop, spawns async threads for every connection
connection.rs: reads, parses request, has keep-alive, handles timeouts
request.rs: has parsing fn for request from raw bytes, Method enum, HttpRequest struct
response.rs: HttpResponse struct, serializes to bytes, functions for diff status codes
router.rs: matches route by method and path, dispatches to handler, has associated fns for Router
handler.rs: hardcoded (for now) endpoint handlers
static_files.rs: serves static files, matches MIME content type, error checking
compression.rs: has gzip compression with flate2
proxy.rs: reverse proxy, forwards requests to other server and returns response
error.rs ServerError enum, implements display 
## How to run from .exe
1. Download `httpfromscratch.exe` from the latest release
2. Create a `public/` folder in the same directory with an `index.html` file inside it
3. Open a terminal in that directory and run:
```
httpfromscratch.exe
```
4. Open `http://localhost:8000/index.html` and you should see what a page with whatever is inside your index.html file

5. Open a new terminal window and you can use these commands to test (curl.exe if you're on powershell):

```
curl http://localhost:8000/index.html
curl http://localhost:8000/api/health
curl -X POST -d "hello" http://localhost:8000/api/echo
curl -H "Accept-Encoding: gzip" --compressed http://localhost:8000/index.html
```

6. You can also test the reverse proxy by starting another backend server on a different terminal window:
```
python3 -m http.server 3001
```

then: 

```
curl http://localhost:8000/proxy/
```

## How to run locally (must have git installed)
```
git clone https://github.com/RyanLin5967/HTTP-server.git
cd httpfromscratch
cargo run
```
Then go to step 4 above


## Ai use
Used ai for learning rust concepts/syntax, some debugging, and understanding how http servers work. 
