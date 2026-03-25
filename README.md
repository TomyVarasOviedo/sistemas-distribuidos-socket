# TPNº3 - Sistemas Distribuidos

This is a university project for the course **Sistemas Distribuidos** (Distributed Systems). The project implements a WebSocket-based chat server with mathematical expression evaluation capabilities.

## Enunciado

1. Crear una calculadora cliente/Servidor.

- El cliente envía la formula al servidor quien realizara los cálculos y retornara el resultado.
- El servidor deberá soporta las siguientes operaciones:
- - (SUMA),
- - (RESTA),
- - (MULTIPLICACION),
- - (DIVISION),
- - (Potencia),

- El Servidor deberá respetar el orden de aplicación de operaciones.
- Deberá soportar el uso de paréntesis ( ), con el consecuente cambio en el orden de las operaciones.

2. Probar el [programa](https://3523.campusinstituto.com.ar/mod/resource/view.php?id=531089) resolviendo los siguientes cálculos:

- 1+2+3+4+5+6+7+8+9+10
- 1*2+3^4-5*6+7/8+9/10
- 1*(2+3^4)-5*(6+7)/(8+9/10)
- 1*(2+3^4)-5*((6+7)/(8+9/10))

3. Entregar:

- Archivo comprimido conteniendo el Código Fuente y el Executable
- Video (link a youtube) o Documento (.pdf) explicando el código y demostrando el funcionamiento del [programa](https://3523.campusinstituto.com.ar/mod/resource/view.php?id=531089) al resolver los cálculos detallados previamente.

### Observaciones

- Puede realizarse en cualquier lenguaje de programación, pero deberá ser un trabajo propio.
- Se provee código fuente ejemplo en diferentes lenguajes, el alumno deberá hacerlo funcional y adaptarlo a sus necesidades.
- Puede Realizarse en forma grupal, PERO deberán repartirse las actividades (documentar quien hizo cada cosa) y no podrán repetir los roles en trabajos posteriores.

## Project Structure

```
TPNº3/
├── server/                 # Backend server
│   ├── main.py            # FastAPI server con WebSocket
│   ├── test.py            # Unit tests
│   └── requeriments.txt   # Dependencias Python
├── client/                 # Frontend React + Vite
│   ├── src/
│   │   ├── components/
│   │   │   ├── Calculator.jsx        # Componente calculadora
│   │   │   └── Calculator.css
│   │   ├── App.jsx         # Componente principal
│   │   ├── App.css
│   │   ├── main.jsx
│   │   └── index.css
│   ├── index.html
│   ├── package.json        # Dependencias Node.js
│   └── vite.config.js      # Configuración Vite
└── README.md
```

## Instalación y Ejecución

### Servidor (FastAPI)

```bash
# Navegar a la carpeta del servidor
cd server

# Instalar dependencias
pip install -r requeriments.txt

# Ejecutar servidor (en puerto 8000)
uvicorn main:app --reload --host 0.0.0.0 --port 8000
```

### Cliente (React)

```bash
# Abrir otra terminal y navegar a la carpeta del cliente
cd client

# Instalar dependencias Node.js
npm install

# Ejecutar en modo desarrollo (en puerto 5173)
npm run dev
```

Luego abre `http://localhost:5173` en tu navegador.

## Características

✅ Calculadora visual con interfaz moderna
✅ Botones para números, operadores (+, -, \*, /)
✅ Soporte para potencias (^)
✅ Soporte para paréntesis ( )
✅ Comunicación en tiempo real vía WebSocket
✅ Historial del último cálculo
✅ Indicador de conexión con el servidor
✅ Diseño responsive

## Features

### Server (FastAPI + WebSocket)

- **WebSocket Endpoint**: Real-time bidirectional communication at `/ws`
- **Health Check**: Simple HTML interface at `/`
- **Mathematical Expression Evaluator**: Parses and evaluates mathematical expressions

#### Supported Operations

| Operator | Description              |
| -------- | ------------------------ |
| `+`      | Addition                 |
| `-`      | Subtraction              |
| `*`      | Multiplication           |
| `/`      | Division                 |
| `^`      | Exponentiation           |
| `()`     | Parentheses for grouping |

#### Special Handling

- `^` is converted to `**` for Python evaluation
- Negative numbers (e.g., `10 - -5`) are handled correctly
- Division by zero returns `"Error"`

### Client

Currently, a basic HTML/JavaScript client is embedded in the server for testing purposes. The `client/` directory is available for implementing a separate frontend application.

## Installation & Setup

### Prerequisites

- Python 3.11+

### Server Setup

```bash
cd server
python -m venv env
source env/bin/activate  # On Windows: env\Scripts\activate
pip install -r requeriments.txt
```

### Running the Server

```bash
cd server
uvicorn main:app --reload
```

The server will start at `http://localhost:8000`

## Testing

Run the unit tests for the expression evaluator:

```bash
cd server
pytest test.py
```

### Test Cases

The test suite validates:

- Basic arithmetic (`1+2+3+4+5+6+7+8+9+10` → `55`)
- Mixed operations with exponents (`1*2+3^4-5*6+7/8+9/10` → `54.775`)
- Complex nested expressions with parentheses
- Negative numbers (`-5 * -5` → `25`)
- Double negatives (`10 - -5` → `15`)
- Decimal numbers (`0.1 + 0.2` → `0.3`)
- Exponents with division (`0 * (500 / 2.5)^3` → `0`)
- Error handling for division by zero (`0 / 0` → `"Error"`)

## API Usage

Connect to the WebSocket endpoint:

```javascript
const ws = new WebSocket("ws://localhost:8000/ws");

ws.onmessage = function (event) {
  console.log("Response:", event.data);
};

ws.send("2 + 2"); // Sends expression
// Response: 4
```

## Technologies Used

- **FastAPI**: Modern Python web framework
- **Uvicorn**: ASGI server
- **WebSocket**: Real-time communication protocol
- **Pytest**: Testing framework

## License

Academic project for Universidad.
