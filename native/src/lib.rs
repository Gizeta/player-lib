#[macro_use]
extern crate neon;
extern crate rodio;

mod player;

use neon::prelude::*;
use std::sync::{ Arc, RwLock };
use player::{ AudioSource, Player };

pub struct PlayerWrapper {
    player: Arc<RwLock<Player>>,
}

impl PlayerWrapper {
    fn new() -> Self {
        PlayerWrapper {
            player: Arc::new(RwLock::new(Player::new())),
        }
    }
}

struct PlayingTask {
    player: Arc<RwLock<Player>>,
}

impl Task for PlayingTask {
    type Output = ();
    type Error = String;
    type JsEvent = JsUndefined;

    fn perform(&self) -> Result<Self::Output, Self::Error> {
        match self.player.read() {
            Ok(player) => {
                player.block();
            },
            Err(e) => return Err(format!("{}", e)),
        }
        Ok(())
    }

    fn complete(
        self,
        mut cx: TaskContext,
        result: Result<Self::Output, Self::Error>,
    ) -> JsResult<Self::JsEvent> {
        match result {
            Ok(_) => Ok(cx.undefined()),
            Err(e) => cx.throw_error(&e),
        }
    }
}

macro_rules! get_player {
    ($cx:ident, $player:ident, { $($body:tt)* }) => {
        let this = $cx.this();
        let guard = $cx.lock();
        let player = this.borrow(&guard).player.clone();
        let $player = player.read().expect("player is locked");
        $($body)*
    };
}

declare_types! {
    pub class JsPlayer for PlayerWrapper {
        init(_cx) {
            Ok(PlayerWrapper::new())
        }

        method open(mut cx) {
            let path = cx.argument::<JsString>(0)?.value();
            let f = cx.argument::<JsFunction>(1)?;
            let t = {
                let this = cx.this();
                let guard = cx.lock();
                let wrapper = this.borrow(&guard);
                let player = wrapper.player.clone();
                let player = player.read().expect("player is locked");
                player.append(AudioSource::File(path));
                player.pause();
                PlayingTask {
                    player: wrapper.player.clone(),
                }
            };

            t.schedule(f);
            Ok(cx.undefined().upcast())
        }

        method play(mut cx) {
            get_player!(cx, player, {
                player.play();
            });
            Ok(cx.undefined().upcast())
        }

        method pause(mut cx) {
            get_player!(cx, player, {
                player.pause();
            });
            Ok(cx.undefined().upcast())
        }

        method stop(mut cx) {
            get_player!(cx, player, {
                player.stop();
            });
            Ok(cx.undefined().upcast())
        }

        method append(mut cx) {
            let path = cx.argument::<JsString>(0)?.value();
            get_player!(cx, player, {
                player.append(AudioSource::File(path));
            });
            Ok(cx.undefined().upcast())
        }
    }
}

register_module!(mut m, {
    m.export_class::<JsPlayer>("Player")?;

    Ok(())
});
