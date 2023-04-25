use sea_orm_migration::prelude::*;

use crate::{TimestampExt, TimestampIden};

impl TimestampExt for TableCreateStatement {
    fn with_timestamps(&mut self) -> &mut TableCreateStatement {
        self.col(
            ColumnDef::new(TimestampIden::CreatedAt)
                .default(Expr::current_timestamp())
                .not_null()
                .timestamp(),
        );

        cfg_if::cfg_if! {
            if #[cfg(any(feature = "postgres", feature = "sqlite"))] {
                let table_name = self.get_table_name().expect("Call `table(Iden)` before calling `with_timestamps()`");
                let table_name = if let TableRef::Table(table_name) = table_name {
                    table_name.to_string()
                } else {
                    panic!("Unexpected table name! Make a fork to fix this :p")
                };
            }
        }
        cfg_if::cfg_if! {
            if #[cfg(feature = "postgres")] {
                self.col(
                    ColumnDef::new(TimestampIden::UpdatedAt)
                        .default(Expr::current_timestamp())
                        .not_null()
                        .extra(format!(r#";
                            CREATE OR REPLACE FUNCTION trigger_set_timestamp()
                            RETURNS TRIGGER AS $$
                            BEGIN
                                NEW.updated_at = NOW();
                                RETURN NEW;
                            END;
                            $$ LANGUAGE plpgsql;

                            CREATE TRIGGER updated_at_{table_name}
                            BEFORE UPDATE ON {table_name}
                            FOR EACH ROW
                            EXECUTE PROCEDURE trigger_set_timestamp();
                        "#))
                        .timestamp(),
                )
            } else if #[cfg(feature = "mysql")] {
                self.col(
                    ColumnDef::new(TimestampIden::UpdatedAt)
                        .default(Expr::current_timestamp())
                        .extra("ON UPDATE CURRENT_TIMESTAMP".to_string())
                        .not_null()
                        .timestamp(),
                )
            } else if  #[cfg(feature = "sqlite")] {
                let updated_at = TimestampIden::UpdatedAt.to_string();
                self.col(
                    ColumnDef::new(TimestampIden::UpdatedAt)
                        .default(Expr::current_timestamp())
                        .not_null()
                        .extra(format!(r#";
                            CREATE TRIGGER [updated_at_{table_name}]
                                AFTER UPDATE
                                ON {table_name}
                                FOR EACH ROW
                                WHEN NEW.{updated_at} < OLD.{updated_at}
                            BEGIN
                                UPDATE {table_name} SET {updated_at}=CURRENT_TIMESTAMP WHERE rowid = NEW.rowid;
                            END;
                        "#))
                        .timestamp(),
                )
            } else {
                compile_error!("Select one of the three features [postgres, mysql, sqlite]")
            }
        }
    }
}
