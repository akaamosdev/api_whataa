
pub async fn get_all_users(State(pool): State<SqlitePool>) -> Result<Json<Vec<User>>, AppError> {
    let users = sqlx::query_as::<_, User>("SELECT id, name, role_id, boutique_id, email, password_hash, created_at FROM users")
        .fetch_all(&pool)
        .await
        .map_err(AppError::from)?;

    Ok(Json(users))
}