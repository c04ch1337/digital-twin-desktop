use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Sqlite};
use uuid::Uuid;
use anyhow::Result;

use crate::core::domain::{
    models::{SensorData, SensorDataId, TwinId, SensorReading},
    traits::repository::{
        SensorDataRepository, RepositoryResult, RepositoryError,
        FilterCriteria, SortCriteria, Pagination, PaginatedResult,
    },
};

pub struct SqliteSensorDataRepository {
    pool: Pool<Sqlite>,
}

impl SqliteSensorDataRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SensorDataRepository for SqliteSensorDataRepository {
    async fn create(&self, sensor_data: SensorData) -> RepositoryResult<SensorData> {
        sqlx::query(
            "INSERT INTO sensor_data (id, twin_id, sensor_type, unit, metadata) 
             VALUES (?, ?, ?, ?, ?)"
        )
        .bind(sensor_data.id.to_string())
        .bind(sensor_data.twin_id.to_string())
        .bind(&sensor_data.sensor_type)
        .bind(&sensor_data.unit)
        .bind(serde_json::to_string(&sensor_data.metadata).unwrap_or("{}".to_string()))
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(sensor_data)
    }

    async fn get_by_id(&self, id: SensorDataId) -> RepositoryResult<SensorData> {
        let sensor = sqlx::query_as!(
            SensorDataRow,
            "SELECT id, twin_id, sensor_type, unit, created_at, metadata 
             FROM sensor_data 
             WHERE id = ?",
            id.to_string()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?
        .ok_or_else(|| RepositoryError::NotFound {
            entity_type: "SensorData".to_string(),
            id: id.to_string(),
        })?;

        Ok(sensor.into())
    }

    async fn update(&self, sensor_data: SensorData) -> RepositoryResult<SensorData> {
        sqlx::query(
            "UPDATE sensor_data 
             SET sensor_type = ?, unit = ?, metadata = ? 
             WHERE id = ?"
        )
        .bind(&sensor_data.sensor_type)
        .bind(&sensor_data.unit)
        .bind(serde_json::to_string(&sensor_data.metadata).unwrap_or("{}".to_string()))
        .bind(sensor_data.id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(sensor_data)
    }

    async fn delete(&self, id: SensorDataId) -> RepositoryResult<()> {
        sqlx::query("DELETE FROM sensor_data WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn get_by_twin_id(
        &self,
        twin_id: TwinId,
        pagination: Pagination,
    ) -> RepositoryResult<PaginatedResult<SensorData>> {
        let sensors = sqlx::query_as!(
            SensorDataRow,
            "SELECT id, twin_id, sensor_type, unit, created_at, metadata 
             FROM sensor_data 
             WHERE twin_id = ? 
             LIMIT ? OFFSET ?",
            twin_id.to_string(),
            pagination.limit as i64,
            pagination.offset as i64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let total = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM sensor_data WHERE twin_id = ?",
            twin_id.to_string()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))? as usize;

        Ok(PaginatedResult {
            items: sensors.into_iter().map(|s| s.into()).collect(),
            total,
            offset: pagination.offset,
            limit: pagination.limit,
        })
    }

    async fn add_reading(
        &self,
        sensor_data_id: SensorDataId,
        reading: SensorReading,
    ) -> RepositoryResult<()> {
        sqlx::query(
            "INSERT INTO sensor_readings (id, sensor_data_id, value, timestamp, metadata) 
             VALUES (?, ?, ?, ?, ?)"
        )
        .bind(Uuid::new_v4().to_string())
        .bind(sensor_data_id.to_string())
        .bind(reading.value)
        .bind(reading.timestamp)
        .bind(serde_json::to_string(&reading.metadata).unwrap_or("{}".to_string()))
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn get_readings_in_range(
        &self,
        sensor_data_id: SensorDataId,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        pagination: Pagination,
    ) -> RepositoryResult<PaginatedResult<SensorReading>> {
        let readings = sqlx::query_as!(
            ReadingRow,
            "SELECT id, sensor_data_id, value, timestamp, metadata 
             FROM sensor_readings 
             WHERE sensor_data_id = ? AND timestamp BETWEEN ? AND ? 
             ORDER BY timestamp 
             LIMIT ? OFFSET ?",
            sensor_data_id.to_string(),
            start,
            end,
            pagination.limit as i64,
            pagination.offset as i64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let total = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM sensor_readings 
             WHERE sensor_data_id = ? AND timestamp BETWEEN ? AND ?",
            sensor_data_id.to_string(),
            start,
            end
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))? as usize;

        Ok(PaginatedResult {
            items: readings.into_iter().map(|r| r.into()).collect(),
            total,
            offset: pagination.offset,
            limit: pagination.limit,
        })
    }

    async fn get_latest_reading(
        &self,
        sensor_data_id: SensorDataId,
    ) -> RepositoryResult<Option<SensorReading>> {
        let reading = sqlx::query_as!(
            ReadingRow,
            "SELECT id, sensor_data_id, value, timestamp, metadata 
             FROM sensor_readings 
             WHERE sensor_data_id = ? 
             ORDER BY timestamp DESC 
             LIMIT 1",
            sensor_data_id.to_string()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(reading.map(|r| r.into()))
    }

    async fn get_aggregated_data(
        &self,
        sensor_data_id: SensorDataId,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        interval: &str,
        aggregation: &str,
    ) -> RepositoryResult<Vec<(DateTime<Utc>, f64)>> {
        let query = format!(
            "SELECT strftime('%Y-%m-%d %H:%M:%f', timestamp) as time_bucket, 
                    {}(value) as agg_value 
             FROM sensor_readings 
             WHERE sensor_data_id = ? AND timestamp BETWEEN ? AND ? 
             GROUP BY time_bucket 
             ORDER BY time_bucket",
            aggregation
        );

        let results = sqlx::query(&query)
            .bind(sensor_data_id.to_string())
            .bind(start)
            .bind(end)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(results
            .into_iter()
            .map(|row| {
                let time_str: String = row.get(0);
                let value: f64 = row.get(1);
                (
                    DateTime::parse_from_str(&time_str, "%Y-%m-%d %H:%M:%f")
                        .unwrap()
                        .with_timezone(&Utc),
                    value,
                )
            })
            .collect())
    }

    async fn cleanup_old_readings(&self, retention_days: u32) -> RepositoryResult<usize> {
        let result = sqlx::query(
            "DELETE FROM sensor_readings 
             WHERE timestamp < datetime('now', ?)"
        )
        .bind(format!("-{} days", retention_days))
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(result.rows_affected() as usize)
    }
}

// Database row structures
#[derive(sqlx::FromRow)]
struct SensorDataRow {
    id: String,
    twin_id: String,
    sensor_type: String,
    unit: Option<String>,
    created_at: DateTime<Utc>,
    metadata: String,
}

#[derive(sqlx::FromRow)]
struct ReadingRow {
    id: String,
    sensor_data_id: String,
    value: f64,
    timestamp: DateTime<Utc>,
    metadata: String,
}

impl From<SensorDataRow> for SensorData {
    fn from(row: SensorDataRow) -> Self {
        Self {
            id: Uuid::parse_str(&row.id).unwrap_or_else(|_| Uuid::new_v4()),
            twin_id: Uuid::parse_str(&row.twin_id).unwrap(),
            sensor_type: row.sensor_type,
            unit: row.unit,
            created_at: row.created_at,
            metadata: serde_json::from_str(&row.metadata).unwrap_or_default(),
        }
    }
}

impl From<ReadingRow> for SensorReading {
    fn from(row: ReadingRow) -> Self {
        Self {
            value: row.value,
            timestamp: row.timestamp,
            metadata: serde_json::from_str(&row.metadata).unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;
    use tempfile::tempdir;

    async fn create_test_db() -> Pool<Sqlite> {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        
        SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(
                sqlx::sqlite::SqliteConnectOptions::new()
                    .filename(db_path)
                    .create_if_missing(true)
            )
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn test_sensor_data_crud() {
        let pool = create_test_db().await;
        let repo = SqliteSensorDataRepository::new(pool);

        // Create test sensor data
        let sensor = SensorData {
            id: Uuid::new_v4(),
            twin_id: Uuid::new_v4(),
            sensor_type: "temperature".to_string(),
            unit: Some("celsius".to_string()),
            created_at: Utc::now(),
            metadata: Default::default(),
        };

        // Test create
        let created = repo.create(sensor.clone()).await.unwrap();
        assert_eq!(created.id, sensor.id);

        // Test get
        let retrieved = repo.get_by_id(sensor.id).await.unwrap();
        assert_eq!(retrieved.sensor_type, sensor.sensor_type);

        // Test add reading
        let reading = SensorReading {
            value: 25.5,
            timestamp: Utc::now(),
            metadata: Default::default(),
        };
        repo.add_reading(sensor.id, reading.clone()).await.unwrap();

        // Test get latest reading
        let latest = repo.get_latest_reading(sensor.id).await.unwrap().unwrap();
        assert_eq!(latest.value, reading.value);

        // Test delete
        repo.delete(sensor.id).await.unwrap();
        assert!(repo.get_by_id(sensor.id).await.is_err());
    }
}