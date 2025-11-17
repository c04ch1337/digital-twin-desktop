import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.css";

// Initialize error handling for the entire application
window.addEventListener('error', (event) => {
  console.error('Global error caught:', event.error);
  // You could send this to a logging service or display a user-friendly error message
});

// Initialize unhandled promise rejection handling
window.addEventListener('unhandledrejection', (event) => {
  console.error('Unhandled promise rejection:', event.reason);
  // You could send this to a logging service or display a user-friendly error message
});

// Create root and render app
ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);