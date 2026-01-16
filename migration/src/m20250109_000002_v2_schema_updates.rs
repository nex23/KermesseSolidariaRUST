use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. Alert Kermesses: Add financial_goal, qr_code_url
        manager
            .alter_table(
                Table::alter()
                    .table(Kermesses::Table)
                    .add_column(ColumnDef::new(Kermesses::FinancialGoal).decimal().null())
                    .add_column(ColumnDef::new(Kermesses::QrCodeUrl).string().null())
                    .to_owned(),
            )
            .await?;

        // 2. Alert Collaborators: Add status, proposed_role
        manager
            .alter_table(
                Table::alter()
                    .table(Collaborators::Table)
                    .add_column(string(Collaborators::Status).default("ACCEPTED"))
                    .add_column(ColumnDef::new(Collaborators::ProposedRole).string().null())
                    .to_owned(),
            )
            .await?;

         // 3. Alert Sales: Add delivery fields and buyer_id
        manager
            .alter_table(
                Table::alter()
                    .table(Sales::Table)
                    .add_column(string(Sales::DeliveryMethod).default("PICKUP")) // PICKUP, DELIVERY
                    .add_column(ColumnDef::new(Sales::DeliveryAddress).string().null())
                    .add_column(ColumnDef::new(Sales::ContactPhone).string().null())
                    .add_column(ColumnDef::new(Sales::BuyerId).integer().null()) // Logged-in user who bought
                    .to_owned(),
            )
            .await?;

        // 3b. Add FK for Sales Buyer (Separate step)
        manager.create_foreign_key(
             ForeignKey::create()
                .name("fk-sales-buyer")
                .from(Sales::Table, Sales::BuyerId)
                .to(Users::Table, Users::Id)
                .on_delete(ForeignKeyAction::SetNull)
                .to_owned()
        ).await?;

        // 4. Create IngredientDonations Table
        manager
            .create_table(
                Table::create()
                    .table(IngredientDonations::Table)
                    .if_not_exists()
                    .col(pk_auto(IngredientDonations::Id))
                    .col(integer(IngredientDonations::IngredientId))
                    .col(integer(IngredientDonations::UserId))
                    .col(decimal(IngredientDonations::QuantityDonated))
                    .col(timestamp_with_time_zone(IngredientDonations::CreatedAt).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-donations-ingredient")
                            .from(IngredientDonations::Table, IngredientDonations::IngredientId)
                            .to(Ingredients::Table, Ingredients::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-donations-user")
                            .from(IngredientDonations::Table, IngredientDonations::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop Donations Table
        manager
            .drop_table(Table::drop().table(IngredientDonations::Table).to_owned())
            .await?;

        // Revert Sales
        manager
            .alter_table(
                Table::alter()
                    .table(Sales::Table)
                    .drop_column(Sales::BuyerId)
                    .drop_foreign_key(Alias::new("fk-sales-buyer"))
                    .drop_column(Sales::ContactPhone)
                    .drop_column(Sales::DeliveryAddress)
                    .drop_column(Sales::DeliveryMethod)
                    .to_owned(),
            )
            .await?;

         // Revert Collaborators
        manager
            .alter_table(
                Table::alter()
                    .table(Collaborators::Table)
                    .drop_column(Collaborators::ProposedRole)
                    .drop_column(Collaborators::Status)
                    .to_owned(),
            )
            .await?;

        // Revert Kermesses
        manager
            .alter_table(
                Table::alter()
                    .table(Kermesses::Table)
                    .drop_column(Kermesses::QrCodeUrl)
                    .drop_column(Kermesses::FinancialGoal)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Kermesses {
    Table,
    FinancialGoal,
    QrCodeUrl,
}

#[derive(DeriveIden)]
enum Collaborators {
    Table,
    Status,
    ProposedRole,
}

#[derive(DeriveIden)]
enum Sales {
    Table,
    DeliveryMethod,
    DeliveryAddress,
    ContactPhone,
    BuyerId,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Ingredients {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum IngredientDonations {
    Table,
    Id,
    IngredientId,
    UserId,
    QuantityDonated,
    CreatedAt,
}
