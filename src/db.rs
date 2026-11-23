use rusqlite::{Connection, Result};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "PRAGMA journal_mode=WAL;
             PRAGMA foreign_keys=ON;
             CREATE TABLE IF NOT EXISTS settings (
               guild_id TEXT NOT NULL,
               key TEXT NOT NULL,
               value TEXT NOT NULL,
               PRIMARY KEY (guild_id, key)
             );
             CREATE TABLE IF NOT EXISTS usage (
               command TEXT NOT NULL,
               user_id TEXT NOT NULL,
               guild_id TEXT,
               count INTEGER NOT NULL DEFAULT 0,
               PRIMARY KEY(command, user_id, guild_id)
             );"
        )?;
        Ok(Self { conn })
    }

    pub fn record_usage(&self, command: &str, user_id: u64, guild_id: Option<u64>) -> Result<()> {
        self.conn.execute(
            "INSERT INTO usage(command,user_id,guild_id,count) VALUES(?1,?2,?3,1)
             ON CONFLICT(command,user_id,guild_id) DO UPDATE SET count=count+1",
            (command, user_id.to_string(), guild_id.map(|v| v.to_string())),
        )?;
        Ok(())
    }

    pub fn set_setting(&self, guild_id: u64, key: &str, value: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO settings(guild_id,key,value) VALUES(?1,?2,?3)
             ON CONFLICT(guild_id,key) DO UPDATE SET value=excluded.value",
            (guild_id.to_string(), key, value),
        )?;
        Ok(())
    }

    pub fn get_setting(&self, guild_id: u64, key: &str) -> Result<Option<String>> {
        self.conn.query_row(
            "SELECT value FROM settings WHERE guild_id=?1 AND key=?2",
            (guild_id.to_string(), key),
            |row| row.get(0),
        ).optional()
    }
}
use rusqlite::OptionalExtension;

#[cfg(test)]
mod tests {
    use super::Database;
    #[test]
    fn settings_round_trip() {
        let db = Database::open(":memory:").expect("db");
        db.set_setting(42, "prefix", "!").expect("save");
        assert_eq!(db.get_setting(42, "prefix").expect("read"), Some("!".to_string()));
    }
}
