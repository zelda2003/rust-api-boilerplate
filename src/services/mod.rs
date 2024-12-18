use crate::models::{CreateUserRequest, User};
use log::{error, info, warn};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug)]
pub enum ServiceError {
    DatabaseError(String),
    ValidationError(String),
}

impl From<sqlx::Error> for ServiceError {
    fn from(err: sqlx::Error) -> Self {
        ServiceError::DatabaseError(err.to_string())
    }
}

/// Creates a new user in the database.
/// Returns the created user on success or a ServiceError on failure.
pub async fn create_user(
    db_pool: &PgPool,
    user_data: CreateUserRequest,
) -> Result<User, ServiceError> {
    // Log validation checks
    if user_data.name.trim().is_empty() || user_data.email.trim().is_empty() {
        warn!("⚠️ Validation failed: Name or email is empty");
        return Err(ServiceError::ValidationError(
            "Name and email cannot be empty".to_string(),
        ));
    }

    info!(
        "📝 Attempting to create user with name: {} and email: {}",
        user_data.name, user_data.email
    );

    // Execute database query
    match sqlx::query_as!(
        User,
        "INSERT INTO users (id, name, email) VALUES ($1, $2, $3) RETURNING id, name, email",
        Uuid::new_v4(),
        user_data.name,
        user_data.email
    )
    .fetch_one(db_pool)
    .await
    {
        Ok(user) => {
            info!("✅ User created successfully: {:?}", user);
            Ok(user)
        }
        Err(err) => {
            error!("❌ Database error while creating user: {}", err);
            Err(ServiceError::from(err))
        }
    }
}

/// Retrieves all users from the database.
/// Returns a list of users or a ServiceError on failure.
pub async fn get_all_users(db_pool: &PgPool) -> Result<Vec<User>, ServiceError> {
    info!("📥 Fetching all users from the database...");

    // Execute database query
    match sqlx::query_as!(User, "SELECT id, name, email FROM users")
        .fetch_all(db_pool)
        .await
    {
        Ok(users) => {
            info!("✅ Successfully fetched {} users", users.len());
            Ok(users)
        }
        Err(err) => {
            error!("❌ Database error while fetching users: {}", err);
            Err(ServiceError::from(err))
        }
    }
}
