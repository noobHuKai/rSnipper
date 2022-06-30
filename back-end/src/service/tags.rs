use crate::{common::TIME_FORMAT, model::TagsModel};
use anyhow::{Ok, Result};
use chrono::Local;
use sqlx::SqlitePool;
pub struct TagService;

impl TagService {
    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<String>> {
        let rows = sqlx::query_as::<_, TagsModel>("select * from tags order by create_time asc")
            .fetch_all(pool)
            .await?;
        let mut res = Vec::new();
        for row in rows {
            res.push(row.tag);
        }
        Ok(res)
    }

    pub async fn insert(pool: &SqlitePool, tag: String) -> Result<()> {
        if Self::is_exist_by_tag(pool, tag.clone()).await? {
            return Err(anyhow::anyhow!("tag is exist"));
        }
        let mut conn = pool.acquire().await?;
        sqlx::query(
            r#"
            INSERT INTO tags ( tag,create_time )
            VALUES ( ?1 ,?2)
            "#,
        )
        .bind(tag)
        .bind(Local::now().format(TIME_FORMAT).to_string())
        .execute(&mut conn)
        .await?;
        Ok(())
    }

    pub async fn is_exist_by_tag(pool: &SqlitePool, tag: String) -> Result<bool> {
        let res = sqlx::query("select 1 from tags where tag= ?1")
            .bind(tag)
            .fetch_optional(pool)
            .await?;
        match res {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    pub async fn delete(pool: &SqlitePool, tag: String) -> Result<()> {
        let mut conn = pool.acquire().await?;
        sqlx::query(
            r#"
            DELETE FROM tags WHERE tag = ?1
            "#,
        )
        .bind(tag)
        .execute(&mut conn)
        .await?;
        Ok(())
    }
}
