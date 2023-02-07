async function startWebSocketClient() {
    console.log("web-socket, startWebSocketClient function")

    initWebSocketClient()
}

function initWebSocketClient() {
    const socket = new WebSocket('ws://40.88.126.144:8000');

    socket.addEventListener('open', (event) => {
        console.log("web-socket, connection opened.")
    });

    socket.addEventListener('message', (event) => {
        console.log('web-socket, message from server ', event.data);

        const eventObj = JSON.parse(event.data);

        switch(eventObj.message_type) {
            case "LiveReload":
              console.log('web-socket, LiveReload')
              location.reload();
              break;
            default:
              // code block
          } 

    });

    setInterval(pingServer, 5000, socket);
}

async function pingServer(socket) {
    console.log("startPingProcess function")
    socket.send('PING');
}
