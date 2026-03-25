import React, { useEffect, useState } from 'react'
import Calculator from './components/Calculator'
import './App.css'

function App() {
  const [connected, setConnected] = useState(false)
  const [error, setError] = useState(null)

  useEffect(() => {
    // Intentar conectar al servidor WebSocket
    try {
      const ws = new WebSocket('ws://localhost:8000/ws')
      ws.onopen = () => {
        setConnected(true)
        setError(null)
      }
      ws.onerror = () => {
        setConnected(false)
        setError('No se puede conectar al servidor. Asegúrate de que el servidor está corriendo en puerto 8000')
      }
      
      return () => ws.close()
    } catch (err) {
      setConnected(false)
      setError('Error al conectar con el servidor')
    }
  }, [])

  return (
    <div className="app">
      <div className="container">
        <h1>Calculadora Distribuida</h1>
        
        {error && (
          <div className="error-message">
            ⚠️ {error}
          </div>
        )}
        
        {connected && (
          <div className="status-ok">
            ✓ Conectado al servidor
          </div>
        )}
        
        <Calculator />
      </div>
    </div>
  )
}

export default App
