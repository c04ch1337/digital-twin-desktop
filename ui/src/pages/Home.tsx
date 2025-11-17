import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Link } from 'react-router-dom';
import Header from '../components/common/Header';
import Sidebar from '../components/common/Sidebar';
import LoadingSpinner from '../components/common/LoadingSpinner';

interface DigitalTwin {
  id: string;
  name: string;
  twin_type: string;
  created_at: string;
  updated_at: string;
}

interface Conversation {
  id: string;
  title: string;
  created_at: string;
  updated_at: string;
  message_count: number;
}

interface Simulation {
  id: string;
  twin_id: string;
  twin_name: string;
  status: 'running' | 'completed' | 'failed';
  start_time: string;
  end_time?: string;
}

const Home: React.FC = () => {
  const [recentTwins, setRecentTwins] = useState<DigitalTwin[]>([]);
  const [recentConversations, setRecentConversations] = useState<Conversation[]>([]);
  const [activeSimulations, setActiveSimulations] = useState<Simulation[]>([]);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadDashboardData();
  }, []);

  const loadDashboardData = async () => {
    setIsLoading(true);
    setError(null);
    
    try {
      // Load recent twins
      const twins = await invoke<DigitalTwin[]>('list_digital_twins', {
        limit: 5,
        sortBy: 'updated_at',
        sortDirection: 'desc'
      });
      setRecentTwins(twins);
      
      // Load recent conversations
      const conversations = await invoke<Conversation[]>('list_conversations', {
        limit: 5,
        sortBy: 'updated_at',
        sortDirection: 'desc'
      });
      setRecentConversations(conversations);
      
      // Load active simulations
      const simulations = await invoke<Simulation[]>('list_active_simulations');
      setActiveSimulations(simulations);
    } catch (err) {
      console.error('Failed to load dashboard data:', err);
      setError('Failed to load dashboard data. Please try again.');
    } finally {
      setIsLoading(false);
    }
  };

  const formatDate = (dateString: string): string => {
    const date = new Date(dateString);
    return date.toLocaleString();
  };

  return (
    <div className="app-container">
      <Header />
      <div className="main-content">
        <Sidebar />
        <div className="page-content">
          <div className="dashboard">
            <div className="dashboard-header">
              <h1>Dashboard</h1>
              <button onClick={loadDashboardData} className="refresh-button">
                <span className="refresh-icon">🔄</span> Refresh
              </button>
            </div>
            
            {isLoading ? (
              <LoadingSpinner size="large" message="Loading dashboard data..." />
            ) : error ? (
              <div className="error-message">
                <p>{error}</p>
                <button onClick={loadDashboardData} className="btn btn-primary">
                  Retry
                </button>
              </div>
            ) : (
              <div className="dashboard-grid">
                <div className="dashboard-card recent-twins">
                  <div className="card-header">
                    <h2>Recent Digital Twins</h2>
                    <Link to="/twins" className="view-all-link">View All</Link>
                  </div>
                  {recentTwins.length === 0 ? (
                    <p className="no-data-message">No digital twins created yet.</p>
                  ) : (
                    <ul className="dashboard-list">
                      {recentTwins.map(twin => (
                        <li key={twin.id} className="dashboard-list-item">
                          <Link to={`/twins/${twin.id}`} className="item-link">
                            <div className="item-name">{twin.name}</div>
                            <div className="item-meta">
                              <span className="item-type">{twin.twin_type}</span>
                              <span className="item-date">Updated: {formatDate(twin.updated_at)}</span>
                            </div>
                          </Link>
                        </li>
                      ))}
                    </ul>
                  )}
                  <div className="card-footer">
                    <Link to="/twins/new" className="create-button">
                      <span className="create-icon">+</span> Create New Twin
                    </Link>
                  </div>
                </div>
                
                <div className="dashboard-card recent-conversations">
                  <div className="card-header">
                    <h2>Recent Conversations</h2>
                    <Link to="/conversations" className="view-all-link">View All</Link>
                  </div>
                  {recentConversations.length === 0 ? (
                    <p className="no-data-message">No conversations started yet.</p>
                  ) : (
                    <ul className="dashboard-list">
                      {recentConversations.map(conversation => (
                        <li key={conversation.id} className="dashboard-list-item">
                          <Link to={`/conversations/${conversation.id}`} className="item-link">
                            <div className="item-name">{conversation.title}</div>
                            <div className="item-meta">
                              <span className="item-count">{conversation.message_count} messages</span>
                              <span className="item-date">Updated: {formatDate(conversation.updated_at)}</span>
                            </div>
                          </Link>
                        </li>
                      ))}
                    </ul>
                  )}
                  <div className="card-footer">
                    <Link to="/conversations/new" className="create-button">
                      <span className="create-icon">+</span> Start New Conversation
                    </Link>
                  </div>
                </div>
                
                <div className="dashboard-card active-simulations">
                  <div className="card-header">
                    <h2>Active Simulations</h2>
                    <Link to="/simulations" className="view-all-link">View All</Link>
                  </div>
                  {activeSimulations.length === 0 ? (
                    <p className="no-data-message">No active simulations running.</p>
                  ) : (
                    <ul className="dashboard-list">
                      {activeSimulations.map(simulation => (
                        <li key={simulation.id} className="dashboard-list-item">
                          <Link to={`/simulations/${simulation.id}`} className="item-link">
                            <div className="item-name">{simulation.twin_name} Simulation</div>
                            <div className="item-meta">
                              <span className={`item-status status-${simulation.status}`}>
                                {simulation.status.charAt(0).toUpperCase() + simulation.status.slice(1)}
                              </span>
                              <span className="item-date">Started: {formatDate(simulation.start_time)}</span>
                            </div>
                          </Link>
                        </li>
                      ))}
                    </ul>
                  )}
                  <div className="card-footer">
                    <Link to="/simulations/new" className="create-button">
                      <span className="create-icon">+</span> Start New Simulation
                    </Link>
                  </div>
                </div>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};

export default Home;