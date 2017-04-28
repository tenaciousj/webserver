# webserver

The purpose of webserver is to respond to the single command of HTTP 0.9, the GET method, which has the following shape:

    GET /path/to/file HTTP

That is, it is the literal world GET, followed by a blank space, followed by a Unix-style absolute path to a file, followed by another blank space and the literal token HTTP. The following line is a blank line. For forward compatibility, you should also accept newer HTTP versions, which will end their request with a token that includes the version, e.g., HTTP/1.1. And you should skip over any header lines following the request but preceding the blank line.

In return to a valid GET request, the web server spawns a thread that retrieves the request, records it to a log file, and generates a response. For this assignment, the following four response statuses are appropriate:

* 200 OK, which starts a reply that serves the specified file;

* 400 Bad Request, which indicates that the command is not a properly formatted GET command;

* 403 Forbidden, which rejects a command because it specifies a file that is off-limits; and

* 404 Not Found, which informs the client that the specified file does not exist.

Each response is preceded by HTTP/1.0 and blank space.
The complete header of a 200 OK response is formatted as follows:

    HTTP/1.0 200 OK
    {name-of-web-server}
    Content-type: text/{plain-or-html}
    Content-length: {number-of-bytes-sent}
    
including a blank line afterward. To keep things simple, the {plain-or-html} property is either html for files whose suffix is .html or plain for all others. 
The remainder of a 200 OK message is the content of the specified file.

A path specification {path-to-file} must start with / and is interpreted after concatenating it with the server’s root path:
If the resulting path points to a file, the file is served with a 200 OK response unless its permissions do not allow so.

If the resulting path points to a directory, it is interpreted as pointing to one of these files: index.html, index.shtml, and index.txt. The first file found is served assuming it is accessible. Otherwise the path triggers a 404-message.

Otherwise the server responds with an error message.
