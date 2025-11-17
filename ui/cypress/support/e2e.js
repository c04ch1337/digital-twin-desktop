// ***********************************************************
// This example support/e2e.js is processed and
// loaded automatically before your test files.
//
// This is a great place to put global configuration and
// behavior that modifies Cypress.
//
// You can change the location of this file or turn off
// automatically serving support files with the
// 'supportFile' configuration option.
//
// You can read more here:
// https://on.cypress.io/configuration
// ***********************************************************

// Import commands.js using ES2015 syntax:
import './commands';

// Alternatively you can use CommonJS syntax:
// require('./commands')

// Prevent uncaught exceptions from failing tests
Cypress.on('uncaught:exception', (err, runnable) => {
  // returning false here prevents Cypress from
  // failing the test
  return false;
});

// Log all API requests during tests
Cypress.on('log:added', (log) => {
  if (log.displayName === 'xhr' || log.displayName === 'fetch') {
    console.log(`${log.displayName} to ${log.url}`);
  }
});

// Add better error messages for assertions
Cypress.Commands.overwrite('should', (originalFn, subject, expectation, ...args) => {
  const customMatchers = {
    'be.visible': () => `Expected element to be visible`,
    'exist': () => `Expected element to exist`,
    'have.text': (text) => `Expected element to have text: "${text}"`,
    'contain': (text) => `Expected element to contain: "${text}"`,
    'have.class': (className) => `Expected element to have class: "${className}"`,
    'have.attr': (attr, value) => `Expected element to have attribute "${attr}" with value "${value}"`,
  };

  const origMsg = Cypress._.get(args, '0');
  const customMsg = customMatchers[expectation]
    ? customMatchers[expectation](...args)
    : origMsg;

  return originalFn(subject, expectation, ...args, customMsg);
});