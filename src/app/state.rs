#[derive(Clone)]
pub enum AppState {
    Init,
    Initialized {
        search: String,
        counter_tick: u64,
    },
}

impl AppState {
    pub fn initialized() -> Self {
        let counter_tick = 0;
        let search = String::new();
        Self::Initialized {
            counter_tick,
            search,
        }
    }

    pub fn is_initialized(&self) -> bool {
        matches!(self, &Self::Initialized { .. })
    }

    pub fn incr_tick(&mut self) {
        if let Self::Initialized { counter_tick, .. } = self {
            *counter_tick += 1;
        }
    }

    pub fn count_tick(&self) -> Option<u64> {
        if let Self::Initialized { counter_tick, .. } = self {
            Some(*counter_tick)
        } else {
            None
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::Init
    }
}
