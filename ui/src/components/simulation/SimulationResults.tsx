import React, { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import LoadingSpinner from '../common/LoadingSpinner';

interface SimulationResult {
  id: string;
  simulation_id: string;
  twin_id: string;
  timestamp: string;
  data: Record<string, any>;
  metrics: {
    [key: string]: number;
  };
}

interface SimulationResultsProps {
  simulationId?: string;
  twinId: string;
  isRunning: boolean;
}

const SimulationResults: React.FC<SimulationResultsProps> = ({
  simulationId,
  twinId,
  isRunning
}) => {
  const [results, setResults] = useState<SimulationResult[]>([]);
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const [selectedMetric, setSelectedMetric] = useState<string | null>(null);
  const [autoRefresh, setAutoRefresh] = useState<boolean>(true);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const refreshIntervalRef = useRef<number>();

  useEffect(() => {
    if (simulationId) {
      loadResults();
    } else {
      setResults([]);
    }
    
    return () => {
      if (refreshIntervalRef.current) {
        window.clearInterval(refreshIntervalRef.current);
      }
    };
  }, [simulationId]);

  useEffect(() => {
    if (refreshIntervalRef.current) {
      window.clearInterval(refreshIntervalRef.current);
    }
    
    if (autoRefresh && isRunning && simulationId) {
      refreshIntervalRef.current = window.setInterval(() => {
        loadResults();
      }, 1000);
    }
    
    return () => {
      if (refreshIntervalRef.current) {
        window.clearInterval(refreshIntervalRef.current);
      }
    };
  }, [autoRefresh, isRunning, simulationId]);

  useEffect(() => {
    if (results.length > 0 && canvasRef.current) {
      drawChart();
    }
  }, [results, selectedMetric]);

  const loadResults = async () => {
    if (!simulationId && !twinId) return;
    
    setIsLoading(true);
    setError(null);
    
    try {
      let loadedResults;
      
      if (simulationId) {
        loadedResults = await invoke<SimulationResult[]>('get_simulation_results', {
          simulationId
        });
      } else {
        loadedResults = await invoke<SimulationResult[]>('get_latest_simulation_results', {
          twinId
        });
      }
      
      setResults(loadedResults);
      
      // Set default selected metric if none is selected
      if (!selectedMetric && loadedResults.length > 0 && loadedResults[0].metrics) {
        const metricKeys = Object.keys(loadedResults[0].metrics);
        if (metricKeys.length > 0) {
          setSelectedMetric(metricKeys[0]);
        }
      }
    } catch (err) {
      console.error('Failed to load simulation results:', err);
      setError('Failed to load simulation results. Please try again.');
    } finally {
      setIsLoading(false);
    }
  };

  const drawChart = () => {
    const canvas = canvasRef.current;
    if (!canvas || !selectedMetric) return;
    
    const ctx = canvas.getContext('2d');
    if (!ctx) return;
    
    const width = canvas.width;
    const height = canvas.height;
    
    // Clear canvas
    ctx.clearRect(0, 0, width, height);
    
    // Draw background
    ctx.fillStyle = '#f9f9f9';
    ctx.fillRect(0, 0, width, height);
    
    // Get metric values
    const metricValues = results.map(result => result.metrics[selectedMetric] || 0);
    
    if (metricValues.length === 0) {
      // No data to display
      ctx.fillStyle = '#666';
      ctx.font = '14px Arial';
      ctx.textAlign = 'center';
      ctx.fillText('No data available', width / 2, height / 2);
      return;
    }
    
    // Calculate min and max values
    const minValue = Math.min(...metricValues);
    const maxValue = Math.max(...metricValues);
    const valueRange = maxValue - minValue;
    
    // Draw axes
    ctx.strokeStyle = '#333';
    ctx.lineWidth = 1;
    
    // X-axis
    ctx.beginPath();
    ctx.moveTo(50, height - 30);
    ctx.lineTo(width - 20, height - 30);
    ctx.stroke();
    
    // Y-axis
    ctx.beginPath();
    ctx.moveTo(50, 20);
    ctx.lineTo(50, height - 30);
    ctx.stroke();
    
    // Draw labels
    ctx.fillStyle = '#333';
    ctx.font = '12px Arial';
    ctx.textAlign = 'center';
    
    // X-axis labels (timestamps)
    const xStep = (width - 70) / (results.length > 1 ? results.length - 1 : 1);
    results.forEach((result, index) => {
      if (index % Math.ceil(results.length / 10) === 0 || index === results.length - 1) {
        const x = 50 + index * xStep;
        const timestamp = new Date(result.timestamp);
        const timeLabel = timestamp.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit' });
        
        ctx.fillText(timeLabel, x, height - 10);
      }
    });
    
    // Y-axis labels
    const yStep = (height - 50) / 5;
    for (let i = 0; i <= 5; i++) {
      const y = height - 30 - i * yStep;
      const value = minValue + (i / 5) * valueRange;
      
      ctx.textAlign = 'right';
      ctx.fillText(value.toFixed(2), 45, y + 5);
      
      // Draw horizontal grid line
      ctx.strokeStyle = '#ddd';
      ctx.beginPath();
      ctx.moveTo(50, y);
      ctx.lineTo(width - 20, y);
      ctx.stroke();
    }
    
    // Draw title
    ctx.fillStyle = '#333';
    ctx.font = 'bold 14px Arial';
    ctx.textAlign = 'center';
    ctx.fillText(selectedMetric, width / 2, 15);
    
    // Draw data line
    ctx.strokeStyle = '#2196F3';
    ctx.lineWidth = 2;
    ctx.beginPath();
    
    results.forEach((result, index) => {
      const x = 50 + index * xStep;
      const value = result.metrics[selectedMetric] || 0;
      const normalizedValue = valueRange === 0 
        ? height - 30 - ((height - 50) / 2) 
        : height - 30 - ((value - minValue) / valueRange) * (height - 50);
      
      if (index === 0) {
        ctx.moveTo(x, normalizedValue);
      } else {
        ctx.lineTo(x, normalizedValue);
      }
    });
    
    ctx.stroke();
    
    // Draw data points
    ctx.fillStyle = '#2196F3';
    results.forEach((result, index) => {
      const x = 50 + index * xStep;
      const value = result.metrics[selectedMetric] || 0;
      const normalizedValue = valueRange === 0 
        ? height - 30 - ((height - 50) / 2) 
        : height - 30 - ((value - minValue) / valueRange) * (height - 50);
      
      ctx.beginPath();
      ctx.arc(x, normalizedValue, 4, 0, Math.PI * 2);
      ctx.fill();
    });
  };

  const getAvailableMetrics = (): string[] => {
    if (results.length === 0) return [];
    
    const allMetrics = new Set<string>();
    
    results.forEach(result => {
      if (result.metrics) {
        Object.keys(result.metrics).forEach(key => allMetrics.add(key));
      }
    });
    
    return Array.from(allMetrics);
  };

  const handleExportResults = () => {
    if (results.length === 0) return;
    
    const jsonStr = JSON.stringify(results, null, 2);
    const dataUri = `data:application/json;charset=utf-8,${encodeURIComponent(jsonStr)}`;
    
    const link = document.createElement('a');
    link.href = dataUri;
    link.download = `simulation-results-${simulationId || twinId}-${new Date().toISOString()}.json`;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  };

  if (isLoading && results.length === 0) {
    return <LoadingSpinner size="medium" message="Loading simulation results..." />;
  }

  return (
    <div className="simulation-results">
      <div className="results-header">
        <h3>Simulation Results</h3>
        <div className="results-actions">
          <label className="auto-refresh-toggle">
            <input
              type="checkbox"
              checked={autoRefresh}
              onChange={() => setAutoRefresh(!autoRefresh)}
              disabled={!isRunning}
            />
            Auto-refresh
          </label>
          <button 
            onClick={loadResults} 
            className="refresh-button"
            disabled={isLoading}
          >
            <span className="refresh-icon">🔄</span>
          </button>
          <button 
            onClick={handleExportResults} 
            className="export-button"
            disabled={results.length === 0}
          >
            Export
          </button>
        </div>
      </div>
      
      {error && (
        <div className="error-message">
          <p>{error}</p>
          <button onClick={() => setError(null)} className="dismiss-button">
            Dismiss
          </button>
        </div>
      )}
      
      {results.length === 0 ? (
        <div className="no-results">
          <p>No simulation results available.</p>
          {!simulationId && (
            <p>Start a simulation to see results here.</p>
          )}
        </div>
      ) : (
        <div className="results-content">
          <div className="metric-selector">
            <label htmlFor="metric-select">Select Metric:</label>
            <select
              id="metric-select"
              value={selectedMetric || ''}
              onChange={(e) => setSelectedMetric(e.target.value)}
            >
              {getAvailableMetrics().map(metric => (
                <option key={metric} value={metric}>{metric}</option>
              ))}
            </select>
          </div>
          
          <div className="chart-container">
            <canvas 
              ref={canvasRef} 
              width={800} 
              height={400}
              className="results-chart"
            />
          </div>
          
          <div className="results-summary">
            <h4>Summary</h4>
            <table className="summary-table">
              <thead>
                <tr>
                  <th>Metric</th>
                  <th>Min</th>
                  <th>Max</th>
                  <th>Average</th>
                  <th>Latest</th>
                </tr>
              </thead>
              <tbody>
                {getAvailableMetrics().map(metric => {
                  const values = results.map(r => r.metrics[metric] || 0);
                  const min = Math.min(...values);
                  const max = Math.max(...values);
                  const avg = values.reduce((sum, val) => sum + val, 0) / values.length;
                  const latest = values[values.length - 1];
                  
                  return (
                    <tr key={metric} className={metric === selectedMetric ? 'selected-row' : ''}>
                      <td>{metric}</td>
                      <td>{min.toFixed(2)}</td>
                      <td>{max.toFixed(2)}</td>
                      <td>{avg.toFixed(2)}</td>
                      <td>{latest.toFixed(2)}</td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
        </div>
      )}
    </div>
  );
};

export default SimulationResults;