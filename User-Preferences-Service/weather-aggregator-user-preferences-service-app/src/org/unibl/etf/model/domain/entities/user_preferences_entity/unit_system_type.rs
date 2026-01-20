
#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "unit_system_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UnitSystemType {
    METRIC,
    IMPERIAL
}