import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useParams, useNavigate, Link } from 'react-router-dom';
import Header from '../components/common/Header';
import Sidebar from '../components/common/Sidebar';
import SimulationResults from '../components/simulation/SimulationResults';
import LoadingSpinner from '../components/common/LoadingSpinner';

interface Simulation {
  id: string;
  twin_id: string;
  twin_name: string;
  status: 'running' | 'completed' | 'failed';
  start_time: string;
  end_time?: string;
  duration_seconds?: number;
  parameters: Record<string, any>;
  error_message?: string;
}

const SimulationDetail: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [simulation, setSimulation] = useState<Simulation | null>(null);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [isStoppingSimulation, setIsStoppingSimulation] = useState<boolean>(false);

  useEffect(() => {
    if (id) {
      loadSimulation();
      
      // If simulation is running, set up polling
      const intervalId = setInterval(() => {
        if (simulation?.status === 'running') {
          loadSimulation(false);
        }
      }, 5000);
      
      return () => clearInterval(intervalId);
    }
  }, [id]);

  const loadSimulation = async (showLoading = true) => {
    if (!id) return;
    
    if (showLoading) {
      setIsLoading(true);
    }
    setError(null);
    
    try {
      const simulationData = await invoke<Simulation>('get_simulation', {
        simulationId: id
      });
      setSimulation(simulationData);
    } catch (err) {
      console.error('Failed to load simulation:', err);
      setError('Failed to load simulation data. Please try again.');
    } finally {
      if (showLoading) {
        setIsLoading(false);
      }
    }
  };

  const handleStopSimulation = async () => {
    if (!id || !simulation) return;
    
    setIsStoppingSimulation(true);
    
    try {
      await invoke('stop_simulation', {
        twinId: simulation.twin_id
      });
      
      // Reload simulation to get updated status
      await loadSimulation(false);
    } catch (err) {
      console.error('Failed to stop simulation:', err);
      setError('Failed to stop simulation. Please try again.');
    } finally {
      setIsStoppingSimulation(false);
    }
  };

  const handleDeleteSimulation = async () => {
    if (!id) return;
    
    if (!window.confirm('Are you sure you want to delete this simulation? This action cannot be undone.')) {
      return;
    }
    
    try {
      await invoke('delete_simulation', {
        simulationId: id
      });
      
      navigate('/simulations');
    } catch (err) {
      console.error('Failed to delete simulation:', err);
      setError('Failed to delete simulation. Please try again.');
    }
  };

  const formatDate = (dateString?: string): string => {
    if (!dateString) return 'N/A';
    const date = new Date(dateString);
    return date.toLocaleString();
  };

  const formatDuration = (seconds?: number): string => {
    if (!seconds) return 'N/A';
    
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const remainingSeconds = seconds % 60;
    
    if (hours > 0) {
      return `${hours}h ${minutes}m ${remainingSeconds}s`;
    } else if (minutes > 0) {
      return `${minutes}m ${remainingSeconds}s`;
    } else {
      return `${remainingSeconds}s`;
    }
  };

  const getStatusClass = (status: Simulation['status']): string => {
    switch (status) {
      case 'running': return 'status-running';
      case 'completed': return 'status-completed';
      case 'failed': return 'status-failed';
      default: return '';
    }
  };

  if (isLoading && !simulation) {
    return (
      <div className="app-container">
        <Header />
        <div className="main-content">
          <Sidebar />
          <div className="page-content">
            <LoadingSpinner size="large" message="Loading simulation..." />
          </div>
        </div>
      </div>
    );
  }

  if (error && !simulation) {
    return (
      <div className="app-container">
        <Header />
        <div className="main-content">
          <Sidebar />
          <div className="page-content">
            <div className="error-container">
              <p className="error-message">{error}</p>
              <button onClick={() => loadSimulation()} className="btn btn-primary">
                Retry
              </button>
            </div>
          </div>
        </div>
      </div>
    );
  }

  if (!simulation) {
    return (
      <div className="app-container">
        <Header />
        <div className="main-content">
          <Sidebar />
          <div className="page-content">
            <div className="not-found-container">
              <h2>Simulation Not Found</h2>
              <p>The simulation you're looking for doesn't exist or has been deleted.</p>
              <button onClick={() => navigate('/simulations')} className="btn btn-primary">
                Back to Simulations
              </button>
            </div>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="app-container">
      <Header />
      <div className="main-content">
        <Sidebar />
        <div className="page-content">
          <div className="simulation-detail">
            <div className="simulation-header">
              <div className="simulation-title">
                <h1>Simulation for {simulation.twin_name}</h1>
                <span className={`status-badge ${getStatusClass(simulation.status)}`}>
                  {simulation.status.charAt(0).toUpperCase() + simulation.status.slice(1)}
                </span>
              </div>
              
              <div className="simulation-actions">
                {simulation.status === 'running' && (
                  <button 
                    onClick={handleStopSimulation}
                    disabled={isStoppingSimulation}
                    className="stop-button"
                  >
                    {isStoppingSimulation ? 'Stopping...' : 'Stop Simulation'}
                  </button>
                )}
                <button 
                  onClick={handleDeleteSimulation}
                  className="delete-button"
                >
                  Delete
                </button>
                <Link 
                  to={`/twins/${simulation.twin_id}`}
                  className="view-twin-button"
                >
                  View Twin
                </Link>
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
            
            <div className="simulation-info">
              <div className="info-card">
                <div className="info-item">
                  <span className="info-label">ID:</span>
                  <span className="info-value">{simulation.id}</span>
                </div>
                <div className="info-item">
                  <span className="info-label">Started:</span>
                  <span className="info-value">{formatDate(simulation.start_time)}</span>
                </div>
                <div className="info-item">
                  <span className="info-label">Ended:</span>
                  <span className="info-value">{formatDate(simulation.end_time)}</span>
                </div>
                <div className="info-item">
                  <span className="info-label">Duration:</span>
                  <span className="info-value">{formatDuration(simulation.duration_seconds)}</span>
                </div>
              </div>
              
              <div className="parameters-card">
                <h3>Parameters</h3>
                <pre className="parameters-json">
                  {JSON.stringify(simulation.parameters, null, 2)}
                </pre>
              </div>
              
              {simulation.error_message && (
                <div className="error-card">
                  <h3>Error</h3>
                  <p className="error-message">{simulation.error_message}</p>
                </div>
              )}
            </div>
            
            <div className="simulation-results-container">
              <SimulationResults 
                simulationId={id}
                twinId={simulation.twin_id}
                isRunning={simulation.status === 'running'}
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default SimulationDetail;