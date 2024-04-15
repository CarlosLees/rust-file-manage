use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FilePath::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FilePath::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(FilePath::ParentId).integer())
                    .col(ColumnDef::new(FilePath::CreateTime).date_time().not_null())
                    .col(ColumnDef::new(FilePath::UpdateTime).date_time().not_null())
                    .col(ColumnDef::new(FilePath::FolderName).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FilePath::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum FilePath {
    Table,
    Id,
    ParentId,
    CreateTime,
    UpdateTime,
    FolderName,
}
