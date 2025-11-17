import React, { useState, useEffect, useRef } from 'react';
import MessageList from './MessageList';
import MessageInput from './MessageInput';
import { invoke } from '@tauri-apps/api/core';

interface Message {
  id: string;
  content: string;
  sender: 'user' | 'agent';
  timestamp: string;
  isStreaming?: boolean;
}

interface ChatWindowProps {
  conversationId: string;
  agentId?: string;
}

const ChatWindow: React.FC<ChatWindowProps> = ({ conversationId, agentId }) => {
  const [messages, setMessages] = useState<Message[]>([]);
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    loadMessages();
  }, [conversationId]);

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  const loadMessages = async () => {
    if (!conversationId) return;
    
    setIsLoading(true);
    setError(null);
    
    try {
      const loadedMessages = await invoke<Message[]>('get_conversation_messages', {
        conversationId
      });
      setMessages(loadedMessages);
    } catch (err) {
      console.error('Failed to load messages:', err);
      setError('Failed to load messages. Please try again.');
    } finally {
      setIsLoading(false);
    }
  };

  const sendMessage = async (content: string) => {
    if (!content.trim() || !conversationId) return;
    
    // Create a temporary message with a streaming flag
    const tempMessage: Message = {
      id: `temp-${Date.now()}`,
      content,
      sender: 'user',
      timestamp: new Date().toISOString()
    };
    
    setMessages(prev => [...prev, tempMessage]);
    
    try {
      // Add streaming response placeholder
      const streamingResponse: Message = {
        id: `stream-${Date.now()}`,
        content: '',
        sender: 'agent',
        timestamp: new Date().toISOString(),
        isStreaming: true
      };
      
      setMessages(prev => [...prev, streamingResponse]);
      
      // Send the message to the backend
      const response = await invoke<{ messageId: string }>('send_message', {
        conversationId,
        content,
        agentId
      });
      
      // After sending, refresh the messages to get the actual response
      loadMessages();
    } catch (err) {
      console.error('Failed to send message:', err);
      setError('Failed to send message. Please try again.');
      
      // Remove the streaming message on error
      setMessages(prev => prev.filter(msg => !msg.isStreaming));
    }
  };

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  return (
    <div className="chat-window">
      <div className="chat-header">
        <h2>Conversation {conversationId}</h2>
        {agentId && <div className="agent-badge">Agent: {agentId}</div>}
      </div>
      
      <MessageList 
        messages={messages} 
        isLoading={isLoading} 
      />
      
      <div ref={messagesEndRef} />
      
      {error && (
        <div className="error-message">
          {error}
          <button onClick={() => setError(null)}>Dismiss</button>
        </div>
      )}
      
      <MessageInput 
        onSendMessage={sendMessage} 
        disabled={isLoading || !!error} 
      />
    </div>
  );
};

export default ChatWindow;