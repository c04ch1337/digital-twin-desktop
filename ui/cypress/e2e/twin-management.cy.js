/// <reference types="cypress" />

describe('Twin Management', () => {
  beforeEach(() => {
    // Visit the application
    cy.visit('/');
    
    // Mock API responses
    cy.intercept('GET', '/api/twins', {
      statusCode: 200,
      body: [
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
      ]
    }).as('getTwins');
  });

  it('should display a list of digital twins', () => {
    // Navigate to twins page
    cy.get('[data-cy=nav-twins]').click();
    
    // Wait for API response
    cy.wait('@getTwins');
    
    // Verify twins are displayed
    cy.contains('Factory Twin').should('be.visible');
    cy.contains('Digital twin of a factory').should('be.visible');
    cy.contains('Building Twin').should('be.visible');
    cy.contains('Digital twin of a building').should('be.visible');
  });

  it('should create a new digital twin', () => {
    // Mock the create twin API
    cy.intercept('POST', '/api/twins', {
      statusCode: 201,
      body: {
        id: '3',
        name: 'Test Twin',
        description: 'A test digital twin',
        status: 'active',
        created_at: '2025-11-17T00:00:00Z',
        updated_at: '2025-11-17T00:00:00Z'
      }
    }).as('createTwin');
    
    // Navigate to twins page
    cy.get('[data-cy=nav-twins]').click();
    
    // Click the create button
    cy.get('[data-cy=create-twin-button]').click();
    
    // Fill the form
    cy.get('[data-cy=twin-name-input]').type('Test Twin');
    cy.get('[data-cy=twin-description-input]').type('A test digital twin');
    cy.get('[data-cy=twin-status-select]').select('active');
    
    // Submit the form
    cy.get('[data-cy=submit-button]').click();
    
    // Wait for API response
    cy.wait('@createTwin');
    
    // Verify success message
    cy.contains('Digital twin created successfully').should('be.visible');
    
    // Verify the twin appears in the list (mock the updated list)
    cy.intercept('GET', '/api/twins', {
      statusCode: 200,
      body: [
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
        },
        {
          id: '3',
          name: 'Test Twin',
          description: 'A test digital twin',
          status: 'active',
          created_at: '2025-11-17T00:00:00Z',
          updated_at: '2025-11-17T00:00:00Z'
        }
      ]
    }).as('getUpdatedTwins');
    
    // Refresh the list
    cy.get('[data-cy=refresh-button]').click();
    cy.wait('@getUpdatedTwins');
    
    // Verify the new twin is in the list
    cy.contains('Test Twin').should('be.visible');
  });

  it('should view twin details', () => {
    // Mock the twin details API
    cy.intercept('GET', '/api/twins/1', {
      statusCode: 200,
      body: {
        id: '1',
        name: 'Factory Twin',
        description: 'Digital twin of a factory',
        status: 'active',
        created_at: '2025-11-17T00:00:00Z',
        updated_at: '2025-11-17T00:00:00Z',
        metadata: {
          location: 'Building A',
          sensors: 24,
          type: 'manufacturing'
        }
      }
    }).as('getTwinDetails');
    
    // Navigate to twins page
    cy.get('[data-cy=nav-twins]').click();
    
    // Wait for the twins list to load
    cy.wait('@getTwins');
    
    // Click on a twin
    cy.contains('Factory Twin').click();
    
    // Wait for details to load
    cy.wait('@getTwinDetails');
    
    // Verify details are displayed
    cy.contains('Factory Twin').should('be.visible');
    cy.contains('Digital twin of a factory').should('be.visible');
    cy.contains('Status: active').should('be.visible');
    cy.contains('Location: Building A').should('be.visible');
    cy.contains('Sensors: 24').should('be.visible');
    cy.contains('Type: manufacturing').should('be.visible');
  });

  it('should update twin status', () => {
    // Mock the update status API
    cy.intercept('PATCH', '/api/twins/1/status', {
      statusCode: 200,
      body: {
        id: '1',
        name: 'Factory Twin',
        description: 'Digital twin of a factory',
        status: 'maintenance',
        created_at: '2025-11-17T00:00:00Z',
        updated_at: '2025-11-17T00:00:00Z'
      }
    }).as('updateStatus');
    
    // Navigate to twins page
    cy.get('[data-cy=nav-twins]').click();
    
    // Wait for the twins list to load
    cy.wait('@getTwins');
    
    // Click on a twin
    cy.contains('Factory Twin').click();
    
    // Click on status dropdown
    cy.get('[data-cy=status-dropdown]').click();
    
    // Select maintenance status
    cy.get('[data-cy=status-maintenance]').click();
    
    // Wait for update to complete
    cy.wait('@updateStatus');
    
    // Verify status was updated
    cy.contains('Status: maintenance').should('be.visible');
    cy.contains('Status updated successfully').should('be.visible');
  });
});