import React, { useContext } from 'react';
import { Navigate, useLocation, Outlet } from 'react-router-dom';
import { AuthContext } from './AuthContext';
import LoadingSpinner from '../components/common/LoadingSpinner';

// Protected route props
interface ProtectedRouteProps {
  requiredPermission?: string;
  requiredRole?: string;
}

/**
 * Protected route component that requires authentication
 * Optionally can require specific permissions or roles
 */
const ProtectedRoute: React.FC<ProtectedRouteProps> = ({
  requiredPermission,
  requiredRole,
}) => {
  const { isAuthenticated, isLoading, hasPermission, hasRole } = useContext(AuthContext);
  const location = useLocation();

  // Show loading spinner while checking authentication
  if (isLoading) {
    return (
      <div className="protected-route-loading">
        <LoadingSpinner size="medium" message="Checking authentication..." />
      </div>
    );
  }

  // If not authenticated, redirect to login
  if (!isAuthenticated) {
    return <Navigate to="/login" state={{ from: location }} replace />;
  }

  // If permission is required, check if user has it
  if (requiredPermission && !hasPermission(requiredPermission)) {
    return (
      <div className="permission-denied">
        <h2>Permission Denied</h2>
        <p>You don't have the required permission to access this page.</p>
        <p>Required permission: {requiredPermission}</p>
        <button onClick={() => window.history.back()}>Go Back</button>
      </div>
    );
  }

  // If role is required, check if user has it
  if (requiredRole && !hasRole(requiredRole)) {
    return (
      <div className="permission-denied">
        <h2>Access Denied</h2>
        <p>You don't have the required role to access this page.</p>
        <p>Required role: {requiredRole}</p>
        <button onClick={() => window.history.back()}>Go Back</button>
      </div>
    );
  }

  // If all checks pass, render the protected content
  return <Outlet />;
};

export default ProtectedRoute;