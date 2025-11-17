import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useParams, useNavigate, Link } from 'react-router-dom';
import Header from '../components/common/Header';
import Sidebar from '../components/common/Sidebar';
import TwinDashboard from '../components/twin/TwinDashboard';
import TwinEditor from '../components/twin/TwinEditor';
import SimulationControls from '../components/simulation/SimulationControls';
import SimulationResults from '../components/simulation/SimulationResults';
import LoadingSpinner from '../components/common/LoadingSpinner';

interface DigitalTwin {
  id: string;
  name: string;
  twin_type: string;
  configuration: any;
  created_at: string;
  updated_at: string;
}

const TwinDetail: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [twin, setTwin] = useState<DigitalTwin | null>(null);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [activeTab, setActiveTab] = useState<'dashboard' | 'edit' | 'simulate'>('dashboard');
  const [isEditing, setIsEditing] = useState<boolean>(false);
  const [activeSimulationId, setActiveSimulationId] = useState<string | undefined>(undefined);
  const [isSimulationRunning, setIsSimulationRunning] = useState<boolean>(false);

  useEffect(() => {
    if (id) {
      loadTwin();
      checkActiveSimulation();
    }
  }, [id]);

  const loadTwin = async () => {
    if (!id) return;
    
    setIsLoading(true);
    setError(null);
    
    try {
      const twinData = await invoke<DigitalTwin>('get_digital_twin', {
        twinId: id
      });
      setTwin(twinData);
    } catch (err) {
      console.error('Failed to load twin:', err);
      setError('Failed to load twin data. Please try again.');
    } finally {
      setIsLoading(false);
    }
  };

  const checkActiveSimulation = async () => {
    if (!id) return;
    
    try {
      const result = await invoke<{ simulationId?: string, isRunning: boolean }>('get_active_simulation', {
        twinId: id
      });
      
      setActiveSimulationId(result.simulationId);
      setIsSimulationRunning(result.isRunning);
    } catch (err) {
      console.error('Failed to check active simulation:', err);
      // Don't set error state here to avoid disrupting the main view
    }
  };

  const handleDeleteTwin = async () => {
    if (!id) return;
    
    if (!window.confirm('Are you sure you want to delete this digital twin? This action cannot be undone.')) {
      return;
    }
    
    try {
      await invoke('delete_digital_twin', {
        twinId: id
      });
      
      navigate('/twins');
    } catch (err) {
      console.error('Failed to delete twin:', err);
      setError('Failed to delete twin. Please try again.');
    }
  };

  const handleSaveTwin = async (updatedTwin: any) => {
    setTwin(updatedTwin);
    setIsEditing(false);
    setActiveTab('dashboard');
  };

  const handleStartSimulation = (simulationId: string) => {
    setActiveSimulationId(simulationId);
    setIsSimulationRunning(true);
  };

  const handleStopSimulation = () => {
    setIsSimulationRunning(false);
  };

  if (isLoading && !twin) {
    return (
      <div className="app-container">
        <Header />
        <div className="main-content">
          <Sidebar />
          <div className="page-content">
            <LoadingSpinner size="large" message="Loading digital twin..." />
          </div>
        </div>
      </div>
    );
  }

  if (error && !twin) {
    return (
      <div className="app-container">
        <Header />
        <div className="main-content">
          <Sidebar />
          <div className="page-content">
            <div className="error-container">
              <p className="error-message">{error}</p>
              <button onClick={loadTwin} className="btn btn-primary">
                Retry
              </button>
            </div>
          </div>
        </div>
      </div>
    );
  }

  if (!twin) {
    return (
      <div className="app-container">
        <Header />
        <div className="main-content">
          <Sidebar />
          <div className="page-content">
            <div className="not-found-container">
              <h2>Digital Twin Not Found</h2>
              <p>The digital twin you're looking for doesn't exist or has been deleted.</p>
              <button onClick={() => navigate('/twins')} className="btn btn-primary">
                Back to Twins
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
          <div className="twin-detail">
            <div className="twin-header">
              <div className="twin-title">
                <h1>{twin.name}</h1>
                <span className="twin-type-badge">{twin.twin_type}</span>
              </div>
              
              <div className="twin-actions">
                <button onClick={handleDeleteTwin} className="delete-button">
                  Delete
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
            
            <div className="twin-tabs">
              <button 
                className={`tab-button ${activeTab === 'dashboard' ? 'active' : ''}`}
                onClick={() => setActiveTab('dashboard')}
              >
                Dashboard
              </button>
              <button 
                className={`tab-button ${activeTab === 'edit' ? 'active' : ''}`}
                onClick={() => setActiveTab('edit')}
              >
                Edit
              </button>
              <button 
                className={`tab-button ${activeTab === 'simulate' ? 'active' : ''}`}
                onClick={() => setActiveTab('simulate')}
              >
                Simulate
              </button>
            </div>
            
            <div className="twin-content">
              {activeTab === 'dashboard' && (
                <TwinDashboard twinId={id || ''} />
              )}
              
              {activeTab === 'edit' && (
                <TwinEditor 
                  twinId={id}
                  onSave={handleSaveTwin}
                  onCancel={() => setActiveTab('dashboard')}
                />
              )}
              
              {activeTab === 'simulate' && (
                <div className="simulation-container">
                  <div className="simulation-controls-container">
                    <SimulationControls 
                      twinId={id || ''}
                      onStartSimulation={handleStartSimulation}
                      onStopSimulation={handleStopSimulation}
                      isRunning={isSimulationRunning}
                    />
                  </div>
                  
                  <div className="simulation-results-container">
                    <SimulationResults 
                      simulationId={activeSimulationId}
                      twinId={id || ''}
                      isRunning={isSimulationRunning}
                    />
                  </div>
                </div>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default TwinDetail;