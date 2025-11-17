import { useState, useEffect } from 'react';
import { getTwin, getTwinSensorData } from '../api';

export const useTwin = (twinId: string | undefined) => {
  const [twin, setTwin] = useState<any | null>(null);
  const [sensorData, setSensorData] = useState<any[]>([]);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (twinId) {
      loadTwin();
    } else {
      setTwin(null);
      setSensorData([]);
      setIsLoading(false);
    }
  }, [twinId]);

  const loadTwin = async () => {
    if (!twinId) return;
    
    setIsLoading(true);
    setError(null);
    
    try {
      const twinData = await getTwin(twinId);
      setTwin(twinData);
      
      await loadSensorData();
    } catch (err) {
      console.error('Failed to load twin:', err);
      setError('Failed to load twin data. Please try again.');
    } finally {
      setIsLoading(false);
    }
  };

  const loadSensorData = async () => {
    if (!twinId) return;
    
    try {
      const data = await getTwinSensorData(twinId);
      setSensorData(data);
    } catch (err) {
      console.error('Failed to load sensor data:', err);
      // Don't set error state here to avoid disrupting the main view
    }
  };

  return {
    twin,
    sensorData,
    isLoading,
    error,
    loadTwin,
    loadSensorData
  };
};