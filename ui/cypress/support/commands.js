// ***********************************************
// This example commands.js shows you how to
// create various custom commands and overwrite
// existing commands.
//
// For more comprehensive examples of custom
// commands please read more here:
// https://on.cypress.io/custom-commands
// ***********************************************

// -- This is a parent command --
Cypress.Commands.add('login', (username, password) => {
  cy.visit('/login');
  cy.get('[data-cy=username-input]').type(username);
  cy.get('[data-cy=password-input]').type(password);
  cy.get('[data-cy=login-button]').click();
  
  // Wait for login to complete
  cy.url().should('not.include', '/login');
});

// Command to create a digital twin
Cypress.Commands.add('createTwin', (name, description, status = 'active') => {
  // Navigate to twins page
  cy.get('[data-cy=nav-twins]').click();
  
  // Click the create button
  cy.get('[data-cy=create-twin-button]').click();
  
  // Fill the form
  cy.get('[data-cy=twin-name-input]').type(name);
  cy.get('[data-cy=twin-description-input]').type(description);
  cy.get('[data-cy=twin-status-select]').select(status);
  
  // Submit the form
  cy.get('[data-cy=submit-button]').click();
  
  // Wait for success message
  cy.contains('Digital twin created successfully').should('be.visible');
});

// Command to create an agent for a twin
Cypress.Commands.add('createAgent', (twinId, name, description, model = 'gpt-4') => {
  // Navigate to agents page
  cy.get('[data-cy=nav-agents]').click();
  
  // Click the create button
  cy.get('[data-cy=create-agent-button]').click();
  
  // Fill the form
  cy.get('[data-cy=agent-name-input]').type(name);
  cy.get('[data-cy=agent-description-input]').type(description);
  cy.get('[data-cy=agent-twin-select]').select(twinId);
  cy.get('[data-cy=agent-model-select]').select(model);
  
  // Submit the form
  cy.get('[data-cy=submit-button]').click();
  
  // Wait for success message
  cy.contains('Agent created successfully').should('be.visible');
});

// Command to start a conversation
Cypress.Commands.add('startConversation', (twinId, agentId, title) => {
  // Navigate to conversations page
  cy.get('[data-cy=nav-conversations]').click();
  
  // Click the create button
  cy.get('[data-cy=create-conversation-button]').click();
  
  // Fill the form
  cy.get('[data-cy=conversation-title-input]').type(title);
  cy.get('[data-cy=conversation-twin-select]').select(twinId);
  cy.get('[data-cy=conversation-agent-select]').select(agentId);
  
  // Submit the form
  cy.get('[data-cy=submit-button]').click();
  
  // Wait for the conversation to load
  cy.contains(title).should('be.visible');
});

// Command to send a message in a conversation
Cypress.Commands.add('sendMessage', (message) => {
  cy.get('[data-cy=message-input]').type(message);
  cy.get('[data-cy=send-button]').click();
  
  // Wait for the message to appear
  cy.contains(message).should('be.visible');
});

// Command to run a simulation
Cypress.Commands.add('runSimulation', (twinId, duration, parameters = {}) => {
  // Navigate to simulations page
  cy.get('[data-cy=nav-simulations]').click();
  
  // Click the create button
  cy.get('[data-cy=create-simulation-button]').click();
  
  // Fill the form
  cy.get('[data-cy=simulation-twin-select]').select(twinId);
  cy.get('[data-cy=simulation-duration-input]').type(duration);
  
  // Add parameters if provided
  if (Object.keys(parameters).length > 0) {
    cy.get('[data-cy=add-parameter-button]').click();
    
    Object.entries(parameters).forEach(([key, value], index) => {
      cy.get(`[data-cy=parameter-key-${index}]`).type(key);
      cy.get(`[data-cy=parameter-value-${index}]`).type(value);
      
      if (index < Object.keys(parameters).length - 1) {
        cy.get('[data-cy=add-parameter-button]').click();
      }
    });
  }
  
  // Submit the form
  cy.get('[data-cy=submit-button]').click();
  
  // Wait for the simulation to start
  cy.contains('Simulation started').should('be.visible');
});