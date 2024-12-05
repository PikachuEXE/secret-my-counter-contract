use secret_toolkit::storage::{Item};

// Idea from rails active record

pub static SCHEMA_MIGRATION_VERSIONS: Item<Vec<String>> = Item::new(b"schema_migration_versions");
