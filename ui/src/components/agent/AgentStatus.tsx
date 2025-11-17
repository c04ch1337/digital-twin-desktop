import React from 'react';

type AgentStatusType = 'online' | 'offline' | 'busy' | 'error';

interface AgentStatusProps {
  status: AgentStatusType;
  lastActive?: string;
  name?: string;
}

const AgentStatus: React.FC<AgentStatusProps> = ({ 
  status, 
  lastActive, 
  name 
}) => {
  const getStatusColor = (): string => {
    switch (status) {
      case 'online':
        return 'status-green';
      case 'busy':
        return 'status-yellow';
      case 'offline':
        return 'status-gray';
      case 'error':
        return 'status-red';
      default:
        return 'status-gray';
    }
  };

  const getStatusText = (): string => {
    switch (status) {
      case 'online':
        return 'Online';
      case 'busy':
        return 'Busy';
      case 'offline':
        return 'Offline';
      case 'error':
        return 'Error';
      default:
        return 'Unknown';
    }
  };

  const formatLastActive = (): string => {
    if (!lastActive) return 'Never';
    
    const lastActiveDate = new Date(lastActive);
    const now = new Date();
    const diffMs = now.getTime() - lastActiveDate.getTime();
    const diffMins = Math.floor(diffMs / 60000);
    
    if (diffMins < 1) return 'Just now';
    if (diffMins < 60) return `${diffMins} min ago`;
    
    const diffHours = Math.floor(diffMins / 60);
    if (diffHours < 24) return `${diffHours} hr ago`;
    
    const diffDays = Math.floor(diffHours / 24);
    return `${diffDays} day${diffDays > 1 ? 's' : ''} ago`;
  };

  return (
    <div className="agent-status">
      {name && <span className="agent-name">{name}</span>}
      <div className={`status-indicator ${getStatusColor()}`}>
        <span className="status-dot"></span>
        <span className="status-text">{getStatusText()}</span>
      </div>
      {lastActive && (
        <div className="last-active">
          Last active: {formatLastActive()}
        </div>
      )}
    </div>
  );
};

export default AgentStatus;