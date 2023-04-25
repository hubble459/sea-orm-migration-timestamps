use sea_orm_migration::prelude::*;

pub mod create_table;
pub mod alter_table;

#[derive(Iden)]
pub enum TimestampIden {
    CreatedAt,
    UpdatedAt,
}

pub trait CreateTableExt {
    fn with_timestamps(&mut self) -> &mut TableCreateStatement;
}

pub trait AlterTableExt {
    fn add_timestamps<T: IntoTableRef>(&mut self, table: T) -> &mut TableAlterStatement;
}
