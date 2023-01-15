import argparse
import socket
import sys
from _thread import *


max_connection = 5
buffer_size = 8192
HOST = ''                 # Symbolic name meaning all available interfaces
PORT = 3000              # Arbitrary non-privileged port

def start():
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.bind((HOST, PORT))
        sock.listen(1)
        print("[*] Server started successfully")
    except Exception:
        print("[*] Unable to Initialize Socket")
        print(Exception)
        sys.exit(2)



    while True:
        try:
            conn, addr = sock.accept() #Accept connection from client browser
            data = conn.recv(buffer_size) #Recieve client data
            print ("RECV DATA >>>>>>>>>>>>>>>>>>>>>>>");
            start_new_thread(conn_string, (conn,data, addr)) #Starting a thread
        except KeyboardInterrupt:
            sock.close()
            print("\n[*] Graceful Shutdown")
            sys.exit(1)

def conn_string(conn, data, addr):
    try:
        print(data)
        proxy_server(conn, addr, data)
    except Exception:
        print ("Con ERRROR >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
        print (Exception)
        pass

def proxy_server( conn, addr, data):
    try:
        print("proxy_server")
        sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        sock.connect("/tmp/mdb.socket")
        sock.send(data)

        while 1:
            print ("Reading UNIX")
            reply = sock.recv(4096)
            print ("UNIX reply ________")
            print (reply)
            if(len(reply)>0):
                conn.send(reply)
            else:
                break
                print ("break")
        sock.close()
        conn.close()
    except socket.error:
        print ("Proxy error >>>>>>>>>>>>>>>>>>>>>>");
        sock.close()
        conn.close()
        print(sock.error)
        sys.exit(1)

if __name__== "__main__":
    start()


