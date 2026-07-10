//! Auth API DTOs for optional `security` plugin routes (`/api/auth/tokens`).

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Auth role
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Viewer,
    Operator,
    Admin,
}

/// Auth token record
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthRecord {
    pub id: String,
    pub name: String,
    pub token_hash: String,
    pub token_prefix: String,
    pub role: UserRole,
    pub created_at: u64,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateTokenRequest {
    pub name: String,
    pub role: UserRole,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateTokenResponse {
    pub token: String,
    pub record: AuthRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserContext {
    pub token_id: String,
    pub name: String,
    pub role: UserRole,
}
