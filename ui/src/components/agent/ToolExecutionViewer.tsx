import React, { useState } from 'react';
import LoadingSpinner from '../common/LoadingSpinner';

interface ToolExecution {
  id: string;
  toolName: string;
  parameters: Record<string, any>;
  result?: any;
  status: 'pending' | 'running' | 'completed' | 'failed';
  startTime: string;
  endTime?: string;
  error?: string;
}

interface ToolExecutionViewerProps {
  toolExecutions: ToolExecution[];
  isLoading?: boolean;
  onRetry?: (executionId: string) => void;
}

const ToolExecutionViewer: React.FC<ToolExecutionViewerProps> = ({
  toolExecutions,
  isLoading = false,
  onRetry
}) => {
  const [expandedExecutions, setExpandedExecutions] = useState<Set<string>>(new Set());

  const toggleExpand = (executionId: string) => {
    const newExpanded = new Set(expandedExecutions);
    if (newExpanded.has(executionId)) {
      newExpanded.delete(executionId);
    } else {
      newExpanded.add(executionId);
    }
    setExpandedExecutions(newExpanded);
  };

  const formatTime = (timestamp: string): string => {
    return new Date(timestamp).toLocaleTimeString();
  };

  const calculateDuration = (start: string, end?: string): string => {
    if (!end) return 'In progress';
    
    const startTime = new Date(start).getTime();
    const endTime = new Date(end).getTime();
    const durationMs = endTime - startTime;
    
    if (durationMs < 1000) return `${durationMs}ms`;
    if (durationMs < 60000) return `${Math.floor(durationMs / 1000)}s`;
    
    const minutes = Math.floor(durationMs / 60000);
    const seconds = Math.floor((durationMs % 60000) / 1000);
    return `${minutes}m ${seconds}s`;
  };

  const getStatusIcon = (status: ToolExecution['status']): string => {
    switch (status) {
      case 'pending': return '⏳';
      case 'running': return '🔄';
      case 'completed': return '✅';
      case 'failed': return '❌';
      default: return '❓';
    }
  };

  if (isLoading) {
    return <LoadingSpinner size="small" message="Loading tool executions..." />;
  }

  if (toolExecutions.length === 0) {
    return <p className="no-executions">No tool executions to display.</p>;
  }

  return (
    <div className="tool-execution-viewer">
      <h3>Tool Executions</h3>
      <div className="executions-list">
        {toolExecutions.map((execution) => (
          <div key={execution.id} className={`execution-item status-${execution.status}`}>
            <div 
              className="execution-header" 
              onClick={() => toggleExpand(execution.id)}
            >
              <span className="status-icon">{getStatusIcon(execution.status)}</span>
              <span className="tool-name">{execution.toolName}</span>
              <span className="execution-time">
                {formatTime(execution.startTime)}
                {execution.endTime && ` - ${formatTime(execution.endTime)}`}
              </span>
              <span className="execution-duration">
                {calculateDuration(execution.startTime, execution.endTime)}
              </span>
              <span className="expand-icon">
                {expandedExecutions.has(execution.id) ? '▼' : '▶'}
              </span>
            </div>
            
            {expandedExecutions.has(execution.id) && (
              <div className="execution-details">
                <div className="parameters-section">
                  <h4>Parameters</h4>
                  <pre>{JSON.stringify(execution.parameters, null, 2)}</pre>
                </div>
                
                {execution.result && (
                  <div className="result-section">
                    <h4>Result</h4>
                    <pre>{JSON.stringify(execution.result, null, 2)}</pre>
                  </div>
                )}
                
                {execution.error && (
                  <div className="error-section">
                    <h4>Error</h4>
                    <p className="error-message">{execution.error}</p>
                    {onRetry && (
                      <button 
                        onClick={() => onRetry(execution.id)}
                        className="retry-button"
                      >
                        Retry
                      </button>
                    )}
                  </div>
                )}
              </div>
            )}
          </div>
        ))}
      </div>
    </div>
  );
};

export default ToolExecutionViewer;