import uuid
from datetime import datetime

from fastapi import FastAPI, WebSocket
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import HTMLResponse
from starlette.websockets import WebSocketDisconnect

app = FastAPI()
app.add_middleware(
    CORSMiddleware,
    allow_origins=[
        "*"
    ],  # Permite conexiones desde cualquier origen (simplificado para desarrollo)
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)
CYAN = "\033[96m"
RESET = "\033[0m"
BLUE = "\033[34m"
MAGENTA = "\033[35m"
RED = "\033[31m"
active_conection: list[WebSocket] = []
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
    await websocket.accept()
    client_id = str(uuid.uuid4())
    conecction_time = datetime.now()
    active_conection.append(websocket)
    print(f"{MAGENTA}LOG:{RESET}[{CYAN}{client_id}{RESET}] ")
    while True:
        try:
            data = await websocket.receive_text()
            response = evalute_text(data)
            await websocket.send_text(str(response))
        except (WebSocketDisconnect, RuntimeError) as e:
            diconnect_time = datetime.now()
            duration = diconnect_time - conecction_time
            print(f"[{BLUE}{client_id}{RESET}] -> {CYAN}{duration.seconds} sg {RESET}")
            if isinstance(e, WebSocketDisconnect):
                if e.code == 1000:
                    pass
                elif e.code == 1001:  # Cliente abandonó
                    print(
                        f"{RED}ERROR:{RESET}[{CYAN}{client_id}{RESET}] Cliente abandonado"
                    )
                else:  # Cierre anormal
                    print(
                        f"{RED}ERROR:{RESET}[{CYAN}{client_id}{RESET}] Desconexión inesperada | Código: {e.code}"
                    )
            else:
                print(
                    f"{RED}ERROR:{RESET}[{CYAN}{client_id}{RESET}] -> Error de conexion"
                )
            if websocket in active_conection:
                active_conection.remove(websocket)
            break


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(host="0.0.0.0", port=8000, app=app)
