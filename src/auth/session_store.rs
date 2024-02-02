use bb8_redis::{redis::AsyncCommands, RedisConnectionManager};
use time::OffsetDateTime;
use tower_sessions::{
    session::{Id, Record},
    session_store, SessionStore,
};

#[derive(Debug, Clone)]
pub struct RedisStore {
    client: bb8::Pool<RedisConnectionManager>,
}

impl RedisStore {
    pub fn new(client: bb8::Pool<RedisConnectionManager>) -> Self {
        Self { client }
    }
}

#[derive(Debug)]
pub enum RedisStoreError {
    Redis(bb8_redis::redis::RedisError),
    Runtime(bb8_redis::bb8::RunError<bb8_redis::redis::RedisError>),
    Encode(rmp_serde::encode::Error),
    Decode(rmp_serde::decode::Error),
}

impl From<RedisStoreError> for session_store::Error {
    fn from(err: RedisStoreError) -> Self {
        match err {
            RedisStoreError::Redis(inner) => session_store::Error::Backend(inner.to_string()),
            RedisStoreError::Runtime(inner) => session_store::Error::Backend(inner.to_string()),
            RedisStoreError::Decode(inner) => session_store::Error::Decode(inner.to_string()),
            RedisStoreError::Encode(inner) => session_store::Error::Encode(inner.to_string()),
        }
    }
}

#[async_trait::async_trait]
impl SessionStore for RedisStore {
    async fn save(&self, record: &Record) -> session_store::Result<()> {
        let mut conn = match self.client.get().await {
            Ok(conn) => conn,
            Err(err) => return Err(RedisStoreError::Runtime(err).into()),
        };
        let key = record.id.to_string();
        let expire_seconds: i64 = OffsetDateTime::unix_timestamp(record.expiry_date);

        conn.set(
            key.clone(),
            rmp_serde::to_vec(&record)
                .map_err(RedisStoreError::Encode)?
                .as_slice(),
        )
        .await
        .map_err(RedisStoreError::Redis)?;

        conn.expire(key, expire_seconds)
            .await
            .map_err(RedisStoreError::Redis)?;
        Ok(())
    }

    async fn load(&self, session_id: &Id) -> session_store::Result<Option<Record>> {
        let mut conn = match self.client.get().await {
            Ok(conn) => conn,
            Err(err) => return Err(RedisStoreError::Runtime(err).into()),
        };

        let data = conn
            .get::<_, Option<Vec<u8>>>(session_id.to_string())
            .await
            .map_err(RedisStoreError::Redis)?;

        match data {
            Some(data) => {
                let record: Record =
                    rmp_serde::from_slice(&data).map_err(RedisStoreError::Decode)?;
                Ok(Some(record))
            }
            None => Ok(None),
        }
    }

    async fn delete(&self, session_id: &Id) -> session_store::Result<()> {
        let mut conn = match self.client.get().await {
            Ok(conn) => conn,
            Err(err) => return Err(RedisStoreError::Runtime(err).into()),
        };
        conn.del(session_id.to_string())
            .await
            .map_err(RedisStoreError::Redis)?;
        Ok(())
    }
}
