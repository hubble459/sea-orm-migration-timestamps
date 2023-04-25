# Sea ORM Migration Timestamps

Add created_at and updated_at columns to your tables,
which will autoupdate with the help of triggers

## Install

In Cargo.toml

```toml
# If you use PostgreSQL
sea-orm-migration-timestamps = { git = "https://github.com/hubble459/sea-orm-migration-timestamps.git", features = [ "postgres" ] }
# If you use MySQL
sea-orm-migration-timestamps = { git = "https://github.com/hubble459/sea-orm-migration-timestamps.git", features = [ "mysql" ] }
# If you use SQLite
sea-orm-migration-timestamps = { git = "https://github.com/hubble459/sea-orm-migration-timestamps.git", features = [ "sqlite" ] }
```

Or in terminal use

```shell
# If you use PostgreSQL
cargo add --git https://github.com/hubble459/sea-orm-migration-timestamps.git -Fpostgres
# If you use MySQL
cargo add --git https://github.com/hubble459/sea-orm-migration-timestamps.git -Fmysql
# If you use SQLite
cargo add --git https://github.com/hubble459/sea-orm-migration-timestamps.git -Fsqlite
```
