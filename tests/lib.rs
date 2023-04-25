#[cfg(test)]
mod tests {
    use sea_orm_migration::prelude::*;
    use sea_orm_migration_timestamps::CreateTableExt;

    #[derive(Iden)]
    enum ExampleTable {
        Table,
        Id,
    }

    #[test]
    #[cfg(feature = "postgres")]
    fn test_postgres() {
        let table = Table::create()
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

        let query = table.to_string(PostgresQueryBuilder);
        assert_ne!(query, "");
    }

    #[test]
    #[cfg(feature = "mysql")]
    fn test_mysql() {
        let table = Table::create()
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

        let query = table.to_string(MySqlQueryBuilder);
        assert_ne!(query, "");
    }

    #[test]
    #[cfg(feature = "sqlite")]
    fn test_sqlite() {
        let table = Table::create()
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

        let query = table.to_string(SqliteQueryBuilder);
        assert_ne!(query, "");
    }
}
