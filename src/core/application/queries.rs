//! Query objects for read operations
//!
//! Queries represent requests to retrieve data from the system
//! without modifying its state.

use crate::core::domain::{
    models::{
        agent::AgentId,
        conversation::ConversationId,
        digital_twin::TwinId,
        tool::{ToolId, ToolCategory},
    },
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Query to get an agent by ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAgentByIdQuery {
    pub agent_id: AgentId,
}

/// Query to get all agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAllAgentsQuery {
    pub include_disabled: bool,
    pub sort_by: Option<AgentSortField>,
    pub sort_order: Option<SortOrder>,
}

/// Query to get a conversation by ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetConversationByIdQuery {
    pub conversation_id: ConversationId,
    pub include_messages: bool,
}

/// Query to get conversations by agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetConversationsByAgentQuery {
    pub agent_id: AgentId,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub include_messages: bool,
}

/// Query to get recent conversations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRecentConversationsQuery {
    pub limit: usize,
    pub include_messages: bool,
}

/// Query to get a digital twin by ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTwinByIdQuery {
    pub twin_id: TwinId,
    pub include_sensor_data: bool,
    pub include_simulation_results: bool,
}

/// Query to get all digital twins
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAllTwinsQuery {
    pub twin_type: Option<String>,
    pub status_filter: Option<Vec<String>>,
    pub tag_filter: Option<Vec<String>>,
    pub sort_by: Option<TwinSortField>,
    pub sort_order: Option<SortOrder>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// Query to get twins by type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTwinsByTypeQuery {
    pub twin_type: String,
    pub include_inactive: bool,
}

/// Query to get sensor data for a twin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSensorDataQuery {
    pub twin_id: TwinId,
    pub sensor_name: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub limit: Option<usize>,
}

/// Query to get simulation results for a twin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSimulationResultsQuery {
    pub twin_id: TwinId,
    pub simulation_type: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

/// Query to get a specific simulation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSimulationResultByIdQuery {
    pub twin_id: TwinId,
    pub simulation_id: String,
}

/// Query to get a tool by ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetToolByIdQuery {
    pub tool_id: ToolId,
}

/// Query to get a tool by name
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetToolByNameQuery {
    pub tool_name: String,
}

/// Query to get all tools
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAllToolsQuery {
    pub category: Option<ToolCategory>,
    pub enabled_only: bool,
    pub sort_by: Option<ToolSortField>,
    pub sort_order: Option<SortOrder>,
}

/// Query to get tools by category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetToolsByCategoryQuery {
    pub category: ToolCategory,
    pub enabled_only: bool,
}

/// Query to search conversations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConversationsQuery {
    pub search_text: String,
    pub agent_id: Option<AgentId>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub limit: usize,
    pub offset: usize,
}

/// Query to search digital twins
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchTwinsQuery {
    pub search_text: String,
    pub twin_type: Option<String>,
    pub tags: Option<Vec<String>>,
    pub property_filters: Option<Vec<PropertyFilter>>,
    pub limit: usize,
    pub offset: usize,
}

/// Query to get agent statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAgentStatisticsQuery {
    pub agent_id: AgentId,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

/// Query to get twin statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTwinStatisticsQuery {
    pub twin_id: TwinId,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

/// Query to get system statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSystemStatisticsQuery {
    pub include_agent_stats: bool,
    pub include_twin_stats: bool,
    pub include_tool_stats: bool,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

/// Query to get tool execution history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetToolExecutionHistoryQuery {
    pub tool_id: Option<ToolId>,
    pub agent_id: Option<AgentId>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub success_only: bool,
    pub limit: usize,
    pub offset: usize,
}

/// Query to check tool availability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckToolAvailabilityQuery {
    pub tool_names: Vec<String>,
}

/// Query to get twins needing sync
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTwinsNeedingSyncQuery {
    pub hours_since_last_sync: u32,
    pub twin_type: Option<String>,
    pub limit: Option<usize>,
}

/// Query to get active simulations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetActiveSimulationsQuery {
    pub twin_id: Option<TwinId>,
    pub simulation_type: Option<String>,
}

/// Sort order enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortOrder {
    Asc,
    Desc,
}

/// Agent sort fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentSortField {
    Name,
    CreatedAt,
    UpdatedAt,
}

/// Twin sort fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TwinSortField {
    Name,
    Type,
    Status,
    CreatedAt,
    UpdatedAt,
    LastSync,
}

/// Tool sort fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolSortField {
    Name,
    Category,
    CreatedAt,
    UpdatedAt,
}

/// Property filter for twin searches
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyFilter {
    pub property_name: String,
    pub operator: FilterOperator,
    pub value: serde_json::Value,
}

/// Filter operators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Contains,
    StartsWith,
    EndsWith,
    In,
    NotIn,
}

/// Pagination parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    pub page: u32,
    pub page_size: u32,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 20,
        }
    }
}

impl PaginationParams {
    pub fn offset(&self) -> usize {
        ((self.page - 1) * self.page_size) as usize
    }

    pub fn limit(&self) -> usize {
        self.page_size as usize
    }
}

/// Query builder for complex queries
pub struct TwinQueryBuilder {
    twin_type: Option<String>,
    status_filter: Option<Vec<String>>,
    tag_filter: Option<Vec<String>>,
    property_filters: Vec<PropertyFilter>,
    sort_by: Option<TwinSortField>,
    sort_order: Option<SortOrder>,
    limit: Option<usize>,
    offset: Option<usize>,
}

impl TwinQueryBuilder {
    pub fn new() -> Self {
        Self {
            twin_type: None,
            status_filter: None,
            tag_filter: None,
            property_filters: Vec::new(),
            sort_by: None,
            sort_order: None,
            limit: None,
            offset: None,
        }
    }

    pub fn with_type(mut self, twin_type: String) -> Self {
        self.twin_type = Some(twin_type);
        self
    }

    pub fn with_status(mut self, status: Vec<String>) -> Self {
        self.status_filter = Some(status);
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tag_filter = Some(tags);
        self
    }

    pub fn with_property_filter(mut self, filter: PropertyFilter) -> Self {
        self.property_filters.push(filter);
        self
    }

    pub fn sort_by(mut self, field: TwinSortField, order: SortOrder) -> Self {
        self.sort_by = Some(field);
        self.sort_order = Some(order);
        self
    }

    pub fn paginate(mut self, limit: usize, offset: usize) -> Self {
        self.limit = Some(limit);
        self.offset = Some(offset);
        self
    }

    pub fn build(self) -> GetAllTwinsQuery {
        GetAllTwinsQuery {
            twin_type: self.twin_type,
            status_filter: self.status_filter,
            tag_filter: self.tag_filter,
            sort_by: self.sort_by,
            sort_order: self.sort_order,
            limit: self.limit,
            offset: self.offset,
        }
    }
}

impl Default for TwinQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}