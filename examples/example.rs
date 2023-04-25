use sea_orm_migration::prelude::*;
use sea_orm_migration_timestamps::TimestampExt;

#[derive(Iden)]
enum ExampleTable {
    Table,
    Id,
}

fn main() {
    Table::create()
        .table(ExampleTable::Table)
        .col(
            ColumnDef::new(ExampleTable::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .with_timestamps()
        .take();
}
