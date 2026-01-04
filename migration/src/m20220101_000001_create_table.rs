use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Users
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(pk_auto(Users::Id))
                    .col(string(Users::Username).unique_key())
                    .col(string(Users::Email).unique_key())
                    .col(string(Users::PasswordHash))
                    .col(string(Users::FullName))
                    .col(string(Users::Phone))
                    .col(timestamp_with_time_zone(Users::CreatedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        // Kermesses
        manager
            .create_table(
                Table::create()
                    .table(Kermesses::Table)
                    .if_not_exists()
                    .col(pk_auto(Kermesses::Id))
                    .col(string(Kermesses::Name))
                    .col(string(Kermesses::Slug).unique_key())
                    .col(string(Kermesses::Description))
                    .col(date(Kermesses::EventDate))
                    .col(integer(Kermesses::OrganizerId))
                    .col(string(Kermesses::BeneficiaryName))
                    .col(string(Kermesses::BeneficiaryReason))
                    .col(ColumnDef::new(Kermesses::BeneficiaryImageUrl).string().null())
                    .col(ColumnDef::new(Kermesses::StartTime).string().null())
                    .col(ColumnDef::new(Kermesses::EndTime).string().null())
                    .col(string(Kermesses::Status).default("DRAFT")) // DRAFT, ACTIVE, FINISHED
                    .col(timestamp_with_time_zone(Kermesses::CreatedAt).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-kermesses-organizer")
                            .from(Kermesses::Table, Kermesses::OrganizerId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Dishes (Platos)
        manager
            .create_table(
                Table::create()
                    .table(Dishes::Table)
                    .if_not_exists()
                    .col(pk_auto(Dishes::Id))
                    .col(integer(Dishes::KermesseId))
                    .col(string(Dishes::Name))
                    .col(string(Dishes::Description))
                    .col(decimal(Dishes::Price))
                    .col(integer(Dishes::QuantityAvailable))
                    .col(ColumnDef::new(Dishes::ImageUrl).string().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-dishes-kermesse")
                            .from(Dishes::Table, Dishes::KermesseId)
                            .to(Kermesses::Table, Kermesses::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Ingredients (Insumos)
        manager
            .create_table(
                Table::create()
                    .table(Ingredients::Table)
                    .if_not_exists()
                    .col(pk_auto(Ingredients::Id))
                    .col(integer(Ingredients::KermesseId))
                    .col(string(Ingredients::Name))
                    .col(decimal(Ingredients::QuantityNeeded))
                    .col(string(Ingredients::Unit)) // e.g., kg, liters
                    .col(boolean(Ingredients::IsDonated).default(false))
                    .col(ColumnDef::new(Ingredients::DonatedByUserId).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-ingredients-kermesse")
                            .from(Ingredients::Table, Ingredients::KermesseId)
                            .to(Kermesses::Table, Kermesses::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-ingredients-user")
                            .from(Ingredients::Table, Ingredients::DonatedByUserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        // Collaborators
        manager
            .create_table(
                Table::create()
                    .table(Collaborators::Table)
                    .if_not_exists()
                    .col(pk_auto(Collaborators::Id))
                    .col(integer(Collaborators::KermesseId))
                    .col(integer(Collaborators::UserId))
                    .col(string(Collaborators::Role)) // COLLABORATOR, SELLER, DISTRIBUTOR
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-collaborators-kermesse")
                            .from(Collaborators::Table, Collaborators::KermesseId)
                            .to(Kermesses::Table, Kermesses::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-collaborators-user")
                            .from(Collaborators::Table, Collaborators::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Sales (Ventas)
        manager
            .create_table(
                Table::create()
                    .table(Sales::Table)
                    .if_not_exists()
                    .col(pk_auto(Sales::Id))
                    .col(integer(Sales::KermesseId))
                    .col(integer(Sales::SellerId))
                    .col(string(Sales::CustomerName)) // As per requirement 8
                    .col(decimal(Sales::TotalAmount))
                    .col(string(Sales::Status).default("PENDING")) // PENDING, PAID, DELIVERED
                    .col(timestamp_with_time_zone(Sales::CreatedAt).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-sales-kermesse")
                            .from(Sales::Table, Sales::KermesseId)
                            .to(Kermesses::Table, Kermesses::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-sales-seller")
                            .from(Sales::Table, Sales::SellerId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Sale Items (Detalle Venta)
        manager
            .create_table(
                Table::create()
                    .table(SaleItems::Table)
                    .if_not_exists()
                    .col(pk_auto(SaleItems::Id))
                    .col(integer(SaleItems::SaleId))
                    .col(integer(SaleItems::DishId))
                    .col(integer(SaleItems::Quantity))
                    .col(decimal(SaleItems::Subtotal))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-sale-items-sale")
                            .from(SaleItems::Table, SaleItems::SaleId)
                            .to(Sales::Table, Sales::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-sale-items-dish")
                            .from(SaleItems::Table, SaleItems::DishId)
                            .to(Dishes::Table, Dishes::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(SaleItems::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Sales::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Collaborators::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Ingredients::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Dishes::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Kermesses::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Users::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    Email,
    PasswordHash,
    FullName,
    Phone,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Kermesses {
    Table,
    Id,
    Name,
    Slug,
    Description,
    EventDate,
    OrganizerId,
    BeneficiaryName,
    BeneficiaryReason,
    BeneficiaryImageUrl,
    StartTime,
    EndTime,
    Status,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Dishes {
    Table,
    Id,
    KermesseId,
    Name,
    Description,
    Price,
    QuantityAvailable,
    ImageUrl,
}

#[derive(DeriveIden)]
enum Ingredients {
    Table,
    Id,
    KermesseId,
    Name,
    QuantityNeeded,
    Unit,
    IsDonated,
    DonatedByUserId,
}

#[derive(DeriveIden)]
enum Collaborators {
    Table,
    Id,
    KermesseId,
    UserId,
    Role,
}

#[derive(DeriveIden)]
enum Sales {
    Table,
    Id,
    KermesseId,
    SellerId,
    CustomerName,
    TotalAmount,
    Status,
    CreatedAt,
}

#[derive(DeriveIden)]
enum SaleItems {
    Table,
    Id,
    SaleId,
    DishId,
    Quantity,
    Subtotal,
}
