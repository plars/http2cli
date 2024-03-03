# HTTP Command Executor

This is a simple HTTP server that executes shell commands. It's written in Rust and uses the Warp web framework.

**Note:**
You should probably not be running this at all.

Seriously... just don't, it's a toy for my own amusement

Still here? ok... if you really want to know, this is just a small tool to allow execution of arbitrary shell commands via http.  Clearly, this is a VERY bad idea to do. If you do run this (please don't), be aware that anyone who knows about it can execute arbitrary commands on your system via http while it's running.

## Running the Server

First, build the Docker image:

```bash
docker build -t http2cli .
```

Then, run the server:
```bash
docker run --rm -p 8000:8000 http2cli
```

The server listens on port 8000 by default. You can change this by setting the HTTP2CLI_PORT environment variable.

## Using the Server
To execute a command, make a GET request to /command/<command>, where <command> is the URL-encoded command you want to execute. For example, to list the files in the current directory, you can do:

```bash
curl 'http://localhost:8000/command/ls%20-l'
```

The server will execute the command and return the output.

Stopping the Server
To stop the server, press ^C in the terminal where the server is running, or stop the container if it's running in docker.
