use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Users::Id).uuid().primary_key())
                    .col(
                        ColumnDef::new(Users::Username)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Users::Email)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Users::Password).string().not_null())
                    .col(ColumnDef::new(Users::FullName).string().not_null())
                    .col(ColumnDef::new(Users::LastLogin).timestamp_with_time_zone())
                    .col(ColumnDef::new(Users::ProfilePicture).string())
                    .col(ColumnDef::new(Users::IsAdmin).boolean().not_null())
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Donas::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Donas::Id).uuid().primary_key())
                    .col(ColumnDef::new(Donas::Msg).string().not_null())
                    .col(ColumnDef::new(Donas::Amount).decimal().not_null())
                    .col(ColumnDef::new(Donas::Status).string().not_null())
                    .col(ColumnDef::new(Donas::OptionMethod).string().not_null())
                    .col(ColumnDef::new(Donas::UserId).string().not_null())
                    .col(ColumnDef::new(Donas::SenderId).string().not_null())
                    .col(
                        ColumnDef::new(Donas::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Donas::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Donas::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    Email,
    Password,
    FullName,
    LastLogin,
    ProfilePicture,
    IsAdmin,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Donas {
    Table,
    Id,
    Msg,
    Amount,
    Status,
    OptionMethod,
    UserId,
    SenderId,
    CreatedAt,
    UpdatedAt,
}
