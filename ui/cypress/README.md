# Cypress E2E Tests

This directory contains end-to-end tests for the Digital Twin Desktop application using Cypress.

## Structure

- `e2e/` - End-to-end test files
- `fixtures/` - Test data
- `support/` - Support files and custom commands
- `plugins/` - Cypress plugins

## Running Tests

To run the Cypress tests, use the following command:

```bash
cd digital-twin-desktop/ui
npm run cypress:open  # Opens the Cypress Test Runner
npm run cypress:run   # Runs tests headlessly
```

## Writing Tests

Tests are written using Cypress. Each test file should focus on a specific feature or workflow.

Example:

```javascript
// cypress/e2e/twin-management.cy.js
describe('Twin Management', () => {
  beforeEach(() => {
    // Visit the application
    cy.visit('/');
    
    // Log in if needed
    cy.login('test-user', 'password');
  });

  it('should create a new digital twin', () => {
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
    
    // Verify the twin was created
    cy.contains('Test Twin').should('be.visible');
  });

  it('should edit an existing digital twin', () => {
    // Navigate to twins page
    cy.get('[data-cy=nav-twins]').click();
    
    // Find and click on a twin
    cy.contains('Test Twin').click();
    
    // Click edit button
    cy.get('[data-cy=edit-twin-button]').click();
    
    // Update the description
    cy.get('[data-cy=twin-description-input]')
      .clear()
      .type('Updated description');
    
    // Save changes
    cy.get('[data-cy=submit-button]').click();
    
    // Verify the changes
    cy.contains('Updated description').should('be.visible');
  });
});