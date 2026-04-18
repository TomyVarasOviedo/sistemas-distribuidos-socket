# TP N°5 - Sistemas Distribuidos

### Universidad de Belgrano

---

## Integrantes y Roles

| Integrante | Rol |
|---|---|
| Diego Escorche | Backend (servidor FastAPI + WebSocket) |
| Tomás Varas | Frontend (cliente React + Vite & Rust + ratatui) |
| Joaquin Gamboa | Documentación |
| Audrey Barrientos | Módulo de pruebas y testing |

---

## Descripción del Proyecto

Este proyecto implementa una **calculadora cliente/servidor** utilizando el protocolo **WebSocket** para la comunicación en tiempo real. El cliente envía una expresión matemática al servidor, el servidor la evalúa respetando el orden de operaciones y devuelve el resultado.

La arquitectura es completamente distribuida: la lógica de cálculo reside exclusivamente en el servidor, mientras que el cliente solo se encarga de la interfaz y de enviar/recibir mensajes.

---

## Arquitectura del Sistema

```
┌─────────────────────────────────────────────────────────────┐
│                     CLIENTES                                │
├─────────────────────────────────────────────────────────────┤
│  React + Vite (GUI)           │  Rust + Ratatui (TUI)       │
│        :5173                  │        Terminal             │
│                                                             │
│   [Interfaz gráfica]          │  [Interfaz terminal]        │
│   Usuario ingresa expresión   │  Usuario escribe expr       │
│   Presiona "=" → WebSocket    │  Enter → WebSocket          │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       │  WebSocket  ws://localhost:8000/ws
                       │  (protocolo full-duplex sobre TCP)
                       │
┌──────────────────────▼───────────────────────────────────────┐
│                        SERVIDOR                              │
|               FastAPI + Uvicorn  :8000                       |
|                                                              |
|   Recibe la expresión como texto                             |
|   ↓                                                          |
|   evalute_text(): normaliza y evalúa la expresión            |
|     · convierte "^" a "**"                                   |
|     · maneja doble negativo "- -" → "+"                      |
|     · evalúa con eval() de Python                            |
|     · captura ZeroDivisionError → devuelve "Error"           |
|   ↓                                                          |
|   Devuelve el resultado como string al cliente               |
└──────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│                        CLIENTE                              │
│                  React + Vite  :5173                        │
│                                                             │
│   [Interfaz gráfica de calculadora]                         │
│   Usuario ingresa expresión → se muestra en pantalla        │
│   Al presionar "=" → se envía al servidor por WebSocket     │
│   El resultado llega y se muestra en el display             │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       │  WebSocket  ws://localhost:8000/ws
                       │  (protocolo full-duplex sobre TCP)
                       │
┌──────────────────────▼──────────────────────────────────────┐
│                        SERVIDOR                             │
│               FastAPI + Uvicorn  :8000                      │
│                                                             │
│   Recibe la expresión como texto                            │
│   ↓                                                         │
│   evalute_text(): normaliza y evalúa la expresión           │
│     · convierte "^" a "**"                                  │
│     · maneja doble negativo "- -" → "+"                     │
│     · evalúa con eval() de Python                           │
│     · captura ZeroDivisionError → devuelve "Error"          │
│   ↓                                                         │
│   Devuelve el resultado como string al cliente              │
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
   │  ws.send("75.69662921438315")      │
   │ <───────────────────────────────── │
   │                                    │
   │  display muestra: 75.69...         │

```

---

## Manejo de Múltiples Clientes

El servidor implementa soporte para múltiples clientes concurrentes mediante dos mecanismos clave:

### 1. Corrutinas Asíncronas

FastAPI ejecuta cada conexión WebSocket como una **corrutina independiente** (`async def`). Cuando un cliente se conecta, se crea una nueva instancia de `websocket_endpoint` que corre de forma paralela a las demás conexiones. Esto lo permite el modelo async de Python + Uvicorn: múltiples clientes no bloquean el servidor.

### 2. Loop por Cliente

Cada conexión mantiene su propio `while True:` en `server/main.py:86-91`:

```python
while True:
    data = await websocket.receive_text()    # espera mensaje de ESTE cliente
    response = evalute_text(data)              # procesa expresión
    await websocket.send_text(str(response))   # responde solo a ese cliente
```

### 3. Gestión de Conexiones

- **Identificador único**: Se genera un `client_id` con `uuid.uuid4()` para cada cliente
- **Lista global**: Las conexiones se almacenan en `active_conection: list[WebSocket]`
- **Tracking de tiempo**: Se registra `connection_time` y `duration` para logging
- **Desconexión limpia**: Se remueve el WebSocket de la lista cuando se desconecta (líneas 110-111)

### 4. Manejo de Errores

El servidor captura `WebSocketDisconnect` y `RuntimeError` para manejar diferentes tipos de cierre:
- Código 1000: cierre normal
- Código 1001: cliente abandonó
- Otros: desconexión inesperada

**Resumen**: El servidor permite múltiples clientes porque cada uno corre en su propia corrutina async, sin bloquear a los demás. La lista `active_conection` existe para tracking, aunque actualmente no se usa para broadcast.

---

## Estructura del Proyecto

```

TPNº3/
├── server/
│   ├── main.py             # Servidor FastAPI con endpoint WebSocket
│   ├── test.py             # Tests unitarios con pytest
│   └── requeriments.txt    # Dependencias Python
├── client/                 # Cliente React + Vite (interfaz gráfica)
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
├── client_rust/            # Cliente Rust TUI (interfaz terminal)
│   ├── src/
│   │   ├── main.rs         # Punto de entrada y loop principal
│   │   └── ui.rs           # Componentes de UI y lógica de la app
│   ├── Cargo.toml          # Dependencias Rust
│   └── target/             # Binario compilado
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
| Rust | 1.80+ | Lenguaje del cliente TUI |
| tungstenite | 0.23 | Cliente WebSocket en Rust |
| ratatui | 0.30 | Biblioteca TUI para interfaz terminal |
| crossterm | 0.29 | Manejo de terminal cruzada |

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
- Rust toolchain (para el cliente TUI)

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

### Cliente React

```bash
# En otra terminal, navegar a la carpeta del cliente
cd client

# Instalar dependencias
npm install

# Ejecutar en modo desarrollo en puerto 5173
npm run dev
```

Abrir `http://localhost:5173` en el navegador.

### Cliente Rust (TUI)

```bash
# Navegar a la carpeta del cliente Rust
cd client_rust

# Compilar (modo debug)
cargo build

# O compilar en modo release para mejor rendimiento
cargo build --release

# Ejecutar
cargo run --release
```

El cliente se conecta a `ws://localhost:8000/ws`. Requiere que el servidor esté corriendo.

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

### Cliente Rust — `client_rust/src/`

El cliente Rust es una **interfaz de terminal (TUI)** que se conecta al servidor WebSocket para realizar cálculos. Utiliza la biblioteca `ratatui` para renderizar la interfaz y `tungstenite` para la conexión WebSocket.

#### `main.rs`

Punto de entrada. Establece la conexión WebSocket con el servidor en `ws://localhost:8000/ws` y configura el entorno terminal (modo raw, pantalla alternativa). El loop principal:

- Renderiza la interfaz en cada tick
- Procesa eventos de teclado (Enter para calcular, Esc para limpiar, `?` para ayuda, Ctrl+C para salir)
- Navegación con flechas y edición con Backspace/Delete

#### `ui.rs`

Contiene la lógica de la aplicación y las funciones de renderizado:

- **`App`**: estructura principal que mantiene el estado (input, cursor, historial, conexión WebSocket)
- **`evaluate()`**: envía la expresión al servidor y parsea la respuesta
- **`render_body()`**: renderiza el panel izquierdo con el logo ASCII de la calculadora
- **`render_history()`**: muestra el historial de operaciones con resultados formateados
- **`render_prompt()`**: área de entrada con cursor animado y atajos de teclado
- **`render_help()`**: popup de ayuda con operadores y ejemplos

##### Atajos de teclado

| Tecla | Acción |
|---|---|
| `Enter` | Enviar expresión al servidor |
| `Esc` | Limpiar entrada / Cerrar ayuda |
| `?` | Mostrar ayuda |
| `←` / `→` | Mover cursor |
| `Backspace` | Borrar carácter antes del cursor |
| `Delete` | Borrar carácter después del cursor |
| `Ctrl+C` | Salir |

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
