import React, { useMemo, useState } from 'react'
import ReactDOM from 'react-dom/client'
import './styles.css'

type Template = {
  id: string
  name: string
  summary: string
}

const templates: Template[] = [
  { id: 'boutique-wealth-desk', name: 'Boutique Wealth Desk', summary: 'Private portfolios and advisory mandates.' },
  { id: 'treasury-desk', name: 'Treasury Desk', summary: 'Liquidity and balance-sheet optimization.' },
  { id: 'advisory-allocation', name: 'Advisory Allocation Team', summary: 'Model allocation and rebalancing workflows.' },
]

function App() {
  const [template, setTemplate] = useState('boutique-wealth-desk')
  const [solverMode, setSolverMode] = useState('simulator')

  const comparison = useMemo(() => ({
    baselineReturn: 10.9,
    optimizedReturn: solverMode === 'hybrid' ? 12.3 : solverMode === 'hardware-ready' ? 12.6 : 11.8,
    baselineVariance: 8.1,
    optimizedVariance: solverMode === 'hybrid' ? 7.1 : solverMode === 'hardware-ready' ? 6.9 : 7.4,
    annualGain: solverMode === 'hybrid' ? '£185k' : solverMode === 'hardware-ready' ? '£210k' : '£145k',
  }), [solverMode])

  return (
    <div className="shell">
      <header>
        <h1>QuantumEdge</h1>
        <p>Quantum-ready optimization for investment banks, delivered with a simulator-first workflow.</p>
      </header>

      <section className="grid two">
        <article className="card">
          <h2>Scenario Builder</h2>
          <label>
            Template
            <select value={template} onChange={(e) => setTemplate(e.target.value)}>
              {templates.map((item) => <option key={item.id} value={item.id}>{item.name}</option>)}
            </select>
          </label>
          <label>
            Solver mode
            <select value={solverMode} onChange={(e) => setSolverMode(e.target.value)}>
              <option value="simulator">Simulator</option>
              <option value="hybrid">Hybrid</option>
              <option value="hardware-ready">Hardware-ready</option>
            </select>
          </label>
          <ul>
            <li>Budget cap</li>
            <li>Sector limits</li>
            <li>Max variance</li>
            <li>Turnover limit</li>
            <li>Liquidity constraints</li>
          </ul>
        </article>

        <article className="card">
          <h2>Results Comparison</h2>
          <div className="metric"><span>Baseline expected return</span><strong>{comparison.baselineReturn}%</strong></div>
          <div className="metric"><span>Optimized expected return</span><strong>{comparison.optimizedReturn}%</strong></div>
          <div className="metric"><span>Baseline variance</span><strong>{comparison.baselineVariance}%</strong></div>
          <div className="metric"><span>Optimized variance</span><strong>{comparison.optimizedVariance}%</strong></div>
          <div className="metric"><span>Annualized gain estimate</span><strong>{comparison.annualGain}</strong></div>
          <div className="metric"><span>Solver mode</span><strong>{solverMode}</strong></div>
        </article>
      </section>

      <section className="grid three">
        {templates.map((item) => (
          <article key={item.id} className={`card ${template === item.id ? 'active' : ''}`}>
            <h2>{item.name}</h2>
            <p>{item.summary}</p>
          </article>
        ))}
      </section>

      <section className="panel">
        <h2>Explainability notes</h2>
        <ul>
          <li>The optimized scenario improves expected return while reducing the risk proxy.</li>
          <li>Budget utilization stays near full deployment with lower turnover than the baseline.</li>
          <li>The adapter layer keeps simulator and future D-Wave backends separate from business logic.</li>
        </ul>
      </section>
    </div>
  )
}

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
)
