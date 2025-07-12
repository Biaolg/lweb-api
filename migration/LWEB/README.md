# Running Migrator CLI

- 生成新的迁移文件
    ```sh
    cargo run -- generate MIGRATION_NAME
    ```
- 应用所有待处理的迁移
    ```sh
    cargo run
    ```
    ```sh
    cargo run -- up
    ```
- 应用前 10 个待处理迁移
    ```sh
    cargo run -- up -n 10
    ```
- 回滚上次应用的迁移
    ```sh
    cargo run -- down
    ```
- 回滚最近 10 次应用的迁移
    ```sh
    cargo run -- down -n 10
    ```
- 从数据库中删除所有表，然后重新应用所有迁移
    ```sh
    cargo run -- fresh
    ```
- 回滚所有应用的迁移，然后重新应用所有迁移
    ```sh
    cargo run -- refresh
    ```
- 回滚所有应用的迁移
    ```sh
    cargo run -- reset
    ```
- 检查所有迁移的状态
    ```sh
    cargo run -- status
    ```
