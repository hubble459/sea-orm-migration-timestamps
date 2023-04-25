use sea_orm_migration::prelude::*;

mod create_table;

pub use create_table::*;
pub mod alter_table;

#[derive(Iden)]
pub enum TimestampIden {
    CreatedAt,
    UpdatedAt,
}

pub trait AlterTableExt {
    fn add_timestamps<T: IntoTableRef>(&mut self, table: T) -> &mut TableAlterStatement;
}

use std::any::{Any, TypeId};

pub(crate) trait InstanceOf
where
    Self: Any,
{
    fn instance_of<U: ?Sized + Any>(&self) -> bool {
        TypeId::of::<Self>() == TypeId::of::<U>()
    }
}

// implement this trait for every type that implements `Any` (which is most types)
impl<T: ?Sized + Any> InstanceOf for T {}