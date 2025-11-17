import React, { useState, useContext } from 'react';
import { useNavigate, useLocation } from 'react-router-dom';
import { AuthContext } from './AuthContext';

// Login page props
interface LoginPageProps {
  redirectPath?: string;
}

// Login page component
const LoginPage: React.FC<LoginPageProps> = ({ redirectPath = '/' }) => {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [apiKey, setApiKey] = useState('');
  const [loginMethod, setLoginMethod] = useState<'credentials' | 'apiKey'>('credentials');
  const [isSubmitting, setIsSubmitting] = useState(false);
  
  const { login, loginWithApiKey, error } = useContext(AuthContext);
  const navigate = useNavigate();
  const location = useLocation();
  
  // Get the redirect path from location state or use the default
  const from = (location.state as any)?.from?.pathname || redirectPath;
  
  // Handle form submission
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (isSubmitting) return;
    
    try {
      setIsSubmitting(true);
      
      if (loginMethod === 'credentials') {
        await login(username, password);
      } else {
        await loginWithApiKey(apiKey);
      }
      
      // Redirect to the previous page or home
      navigate(from, { replace: true });
    } catch (err) {
      console.error('Login failed:', err);
    } finally {
      setIsSubmitting(false);
    }
  };
  
  return (
    <div className="login-page">
      <div className="login-container">
        <h1>Digital Twin Desktop</h1>
        <h2>Login</h2>
        
        {error && (
          <div className="error-message">
            {error}
          </div>
        )}
        
        <div className="login-tabs">
          <button
            className={`tab-button ${loginMethod === 'credentials' ? 'active' : ''}`}
            onClick={() => setLoginMethod('credentials')}
          >
            Username & Password
          </button>
          <button
            className={`tab-button ${loginMethod === 'apiKey' ? 'active' : ''}`}
            onClick={() => setLoginMethod('apiKey')}
          >
            API Key
          </button>
        </div>
        
        <form onSubmit={handleSubmit}>
          {loginMethod === 'credentials' ? (
            <>
              <div className="form-group">
                <label htmlFor="username">Username</label>
                <input
                  type="text"
                  id="username"
                  value={username}
                  onChange={(e) => setUsername(e.target.value)}
                  required
                  autoFocus
                />
              </div>
              
              <div className="form-group">
                <label htmlFor="password">Password</label>
                <input
                  type="password"
                  id="password"
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  required
                />
              </div>
            </>
          ) : (
            <div className="form-group">
              <label htmlFor="apiKey">API Key</label>
              <input
                type="password"
                id="apiKey"
                value={apiKey}
                onChange={(e) => setApiKey(e.target.value)}
                required
                autoFocus
              />
            </div>
          )}
          
          <div className="form-actions">
            <button
              type="submit"
              className="login-button"
              disabled={isSubmitting}
            >
              {isSubmitting ? 'Logging in...' : 'Login'}
            </button>
          </div>
        </form>
      </div>
      
      <style>{`
        .login-page {
          display: flex;
          justify-content: center;
          align-items: center;
          min-height: 100vh;
          background-color: var(--bg-color);
        }
        
        .login-container {
          width: 100%;
          max-width: 400px;
          padding: 2rem;
          background-color: var(--card-bg-color);
          border-radius: 8px;
          box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        }
        
        h1 {
          text-align: center;
          margin-bottom: 0.5rem;
          color: var(--primary-color);
        }
        
        h2 {
          text-align: center;
          margin-bottom: 1.5rem;
          color: var(--text-color);
        }
        
        .error-message {
          background-color: var(--error-bg-color);
          color: var(--error-text-color);
          padding: 0.75rem;
          border-radius: 4px;
          margin-bottom: 1rem;
        }
        
        .login-tabs {
          display: flex;
          margin-bottom: 1.5rem;
          border-bottom: 1px solid var(--border-color);
        }
        
        .tab-button {
          flex: 1;
          background: none;
          border: none;
          padding: 0.75rem;
          cursor: pointer;
          font-size: 0.9rem;
          color: var(--text-color);
          transition: all 0.2s;
        }
        
        .tab-button.active {
          color: var(--primary-color);
          border-bottom: 2px solid var(--primary-color);
        }
        
        .form-group {
          margin-bottom: 1.5rem;
        }
        
        label {
          display: block;
          margin-bottom: 0.5rem;
          color: var(--text-color);
        }
        
        input {
          width: 100%;
          padding: 0.75rem;
          border: 1px solid var(--border-color);
          border-radius: 4px;
          background-color: var(--input-bg-color);
          color: var(--text-color);
          transition: border-color 0.2s;
        }
        
        input:focus {
          border-color: var(--primary-color);
          outline: none;
        }
        
        .form-actions {
          margin-top: 2rem;
        }
        
        .login-button {
          width: 100%;
          padding: 0.75rem;
          background-color: var(--primary-color);
          color: white;
          border: none;
          border-radius: 4px;
          cursor: pointer;
          font-size: 1rem;
          transition: background-color 0.2s;
        }
        
        .login-button:hover {
          background-color: var(--primary-color-dark);
        }
        
        .login-button:disabled {
          background-color: var(--disabled-color);
          cursor: not-allowed;
        }
      `}</style>
    </div>
  );
};

export default LoginPage;