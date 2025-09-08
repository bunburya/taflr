use hnefatafl::game::MediumBasicGame;
use rusqlite::Connection;
use crate::gamectrl::Player;

thread_local! {
    pub static DB: Connection = {
        let conn = Connection::open("taflr.db").expect("Failed to open database");
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS games (
                id INTEGER PRIMARY KEY,
                json TEXT NOT NULL,
                attacker_name TEXT NOT NULL,
                attacker_ai_ttp INTEGER,
                defender_name TEXT NOT NULL,
                defender_ai_ttp INTEGER
            );"
        ).unwrap();

        conn
    };
}

pub fn add_to_db(game: &MediumBasicGame, attacker: &Player, defender: &Player) -> usize {
    DB.with(|f| {
        f.execute(
            "INSERT INTO games (json, attacker_name, attacker_ai_ttp, defender_name, defender_ai_ttp) VALUES (?, ?, ?, ?, ?)",
            (
                serde_json::to_string(game).unwrap(),
                &attacker.name,
                attacker.ai_play_time.map(|d| d.as_secs()),
                &defender.name,
                defender.ai_play_time.map(|d| d.as_secs())
            )).unwrap();
        f.last_insert_rowid()
    }) as usize
}

pub fn save_to_db(id: usize, game: &MediumBasicGame, attacker: &Player, defender: &Player) {
    DB.with(|f| {
        f.execute(
            "INSERT OR REPLACE INTO games (id, json, attacker_name, attacker_ai_ttp, defender_name, defender_ai_ttp) VALUES (?, ?, ?, ?, ?, ?)",
            (
                id,
                serde_json::to_string(game).unwrap(),
                &attacker.name,
                attacker.ai_play_time.map(|d| d.as_secs()),
                &defender.name,
                defender.ai_play_time.map(|d| d.as_secs())
            )
        ).unwrap();
    })
}