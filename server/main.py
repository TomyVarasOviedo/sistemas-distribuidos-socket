from fastapi import FastAPI, WebSocket
from fastapi.responses import HTMLResponse

app = FastAPI()
html = """
<!DOCTYPE html>
<html>
    <head>
        <title>Chat</title>
    </head>
    <body>
        <h1>WebSocket Chat</h1>
        <form action="" onsubmit="sendMessage(event)">
            <input type="text" id="messageText" autocomplete="off"/>
            <button>Send</button>
        </form>
        <ul id='messages'>
        </ul>
        <script>
            var ws = new WebSocket("ws://localhost:8000/ws");
            ws.onmessage = function(event) {
                var messages = document.getElementById('messages')
                var message = document.createElement('li')
                var content = document.createTextNode(event.data)
                message.appendChild(content)
                messages.appendChild(message)
            };
            function sendMessage(event) {
                var input = document.getElementById("messageText")
                ws.send(input.value)
                input.value = ''
                event.preventDefault()
            }
        </script>
    </body>
</html>
"""


def evalute_text(expression: str) -> str:
    """
    Metodo para capturar adaptar y evaluar la expresion enviada por el socket 
    @param: expresion Type String
    @return resultado Type String
    """
    match expression:
        case expression if "^" in expression:
            expression = expression.replace("^", "**")
        # Se ajusta para cuando tiene el caracter  especial "^"
        case expression if "- -" in expression:
            expression = expression.replace("- -", "+")

    try:
        return eval(expression)
    except ZeroDivisionError:
        return "Error"


@app.get("/")
async def check_healt():
    return HTMLResponse(html)


@app.websocket("/ws")
async def websocket_endpoint(websocket: WebSocket):
    await websocket.accept() # Se crea el socket de forma asincrona
    while True:
        data = await websocket.receive_text() # El socket se coloca en modo escucha, para recibir mensajes
        response = evalute_text(data) # Se le envia la informacion a la funcion para evaluar la expresion
        await websocket.send_text(str(response)) # El socket responde al cliente con el resultado
