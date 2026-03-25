import React, { useState, useRef, useEffect } from 'react'
import './Calculator.css'

function Calculator() {
  const [display, setDisplay] = useState('0')
  const [history, setHistory] = useState('')
  const [wsStatus, setWsStatus] = useState('conectando')
  const wsRef = useRef(null)

  useEffect(() => {
    wsRef.current = new WebSocket('ws://localhost:8000/ws')

    wsRef.current.onopen = () => {
      setWsStatus('conectado')
      if (display === 'Error') setDisplay('0')
    }

    wsRef.current.onmessage = (event) => {
      const result = event.data
      setDisplay(result)
      setHistory((prevHistory) => `${prevHistory}${display} = ${result}\n`)
    }

    wsRef.current.onerror = (error) => {
      console.error('WebSocket error:', error)
      setWsStatus('error')
      // No cambiar display aquí; mantendremos lo actual (ej. 0) hasta calcular.
    }

    wsRef.current.onclose = () => {
      setWsStatus('cerrado')
    }

    return () => {
      if (wsRef.current) {
        wsRef.current.close()
      }
    }
  }, [])

  const handleNumber = (num) => {
    if (display === '0' && num !== '.') {
      setDisplay(String(num))
    } else if (display !== 'Error') {
      setDisplay(display + num)
    }
  }

  const handleOperator = (operator) => {
    if (display !== 'Error') {
      setDisplay(display + operator)
    }
  }

  const handleClear = () => {
    setDisplay('0')
    setHistory('')
  }

  const handleDelete = () => {
    if (display.length === 1) {
      setDisplay('0')
    } else {
      setDisplay(display.slice(0, -1))
    }
  }

  const handlePower = () => {
    if (display !== 'Error') {
      setDisplay(display + '^')
    }
  }

  const handleEqual = () => {
    if (display !== 'Error' && display !== '0') {
      // Enviar la expresión al servidor
      if (wsRef.current && wsRef.current.readyState === WebSocket.OPEN) {
        wsRef.current.send(display)
      } else {
        setDisplay('Error: No conectado')
      }
    }
  }

  return (
    <div className="calculator">
      <div className="display">
        <input 
          type="text" 
          value={display} 
          readOnly
          className="display-input"
        />
      </div>

      <div className="ws-status">
        Estado WS: {wsStatus === 'conectado' ? '✅ conectado' : wsStatus === 'conectando' ? '⌛ conectando...' : wsStatus === 'cerrado' ? '❌ desconectado' : '⚠️ error de conexión'}
      </div>

      <div className="buttons">
        {/* Fila 1 */}
        <button onClick={handleClear} className="btn btn-function">C</button>
        <button onClick={handleDelete} className="btn btn-function">DEL</button>
        <button onClick={() => handleOperator('/')} className="btn btn-operator">÷</button>
        <button onClick={() => handleOperator('*')} className="btn btn-operator">×</button>

        {/* Fila 2 */}
        <button onClick={() => handleNumber(7)} className="btn btn-number">7</button>
        <button onClick={() => handleNumber(8)} className="btn btn-number">8</button>
        <button onClick={() => handleNumber(9)} className="btn btn-number">9</button>
        <button onClick={() => handleOperator('-')} className="btn btn-operator">−</button>

        {/* Fila 3 */}
        <button onClick={() => handleNumber(4)} className="btn btn-number">4</button>
        <button onClick={() => handleNumber(5)} className="btn btn-number">5</button>
        <button onClick={() => handleNumber(6)} className="btn btn-number">6</button>
        <button onClick={() => handleOperator('+')} className="btn btn-operator">+</button>

        {/* Fila 4 */}
        <button onClick={() => handleNumber(1)} className="btn btn-number">1</button>
        <button onClick={() => handleNumber(2)} className="btn btn-number">2</button>
        <button onClick={() => handleNumber(3)} className="btn btn-number">3</button>
        <button onClick={handlePower} className="btn btn-operator">^</button>

        {/* Fila 5 */}
        <button onClick={() => handleNumber(0)} className="btn btn-number zero">0</button>
        <button onClick={() => handleNumber('.')} className="btn btn-number">.</button>
        <button onClick={() => handleOperator('(')} className="btn btn-paren">(</button>
        <button onClick={() => handleOperator(')')} className="btn btn-paren">)</button>

        {/* Fila 6 */}
        <button onClick={handleEqual} className="btn btn-equal">=</button>
      </div>

      {history && (
        <div className="history">
          <p className="history-title">Último cálculo:</p>
          <p className="history-text">{history}</p>
        </div>
      )}
    </div>
  )
}

export default Calculator
