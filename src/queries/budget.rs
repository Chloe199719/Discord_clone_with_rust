use sqlx::PgPool;

#[tracing::instrument(name = "Check Budget exists in DB", skip(pool))]
pub async fn check_budget_exists_db(
    pool: &PgPool,
    budget_id: i32,
    user_id: uuid::Uuid,
) -> Result<bool, sqlx::Error> {
    match sqlx::query!(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM budgets
            WHERE budget_id = $1 AND user_id = $2
        ) AS "exists!";
        "#,
        budget_id,
        user_id
    )
    .fetch_one(pool)
    .await
    {
        Ok(result) => Ok(result.exists),
        Err(e) => Err(e),
    }
}

#[tracing::instrument(name = "Delete Budget in DB", skip(pool))]
pub async fn delete_budget_db(
    pool: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    budget_id: i32,
    user_id: uuid::Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM budgets
        WHERE budget_id = $1 AND user_id = $2
        RETURNING budget_id, category_id;
        "#,
        budget_id,
        user_id
    )
    .fetch_one(pool.as_mut())
    .await?;
    sqlx::query!(
        r#"
        UPDATE categories
        SET budget_id = NULL
        WHERE budget_id = $1 AND user_id = $2;
        "#,
        budget_id,
        user_id
    )
    .execute(pool.as_mut())
    .await?;
    Ok(())
}
