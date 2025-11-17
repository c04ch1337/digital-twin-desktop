import React from 'react';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import TwinList from '../../../components/twin/TwinList';

// Mock the API context
jest.mock('../../../api', () => ({
  useTwins: () => ({
    twins: [
      { 
        id: '1', 
        name: 'Factory Twin', 
        description: 'Digital twin of a factory',
        status: 'active',
        created_at: '2025-11-17T00:00:00Z',
        updated_at: '2025-11-17T00:00:00Z'
      },
      { 
        id: '2', 
        name: 'Building Twin', 
        description: 'Digital twin of a building',
        status: 'maintenance',
        created_at: '2025-11-17T00:00:00Z',
        updated_at: '2025-11-17T00:00:00Z'
      }
    ],
    loading: false,
    error: null,
    refetch: jest.fn()
  })
}));

describe('TwinList Component', () => {
  it('renders a list of twins', () => {
    render(<TwinList onSelect={jest.fn()} />);
    
    // Check if twin names are rendered
    expect(screen.getByText('Factory Twin')).toBeInTheDocument();
    expect(screen.getByText('Building Twin')).toBeInTheDocument();
    
    // Check if descriptions are rendered
    expect(screen.getByText('Digital twin of a factory')).toBeInTheDocument();
    expect(screen.getByText('Digital twin of a building')).toBeInTheDocument();
    
    // Check if status indicators are rendered
    const statusIndicators = screen.getAllByRole('status');
    expect(statusIndicators).toHaveLength(2);
  });
  
  it('calls onSelect when a twin is clicked', async () => {
    const handleSelect = jest.fn();
    render(<TwinList onSelect={handleSelect} />);
    
    // Click on the first twin
    await userEvent.click(screen.getByText('Factory Twin'));
    
    // Check if onSelect was called with the correct twin ID
    expect(handleSelect).toHaveBeenCalledWith('1');
  });
  
  it('displays a loading state when twins are loading', () => {
    // Override the mock for this test
    jest.spyOn(require('../../../api'), 'useTwins').mockImplementation(() => ({
      twins: [],
      loading: true,
      error: null,
      refetch: jest.fn()
    }));
    
    render(<TwinList onSelect={jest.fn()} />);
    
    // Check if loading indicator is displayed
    expect(screen.getByTestId('loading-spinner')).toBeInTheDocument();
    
    // Restore the original mock
    jest.restoreAllMocks();
  });
  
  it('displays an error message when there is an error', () => {
    // Override the mock for this test
    jest.spyOn(require('../../../api'), 'useTwins').mockImplementation(() => ({
      twins: [],
      loading: false,
      error: 'Failed to load twins',
      refetch: jest.fn()
    }));
    
    render(<TwinList onSelect={jest.fn()} />);
    
    // Check if error message is displayed
    expect(screen.getByText('Failed to load twins')).toBeInTheDocument();
    
    // Restore the original mock
    jest.restoreAllMocks();
  });
  
  it('displays an empty state when there are no twins', () => {
    // Override the mock for this test
    jest.spyOn(require('../../../api'), 'useTwins').mockImplementation(() => ({
      twins: [],
      loading: false,
      error: null,
      refetch: jest.fn()
    }));
    
    render(<TwinList onSelect={jest.fn()} />);
    
    // Check if empty state message is displayed
    expect(screen.getByText('No digital twins found')).toBeInTheDocument();
    
    // Restore the original mock
    jest.restoreAllMocks();
  });
});