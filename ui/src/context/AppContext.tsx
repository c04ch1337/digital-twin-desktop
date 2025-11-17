import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import { getAppSettings, saveAppSettings } from '../api';

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

interface AppContextType {
  settings: AppSettings | null;
  isLoading: boolean;
  error: string | null;
  updateSettings: (newSettings: AppSettings) => Promise<void>;
  applyTheme: (theme: 'light' | 'dark' | 'system') => void;
}

const defaultSettings: AppSettings = {
  theme: 'system',
  language: 'en',
  notifications_enabled: true,
  auto_refresh_interval: 5,
  data_storage_path: '',
  log_level: 'info',
  api_settings: {
    default_llm_provider: 'anthropic',
    default_model: 'claude-3-sonnet'
  }
};

const AppContext = createContext<AppContextType | undefined>(undefined);

export const AppProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [settings, setSettings] = useState<AppSettings | null>(null);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadSettings();
  }, []);

  useEffect(() => {
    if (settings) {
      applyTheme(settings.theme);
    }
  }, [settings?.theme]);

  const loadSettings = async () => {
    setIsLoading(true);
    setError(null);
    
    try {
      const appSettings = await getAppSettings();
      setSettings(appSettings || defaultSettings);
    } catch (err) {
      console.error('Failed to load settings:', err);
      setError('Failed to load application settings. Using defaults.');
      setSettings(defaultSettings);
    } finally {
      setIsLoading(false);
    }
  };

  const updateSettings = async (newSettings: AppSettings) => {
    setIsLoading(true);
    setError(null);
    
    try {
      await saveAppSettings(newSettings);
      setSettings(newSettings);
    } catch (err) {
      console.error('Failed to save settings:', err);
      setError('Failed to save settings. Please try again.');
    } finally {
      setIsLoading(false);
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

  return (
    <AppContext.Provider
      value={{
        settings,
        isLoading,
        error,
        updateSettings,
        applyTheme
      }}
    >
      {children}
    </AppContext.Provider>
  );
};

export const useAppContext = (): AppContextType => {
  const context = useContext(AppContext);
  
  if (context === undefined) {
    throw new Error('useAppContext must be used within an AppProvider');
  }
  
  return context;
};