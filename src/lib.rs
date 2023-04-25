use sea_orm_migration::prelude::*;

pub mod create_table;

#[derive(Iden)]
pub enum TimestampIden {
    CreatedAt,
    UpdatedAt,
}

pub trait TimestampExt {
    fn with_timestamps(&mut self) -> &mut TableCreateStatement;
}
