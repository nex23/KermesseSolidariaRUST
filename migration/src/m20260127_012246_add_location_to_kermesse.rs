use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Kermesses::Table)
                    .add_column_if_not_exists(ColumnDef::new(Kermesses::Department).string().null())
                    .add_column_if_not_exists(ColumnDef::new(Kermesses::City).string().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Kermesses::Table)
                    .drop_column(Kermesses::Department)
                    .drop_column(Kermesses::City)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Kermesses {
    Table,
    Department,
    City,
}
