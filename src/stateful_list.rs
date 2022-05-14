use tui::widgets::ListState;

pub struct StatefulList<T: Clone> {
    pub state: ListState,
    pub items: Vec<T>,
    pub items_to_render: Vec<T>,
}

impl<T: Clone> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        let mut state = ListState::default();
        let items_to_render = items.to_vec();
        state.select(Some(0));

        StatefulList {
            state,
            items,
            items_to_render,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items_to_render.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items_to_render.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    // pub fn unselect(&mut self) {
    //     self.state.select(None);
    // }

    pub fn get_selected(&self) -> Option<&T> {
        let selected = self.state.selected();

        if let Some(i) = selected {
            return self.items_to_render.get(i);
        }

        None
    }
}
