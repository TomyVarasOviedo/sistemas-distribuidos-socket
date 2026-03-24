from fastapi import FastAPI, WebSocket
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import HTMLResponse
from starlette.websockets import WebSocketDisconnect

app = FastAPI()
app.add_middleware(
    CORSMiddleware,
    allow_origins=["http://localhost:8000"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)
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
    while True:
        try:
            data = await websocket.receive_text()
            response = evalute_text(data)
            await websocket.send_text(str(response))
        except WebSocketDisconnect as e:
            print(f"Error: Desconexion inesperada: {e}")
            pass


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(host="0.0.0.0", port=8000, app=app)
