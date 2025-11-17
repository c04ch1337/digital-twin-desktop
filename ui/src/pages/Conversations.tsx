import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Link } from 'react-router-dom';
import Header from '../components/common/Header';
import Sidebar from '../components/common/Sidebar';
import LoadingSpinner from '../components/common/LoadingSpinner';

interface Conversation {
  id: string;
  title: string;
  created_at: string;
  updated_at: string;
  message_count: number;
  agent_id?: string;
  agent_name?: string;
  last_message?: string;
}

const Conversations: React.FC = () => {
  const [conversations, setConversations] = useState<Conversation[]>([]);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [searchQuery, setSearchQuery] = useState<string>('');
  const [sortBy, setSortBy] = useState<'updated_at' | 'created_at' | 'title'>('updated_at');
  const [sortDirection, setSortDirection] = useState<'asc' | 'desc'>('desc');

  useEffect(() => {
    loadConversations();
  }, [sortBy, sortDirection]);

  const loadConversations = async () => {
    setIsLoading(true);
    setError(null);
    
    try {
      const conversationsList = await invoke<Conversation[]>('list_conversations', {
        sortBy,
        sortDirection
      });
      setConversations(conversationsList);
    } catch (err) {
      console.error('Failed to load conversations:', err);
      setError('Failed to load conversations. Please try again.');
    } finally {
      setIsLoading(false);
    }
  };

  const handleSort = (field: 'updated_at' | 'created_at' | 'title') => {
    if (field === sortBy) {
      setSortDirection(sortDirection === 'asc' ? 'desc' : 'asc');
    } else {
      setSortBy(field);
      setSortDirection('desc');
    }
  };

  const formatDate = (dateString: string): string => {
    const date = new Date(dateString);
    return date.toLocaleString();
  };

  const filteredConversations = conversations.filter(conversation => 
    conversation.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
    (conversation.agent_name && conversation.agent_name.toLowerCase().includes(searchQuery.toLowerCase())) ||
    (conversation.last_message && conversation.last_message.toLowerCase().includes(searchQuery.toLowerCase()))
  );

  return (
    <div className="app-container">
      <Header />
      <div className="main-content">
        <Sidebar />
        <div className="page-content">
          <div className="conversations-page">
            <div className="page-header">
              <h1>Conversations</h1>
              <div className="header-actions">
                <button onClick={loadConversations} className="refresh-button">
                  <span className="refresh-icon">🔄</span> Refresh
                </button>
                <Link to="/conversations/new" className="new-button">
                  <span className="new-icon">+</span> New Conversation
                </Link>
              </div>
            </div>
            
            <div className="search-filter">
              <input
                type="text"
                placeholder="Search conversations..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="search-input"
              />
            </div>
            
            {isLoading ? (
              <LoadingSpinner size="large" message="Loading conversations..." />
            ) : error ? (
              <div className="error-message">
                <p>{error}</p>
                <button onClick={loadConversations} className="btn btn-primary">
                  Retry
                </button>
              </div>
            ) : (
              <>
                {filteredConversations.length === 0 ? (
                  <div className="no-data-container">
                    <p className="no-data-message">
                      {searchQuery 
                        ? 'No conversations match your search criteria.' 
                        : 'No conversations found. Start a new conversation!'}
                    </p>
                    {!searchQuery && (
                      <Link to="/conversations/new" className="btn btn-primary">
                        Start New Conversation
                      </Link>
                    )}
                  </div>
                ) : (
                  <div className="conversations-list">
                    <div className="list-header">
                      <div 
                        className="header-title" 
                        onClick={() => handleSort('title')}
                      >
                        Title
                        {sortBy === 'title' && (
                          <span className="sort-indicator">
                            {sortDirection === 'asc' ? ' ▲' : ' ▼'}
                          </span>
                        )}
                      </div>
                      <div className="header-agent">Agent</div>
                      <div className="header-messages">Messages</div>
                      <div 
                        className="header-created" 
                        onClick={() => handleSort('created_at')}
                      >
                        Created
                        {sortBy === 'created_at' && (
                          <span className="sort-indicator">
                            {sortDirection === 'asc' ? ' ▲' : ' ▼'}
                          </span>
                        )}
                      </div>
                      <div 
                        className="header-updated" 
                        onClick={() => handleSort('updated_at')}
                      >
                        Last Updated
                        {sortBy === 'updated_at' && (
                          <span className="sort-indicator">
                            {sortDirection === 'asc' ? ' ▲' : ' ▼'}
                          </span>
                        )}
                      </div>
                    </div>
                    
                    {filteredConversations.map(conversation => (
                      <Link 
                        key={conversation.id} 
                        to={`/conversations/${conversation.id}`}
                        className="conversation-item"
                      >
                        <div className="item-title">{conversation.title}</div>
                        <div className="item-agent">
                          {conversation.agent_name || 'No agent'}
                        </div>
                        <div className="item-messages">
                          {conversation.message_count}
                        </div>
                        <div className="item-created">
                          {formatDate(conversation.created_at)}
                        </div>
                        <div className="item-updated">
                          {formatDate(conversation.updated_at)}
                        </div>
                      </Link>
                    ))}
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

export default Conversations;