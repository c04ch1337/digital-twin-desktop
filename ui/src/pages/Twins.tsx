import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Link } from 'react-router-dom';
import Header from '../components/common/Header';
import Sidebar from '../components/common/Sidebar';
import TwinList from '../components/twin/TwinList';
import LoadingSpinner from '../components/common/LoadingSpinner';

interface DigitalTwin {
  id: string;
  name: string;
  twin_type: string;
  configuration: any;
  created_at: string;
  updated_at: string;
}

interface TwinStats {
  total: number;
  by_type: Record<string, number>;
  active_simulations: number;
  connected_sensors: number;
}

const Twins: React.FC = () => {
  const [stats, setStats] = useState<TwinStats | null>(null);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadTwinStats();
  }, []);

  const loadTwinStats = async () => {
    setIsLoading(true);
    setError(null);
    
    try {
      const twinStats = await invoke<TwinStats>('get_twin_stats');
      setStats(twinStats);
    } catch (err) {
      console.error('Failed to load twin statistics:', err);
      setError('Failed to load twin statistics. Please try again.');
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="app-container">
      <Header />
      <div className="main-content">
        <Sidebar />
        <div className="page-content">
          <div className="twins-page">
            <div className="page-header">
              <h1>Digital Twins</h1>
              <div className="header-actions">
                <button onClick={loadTwinStats} className="refresh-button">
                  <span className="refresh-icon">🔄</span> Refresh
                </button>
                <Link to="/twins/new" className="new-button">
                  <span className="new-icon">+</span> New Twin
                </Link>
              </div>
            </div>
            
            {isLoading && !stats ? (
              <LoadingSpinner size="medium" message="Loading twin statistics..." />
            ) : (
              <>
                {stats && (
                  <div className="stats-cards">
                    <div className="stat-card">
                      <div className="stat-value">{stats.total}</div>
                      <div className="stat-label">Total Twins</div>
                    </div>
                    <div className="stat-card">
                      <div className="stat-value">{stats.active_simulations}</div>
                      <div className="stat-label">Active Simulations</div>
                    </div>
                    <div className="stat-card">
                      <div className="stat-value">{stats.connected_sensors}</div>
                      <div className="stat-label">Connected Sensors</div>
                    </div>
                    
                    <div className="stat-card types-card">
                      <div className="stat-label">Twins by Type</div>
                      <div className="types-list">
                        {Object.entries(stats.by_type).map(([type, count]) => (
                          <div key={type} className="type-item">
                            <div className="type-name">{type}</div>
                            <div className="type-count">{count}</div>
                          </div>
                        ))}
                      </div>
                    </div>
                  </div>
                )}
                
                {error && (
                  <div className="error-message">
                    <p>{error}</p>
                    <button onClick={() => setError(null)} className="dismiss-button">
                      Dismiss
                    </button>
                  </div>
                )}
                
                <div className="twins-list-container">
                  <TwinList />
                </div>
              </>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};

export default Twins;