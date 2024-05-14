use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        if !manager.has_column("file_path", "type").await? {
            db.execute_unprepared(
                "ALTER TABLE file_path ADD COLUMN type int"
            ).await?;
        }
        Ok(())
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        if manager.has_column("file_path", "type").await? {
            db.execute_unprepared(
                "ALTER TABLE file_path DROP COLUMN type"
            ).await?;
        }
        Ok(())
    }
}

