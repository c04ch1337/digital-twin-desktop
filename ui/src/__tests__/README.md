# React Component Tests

This directory contains tests for React components in the Digital Twin Desktop application.

## Structure

- `components/` - Tests for individual React components
- `pages/` - Tests for page components
- `hooks/` - Tests for custom React hooks
- `utils/` - Tests for utility functions
- `context/` - Tests for React context providers

## Running Tests

To run the tests, use the following command:

```bash
cd digital-twin-desktop/ui
npm test
```

## Writing Tests

Tests are written using Jest and React Testing Library. Each test file should be named after the component it tests, with a `.test.tsx` extension.

Example:

```tsx
// __tests__/components/TwinList.test.tsx
import React from 'react';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import TwinList from '../../components/twin/TwinList';

describe('TwinList', () => {
  it('renders a list of twins', () => {
    const twins = [
      { id: '1', name: 'Twin 1', status: 'active' },
      { id: '2', name: 'Twin 2', status: 'maintenance' },
    ];
    
    render(<TwinList twins={twins} />);
    
    expect(screen.getByText('Twin 1')).toBeInTheDocument();
    expect(screen.getByText('Twin 2')).toBeInTheDocument();
  });
  
  it('calls onSelect when a twin is clicked', async () => {
    const twins = [{ id: '1', name: 'Twin 1', status: 'active' }];
    const onSelect = jest.fn();
    
    render(<TwinList twins={twins} onSelect={onSelect} />);
    
    await userEvent.click(screen.getByText('Twin 1'));
    
    expect(onSelect).toHaveBeenCalledWith('1');
  });
});