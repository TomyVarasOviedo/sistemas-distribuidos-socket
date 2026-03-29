# TP N°3 - Sistemas Distribuidos
### Universidad de Belgrano

---

## Integrantes y Roles

| Integrante | Rol |
|---|---|
| Tomas Varas | Backend (servidor FastAPI + WebSocket) |
| Diego Escorche | Frontend (cliente React + Vite) |
| Audrey Barrientos | Documentación |
| Joaquin Gamboa | Módulo de pruebas y testing con pytest |

---

## Descripción del Proyecto

Este proyecto implementa una **calculadora cliente/servidor** utilizando el protocolo **WebSocket** para la comunicación en tiempo real. El cliente envía una expresión matemática al servidor, el servidor la evalúa respetando el orden de operaciones y devuelve el resultado.

La arquitectura es completamente distribuida: la lógica de cálculo reside exclusivamente en el servidor, mientras que el cliente solo se encarga de la interfaz y de enviar/recibir mensajes.

---

## Arquitectura del Sistema

```
┌─────────────────────────────────────────────────────────────┐
│                        CLIENTE                               │
│                  React + Vite  :5173                         │
│                                                              │
│   [Interfaz gráfica de calculadora]                          │
│   Usuario ingresa expresión → se muestra en pantalla         │
│   Al presionar "=" → se envía al servidor por WebSocket      │
│   El resultado llega y se muestra en el display              │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       │  WebSocket  ws://localhost:8000/ws
                       │  (protocolo full-duplex sobre TCP)
                       │
┌──────────────────────▼──────────────────────────────────────┐
│                        SERVIDOR                              │
│               FastAPI + Uvicorn  :8000                       │
│                                                              │
│   Recibe la expresión como texto                             │
│   ↓                                                          │
│   evalute_text(): normaliza y evalúa la expresión            │
│     · convierte "^" a "**"                                   │
│     · maneja doble negativo "- -" → "+"                      │
│     · evalúa con eval() de Python                            │
│     · captura ZeroDivisionError → devuelve "Error"           │
│   ↓                                                          │
│   Devuelve el resultado como string al cliente               │
└─────────────────────────────────────────────────────────────┘
```

### Flujo de una operación

```
Cliente                              Servidor
   │                                    │
   │  Presiona "="                      │
   │  ws.send("1*(2+3^4)-5*(6+7)")      │
   │ ─────────────────────────────────> │
   │                                    │  evalute_text("1*(2+3^4)-5*(6+7)")
   │                                    │  → "1*(2+3**4)-5*(6+7)"
   │                                    │  → eval(...) = 75.69...
   │                                    │
   │  ws.send("75.69662921438315")       │
   │ <───────────────────────────────── │
   │                                    │
   │  display muestra: 75.69...         │
```

---

## Estructura del Proyecto

```
TPNº3/
├── server/
│   ├── main.py             # Servidor FastAPI con endpoint WebSocket
│   ├── test.py             # Tests unitarios con pytest
│   └── requeriments.txt    # Dependencias Python
├── client/
│   ├── src/
│   │   ├── components/
│   │   │   ├── Calculator.jsx   # Componente principal de la calculadora
│   │   │   └── Calculator.css   # Estilos del componente
│   │   ├── App.jsx          # Componente raíz, maneja estado de conexión
│   │   ├── App.css
│   │   ├── main.jsx         # Punto de entrada de React
│   │   └── index.css
│   ├── index.html
│   ├── package.json
│   └── vite.config.js
└── README.md
```

---

## Tecnologías Utilizadas

| Tecnología | Versión | Uso |
|---|---|---|
| Python | 3.12 | Lenguaje del servidor |
| FastAPI | 0.135.1 | Framework web del servidor |
| Uvicorn | 0.41.0 | Servidor ASGI |
| WebSockets | 16.0 | Protocolo de comunicación |
| React | 18.2.0 | Biblioteca UI del cliente |
| Vite | 5.0.0 | Bundler y dev server del cliente |
| Pytest | - | Framework de testing |

---

## Operaciones Soportadas

| Operador | Operación | Ejemplo |
|---|---|---|
| `+` | Suma | `3 + 5` → `8` |
| `-` | Resta | `10 - 4` → `6` |
| `*` | Multiplicación | `6 * 7` → `42` |
| `/` | División | `10 / 4` → `2.5` |
| `^` | Potencia | `2 ^ 8` → `256` |
| `( )` | Paréntesis | `(2+3) * 4` → `20` |

El servidor respeta el orden de precedencia de operadores (PEMDAS) y soporta expresiones anidadas con paréntesis.

---

## Instalación y Ejecución

### Requisitos previos

- Python 3.11 o superior
- Node.js 18 o superior

### Servidor

```bash
# Navegar a la carpeta del servidor
cd server

# Crear entorno virtual
python3 -m venv .venv
source .venv/bin/activate        # En Windows: .venv\Scripts\activate

# Instalar dependencias
pip install -r requeriments.txt

# Ejecutar servidor en puerto 8000
uvicorn main:app --host 0.0.0.0 --port 8000
```

### Cliente

```bash
# En otra terminal, navegar a la carpeta del cliente
cd client

# Instalar dependencias
npm install

# Ejecutar en modo desarrollo en puerto 5173
npm run dev
```

Abrir `http://localhost:5173` en el navegador.

---

## Descripción del Código

### Servidor — `server/main.py`

El servidor se construye con **FastAPI** y expone dos endpoints:

#### `GET /`
Devuelve una página HTML de prueba con un cliente WebSocket básico embebido, útil para verificar que el servidor funciona sin necesidad de correr el frontend.

#### `WebSocket /ws`
Endpoint principal. Mantiene la conexión abierta en un loop continuo:

```python
@app.websocket("/ws")
async def websocket_endpoint(websocket: WebSocket):
    await websocket.accept()
    while True:
        data = await websocket.receive_text()   # espera una expresión
        response = evalute_text(data)           # la evalúa
        await websocket.send_text(str(response)) # devuelve el resultado
```

#### `evalute_text(expression: str) → str`
Función encargada de normalizar y evaluar la expresión matemática recibida:

1. Si contiene `^`, lo reemplaza por `**` (sintaxis de Python para potencia).
2. Si contiene `- -`, lo reemplaza por `+` (doble negativo).
3. Llama a `eval()` sobre la expresión normalizada.
4. Si ocurre `ZeroDivisionError`, retorna el string `"Error"`.

#### CORS
Se configura `CORSMiddleware` con `allow_origins=["*"]` para permitir conexiones desde el cliente React en desarrollo.

---

### Cliente — `client/src/`

#### `App.jsx`
Componente raíz. Al montarse, intenta establecer una conexión WebSocket de prueba para verificar que el servidor esté disponible. Muestra un banner de error si no puede conectarse, o un indicador verde si la conexión fue exitosa. Luego renderiza el componente `<Calculator />`.

#### `components/Calculator.jsx`
Componente principal de la interfaz. Maneja:

- **Estado `display`**: la expresión que se muestra en pantalla mientras el usuario construye la operación.
- **Estado `history`**: guarda el último cálculo realizado.
- **Estado `wsStatus`**: indica el estado de la conexión WebSocket (`conectando`, `conectado`, `cerrado`, `error`).
- **`wsRef`**: referencia a la instancia de WebSocket, creada en `useEffect` al montar el componente.

Cuando el usuario presiona `=`, se llama a `handleEqual()`, que verifica que la conexión esté abierta (`readyState === WebSocket.OPEN`) y envía la expresión al servidor con `ws.send(display)`. La respuesta llega por `ws.onmessage` y actualiza el display con el resultado.

---

## Testing — `server/test.py`

Los tests unitarios validan la función `evalute_text` directamente, sin necesidad de levantar el servidor.

Se utiliza `@pytest.mark.parametrize` para ejecutar múltiples casos de prueba con una sola función:

```python
@pytest.mark.parametrize("entrada, esperado", [...])
def test_evalute_text(entrada, esperado):
    assert evalute_text(entrada) == pytest.approx(esperado)
```

`pytest.approx` se usa para comparar resultados con punto flotante, tolerando errores de precisión mínimos.

### Casos de prueba

| Expresión | Resultado esperado | Qué valida |
|---|---|---|
| `1+2+3+4+5+6+7+8+9+10` | `55` | Suma encadenada |
| `1*2+3^4-5*6+7/8+9/10` | `54.775` | Precedencia de operadores |
| `1*(2+3^4)-5*(6+7)/(8+9/10)` | `75.6966...` | Paréntesis anidados |
| `1*(2+3^4)-5*((6+7)/(8+9/10))` | `75.6966...` | Paréntesis dobles |
| `2-2+2*4*6*(56-5-1)+32` | `2432` | Expresión compleja |
| `10 + 5 * 2 - 8 / 4` | `18` | Orden de operaciones |
| `-5 * -5` | `25` | Números negativos |
| `10 - -5` | `15` | Doble negativo |
| `0.1 + 0.2` | `0.3` | Decimales |
| `0 * (500 / 2.5)^3` | `0` | Potencia con resultado cero |
| `(2^3 + 2^2) / 2` | `6` | Potencias con paréntesis |
| `0 / 0` | `"Error"` | División por cero |

### Ejecutar los tests

```bash
cd server
source .venv/bin/activate
pytest test.py -v
```

---

## Uso de la API WebSocket

Es posible conectarse al servidor desde cualquier cliente WebSocket. Ejemplo en JavaScript:

```javascript
const ws = new WebSocket("ws://localhost:8000/ws");

ws.onopen = () => console.log("Conectado");

ws.onmessage = (event) => console.log("Resultado:", event.data);

ws.send("1*(2+3^4)-5*(6+7)/(8+9/10)");
// Resultado: 75.69662921438315
```

---

## Licencia

Trabajo práctico académico — Universidad de Belgrano.
