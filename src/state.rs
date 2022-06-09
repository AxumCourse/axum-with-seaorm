use sea_orm::DatabaseConnection;

pub struct AppState {
    pub conn: DatabaseConnection,
}
