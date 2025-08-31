use crate::ai::{Ai, BasicAi};
use hnefatafl::board::state::BoardState;
use hnefatafl::game::state::GameState;
use hnefatafl::game::Game;
use hnefatafl::pieces::Side;
use hnefatafl::play::ValidPlay;
use std::thread;
use std::time::Duration;


#[derive(Debug)]
enum Message<T: BoardState> {
    Request(GameState<T>),
    Response(ValidPlay, GameState<T>, Vec<String>)
}

#[derive(Debug)]
struct AiChannel<T: BoardState> {
    sender: std::sync::mpsc::Sender<Message<T>>,
    receiver: std::sync::mpsc::Receiver<Message<T>>
}

impl<T: BoardState + Send + 'static> AiChannel<T> {
    fn new(game: &Game<T>, side: Side, time_to_play: Duration) -> Self {
        let (g2ai_tx, g2ai_rx) = std::sync::mpsc::channel::<Message<T>>();
        let (ai2g_tx, ai2g_rx) = std::sync::mpsc::channel::<Message<T>>();
        let logic = game.logic;
        thread::spawn(move || {
            let mut ai = BasicAi::new(logic, side, time_to_play);
            while let Ok(Message::Request(state)) = g2ai_rx.recv() {
                if let Ok((play, lines)) = ai.next_play(&state) {
                    let _ = ai2g_tx.send(Message::Response(play, state, lines));
                }
            }
        });
        Self {
            sender: g2ai_tx,
            receiver: ai2g_rx
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct AiController<T: BoardState> {
    attacker: Option<AiChannel<T>>,
    defender: Option<AiChannel<T>>,
}

impl<T: BoardState + Send + 'static> AiController<T> {
    pub(crate) fn new(
        game: &Game<T>,
        attacker_ai_time: Option<Duration>,
        defender_ai_time: Option<Duration>
    ) -> Self {
        let attacker = if let Some(d) = attacker_ai_time {
            Some(AiChannel::new(game, Side::Attacker, d))
        } else {
            None
        };
        let defender = if let Some(d) = defender_ai_time {
            Some(AiChannel::new(game, Side::Defender, d))
        } else {
            None
        };
        Self { attacker, defender }
    }

    fn get_ai_channel<'a>(
        &mut self,
        side_to_play: Side
    ) -> &mut Option<AiChannel<T>> {
        if side_to_play == Side::Attacker {
            &mut self.attacker
        } else {
            &mut self.defender
        }
    }

    pub fn request_ai_play(&mut self, game: &Game<T>) {
        if let Some(ai_chan) = self.get_ai_channel(game.state.side_to_play) {
            ai_chan.sender.send(Message::Request(game.state)).expect("Failed to send request");
        }
    }

    pub fn receive_ai_play(
        &mut self,
        side_to_play: Side,
        current_state: GameState<T>
    ) -> Option<ValidPlay> {
        if let Some(ai_chan) = self.get_ai_channel(side_to_play) {
            loop {
                if let Message::Response(vp, state, _) = ai_chan.receiver.recv()
                    .expect("Failed to receive response") {
                    if state == current_state {
                        return Some(vp)
                    }
                } else {
                   return None
                }
            }
        } else {
            None
        }
    }
}