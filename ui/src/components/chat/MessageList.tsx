import React from 'react';
import StreamingMessage from './StreamingMessage';
import LoadingSpinner from '../common/LoadingSpinner';

interface Message {
  id: string;
  content: string;
  sender: 'user' | 'agent';
  timestamp: string;
  isStreaming?: boolean;
}

interface MessageListProps {
  messages: Message[];
  isLoading?: boolean;
}

const MessageList: React.FC<MessageListProps> = ({ messages, isLoading = false }) => {
  const formatTimestamp = (timestamp: string) => {
    const date = new Date(timestamp);
    return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  };

  if (isLoading && messages.length === 0) {
    return (
      <div className="message-list-loading">
        <LoadingSpinner size="medium" message="Loading conversation..." />
      </div>
    );
  }

  return (
    <div className="message-list">
      {messages.length === 0 ? (
        <div className="empty-message-list">
          <p>No messages yet. Start the conversation!</p>
        </div>
      ) : (
        messages.map((message) => (
          <div 
            key={message.id} 
            className={`message-item ${message.sender === 'user' ? 'user-message' : 'agent-message'}`}
          >
            <div className="message-avatar">
              {message.sender === 'user' ? '👤' : '🤖'}
            </div>
            <div className="message-content">
              <div className="message-header">
                <span className="message-sender">{message.sender === 'user' ? 'You' : 'Agent'}</span>
                <span className="message-time">{formatTimestamp(message.timestamp)}</span>
              </div>
              <div className="message-body">
                {message.isStreaming ? (
                  <StreamingMessage content={message.content} />
                ) : (
                  <p>{message.content}</p>
                )}
              </div>
            </div>
          </div>
        ))
      )}
    </div>
  );
};

export default MessageList;