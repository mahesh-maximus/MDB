import socket
import sys
from _thread import *

max_connection = 5
buffer_size = 8192
HOST = ''              
PORT = 3000              

def start():
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        sock.bind((HOST, PORT))
        sock.listen(1)

        printc('1','44', '1', "HTTP Proxy-> TCP Socket started successfully.")
    except Exception as e:
        printc('1','37', '41', "HTTP Proxy-> Unable to Initialize Socket.")
        printc('1','37', '41', 'HTTP Proxy-> start.Exception:{}.'.format(e))
        sys.exit(2)

    while True:
        try:
            printc('5','30', '43', "HTTP Proxy-> Waiting to accept client requests.")
            conn, addr = sock.accept()
            printc('6','30', '42', 'HTTP Proxy-> Client request accepted from: {}.'.format(addr))
            data = conn.recv(buffer_size) 
            printc('1','44', '1', 'HTTP Proxy-> Received request {}.'.format(data))
            conn_string(conn, data)
        except KeyboardInterrupt:
            sock.close()
            printc('1','37', '41', "HTTP Proxy-> Graceful Shutdown.")
            sys.exit(1) 
        except Exception as e:
            printc('1','37', '41', 'HTTP Proxy-> start.Exception:{}.'.format(e))
            pass

def conn_string(conn, data):
    try:
        proxy_server(conn, data)
    except Exception as e:
        printc('1','37', '41', 'HTTP Proxy-> conn_string.Exception:{}.'.format(e))
        pass

def proxy_server( conn, data):
    try:
        printc('1','44', '1', "HTTP Proxy-> Connecting to Unix socket")
        sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        sock.connect("/tmp/mdb.socket")

        printc('1','44', '1', "HTTP Proxy-> Connected to Unix socket")
        sock.send(data)
        printc('1','44', '1', "HTTP Proxy-> Sent data to Unix socket")

        while 1:
            printc('1','44', '1', "HTTP Proxy-> Waiting to read data from Unix socket.")
            sock.settimeout(1)
            reply = sock.recv(buffer_size)
            printc('1','44', '1', 'HTTP Proxy-> Rreceived data from Unix socket.')
            if(len(reply) > 0):
                conn.send(reply)
                printc('6','30', '46', "HTTP Proxy-> Sent data to client from Unix socket to client")
            else:
                printc('1','44', '1', "HTTP Proxy-> Completed sending data.")
                break
        sock.close()
        conn.close()
    except socket.timeout as e:
        printc('6','37', '41', 'HTTP Proxy-> proxy_server.timeout:{}.'.format(e))
        conn.close()
        pass
    except socket.error as e:
        printc('1','37', '41', 'HTTP Proxy-> proxy_server.error:{}.'.format(e))
        sock.close()
        conn.close()
        sys.exit(1)

def printc(style, fg, bg, msg):
    print('\x1b[{};{};{}m'.format(style, fg, bg) + msg + '\x1b[0m') 

if __name__== "__main__":
    start()


