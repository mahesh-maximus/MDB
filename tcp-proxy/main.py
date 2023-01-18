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
        sock.bind((HOST, PORT))
        sock.listen(1)
        print("HTTP Proxy-> TCP Socket started successfully.")
    except Exception:
        print("HTTP Proxy-> Unable to Initialize Socket.")
        print(Exception)
        sys.exit(2)



    while True:
        try:
            conn, addr = sock.accept() #Accept connection from client browser
            print ('HTTP Proxy-> Client request accepted from: {}.'.format(addr))
            data = conn.recv(buffer_size) #Recieve client data
            print ('HTTP Proxy-> Received request {}.'.format(data))
            start_new_thread(conn_string, (conn,data)) #Starting a thread
        except KeyboardInterrupt:
            sock.close()
            print("HTTP Proxy-> Graceful Shutdown.")
            sys.exit(1)

def conn_string(conn, data):
    try:
        proxy_server(conn, data)
    except Exception:
        print ('HTTP Proxy-> Exception:{}.'.format(Exception))
        pass

def proxy_server( conn, data):
    try:
        print("HTTP Proxy-> Connecting to Unix socket")
        sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        sock.connect("/tmp/mdb.socket")
        print("HTTP Proxy-> Connected to Unix socket")
        sock.send(data)
        print("HTTP Proxy-> Sent data to Unix socket")

        while 1:
            print ("HTTP Proxy-> Waiting to read data from Unix socket.")
            reply = sock.recv(4096)
            print ('HTTP Proxy-> Rreceived data from Unix socket:{}.'.format(reply))
            if(len(reply) > 0):
                conn.send(reply)
                print ("HTTP Proxy-> Sent data received from Unix socket to client")
            else:
                print ("HTTP Proxy-> Completed sending data.")
                break
        sock.close()
        conn.close()
    except socket.error:
        print ('HTTP Proxy-> Socket error:{}.'.format(socket.error))
        sock.close()
        conn.close()
        print(sock.error)
        sys.exit(1)

if __name__== "__main__":
    start()


