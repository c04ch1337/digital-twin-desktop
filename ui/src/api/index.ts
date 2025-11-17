import { invoke } from '@tauri-apps/api/core';

// Twin API
export const getTwins = async () => {
  return invoke<any[]>('list_digital_twins');
};

export const getTwin = async (twinId: string) => {
  return invoke<any>('get_digital_twin', { twinId });
};

export const createTwin = async (name: string, twinType: string, configuration: any) => {
  return invoke<any>('create_digital_twin', { name, twinType, configuration });
};

export const updateTwin = async (twinId: string, name: string, twinType: string, configuration: any) => {
  return invoke<any>('update_digital_twin', { twinId, name, twinType, configuration });
};

export const deleteTwin = async (twinId: string) => {
  return invoke<void>('delete_digital_twin', { twinId });
};

export const getTwinSensorData = async (twinId: string) => {
  return invoke<any[]>('get_twin_sensor_data', { twinId });
};

// Conversation API
export const getConversations = async () => {
  return invoke<any[]>('list_conversations');
};

export const getConversation = async (conversationId: string) => {
  return invoke<any>('get_conversation', { conversationId });
};

export const createConversation = async (title: string) => {
  return invoke<any>('create_conversation', { title });
};

export const updateConversationTitle = async (conversationId: string, title: string) => {
  return invoke<void>('update_conversation_title', { conversationId, title });
};

export const deleteConversation = async (conversationId: string) => {
  return invoke<void>('delete_conversation', { conversationId });
};

export const getConversationMessages = async (conversationId: string) => {
  return invoke<any[]>('get_conversation_messages', { conversationId });
};

export const sendMessage = async (conversationId: string, content: string, agentId?: string) => {
  return invoke<any>('send_message', { conversationId, content, agentId });
};

// Agent API
export const getAgents = async () => {
  return invoke<any[]>('list_agents');
};

export const getAgent = async (agentId: string) => {
  return invoke<any>('get_agent', { agentId });
};

export const setConversationAgent = async (conversationId: string, agentId: string) => {
  return invoke<void>('set_conversation_agent', { conversationId, agentId });
};

// Simulation API
export const getSimulations = async () => {
  return invoke<any[]>('list_simulations');
};

export const getSimulation = async (simulationId: string) => {
  return invoke<any>('get_simulation', { simulationId });
};

export const startSimulation = async (config: any) => {
  return invoke<any>('start_simulation', { config });
};

export const stopSimulation = async (twinId: string) => {
  return invoke<void>('stop_simulation', { twinId });
};

export const getSimulationResults = async (simulationId: string) => {
  return invoke<any[]>('get_simulation_results', { simulationId });
};

export const getActiveSimulation = async (twinId: string) => {
  return invoke<any>('get_active_simulation', { twinId });
};

// Settings API
export const getAppSettings = async () => {
  return invoke<any>('get_app_settings');
};

export const saveAppSettings = async (settings: any) => {
  return invoke<void>('save_app_settings', { settings });
};

export const resetAppSettings = async () => {
  return invoke<void>('reset_app_settings');
};

// App initialization
export const initializeApp = async () => {
  return invoke<void>('initialize_app');
};