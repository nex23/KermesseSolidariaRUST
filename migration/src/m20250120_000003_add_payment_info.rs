use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Sales::Table)
                    .add_column(ColumnDef::new(Sales::PaymentMethod).string().default("CASH"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Sales::Table)
                    .drop_column(Sales::PaymentMethod)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Sales {
    Table,
    PaymentMethod,
}
