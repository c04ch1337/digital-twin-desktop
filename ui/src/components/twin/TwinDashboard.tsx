import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import LoadingSpinner from '../common/LoadingSpinner';
import SensorGrid from './SensorGrid';
import TwinVisualizer from './TwinVisualizer';

interface DigitalTwin {
  id: string;
  name: string;
  twin_type: string;
  configuration: any;
  created_at: string;
  updated_at: string;
}

interface SensorData {
  id: string;
  twin_id: string;
  sensor_id: string;
  sensor_name: string;
  value: number | string | boolean;
  unit?: string;
  timestamp: string;
}

interface TwinDashboardProps {
  twinId: string;
}

const TwinDashboard: React.FC<TwinDashboardProps> = ({ twinId }) => {
  const [twin, setTwin] = useState<DigitalTwin | null>(null);
  const [sensorData, setSensorData] = useState<SensorData[]>([]);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [refreshInterval, setRefreshInterval] = useState<number>(5000); // 5 seconds
  const [isAutoRefresh, setIsAutoRefresh] = useState<boolean>(true);

  useEffect(() => {
    loadTwinData();
  }, [twinId]);

  useEffect(() => {
    let intervalId: number | undefined;
    
    if (isAutoRefresh) {
      intervalId = window.setInterval(() => {
        loadSensorData();
      }, refreshInterval);
    }
    
    return () => {
      if (intervalId) {
        clearInterval(intervalId);
      }
    };
  }, [isAutoRefresh, refreshInterval, twinId]);

  const loadTwinData = async () => {
    setIsLoading(true);
    setError(null);
    
    try {
      const twinData = await invoke<DigitalTwin>('get_digital_twin', {
        twinId
      });
      setTwin(twinData);
      
      await loadSensorData();
    } catch (err) {
      console.error('Failed to load twin data:', err);
      setError('Failed to load twin data. Please try again.');
    } finally {
      setIsLoading(false);
    }
  };

  const loadSensorData = async () => {
    if (!twinId) return;
    
    try {
      const data = await invoke<SensorData[]>('get_twin_sensor_data', {
        twinId
      });
      setSensorData(data);
    } catch (err) {
      console.error('Failed to load sensor data:', err);
      // Don't set error state here to avoid disrupting the dashboard
      // Just log the error and continue
    }
  };

  const handleRefreshIntervalChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    setRefreshInterval(Number(e.target.value));
  };

  const handleAutoRefreshToggle = () => {
    setIsAutoRefresh(!isAutoRefresh);
  };

  const handleManualRefresh = () => {
    loadSensorData();
  };

  if (isLoading && !twin) {
    return <LoadingSpinner size="large" message="Loading digital twin..." />;
  }

  if (error) {
    return (
      <div className="error-container">
        <p className="error-message">{error}</p>
        <button onClick={loadTwinData} className="btn btn-primary">
          Retry
        </button>
      </div>
    );
  }

  if (!twin) {
    return <p>No twin data available.</p>;
  }

  return (
    <div className="twin-dashboard">
      <div className="dashboard-header">
        <h2>{twin.name} Dashboard</h2>
        <div className="twin-meta">
          <span className="twin-type">{twin.twin_type}</span>
          <span className="twin-updated">Last updated: {new Date(twin.updated_at).toLocaleString()}</span>
        </div>
        <div className="refresh-controls">
          <button 
            onClick={handleManualRefresh}
            className="refresh-button"
            title="Refresh data"
          >
            <span className="refresh-icon">🔄</span>
          </button>
          <div className="auto-refresh">
            <label>
              <input
                type="checkbox"
                checked={isAutoRefresh}
                onChange={handleAutoRefreshToggle}
              />
              Auto-refresh
            </label>
            <select 
              value={refreshInterval} 
              onChange={handleRefreshIntervalChange}
              disabled={!isAutoRefresh}
            >
              <option value={1000}>1 second</option>
              <option value={5000}>5 seconds</option>
              <option value={10000}>10 seconds</option>
              <option value={30000}>30 seconds</option>
              <option value={60000}>1 minute</option>
            </select>
          </div>
        </div>
      </div>
      
      <div className="dashboard-content">
        <div className="visualizer-container">
          <TwinVisualizer 
            twin={twin} 
            sensorData={sensorData} 
          />
        </div>
        
        <div className="sensor-data-container">
          <SensorGrid 
            sensorData={sensorData} 
            isLoading={isLoading} 
          />
        </div>
      </div>
    </div>
  );
};

export default TwinDashboard;