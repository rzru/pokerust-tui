use tui::widgets::TableState;

pub struct SwitchableTableState(pub TableState);

impl SwitchableTableState {
    pub fn new() -> Self {
        Self(TableState::default())
    }

    pub fn next(&mut self, items_count: Option<usize>) {
        if items_count.unwrap_or(0) > 0 {
            self.0.select(Some(match self.0.selected() {
                Some(i) => {
                    if i >= items_count.unwrap_or(0) - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            }))
        }
    }

    pub fn previous(&mut self, items_count: Option<usize>) {
        if items_count.unwrap_or(0) > 0 {
            self.0.select(Some(match self.0.selected() {
                Some(i) => {
                    if i != 0 {
                        i - 1
                    } else {
                        items_count.unwrap_or(0) - 1
                    }
                }
                None => 0,
            }))
        }
    }
}
