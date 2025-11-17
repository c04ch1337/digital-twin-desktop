import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import LoadingSpinner from '../common/LoadingSpinner';

interface DigitalTwin {
  id?: string;
  name: string;
  twin_type: string;
  configuration: any;
  created_at?: string;
  updated_at?: string;
}

interface TwinEditorProps {
  twinId?: string; // If provided, edit existing twin; if not, create new twin
  onSave?: (twin: DigitalTwin) => void;
  onCancel?: () => void;
}

const TwinEditor: React.FC<TwinEditorProps> = ({
  twinId,
  onSave,
  onCancel
}) => {
  const [twin, setTwin] = useState<DigitalTwin>({
    name: '',
    twin_type: 'industrial',
    configuration: {}
  });
  
  const [configJson, setConfigJson] = useState<string>('{}');
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [isSaving, setIsSaving] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const [jsonError, setJsonError] = useState<string | null>(null);

  const isEditMode = !!twinId;

  useEffect(() => {
    if (isEditMode) {
      loadTwin();
    }
  }, [twinId]);

  const loadTwin = async () => {
    if (!twinId) return;
    
    setIsLoading(true);
    setError(null);
    
    try {
      const twinData = await invoke<DigitalTwin>('get_digital_twin', {
        twinId
      });
      setTwin(twinData);
      setConfigJson(JSON.stringify(twinData.configuration, null, 2));
    } catch (err) {
      console.error('Failed to load twin:', err);
      setError('Failed to load twin data. Please try again.');
    } finally {
      setIsLoading(false);
    }
  };

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    const { name, value } = e.target;
    setTwin(prev => ({
      ...prev,
      [name]: value
    }));
  };

  const handleConfigChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    setConfigJson(e.target.value);
    setJsonError(null);
    
    try {
      const parsedConfig = JSON.parse(e.target.value);
      setTwin(prev => ({
        ...prev,
        configuration: parsedConfig
      }));
    } catch (err) {
      setJsonError('Invalid JSON configuration');
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (jsonError) {
      return;
    }
    
    setIsSaving(true);
    setError(null);
    
    try {
      let savedTwin;
      
      if (isEditMode) {
        savedTwin = await invoke<DigitalTwin>('update_digital_twin', {
          twinId,
          name: twin.name,
          twinType: twin.twin_type,
          configuration: twin.configuration
        });
      } else {
        savedTwin = await invoke<DigitalTwin>('create_digital_twin', {
          name: twin.name,
          twinType: twin.twin_type,
          configuration: twin.configuration
        });
      }
      
      if (onSave) {
        onSave(savedTwin);
      }
    } catch (err) {
      console.error('Failed to save twin:', err);
      setError(`Failed to ${isEditMode ? 'update' : 'create'} digital twin. Please try again.`);
    } finally {
      setIsSaving(false);
    }
  };

  const handleCancel = () => {
    if (onCancel) {
      onCancel();
    }
  };

  if (isLoading) {
    return <LoadingSpinner size="medium" message="Loading twin data..." />;
  }

  return (
    <div className="twin-editor">
      <h2>{isEditMode ? 'Edit Digital Twin' : 'Create Digital Twin'}</h2>
      
      {error && (
        <div className="error-message">
          <p>{error}</p>
        </div>
      )}
      
      <form onSubmit={handleSubmit} className="twin-form">
        <div className="form-group">
          <label htmlFor="name">Twin Name</label>
          <input
            type="text"
            id="name"
            name="name"
            value={twin.name}
            onChange={handleInputChange}
            required
            placeholder="Enter twin name"
          />
        </div>
        
        <div className="form-group">
          <label htmlFor="twin_type">Twin Type</label>
          <select
            id="twin_type"
            name="twin_type"
            value={twin.twin_type}
            onChange={handleInputChange}
            required
          >
            <option value="industrial">Industrial</option>
            <option value="infrastructure">Infrastructure</option>
            <option value="process">Process</option>
            <option value="custom">Custom</option>
          </select>
        </div>
        
        <div className="form-group">
          <label htmlFor="configuration">
            Configuration (JSON)
            {jsonError && <span className="json-error"> - {jsonError}</span>}
          </label>
          <textarea
            id="configuration"
            name="configuration"
            value={configJson}
            onChange={handleConfigChange}
            rows={10}
            className={jsonError ? 'json-error-input' : ''}
            placeholder="Enter JSON configuration"
          />
        </div>
        
        <div className="form-actions">
          <button
            type="button"
            onClick={handleCancel}
            className="btn btn-secondary"
            disabled={isSaving}
          >
            Cancel
          </button>
          <button
            type="submit"
            className="btn btn-primary"
            disabled={isSaving || !!jsonError}
          >
            {isSaving ? (
              <>
                <span className="spinner-icon">⏳</span>
                {isEditMode ? 'Updating...' : 'Creating...'}
              </>
            ) : (
              isEditMode ? 'Update Twin' : 'Create Twin'
            )}
          </button>
        </div>
      </form>
    </div>
  );
};

export default TwinEditor;