import React, { useState, useEffect } from 'react';
import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import { invoke } from '@tauri-apps/api/core';
import './App.css';

// Pages
import Home from './pages/Home';
import Conversations from './pages/Conversations';
import ConversationDetail from './pages/ConversationDetail';
import Twins from './pages/Twins';
import TwinDetail from './pages/TwinDetail';
import Simulations from './pages/Simulations';
import SimulationDetail from './pages/SimulationDetail';
import Settings from './pages/Settings';
import LoginPage from './auth/LoginPage';

// Common components
import LoadingSpinner from './components/common/LoadingSpinner';
import ErrorBoundary from './components/common/ErrorBoundary';

// Auth components
import { AuthProvider } from './auth/AuthContext';
import ProtectedRoute from './auth/ProtectedRoute';

interface AppSettings {
  theme: 'light' | 'dark' | 'system';
}

const App: React.FC = () => {
  const [isInitialized, setIsInitialized] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const [settings, setSettings] = useState<AppSettings | null>(null);

  useEffect(() => {
    initializeApp();
  }, []);

  useEffect(() => {
    if (settings) {
      applyTheme(settings.theme);
    }
  }, [settings]);

  const initializeApp = async () => {
    try {
      // Initialize the application
      await invoke('initialize_app');
      
      // Load settings
      const appSettings = await invoke<AppSettings>('get_app_settings');
      setSettings(appSettings);
      
      setIsInitialized(true);
    } catch (err) {
      console.error('Failed to initialize app:', err);
      setError('Failed to initialize application. Please restart the application.');
    }
  };

  const applyTheme = (theme: 'light' | 'dark' | 'system') => {
    const root = document.documentElement;
    
    if (theme === 'system') {
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
      root.setAttribute('data-theme', prefersDark ? 'dark' : 'light');
    } else {
      root.setAttribute('data-theme', theme);
    }
  };

  if (!isInitialized) {
    return (
      <div className="app-loading">
        <LoadingSpinner size="large" message="Initializing application..." />
        {error && <p className="error-message">{error}</p>}
      </div>
    );
  }

  return (
    <ErrorBoundary>
      <AuthProvider>
        <Router>
          <Routes>
            {/* Public routes */}
            <Route path="/login" element={<LoginPage />} />
            
            {/* Protected routes */}
            <Route element={<ProtectedRoute />}>
              <Route path="/" element={<Home />} />
              
              <Route path="/conversations" element={<Conversations />} />
              <Route path="/conversations/:id" element={<ConversationDetail />} />
              <Route path="/conversations/new" element={<ConversationDetail />} />
              
              {/* Twin routes - require twin:read permission */}
              <Route element={<ProtectedRoute requiredPermission="twin:read" />}>
                <Route path="/twins" element={<Twins />} />
                <Route path="/twins/:id" element={<TwinDetail />} />
              </Route>
              
              {/* Twin creation - requires twin:write permission */}
              <Route element={<ProtectedRoute requiredPermission="twin:write" />}>
                <Route path="/twins/new" element={<TwinDetail />} />
              </Route>
              
              {/* Simulation routes - require simulation:read permission */}
              <Route element={<ProtectedRoute requiredPermission="simulation:read" />}>
                <Route path="/simulations" element={<Simulations />} />
                <Route path="/simulations/:id" element={<SimulationDetail />} />
              </Route>
              
              {/* Simulation creation - requires simulation:write permission */}
              <Route element={<ProtectedRoute requiredPermission="simulation:write" />}>
                <Route path="/simulations/new" element={<SimulationDetail />} />
              </Route>
              
              {/* Settings - requires admin role */}
              <Route element={<ProtectedRoute requiredRole="admin" />}>
                <Route path="/settings" element={<Settings />} />
              </Route>
            </Route>
            
            {/* Redirect any unknown routes to home */}
            <Route path="*" element={<Navigate to="/" replace />} />
          </Routes>
        </Router>
      </AuthProvider>
    </ErrorBoundary>
  );
};

export default App;