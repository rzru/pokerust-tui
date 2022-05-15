use tui::widgets::TableState;

pub struct SwitchableTableState(pub TableState);

impl SwitchableTableState {
    pub fn new() -> Self {
        Self(TableState::default())
    }

    pub fn next<T>(&mut self, items: &Vec<T>) {
        self.0.select(Some(match self.0.selected() {
            Some(i) => {
                if i >= items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        }))
    }

    pub fn previous<T>(&mut self, items: &Vec<T>) {
        self.0.select(Some(match self.0.selected() {
            Some(i) => {
                if i != 0 {
                    i - 1
                } else {
                    items.len() - 1
                }
            }
            None => 0,
        }))
    }
}
