

```shell
createdb rust-seaorm # Assumes there is a user called "test" with password "test"
cargo update
cargo install sea-orm-cli
```

[Migrate](https://www.sea-ql.org/SeaORM/docs/migration/running-migration/) the database


```shell
sea-orm-cli migrate up 
# Import the entities from the current database
sea-orm-cli generate entity -o entity/src
```

Add a migration
```shell
sea-orm-cli migrate generate auto_inc_index
```
Then edit the resulting file