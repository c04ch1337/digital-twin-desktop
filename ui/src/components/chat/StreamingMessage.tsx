import React, { useState, useEffect } from 'react';

interface StreamingMessageProps {
  content: string;
  typingSpeed?: number;
}

const StreamingMessage: React.FC<StreamingMessageProps> = ({
  content,
  typingSpeed = 30 // milliseconds per character
}) => {
  const [displayedContent, setDisplayedContent] = useState('');
  const [currentIndex, setCurrentIndex] = useState(0);
  const [isComplete, setIsComplete] = useState(false);

  useEffect(() => {
    // Reset state when content changes
    setDisplayedContent('');
    setCurrentIndex(0);
    setIsComplete(false);
  }, [content]);

  useEffect(() => {
    if (currentIndex >= content.length) {
      setIsComplete(true);
      return;
    }

    const timer = setTimeout(() => {
      setDisplayedContent(prev => prev + content[currentIndex]);
      setCurrentIndex(prev => prev + 1);
    }, typingSpeed);

    return () => clearTimeout(timer);
  }, [currentIndex, content, typingSpeed]);

  return (
    <div className="streaming-message">
      <p>{displayedContent}</p>
      {!isComplete && (
        <span className="typing-indicator">
          <span className="dot"></span>
          <span className="dot"></span>
          <span className="dot"></span>
        </span>
      )}
    </div>
  );
};

export default StreamingMessage;