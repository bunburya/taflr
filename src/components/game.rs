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
pub(crate) fn Game(controller: GameController) -> Element {
    use_context_provider(|| controller);
    let _ = use_future(async move || {
        loop {
            let mut ctrl = use_context::<GameController>();
            if ctrl.is_ai_turn() && (ctrl.time_since_last_move() > ctrl.ai_move_time) {
                if let Some(vp) = poll_ai_play().await.unwrap() {
                    let g = do_play(vp.play).await.unwrap().unwrap();
                    ctrl.game_copy.set(g);
                    ctrl.last_move_time = Instant::now();
                }
            }
            async_sleep(500).await;
        }
    });
    rsx! {
        Board {}
    }
}