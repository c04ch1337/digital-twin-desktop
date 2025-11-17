import { useContext } from 'react';
import { AuthContext } from './AuthContext';

/**
 * Custom hook for accessing authentication context
 * 
 * Provides easy access to authentication state and methods
 * throughout the application.
 */
export const useAuth = () => {
  const context = useContext(AuthContext);
  
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  
  return context;
};

/**
 * Custom hook for checking if a user has a specific permission
 * 
 * @param permission - The permission to check
 * @returns boolean indicating if the user has the permission
 */
export const usePermission = (permission: string) => {
  const { hasPermission } = useAuth();
  return hasPermission(permission);
};

/**
 * Custom hook for checking if a user has a specific role
 * 
 * @param role - The role to check
 * @returns boolean indicating if the user has the role
 */
export const useRole = (role: string) => {
  const { hasRole } = useAuth();
  return hasRole(role);
};

/**
 * Custom hook for requiring a permission to access a component
 * 
 * @param permission - The required permission
 * @param fallback - Optional component to render if permission is denied
 * @returns A higher-order component that checks for the permission
 */
export const withPermission = (permission: string, fallback: React.ReactNode = null) => {
  return function <P extends object>(Component: React.ComponentType<P>) {
    return function WithPermissionComponent(props: P) {
      const hasPermission = usePermission(permission);
      
      if (!hasPermission) {
        return fallback ? (
          <>{fallback}</>
        ) : (
          <div className="permission-denied">
            <h2>Permission Denied</h2>
            <p>You don't have the required permission to access this component.</p>
            <p>Required permission: {permission}</p>
          </div>
        );
      }
      
      return <Component {...props} />;
    };
  };
};

/**
 * Custom hook for requiring a role to access a component
 * 
 * @param role - The required role
 * @param fallback - Optional component to render if role is denied
 * @returns A higher-order component that checks for the role
 */
export const withRole = (role: string, fallback: React.ReactNode = null) => {
  return function <P extends object>(Component: React.ComponentType<P>) {
    return function WithRoleComponent(props: P) {
      const hasRole = useRole(role);
      
      if (!hasRole) {
        return fallback ? (
          <>{fallback}</>
        ) : (
          <div className="permission-denied">
            <h2>Access Denied</h2>
            <p>You don't have the required role to access this component.</p>
            <p>Required role: {role}</p>
          </div>
        );
      }
      
      return <Component {...props} />;
    };
  };
};

export default useAuth;