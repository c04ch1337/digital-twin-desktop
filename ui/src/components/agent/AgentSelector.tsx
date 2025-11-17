import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import LoadingSpinner from '../common/LoadingSpinner';
import AgentStatus from './AgentStatus';

interface Agent {
  id: string;
  name: string;
  description: string;
  status: 'online' | 'offline' | 'busy' | 'error';
  capabilities: string[];
  lastActive?: string;
}

interface AgentSelectorProps {
  onSelectAgent: (agentId: string) => void;
  selectedAgentId?: string;
}

const AgentSelector: React.FC<AgentSelectorProps> = ({
  onSelectAgent,
  selectedAgentId
}) => {
  const [agents, setAgents] = useState<Agent[]>([]);
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const [searchQuery, setSearchQuery] = useState<string>('');

  useEffect(() => {
    loadAgents();
  }, []);

  const loadAgents = async () => {
    setIsLoading(true);
    setError(null);
    
    try {
      const agentsList = await invoke<Agent[]>('list_agents');
      setAgents(agentsList);
    } catch (err) {
      console.error('Failed to load agents:', err);
      setError('Failed to load agents. Please try again.');
    } finally {
      setIsLoading(false);
    }
  };

  const filteredAgents = agents.filter(agent => 
    agent.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
    agent.description.toLowerCase().includes(searchQuery.toLowerCase()) ||
    agent.capabilities.some(cap => cap.toLowerCase().includes(searchQuery.toLowerCase()))
  );

  const handleSelectAgent = (agentId: string) => {
    onSelectAgent(agentId);
  };

  if (isLoading) {
    return <LoadingSpinner size="medium" message="Loading agents..." />;
  }

  if (error) {
    return (
      <div className="error-message">
        <p>{error}</p>
        <button onClick={loadAgents} className="btn btn-primary">
          Retry
        </button>
      </div>
    );
  }

  return (
    <div className="agent-selector">
      <div className="agent-selector-header">
        <h3>Select Agent</h3>
        <input
          type="text"
          placeholder="Search agents..."
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          className="agent-search-input"
        />
        <button onClick={loadAgents} className="refresh-button">
          <span className="refresh-icon">🔄</span>
        </button>
      </div>
      
      <div className="agents-list">
        {filteredAgents.length === 0 ? (
          <p className="no-agents-message">
            {searchQuery ? 'No agents match your search.' : 'No agents available.'}
          </p>
        ) : (
          filteredAgents.map((agent) => (
            <div 
              key={agent.id}
              className={`agent-card ${selectedAgentId === agent.id ? 'selected' : ''}`}
              onClick={() => handleSelectAgent(agent.id)}
            >
              <div className="agent-card-header">
                <h4 className="agent-name">{agent.name}</h4>
                <AgentStatus 
                  status={agent.status} 
                  lastActive={agent.lastActive}
                />
              </div>
              <p className="agent-description">{agent.description}</p>
              <div className="agent-capabilities">
                {agent.capabilities.map((capability, index) => (
                  <span key={index} className="capability-tag">
                    {capability}
                  </span>
                ))}
              </div>
            </div>
          ))
        )}
      </div>
    </div>
  );
};

export default AgentSelector;