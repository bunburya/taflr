use std::str::FromStr;
use std::time::Duration;
use sqlx::{query, query_as, Error, Row, SqlitePool};
use hnefatafl::board::state::BoardState;
use hnefatafl::collections::{PieceMap};
use hnefatafl::game::{Game};
use hnefatafl::game::state::GameState;
use hnefatafl::pieces::Side;
use hnefatafl::play::{Play, PlayEffects, PlayRecord};
use sqlx::sqlite::SqliteRow;
use crate::config::{GameSettings, Variant};
use crate::error::DbError;
use crate::gamectrl::Player;

impl<'r> sqlx::FromRow<'r, SqliteRow> for Variant {
    fn from_row(row: &'r SqliteRow) -> Result<Self, Error> {
        Ok(Self {
            rules: serde_json::from_str(row.try_get("rules")?).expect("Bad JSON"),
            starting_board: row.try_get("starting_board")?,
            name: row.try_get("name")?
        })
    }
}

pub(crate) struct GameSettingsDbObject {
    id: i64,
    game_settings: GameSettings,
}

impl<'r> sqlx::FromRow<'r, SqliteRow> for GameSettingsDbObject {
    fn from_row(row: &'r SqliteRow) -> Result<Self, Error> {
        let id = row.try_get("id")?;
        let game_settings = GameSettings {
            variant: Variant::from_row(row)?,
            name: row.try_get("name")?,
            attacker: Player {
                name: row.try_get("attacker_name")?,
                ai_play_time: row.try_get::<'_, Option<i64>, _>("attacker_ai_ttp")?
                    .map(|s| Duration::from_secs(s as u64)),
            },
            defender: Player {
                name: row.try_get("defender_name")?,
                ai_play_time: row.try_get::<'_, Option<i64>, _>("defender_ai_ttp")?
                    .map(|s| Duration::from_secs(s as u64)),
            },
        };
        Ok(Self {
            id,
            game_settings,
        })
    }
}

pub(crate) struct GameDbObject<B: BoardState> {
    id: u64,
    game: Game<B>
}

pub(crate) struct DbController {
    pub (crate) pool: SqlitePool,
}

impl DbController {
    pub(crate) async fn create_schemas(&mut self) -> Result<(), sqlx::Error> {
        sqlx::query(include_str!("../sql/schema.sqlite")).execute(&self.pool).await?;
        Ok(())
    }

    pub(crate) async fn add_game<B: BoardState>(
        &mut self,
        settings: &GameSettings,
        game: &Game<B>,
    ) -> Result<i64, sqlx::Error> {
        let variant_name = settings.variant.name.to_string();
        let turn = game.state.turn as i64;
        let att_ai_ttp = settings.attacker.ai_play_time.map(|d| d.as_secs_f64());
        let def_ai_ttp = settings.defender.ai_play_time.map(|d| d.as_secs_f64());
        Ok(sqlx::query!(
            r#"
                INSERT OR REPLACE INTO games (
                    name,
                    variant_name,
                    turn,
                    attacker_name,
                    attacker_ai_ttp,
                    defender_name,
                    defender_ai_ttp
                ) VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
            settings.name,
            variant_name,
            turn,
            settings.attacker.name,
            att_ai_ttp,
            settings.defender.name,
            def_ai_ttp,
        ).execute(&self.pool).await?.last_insert_rowid())
    }

    pub(crate) async fn add_state<B: BoardState>(
        &mut self, game_id: i64,
        state: &GameState<B>
    ) -> Result<i64, DbError> {
        let turn = state.turn as i64;
        let board = state.board.to_fen();
        let side_to_play = state.side_to_play.to_string();
        let repetitions = serde_json::to_string(&state.repetitions)?;
        let plays_since_capture = state.plays_since_capture as i64;
        let status = serde_json::to_string(&state.status)?;
        Ok(sqlx::query!(
            r#"
                INSERT INTO states (
                    game_id,
                    turn,
                    board,
                    side_to_play,
                    repetitions,
                    plays_since_capture,
                    status
                ) VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
            game_id, turn, board, side_to_play, repetitions, plays_since_capture, status
        )
            .execute(&self.pool)
            .await?.last_insert_rowid())
    }

    pub(crate) async fn add_play_record<B: BoardState>(
        &mut self,
        game_id: i64,
        turn: i64,
        board_len: u8,
        play_record: &PlayRecord<B>
    ) -> Result<i64, DbError> {
        let side = play_record.side.to_string();
        let play = play_record.play.to_string();
        let captures = play_record.effects.captures.to_fen(board_len);
        let game_outcome = serde_json::to_string(&play_record.effects.game_outcome)?;
        Ok(sqlx::query!(
            r#"
                INSERT INTO play_records (
                    game_id,
                    turn,
                    side,
                    play,
                    captures,
                    game_outcome
                ) VALUES (?, ?, ?, ?, ?, ?)
            "#,
            game_id, turn, side, play, captures, game_outcome
        ).execute(&self.pool)
            .await?
            .last_insert_rowid())
    }

    pub(crate) async fn add_turn<B: BoardState>(
        &mut self,
        game_id: i64,
        play_record: PlayRecord<B>,
        state: GameState<B>,
    ) -> Result<(i64, i64), DbError> {
        let state_id = self.add_state(game_id, &state).await?;
        let record_id = self.add_play_record(
            game_id,
            (state.turn as i64) - 1,
            state.board.side_len(),
            &play_record
        ).await?;

        Ok((state_id, record_id))
    }

    pub(crate) async fn load_game<B: BoardState>(&self, id: i64) -> Result<(GameSettings, Game<B>), DbError> {
        let gso: GameSettingsDbObject = query_as(r"SELECT * FROM games WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        let state_history: Vec<GameState<B>> = query(
            r"SELECT * FROM states WHERE game_id = ? ORDER BY turn"
        )
            .bind(id)
            .fetch_all(&self.pool)
            .await?
            .iter()
            .map(|r | Ok(GameState {
                turn: r.try_get::<i64, _>("turn")? as usize,
                board: B::from_fen(r.try_get("board")?)?,
                side_to_play: Side::from_str(r.try_get("side_to_play")?)?,
                repetitions: serde_json::from_str(r.try_get("repetitions")?)?,
                plays_since_capture: r.try_get::<i64, _>("plays_since_capture")? as usize,
                status: serde_json::from_str(r.try_get("status")?)?
            }))
            .collect::<Result<_, DbError>>()?;
        let play_history: Vec<PlayRecord<B>> = query(
            r"SELECT * FROM play_records WHERE game_id = ? ORDER BY turn"
        )
            .bind(id)
            .fetch_all(&self.pool)
            .await?
            .iter()
            .map(|r| Ok::<_, DbError>(PlayRecord {
                side: Side::from_str(r.try_get("side")?)?,
                play: Play::from_str(r.try_get("play")?)?,
                effects: PlayEffects {
                    captures: B::PieceMap::from_fen(r.try_get("captures")?)?.0,
                    game_outcome: serde_json::from_str(r.try_get("game_outcome")?)?
                }
            }))
            .collect::<Result<_, DbError>>()?;
        let mut game = Game::new(
            gso.game_settings.variant.rules,
            &gso.game_settings.variant.starting_board,
        ).expect("Could not construct game");
        game.play_history = play_history;
        if let Some(s) = state_history.last() {
            game.state = *s;
        }
        game.state_history = state_history;
        Ok((gso.game_settings, game))
    }
}
