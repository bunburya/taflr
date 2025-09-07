use crate::ai::{Ai, BasicAi};
use dioxus::prelude::{Readable, Signal};
use dioxus::signals::GlobalSignal;
use hnefatafl::board::state::{BoardState, MediumBasicBoardState};
use hnefatafl::game::state::GameState;
use hnefatafl::play::ValidPlay;
use std::time::Duration;

pub static AI: GlobalSignal<Option<BasicAi>> = Signal::global(|| None);

pub(crate) struct AiRequest<B: BoardState> {
    pub(crate) game_state: GameState<B>,
    pub(crate) time_to_play: Duration,
}

#[derive(Debug)]
pub(crate) struct AiResponse<B: BoardState> {
    pub(crate) game_state: GameState<B>,
    pub(crate) play: ValidPlay
}

pub(crate) async fn compute_ai_play(request: AiRequest<MediumBasicBoardState>) -> Result<AiResponse<MediumBasicBoardState>, String> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    let ai_clone = AI.read().clone();
    std::thread::spawn(move || {
        match ai_clone {
            Some(mut ai) => {
                match ai.next_play(&request.game_state, request.time_to_play) {
                    Ok((ai_play, _)) => tx.send(Ok(AiResponse {
                        game_state: request.game_state,
                        play: ai_play
                    }))
                        .expect("Could not send AI play from AI thread"),
                    Err(e) => tx.send(Err(format!("Bad AI play: {:?}", e)))
                        .expect("Could not send error from AI thread")

                }
            },
            None => {
                tx.send(Err("No AI set".to_string()))
                    .expect("AI was not set, and that error could not be sent from AI thread");
            }
        }
    });
    rx.await.map_err(|_| "AI error".to_string())?
}