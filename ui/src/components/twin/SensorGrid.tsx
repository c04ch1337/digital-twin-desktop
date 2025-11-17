import React, { useState } from 'react';
import LoadingSpinner from '../common/LoadingSpinner';

interface SensorData {
  id: string;
  twin_id: string;
  sensor_id: string;
  sensor_name: string;
  value: number | string | boolean;
  unit?: string;
  timestamp: string;
}

interface SensorGridProps {
  sensorData: SensorData[];
  isLoading?: boolean;
}

const SensorGrid: React.FC<SensorGridProps> = ({
  sensorData,
  isLoading = false
}) => {
  const [sortField, setSortField] = useState<keyof SensorData>('sensor_name');
  const [sortDirection, setSortDirection] = useState<'asc' | 'desc'>('asc');
  const [filterText, setFilterText] = useState<string>('');

  const handleSort = (field: keyof SensorData) => {
    if (field === sortField) {
      setSortDirection(sortDirection === 'asc' ? 'desc' : 'asc');
    } else {
      setSortField(field);
      setSortDirection('asc');
    }
  };

  const formatValue = (data: SensorData): string => {
    if (typeof data.value === 'boolean') {
      return data.value ? 'True' : 'False';
    }
    
    if (data.unit) {
      return `${data.value} ${data.unit}`;
    }
    
    return String(data.value);
  };

  const formatTimestamp = (timestamp: string): string => {
    return new Date(timestamp).toLocaleString();
  };

  const getSensorStatusClass = (data: SensorData): string => {
    // This is a placeholder for determining sensor status
    // In a real application, you would have logic to determine if a sensor value
    // is within normal range, warning range, or critical range
    
    if (typeof data.value === 'number') {
      // Example logic for numeric values
      if (data.value > 90) return 'status-critical';
      if (data.value > 70) return 'status-warning';
      return 'status-normal';
    }
    
    if (typeof data.value === 'boolean') {
      // Example logic for boolean values
      return data.value ? 'status-normal' : 'status-warning';
    }
    
    return 'status-normal';
  };

  const filteredData = sensorData.filter(data => 
    data.sensor_name.toLowerCase().includes(filterText.toLowerCase()) ||
    String(data.value).toLowerCase().includes(filterText.toLowerCase()) ||
    (data.unit && data.unit.toLowerCase().includes(filterText.toLowerCase()))
  );

  const sortedData = [...filteredData].sort((a, b) => {
    const aValue = a[sortField];
    const bValue = b[sortField];
    
    if (aValue === bValue) return 0;
    
    // Handle undefined values and type comparison safely
    if (aValue === undefined) return sortDirection === 'asc' ? -1 : 1;
    if (bValue === undefined) return sortDirection === 'asc' ? 1 : -1;
    
    const comparison = String(aValue) < String(bValue) ? -1 : 1;
    return sortDirection === 'asc' ? comparison : -comparison;
  });

  if (isLoading && sensorData.length === 0) {
    return <LoadingSpinner size="medium" message="Loading sensor data..." />;
  }

  return (
    <div className="sensor-grid">
      <div className="sensor-grid-header">
        <h3>Sensor Data</h3>
        <div className="sensor-grid-controls">
          <input
            type="text"
            placeholder="Filter sensors..."
            value={filterText}
            onChange={(e) => setFilterText(e.target.value)}
            className="sensor-filter-input"
          />
        </div>
      </div>
      
      {sortedData.length === 0 ? (
        <p className="no-sensors-message">
          {filterText 
            ? 'No sensors match your filter criteria.' 
            : 'No sensor data available.'}
        </p>
      ) : (
        <div className="sensor-table-container">
          <table className="sensor-table">
            <thead>
              <tr>
                <th onClick={() => handleSort('sensor_name')}>
                  Sensor Name
                  {sortField === 'sensor_name' && (
                    <span className="sort-indicator">
                      {sortDirection === 'asc' ? ' ▲' : ' ▼'}
                    </span>
                  )}
                </th>
                <th onClick={() => handleSort('value')}>
                  Value
                  {sortField === 'value' && (
                    <span className="sort-indicator">
                      {sortDirection === 'asc' ? ' ▲' : ' ▼'}
                    </span>
                  )}
                </th>
                <th onClick={() => handleSort('timestamp')}>
                  Timestamp
                  {sortField === 'timestamp' && (
                    <span className="sort-indicator">
                      {sortDirection === 'asc' ? ' ▲' : ' ▼'}
                    </span>
                  )}
                </th>
                <th>Status</th>
              </tr>
            </thead>
            <tbody>
              {sortedData.map((data) => (
                <tr key={data.id} className={getSensorStatusClass(data)}>
                  <td>{data.sensor_name}</td>
                  <td>{formatValue(data)}</td>
                  <td>{formatTimestamp(data.timestamp)}</td>
                  <td>
                    <div className={`status-indicator ${getSensorStatusClass(data)}`}>
                      <span className="status-dot"></span>
                    </div>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
};

export default SensorGrid;