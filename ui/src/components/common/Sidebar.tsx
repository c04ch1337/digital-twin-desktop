import React from 'react';
import { NavLink } from 'react-router-dom';

const Sidebar: React.FC = () => {
  return (
    <aside className="sidebar">
      <div className="sidebar-header">
        <h3>Navigation</h3>
      </div>
      <nav className="sidebar-nav">
        <ul>
          <li>
            <NavLink to="/" className={({ isActive }) => isActive ? 'active' : ''}>
              <span className="icon">🏠</span>
              <span className="label">Dashboard</span>
            </NavLink>
          </li>
          <li>
            <NavLink to="/conversations" className={({ isActive }) => isActive ? 'active' : ''}>
              <span className="icon">💬</span>
              <span className="label">Conversations</span>
            </NavLink>
          </li>
          <li>
            <NavLink to="/twins" className={({ isActive }) => isActive ? 'active' : ''}>
              <span className="icon">🔄</span>
              <span className="label">Digital Twins</span>
            </NavLink>
          </li>
          <li>
            <NavLink to="/simulations" className={({ isActive }) => isActive ? 'active' : ''}>
              <span className="icon">📊</span>
              <span className="label">Simulations</span>
            </NavLink>
          </li>
          <li>
            <NavLink to="/settings" className={({ isActive }) => isActive ? 'active' : ''}>
              <span className="icon">⚙️</span>
              <span className="label">Settings</span>
            </NavLink>
          </li>
        </ul>
      </nav>
      <div className="sidebar-footer">
        <div className="version">v0.1.0</div>
      </div>
    </aside>
  );
};

export default Sidebar;