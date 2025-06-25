/* Based on
 * https://github.com/longbridge/gpui-component/blob/main/crates/ui/src/input/blink_cursor.rs
 */

use gpui::*;
use std::time::Duration;

static INTERVAL: Duration = Duration::from_millis(500);
static PAUSE_DELAY: Duration = Duration::from_millis(500);

pub struct Cursor {
    visible: bool,
    paused: bool,
    epoch: usize,
    pause_epoch: usize,
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            visible: true,
            paused: false,
            epoch: 0,
            pause_epoch: 0,
        }
    }

    /// Start the blinking
    pub fn start(&mut self, cx: &mut Context<Self>) {
        self.blink(self.epoch, cx);
        self.visible = true;
    }

    /// Stop the blinking
    pub fn stop(&mut self, cx: &mut Context<Self>) {
        self.epoch = 0;
        self.visible = false;
        self.paused = false;
        cx.notify();
    }

    fn next_epoch(&mut self) -> usize {
        self.epoch += 1;
        self.epoch
    }

    fn blink(&mut self, epoch: usize, cx: &mut Context<Self>) {
        if self.paused || epoch != self.epoch {
            return;
        }

        self.visible = !self.visible;
        cx.notify();

        // Schedule the next blink
        let epoch = self.next_epoch();
        cx.spawn(async move |this, cx| {
            Timer::after(INTERVAL).await;
            if let Some(this) = this.upgrade() {
                this.update(cx, |this, cx| this.blink(epoch, cx)).ok();
            }
        })
        .detach();
    }

    pub fn visible(&self) -> bool {
        // Keep showing the cursor if paused
        self.paused || self.visible
    }

    /// Pause the blinking and wait for resuming.
    pub fn pause(&mut self, cx: &mut Context<Self>) {
        self.paused = true;
        cx.notify();

        self.pause_epoch += 1;
        let pause_epoch = self.pause_epoch;
        let resume_epoch = self.next_epoch();

        cx.spawn(async move |this, cx| {
            Timer::after(PAUSE_DELAY).await;

            if let Some(this) = this.upgrade() {
                this.update(cx, |this, cx| {
                    if this.pause_epoch == pause_epoch {
                        this.paused = false;
                        this.blink(resume_epoch, cx);
                    }
                })
                .ok();
            }
        })
        .detach();
    }
}
