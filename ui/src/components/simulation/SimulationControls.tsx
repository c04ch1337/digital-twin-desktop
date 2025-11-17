import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface SimulationConfig {
  duration: number;
  interval: number;
  parameters: Record<string, any>;
  twinId: string;
}

interface SimulationControlsProps {
  twinId: string;
  onStartSimulation: (simulationId: string) => void;
  onStopSimulation: () => void;
  isRunning: boolean;
}

const SimulationControls: React.FC<SimulationControlsProps> = ({
  twinId,
  onStartSimulation,
  onStopSimulation,
  isRunning
}) => {
  const [duration, setDuration] = useState<number>(60); // seconds
  const [interval, setInterval] = useState<number>(1); // seconds
  const [parameters, setParameters] = useState<string>('{}');
  const [parametersError, setParametersError] = useState<string | null>(null);
  const [isStarting, setIsStarting] = useState<boolean>(false);
  const [isStopping, setIsStopping] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);

  const handleParametersChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    setParameters(e.target.value);
    setParametersError(null);
    
    try {
      JSON.parse(e.target.value);
    } catch (err) {
      setParametersError('Invalid JSON format');
    }
  };

  const handleStartSimulation = async () => {
    if (parametersError) return;
    
    setIsStarting(true);
    setError(null);
    
    try {
      const parsedParams = JSON.parse(parameters);
      
      const config: SimulationConfig = {
        duration,
        interval,
        parameters: parsedParams,
        twinId
      };
      
      const result = await invoke<{ simulationId: string }>('start_simulation', {
        config
      });
      
      onStartSimulation(result.simulationId);
    } catch (err) {
      console.error('Failed to start simulation:', err);
      setError('Failed to start simulation. Please try again.');
    } finally {
      setIsStarting(false);
    }
  };

  const handleStopSimulation = async () => {
    setIsStopping(true);
    setError(null);
    
    try {
      await invoke('stop_simulation', {
        twinId
      });
      
      onStopSimulation();
    } catch (err) {
      console.error('Failed to stop simulation:', err);
      setError('Failed to stop simulation. Please try again.');
    } finally {
      setIsStopping(false);
    }
  };

  return (
    <div className="simulation-controls">
      <h3>Simulation Controls</h3>
      
      {error && (
        <div className="error-message">
          <p>{error}</p>
          <button onClick={() => setError(null)} className="dismiss-button">
            Dismiss
          </button>
        </div>
      )}
      
      <div className="control-panel">
        <div className="form-group">
          <label htmlFor="duration">Duration (seconds)</label>
          <input
            type="number"
            id="duration"
            min={1}
            max={3600}
            value={duration}
            onChange={(e) => setDuration(Number(e.target.value))}
            disabled={isRunning}
          />
        </div>
        
        <div className="form-group">
          <label htmlFor="interval">Update Interval (seconds)</label>
          <input
            type="number"
            id="interval"
            min={0.1}
            max={60}
            step={0.1}
            value={interval}
            onChange={(e) => setInterval(Number(e.target.value))}
            disabled={isRunning}
          />
        </div>
        
        <div className="form-group">
          <label htmlFor="parameters">
            Parameters (JSON)
            {parametersError && <span className="error-text"> - {parametersError}</span>}
          </label>
          <textarea
            id="parameters"
            rows={5}
            value={parameters}
            onChange={handleParametersChange}
            disabled={isRunning}
            className={parametersError ? 'error-input' : ''}
          />
        </div>
        
        <div className="simulation-actions">
          {!isRunning ? (
            <button
              onClick={handleStartSimulation}
              disabled={isStarting || !!parametersError}
              className="start-button"
            >
              {isStarting ? 'Starting...' : 'Start Simulation'}
            </button>
          ) : (
            <button
              onClick={handleStopSimulation}
              disabled={isStopping}
              className="stop-button"
            >
              {isStopping ? 'Stopping...' : 'Stop Simulation'}
            </button>
          )}
        </div>
      </div>
      
      <div className="simulation-presets">
        <h4>Presets</h4>
        <div className="preset-buttons">
          <button 
            onClick={() => {
              setDuration(60);
              setInterval(1);
              setParameters(JSON.stringify({ mode: 'normal' }, null, 2));
            }}
            disabled={isRunning}
            className="preset-button"
          >
            Normal Operation
          </button>
          <button 
            onClick={() => {
              setDuration(120);
              setInterval(0.5);
              setParameters(JSON.stringify({ mode: 'stress_test', load: 0.8 }, null, 2));
            }}
            disabled={isRunning}
            className="preset-button"
          >
            Stress Test
          </button>
          <button 
            onClick={() => {
              setDuration(300);
              setInterval(2);
              setParameters(JSON.stringify({ mode: 'failure_scenario', component: 'sensor_1' }, null, 2));
            }}
            disabled={isRunning}
            className="preset-button"
          >
            Failure Scenario
          </button>
        </div>
      </div>
    </div>
  );
};

export default SimulationControls;