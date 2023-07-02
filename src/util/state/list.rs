use ratatui::widgets::ListState;

pub struct StatefulList<'l, E> {
    pub state: ListState,
    pub items: &'l Vec<String>,
    pub size: usize,
    pub edit_element: Option<E>
}

impl<'l, E> StatefulList<'l, E> {
    pub fn new_with_items(items: &Vec<String>) -> StatefulList<E> {
        let mut state = ListState::default();
        state.select(Some(0));

        StatefulList {
            state,
            items,
            size: 0,
            edit_element: None
        }
    }

    pub fn next(&mut self) {
        self.state.select(Some(match self.state.selected() {
            Some(i) => (i + 1) % self.items.len(),
            None => 0,
        }));
    }

    pub fn next_size(&mut self) {
        self.state.select(Some(match self.state.selected() {
            Some(i) => (i + 1) % self.size,
            None => 0,
        }));
    }

    pub fn previous(&mut self) {
        self.state.select(Some(match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        }));
    }

    pub fn previous_size(&mut self) {
        self.state.select(Some(match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.size - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        }));
    }
}
