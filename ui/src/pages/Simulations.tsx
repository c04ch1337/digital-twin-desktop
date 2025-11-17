import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Link } from 'react-router-dom';
import Header from '../components/common/Header';
import Sidebar from '../components/common/Sidebar';
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
}

const Simulations: React.FC = () => {
  const [simulations, setSimulations] = useState<Simulation[]>([]);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [filterStatus, setFilterStatus] = useState<'all' | 'running' | 'completed' | 'failed'>('all');
  const [sortBy, setSortBy] = useState<'start_time' | 'end_time' | 'twin_name'>('start_time');
  const [sortDirection, setSortDirection] = useState<'asc' | 'desc'>('desc');

  useEffect(() => {
    loadSimulations();
  }, []);

  const loadSimulations = async () => {
    setIsLoading(true);
    setError(null);
    
    try {
      const simulationsList = await invoke<Simulation[]>('list_simulations');
      setSimulations(simulationsList);
    } catch (err) {
      console.error('Failed to load simulations:', err);
      setError('Failed to load simulations. Please try again.');
    } finally {
      setIsLoading(false);
    }
  };

  const handleSort = (field: 'start_time' | 'end_time' | 'twin_name') => {
    if (field === sortBy) {
      setSortDirection(sortDirection === 'asc' ? 'desc' : 'asc');
    } else {
      setSortBy(field);
      setSortDirection('desc');
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

  const filteredSimulations = simulations.filter(simulation => 
    filterStatus === 'all' || simulation.status === filterStatus
  );

  const sortedSimulations = [...filteredSimulations].sort((a, b) => {
    let aValue: any;
    let bValue: any;
    
    switch (sortBy) {
      case 'twin_name':
        aValue = a.twin_name;
        bValue = b.twin_name;
        break;
      case 'end_time':
        aValue = a.end_time || '9999-12-31T23:59:59Z';
        bValue = b.end_time || '9999-12-31T23:59:59Z';
        break;
      case 'start_time':
      default:
        aValue = a.start_time;
        bValue = b.start_time;
    }
    
    if (aValue === bValue) return 0;
    
    const comparison = aValue < bValue ? -1 : 1;
    return sortDirection === 'asc' ? comparison : -comparison;
  });

  return (
    <div className="app-container">
      <Header />
      <div className="main-content">
        <Sidebar />
        <div className="page-content">
          <div className="simulations-page">
            <div className="page-header">
              <h1>Simulations</h1>
              <div className="header-actions">
                <button onClick={loadSimulations} className="refresh-button">
                  <span className="refresh-icon">🔄</span> Refresh
                </button>
                <Link to="/simulations/new" className="new-button">
                  <span className="new-icon">+</span> New Simulation
                </Link>
              </div>
            </div>
            
            <div className="filter-controls">
              <div className="filter-group">
                <label htmlFor="status-filter">Status:</label>
                <select
                  id="status-filter"
                  value={filterStatus}
                  onChange={(e) => setFilterStatus(e.target.value as any)}
                  className="status-filter"
                >
                  <option value="all">All</option>
                  <option value="running">Running</option>
                  <option value="completed">Completed</option>
                  <option value="failed">Failed</option>
                </select>
              </div>
            </div>
            
            {isLoading ? (
              <LoadingSpinner size="large" message="Loading simulations..." />
            ) : error ? (
              <div className="error-message">
                <p>{error}</p>
                <button onClick={loadSimulations} className="btn btn-primary">
                  Retry
                </button>
              </div>
            ) : (
              <>
                {sortedSimulations.length === 0 ? (
                  <div className="no-data-container">
                    <p className="no-data-message">
                      {filterStatus !== 'all' 
                        ? `No ${filterStatus} simulations found.` 
                        : 'No simulations found.'}
                    </p>
                    <Link to="/simulations/new" className="btn btn-primary">
                      Start New Simulation
                    </Link>
                  </div>
                ) : (
                  <div className="simulations-table">
                    <table>
                      <thead>
                        <tr>
                          <th>Status</th>
                          <th 
                            onClick={() => handleSort('twin_name')}
                            className="sortable-header"
                          >
                            Twin
                            {sortBy === 'twin_name' && (
                              <span className="sort-indicator">
                                {sortDirection === 'asc' ? ' ▲' : ' ▼'}
                              </span>
                            )}
                          </th>
                          <th 
                            onClick={() => handleSort('start_time')}
                            className="sortable-header"
                          >
                            Started
                            {sortBy === 'start_time' && (
                              <span className="sort-indicator">
                                {sortDirection === 'asc' ? ' ▲' : ' ▼'}
                              </span>
                            )}
                          </th>
                          <th 
                            onClick={() => handleSort('end_time')}
                            className="sortable-header"
                          >
                            Ended
                            {sortBy === 'end_time' && (
                              <span className="sort-indicator">
                                {sortDirection === 'asc' ? ' ▲' : ' ▼'}
                              </span>
                            )}
                          </th>
                          <th>Duration</th>
                          <th>Actions</th>
                        </tr>
                      </thead>
                      <tbody>
                        {sortedSimulations.map(simulation => (
                          <tr key={simulation.id} className={getStatusClass(simulation.status)}>
                            <td>
                              <span className={`status-badge ${getStatusClass(simulation.status)}`}>
                                {simulation.status.charAt(0).toUpperCase() + simulation.status.slice(1)}
                              </span>
                            </td>
                            <td>{simulation.twin_name}</td>
                            <td>{formatDate(simulation.start_time)}</td>
                            <td>{formatDate(simulation.end_time)}</td>
                            <td>{formatDuration(simulation.duration_seconds)}</td>
                            <td>
                              <div className="table-actions">
                                <Link 
                                  to={`/simulations/${simulation.id}`}
                                  className="view-button"
                                >
                                  View
                                </Link>
                                <Link 
                                  to={`/twins/${simulation.twin_id}`}
                                  className="twin-button"
                                >
                                  Twin
                                </Link>
                              </div>
                            </td>
                          </tr>
                        ))}
                      </tbody>
                    </table>
                  </div>
                )}
              </>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};

export default Simulations;