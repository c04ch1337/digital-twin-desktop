import React, { createContext, useState, useEffect, useCallback, ReactNode } from 'react';
import { invoke } from '@tauri-apps/api/core';

// Auth user type
export interface AuthUser {
  userId: string;
  username: string;
  roles: string[];
  permissions: string[];
}

// Auth context state
interface AuthContextState {
  user: AuthUser | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  error: string | null;
  login: (username: string, password: string) => Promise<void>;
  loginWithApiKey: (apiKey: string) => Promise<void>;
  logout: () => void;
  hasPermission: (permission: string) => boolean;
  hasRole: (role: string) => boolean;
}

// Create the auth context
export const AuthContext = createContext<AuthContextState>({
  user: null,
  isAuthenticated: false,
  isLoading: true,
  error: null,
  login: async () => {},
  loginWithApiKey: async () => {},
  logout: () => {},
  hasPermission: () => false,
  hasRole: () => false,
});

// Auth provider props
interface AuthProviderProps {
  children: ReactNode;
}

// Auth provider component
export const AuthProvider: React.FC<AuthProviderProps> = ({ children }) => {
  const [user, setUser] = useState<AuthUser | null>(null);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  // Check if the user is authenticated
  const checkAuth = useCallback(async () => {
    try {
      setIsLoading(true);
      
      // Try to get the user from local storage
      const storedUser = localStorage.getItem('auth_user');
      if (storedUser) {
        const parsedUser = JSON.parse(storedUser);
        setUser(parsedUser);
      } else {
        setUser(null);
      }
    } catch (err) {
      console.error('Authentication check failed:', err);
      setUser(null);
      setError('Authentication check failed');
    } finally {
      setIsLoading(false);
    }
  }, []);

  // Login with username and password
  const login = async (username: string, password: string) => {
    try {
      setIsLoading(true);
      setError(null);
      
      // Create Basic auth header
      const authHeader = `Basic ${btoa(`${username}:${password}`)}`;
      
      // Call the authenticate command
      const authResult = await invoke<AuthUser>('authenticate', {
        authHeader,
      });
      
      // Store the user in state and local storage
      setUser(authResult);
      localStorage.setItem('auth_user', JSON.stringify(authResult));
    } catch (err: any) {
      console.error('Login failed:', err);
      setError(err.message || 'Login failed');
      setUser(null);
      localStorage.removeItem('auth_user');
    } finally {
      setIsLoading(false);
    }
  };

  // Login with API key
  const loginWithApiKey = async (apiKey: string) => {
    try {
      setIsLoading(true);
      setError(null);
      
      // Create Bearer auth header
      const authHeader = `Bearer ${apiKey}`;
      
      // Call the authenticate command
      const authResult = await invoke<AuthUser>('authenticate', {
        authHeader,
      });
      
      // Store the user in state and local storage
      setUser(authResult);
      localStorage.setItem('auth_user', JSON.stringify(authResult));
    } catch (err: any) {
      console.error('API key login failed:', err);
      setError(err.message || 'API key login failed');
      setUser(null);
      localStorage.removeItem('auth_user');
    } finally {
      setIsLoading(false);
    }
  };

  // Logout
  const logout = () => {
    setUser(null);
    localStorage.removeItem('auth_user');
  };

  // Check if the user has a permission
  const hasPermission = (permission: string): boolean => {
    if (!user) return false;
    return user.permissions.includes(permission);
  };

  // Check if the user has a role
  const hasRole = (role: string): boolean => {
    if (!user) return false;
    return user.roles.includes(role);
  };

  // Check authentication on mount
  useEffect(() => {
    checkAuth();
  }, [checkAuth]);

  // Context value
  const contextValue: AuthContextState = {
    user,
    isAuthenticated: !!user,
    isLoading,
    error,
    login,
    loginWithApiKey,
    logout,
    hasPermission,
    hasRole,
  };

  return (
    <AuthContext.Provider value={contextValue}>
      {children}
    </AuthContext.Provider>
  );
};