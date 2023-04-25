use std::ops::{Deref, DerefMut};

use sea_orm_migration::prelude::*;

use crate::TimestampIden;

#[derive(Clone)]
pub struct TableCreateStatementExt {
    inner: TableCreateStatement,
    append_statements: Vec<String>,
}

impl TableCreateStatementExt {
    pub fn new(table_create_statement: TableCreateStatement) -> Self {
        Self {
            inner: table_create_statement,
            append_statements: vec![],
        }
    }

    pub fn append_statement(&mut self, stmt: String) -> TableCreateStatementExt {
        self.append_statements.push(stmt);

        self.clone()
    }

    pub fn take(&self) -> Self {
        self.clone()
    }
}

impl Deref for TableCreateStatementExt {
    type Target = TableCreateStatement;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for TableCreateStatementExt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl SchemaStatementBuilder for TableCreateStatementExt {
    fn build<T: SchemaBuilder>(&self, schema_builder: T) -> String {
        let mut sql = String::with_capacity(256);
        schema_builder.prepare_table_create_statement(self, &mut sql);

        for stmt in self.append_statements.iter() {
            write!(sql, "; {}", stmt).ok();
        }

        sql
    }

    fn build_any(&self, schema_builder: &dyn SchemaBuilder) -> String {
        let mut sql = String::with_capacity(256);
        schema_builder.prepare_table_create_statement(self, &mut sql);

        for stmt in self.append_statements.iter() {
            write!(sql, "; {}", stmt).ok();
        }

        sql
    }
}

pub trait CreateTableExt {
    fn with_timestamps(&mut self) -> TableCreateStatementExt;
}

impl CreateTableExt for TableCreateStatement {
    fn with_timestamps(&mut self) -> TableCreateStatementExt {
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
                let updated_at = TimestampIden::UpdatedAt.to_string();
            }
        }
        cfg_if::cfg_if! {
            if #[cfg(feature = "postgres")] {
                TableCreateStatementExt::new(
                    self.col(
                        ColumnDef::new(TimestampIden::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .not_null()
                            .timestamp(),
                    ).take()
                )
                .append_statement(format!(r#"
                    CREATE OR REPLACE FUNCTION trigger_set_timestamp()
                    RETURNS TRIGGER AS $$
                    BEGIN
                        NEW.{updated_at} = NOW();
                        RETURN NEW;
                    END;
                    $$ LANGUAGE plpgsql;

                    CREATE TRIGGER updated_at_{table_name}
                    BEFORE UPDATE ON {table_name}
                    FOR EACH ROW
                    EXECUTE PROCEDURE trigger_set_timestamp();
                "#))
            } else if #[cfg(feature = "mysql")] {
                TableCreateStatementExt::new(
                    self.col(
                        ColumnDef::new(TimestampIden::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .extra("ON UPDATE CURRENT_TIMESTAMP".to_string())
                            .not_null()
                            .timestamp(),
                    ).take()
                )
            } else if  #[cfg(feature = "sqlite")] {
                TableCreateStatementExt::new(
                    self.col(
                        ColumnDef::new(TimestampIden::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .not_null()
                            .timestamp(),
                    ).take()
                )
                .append_statement(format!(r#"
                    CREATE TRIGGER [updated_at_{table_name}]
                        AFTER UPDATE
                        ON {table_name}
                        FOR EACH ROW
                        WHEN NEW.{updated_at} < OLD.{updated_at}
                    BEGIN
                        UPDATE {table_name} SET {updated_at}=CURRENT_TIMESTAMP WHERE rowid = NEW.rowid;
                    END;
                "#))
            } else {
                compile_error!("Select one of the three features [postgres, mysql, sqlite]")
            }
        }
    }
}
