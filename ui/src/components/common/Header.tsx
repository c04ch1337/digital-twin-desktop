import React from 'react';
import { Link } from 'react-router-dom';

const Header: React.FC = () => {
  return (
    <header className="app-header">
      <div className="logo">
        <Link to="/">Digital Twin Desktop</Link>
      </div>
      <nav className="main-nav">
        <ul>
          <li><Link to="/">Dashboard</Link></li>
          <li><Link to="/conversations">Conversations</Link></li>
          <li><Link to="/twins">Digital Twins</Link></li>
          <li><Link to="/simulations">Simulations</Link></li>
          <li><Link to="/settings">Settings</Link></li>
        </ul>
      </nav>
      <div className="header-actions">
        <button className="btn-icon">
          <span className="icon">🔔</span>
        </button>
        <button className="btn-icon">
          <span className="icon">👤</span>
        </button>
      </div>
    </header>
  );
};

export default Header;