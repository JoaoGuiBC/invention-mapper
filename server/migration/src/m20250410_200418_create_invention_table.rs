use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Invention::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Invention::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(string(Invention::Name))
                    .col(text(Invention::Text))
                    .col(string(Invention::ExternalLink))
                    .col(float(Invention::Lat))
                    .col(float(Invention::Lon))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Invention::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Invention {
    Table,
    Id,
    Name,
    Text,
    #[sea_orm(iden = "external_link")]
    ExternalLink,
    Lat,
    Lon,
}
