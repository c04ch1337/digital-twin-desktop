import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Link } from 'react-router-dom';
import LoadingSpinner from '../common/LoadingSpinner';

interface DigitalTwin {
  id: string;
  name: string;
  twin_type: string;
  configuration: any;
  created_at: string;
  updated_at: string;
}

interface TwinListProps {
  onSelectTwin?: (twinId: string) => void;
  selectedTwinId?: string;
}

const TwinList: React.FC<TwinListProps> = ({
  onSelectTwin,
  selectedTwinId
}) => {
  const [twins, setTwins] = useState<DigitalTwin[]>([]);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [searchQuery, setSearchQuery] = useState<string>('');
  const [filterType, setFilterType] = useState<string>('all');

  useEffect(() => {
    loadTwins();
  }, []);

  const loadTwins = async () => {
    setIsLoading(true);
    setError(null);
    
    try {
      const twinsList = await invoke<DigitalTwin[]>('list_digital_twins');
      setTwins(twinsList);
    } catch (err) {
      console.error('Failed to load digital twins:', err);
      setError('Failed to load digital twins. Please try again.');
    } finally {
      setIsLoading(false);
    }
  };

  const handleSelectTwin = (twinId: string) => {
    if (onSelectTwin) {
      onSelectTwin(twinId);
    }
  };

  const filteredTwins = twins.filter(twin => {
    const matchesSearch = twin.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
                          twin.id.toLowerCase().includes(searchQuery.toLowerCase());
    
    const matchesType = filterType === 'all' || twin.twin_type === filterType;
    
    return matchesSearch && matchesType;
  });

  const twinTypes = Array.from(new Set(twins.map(twin => twin.twin_type)));

  if (isLoading && twins.length === 0) {
    return <LoadingSpinner size="medium" message="Loading digital twins..." />;
  }

  return (
    <div className="twin-list">
      <div className="twin-list-header">
        <h3>Digital Twins</h3>
        <div className="twin-list-actions">
          <button onClick={loadTwins} className="refresh-button">
            <span className="refresh-icon">🔄</span>
          </button>
          <Link to="/twins/new" className="new-twin-button">
            <span className="new-icon">+</span> New Twin
          </Link>
        </div>
      </div>
      
      <div className="twin-list-filters">
        <input
          type="text"
          placeholder="Search twins..."
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          className="twin-search-input"
        />
        
        <select 
          value={filterType} 
          onChange={(e) => setFilterType(e.target.value)}
          className="twin-type-filter"
        >
          <option value="all">All Types</option>
          {twinTypes.map(type => (
            <option key={type} value={type}>{type}</option>
          ))}
        </select>
      </div>
      
      {error && (
        <div className="error-message">
          <p>{error}</p>
          <button onClick={loadTwins} className="btn btn-primary">
            Retry
          </button>
        </div>
      )}
      
      <div className="twins-grid">
        {filteredTwins.length === 0 ? (
          <p className="no-twins-message">
            {searchQuery || filterType !== 'all' 
              ? 'No digital twins match your search criteria.' 
              : 'No digital twins available. Create your first one!'}
          </p>
        ) : (
          filteredTwins.map((twin) => (
            <div 
              key={twin.id}
              className={`twin-card ${selectedTwinId === twin.id ? 'selected' : ''}`}
              onClick={() => handleSelectTwin(twin.id)}
            >
              <div className="twin-card-header">
                <h4 className="twin-name">{twin.name}</h4>
                <span className="twin-type-badge">{twin.twin_type}</span>
              </div>
              <div className="twin-card-body">
                <div className="twin-meta">
                  <p className="twin-id">ID: {twin.id}</p>
                  <p className="twin-created">
                    Created: {new Date(twin.created_at).toLocaleDateString()}
                  </p>
                </div>
              </div>
              <div className="twin-card-footer">
                <Link to={`/twins/${twin.id}`} className="view-twin-link">
                  View Details
                </Link>
              </div>
            </div>
          ))
        )}
      </div>
    </div>
  );
};

export default TwinList;