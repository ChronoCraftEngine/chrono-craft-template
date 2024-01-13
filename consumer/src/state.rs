use crate::{TemplateRepository, TemplateStateCache, GROUP_NAME, STREAM_NAME};
use anyhow::Result;
use chrono_craft_engine::repository::Repository;
use chrono_craft_engine::Stream;
use eventstore::Client;
use redis::Client as Redis;

pub async fn cache_state(redis_client: Redis, event_store_db: Client) -> Result<()> {
    let dto_redis = TemplateStateCache::new(redis_client.clone());

    let repo_state = TemplateRepository::new(event_store_db, dto_redis.clone());

    let stream = Stream::Stream(STREAM_NAME);
    repo_state.cache_dto(&stream, GROUP_NAME).await?;

    Ok(())
}
