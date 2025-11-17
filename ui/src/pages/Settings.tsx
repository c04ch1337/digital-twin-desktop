import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import Header from '../components/common/Header';
import Sidebar from '../components/common/Sidebar';
import LoadingSpinner from '../components/common/LoadingSpinner';

interface AppSettings {
  theme: 'light' | 'dark' | 'system';
  language: string;
  notifications_enabled: boolean;
  auto_refresh_interval: number;
  data_storage_path: string;
  log_level: 'debug' | 'info' | 'warn' | 'error';
  api_settings: {
    anthropic_api_key?: string;
    openai_api_key?: string;
    default_llm_provider: 'anthropic' | 'openai';
    default_model: string;
  };
}

const Settings: React.FC = () => {
  const [settings, setSettings] = useState<AppSettings | null>(null);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [isSaving, setIsSaving] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const [successMessage, setSuccessMessage] = useState<string | null>(null);
  const [activeTab, setActiveTab] = useState<'general' | 'api' | 'advanced'>('general');

  useEffect(() => {
    loadSettings();
  }, []);

  const loadSettings = async () => {
    setIsLoading(true);
    setError(null);
    
    try {
      const appSettings = await invoke<AppSettings>('get_app_settings');
      setSettings(appSettings);
    } catch (err) {
      console.error('Failed to load settings:', err);
      setError('Failed to load application settings. Please try again.');
    } finally {
      setIsLoading(false);
    }
  };

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    const { name, value, type } = e.target;
    
    if (!settings) return;
    
    // Handle nested properties
    if (name.includes('.')) {
      const [parent, child] = name.split('.');
      
      if (parent === 'api_settings') {
        setSettings({
          ...settings,
          api_settings: {
            ...settings.api_settings,
            [child]: type === 'checkbox'
              ? (e.target as HTMLInputElement).checked
              : value
          }
        });
      }
    } else {
      // Handle top-level properties
      setSettings({
        ...settings,
        [name]: type === 'checkbox' 
          ? (e.target as HTMLInputElement).checked 
          : value
      });
    }
  };

  const handleSaveSettings = async () => {
    if (!settings) return;
    
    setIsSaving(true);
    setError(null);
    setSuccessMessage(null);
    
    try {
      await invoke('save_app_settings', {
        settings
      });
      
      setSuccessMessage('Settings saved successfully!');
      
      // Clear success message after 3 seconds
      setTimeout(() => {
        setSuccessMessage(null);
      }, 3000);
    } catch (err) {
      console.error('Failed to save settings:', err);
      setError('Failed to save settings. Please try again.');
    } finally {
      setIsSaving(false);
    }
  };

  const handleResetSettings = async () => {
    if (!window.confirm('Are you sure you want to reset all settings to default values? This action cannot be undone.')) {
      return;
    }
    
    setIsLoading(true);
    setError(null);
    
    try {
      await invoke('reset_app_settings');
      await loadSettings();
      setSuccessMessage('Settings reset to defaults successfully!');
      
      // Clear success message after 3 seconds
      setTimeout(() => {
        setSuccessMessage(null);
      }, 3000);
    } catch (err) {
      console.error('Failed to reset settings:', err);
      setError('Failed to reset settings. Please try again.');
    } finally {
      setIsLoading(false);
    }
  };

  if (isLoading && !settings) {
    return (
      <div className="app-container">
        <Header />
        <div className="main-content">
          <Sidebar />
          <div className="page-content">
            <LoadingSpinner size="large" message="Loading settings..." />
          </div>
        </div>
      </div>
    );
  }

  if (!settings) {
    return (
      <div className="app-container">
        <Header />
        <div className="main-content">
          <Sidebar />
          <div className="page-content">
            <div className="error-container">
              <p className="error-message">Failed to load settings.</p>
              <button onClick={loadSettings} className="btn btn-primary">
                Retry
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
          <div className="settings-page">
            <div className="page-header">
              <h1>Settings</h1>
            </div>
            
            {error && (
              <div className="error-message">
                <p>{error}</p>
                <button onClick={() => setError(null)} className="dismiss-button">
                  Dismiss
                </button>
              </div>
            )}
            
            {successMessage && (
              <div className="success-message">
                <p>{successMessage}</p>
              </div>
            )}
            
            <div className="settings-tabs">
              <button 
                className={`tab-button ${activeTab === 'general' ? 'active' : ''}`}
                onClick={() => setActiveTab('general')}
              >
                General
              </button>
              <button 
                className={`tab-button ${activeTab === 'api' ? 'active' : ''}`}
                onClick={() => setActiveTab('api')}
              >
                API Settings
              </button>
              <button 
                className={`tab-button ${activeTab === 'advanced' ? 'active' : ''}`}
                onClick={() => setActiveTab('advanced')}
              >
                Advanced
              </button>
            </div>
            
            <div className="settings-content">
              {activeTab === 'general' && (
                <div className="general-settings">
                  <div className="form-group">
                    <label htmlFor="theme">Theme</label>
                    <select
                      id="theme"
                      name="theme"
                      value={settings.theme}
                      onChange={handleInputChange}
                    >
                      <option value="light">Light</option>
                      <option value="dark">Dark</option>
                      <option value="system">System Default</option>
                    </select>
                  </div>
                  
                  <div className="form-group">
                    <label htmlFor="language">Language</label>
                    <select
                      id="language"
                      name="language"
                      value={settings.language}
                      onChange={handleInputChange}
                    >
                      <option value="en">English</option>
                      <option value="es">Spanish</option>
                      <option value="fr">French</option>
                      <option value="de">German</option>
                      <option value="ja">Japanese</option>
                    </select>
                  </div>
                  
                  <div className="form-group checkbox-group">
                    <label htmlFor="notifications_enabled">
                      <input
                        type="checkbox"
                        id="notifications_enabled"
                        name="notifications_enabled"
                        checked={settings.notifications_enabled}
                        onChange={handleInputChange}
                      />
                      Enable Notifications
                    </label>
                  </div>
                  
                  <div className="form-group">
                    <label htmlFor="auto_refresh_interval">Auto-refresh Interval (seconds)</label>
                    <input
                      type="number"
                      id="auto_refresh_interval"
                      name="auto_refresh_interval"
                      min={1}
                      max={3600}
                      value={settings.auto_refresh_interval}
                      onChange={handleInputChange}
                    />
                  </div>
                </div>
              )}
              
              {activeTab === 'api' && (
                <div className="api-settings">
                  <div className="form-group">
                    <label htmlFor="api_settings.default_llm_provider">Default LLM Provider</label>
                    <select
                      id="api_settings.default_llm_provider"
                      name="api_settings.default_llm_provider"
                      value={settings.api_settings.default_llm_provider}
                      onChange={handleInputChange}
                    >
                      <option value="anthropic">Anthropic</option>
                      <option value="openai">OpenAI</option>
                    </select>
                  </div>
                  
                  <div className="form-group">
                    <label htmlFor="api_settings.default_model">Default Model</label>
                    <select
                      id="api_settings.default_model"
                      name="api_settings.default_model"
                      value={settings.api_settings.default_model}
                      onChange={handleInputChange}
                    >
                      <option value="claude-3-opus">Claude 3 Opus</option>
                      <option value="claude-3-sonnet">Claude 3 Sonnet</option>
                      <option value="gpt-4">GPT-4</option>
                      <option value="gpt-3.5-turbo">GPT-3.5 Turbo</option>
                    </select>
                  </div>
                  
                  <div className="form-group">
                    <label htmlFor="api_settings.anthropic_api_key">Anthropic API Key</label>
                    <input
                      type="password"
                      id="api_settings.anthropic_api_key"
                      name="api_settings.anthropic_api_key"
                      value={settings.api_settings.anthropic_api_key || ''}
                      onChange={handleInputChange}
                      placeholder="Enter your Anthropic API key"
                    />
                  </div>
                  
                  <div className="form-group">
                    <label htmlFor="api_settings.openai_api_key">OpenAI API Key</label>
                    <input
                      type="password"
                      id="api_settings.openai_api_key"
                      name="api_settings.openai_api_key"
                      value={settings.api_settings.openai_api_key || ''}
                      onChange={handleInputChange}
                      placeholder="Enter your OpenAI API key"
                    />
                  </div>
                </div>
              )}
              
              {activeTab === 'advanced' && (
                <div className="advanced-settings">
                  <div className="form-group">
                    <label htmlFor="data_storage_path">Data Storage Path</label>
                    <input
                      type="text"
                      id="data_storage_path"
                      name="data_storage_path"
                      value={settings.data_storage_path}
                      onChange={handleInputChange}
                    />
                  </div>
                  
                  <div className="form-group">
                    <label htmlFor="log_level">Log Level</label>
                    <select
                      id="log_level"
                      name="log_level"
                      value={settings.log_level}
                      onChange={handleInputChange}
                    >
                      <option value="debug">Debug</option>
                      <option value="info">Info</option>
                      <option value="warn">Warning</option>
                      <option value="error">Error</option>
                    </select>
                  </div>
                  
                  <div className="danger-zone">
                    <h3>Danger Zone</h3>
                    <button 
                      onClick={handleResetSettings}
                      className="reset-button"
                    >
                      Reset to Default Settings
                    </button>
                  </div>
                </div>
              )}
            </div>
            
            <div className="settings-actions">
              <button 
                onClick={handleSaveSettings}
                disabled={isSaving}
                className="save-button"
              >
                {isSaving ? 'Saving...' : 'Save Settings'}
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Settings;