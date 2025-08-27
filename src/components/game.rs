#[cfg(target_arch = "wasm32")]
use web_time::Instant;
#[cfg(not(target_arch = "wasm32"))]
use std::time::{Duration, Instant};
use dioxus::prelude::*;
use crate::backend::{do_play, poll_ai_play};
use crate::components::Board;
use crate::gamectrl::GameController;

#[cfg(target_arch = "wasm32")]
async fn async_sleep(ms: u32) {
    use gloo_timers::future::TimeoutFuture;
    TimeoutFuture::new(ms).await;
}


#[cfg(not(target_arch = "wasm32"))]
async fn async_sleep(ms: u32) {
    use async_std::task::sleep;
    sleep(Duration::from_millis(ms.into())).await;
}

#[component]
pub(crate) fn Game(game_ctrl: GameController) -> Element {
    use_context_provider(|| game_ctrl);
    let _ = use_future(async move || {
        loop {
            let mut game_ctrl = use_context::<GameController>();
            if game_ctrl.ai_move_time().is_some_and(|d| game_ctrl.time_since_last_move() > d) {
                if let Some(vp) = poll_ai_play().await.unwrap() {
                    let g = do_play(vp.play).await.unwrap().unwrap();
                    game_ctrl.game_copy.set(g);
                    game_ctrl.last_move_time = Instant::now();
                }
            }
            async_sleep(500).await;
        }
    });
    rsx! {
        Board {}
    }
}