import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useParams, useNavigate } from 'react-router-dom';
import Header from '../components/common/Header';
import Sidebar from '../components/common/Sidebar';
import ChatWindow from '../components/chat/ChatWindow';
import AgentSelector from '../components/agent/AgentSelector';
import ToolExecutionViewer from '../components/agent/ToolExecutionViewer';
import LoadingSpinner from '../components/common/LoadingSpinner';

interface Conversation {
  id: string;
  title: string;
  created_at: string;
  updated_at: string;
  agent_id?: string;
}

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

const ConversationDetail: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [conversation, setConversation] = useState<Conversation | null>(null);
  const [toolExecutions, setToolExecutions] = useState<ToolExecution[]>([]);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [showAgentSelector, setShowAgentSelector] = useState<boolean>(false);
  const [showToolExecutions, setShowToolExecutions] = useState<boolean>(false);
  const [isEditingTitle, setIsEditingTitle] = useState<boolean>(false);
  const [newTitle, setNewTitle] = useState<string>('');

  useEffect(() => {
    if (id) {
      loadConversation();
    }
  }, [id]);

  const loadConversation = async () => {
    if (!id) return;
    
    setIsLoading(true);
    setError(null);
    
    try {
      const conversationData = await invoke<Conversation>('get_conversation', {
        conversationId: id
      });
      setConversation(conversationData);
      setNewTitle(conversationData.title);
      
      // Load tool executions
      loadToolExecutions();
    } catch (err) {
      console.error('Failed to load conversation:', err);
      setError('Failed to load conversation. Please try again.');
    } finally {
      setIsLoading(false);
    }
  };

  const loadToolExecutions = async () => {
    if (!id) return;
    
    try {
      const executions = await invoke<ToolExecution[]>('get_conversation_tool_executions', {
        conversationId: id
      });
      setToolExecutions(executions);
    } catch (err) {
      console.error('Failed to load tool executions:', err);
      // Don't set error state here to avoid disrupting the main conversation view
    }
  };

  const handleAgentChange = async (agentId: string) => {
    if (!id) return;
    
    try {
      await invoke('set_conversation_agent', {
        conversationId: id,
        agentId
      });
      
      // Reload conversation to get updated data
      loadConversation();
      setShowAgentSelector(false);
    } catch (err) {
      console.error('Failed to set conversation agent:', err);
      setError('Failed to set agent for this conversation. Please try again.');
    }
  };

  const handleUpdateTitle = async () => {
    if (!id || !newTitle.trim()) return;
    
    try {
      await invoke('update_conversation_title', {
        conversationId: id,
        title: newTitle
      });
      
      setConversation(prev => prev ? { ...prev, title: newTitle } : null);
      setIsEditingTitle(false);
    } catch (err) {
      console.error('Failed to update conversation title:', err);
      setError('Failed to update conversation title. Please try again.');
    }
  };

  const handleDeleteConversation = async () => {
    if (!id) return;
    
    if (!window.confirm('Are you sure you want to delete this conversation? This action cannot be undone.')) {
      return;
    }
    
    try {
      await invoke('delete_conversation', {
        conversationId: id
      });
      
      navigate('/conversations');
    } catch (err) {
      console.error('Failed to delete conversation:', err);
      setError('Failed to delete conversation. Please try again.');
    }
  };

  const handleRetryToolExecution = async (executionId: string) => {
    try {
      await invoke('retry_tool_execution', {
        executionId
      });
      
      // Reload tool executions to get updated data
      loadToolExecutions();
    } catch (err) {
      console.error('Failed to retry tool execution:', err);
      setError('Failed to retry tool execution. Please try again.');
    }
  };

  if (isLoading && !conversation) {
    return (
      <div className="app-container">
        <Header />
        <div className="main-content">
          <Sidebar />
          <div className="page-content">
            <LoadingSpinner size="large" message="Loading conversation..." />
          </div>
        </div>
      </div>
    );
  }

  if (error && !conversation) {
    return (
      <div className="app-container">
        <Header />
        <div className="main-content">
          <Sidebar />
          <div className="page-content">
            <div className="error-container">
              <p className="error-message">{error}</p>
              <button onClick={loadConversation} className="btn btn-primary">
                Retry
              </button>
            </div>
          </div>
        </div>
      </div>
    );
  }

  if (!conversation) {
    return (
      <div className="app-container">
        <Header />
        <div className="main-content">
          <Sidebar />
          <div className="page-content">
            <div className="not-found-container">
              <h2>Conversation Not Found</h2>
              <p>The conversation you're looking for doesn't exist or has been deleted.</p>
              <button onClick={() => navigate('/conversations')} className="btn btn-primary">
                Back to Conversations
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
          <div className="conversation-detail">
            <div className="conversation-header">
              {isEditingTitle ? (
                <div className="title-edit">
                  <input
                    type="text"
                    value={newTitle}
                    onChange={(e) => setNewTitle(e.target.value)}
                    autoFocus
                    className="title-input"
                  />
                  <button onClick={handleUpdateTitle} className="save-button">
                    Save
                  </button>
                  <button onClick={() => setIsEditingTitle(false)} className="cancel-button">
                    Cancel
                  </button>
                </div>
              ) : (
                <h1 className="conversation-title" onClick={() => setIsEditingTitle(true)}>
                  {conversation.title}
                  <span className="edit-icon">✏️</span>
                </h1>
              )}
              
              <div className="conversation-actions">
                <button 
                  onClick={() => setShowAgentSelector(!showAgentSelector)}
                  className="agent-button"
                >
                  {conversation.agent_id ? 'Change Agent' : 'Assign Agent'}
                </button>
                <button 
                  onClick={() => setShowToolExecutions(!showToolExecutions)}
                  className="tools-button"
                >
                  {showToolExecutions ? 'Hide Tools' : 'Show Tools'}
                </button>
                <button 
                  onClick={handleDeleteConversation}
                  className="delete-button"
                >
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
            
            <div className="conversation-content">
              <div className="chat-container">
                <ChatWindow 
                  conversationId={id || ''} 
                  agentId={conversation.agent_id}
                />
              </div>
              
              {showAgentSelector && (
                <div className="agent-selector-container">
                  <AgentSelector 
                    onSelectAgent={handleAgentChange}
                    selectedAgentId={conversation.agent_id}
                  />
                </div>
              )}
              
              {showToolExecutions && (
                <div className="tool-executions-container">
                  <ToolExecutionViewer 
                    toolExecutions={toolExecutions}
                    onRetry={handleRetryToolExecution}
                  />
                </div>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default ConversationDetail;