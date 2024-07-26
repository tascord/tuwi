use std::sync::{Arc, RwLock};

use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::{self, Constraint, Flex, Layout, Margin, Position, Rect},
    text::{Line, Text},
    widgets::{Block, Borders, Clear, Padding, Paragraph, Widget, Wrap},
};

mod button;
mod input;
mod list;
mod num_input;
mod radio;
mod scroll;
mod secret;
mod slider;

pub use button::*;
pub use input::*;
pub use list::*;
pub use num_input::*;
pub use radio::*;
pub use scroll::*;
pub use secret::*;
pub use slider::*;

#[derive(Clone)]
pub struct Popup<'a> {
    pub title: Line<'a>,
    pub content: Text<'a>,
    pub form: Option<Form>,
}

impl Widget for Popup<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Clear.render(area, buf);
        let block = Block::new()
            .title(self.title)
            .borders(Borders::ALL)
            .padding(Padding::symmetric(1, 1));

        let para = Paragraph::new(self.content).wrap(Wrap { trim: true });

        let height = para.line_count(area.width) as u16;
        para.block(block).render(area, buf);

        if let Some(form) = &self.form {
            let layout = Layout::new(
                layout::Direction::Vertical,
                vec![Constraint::Length(height + 2), Constraint::Fill(1)],
            )
            .split(area);

            let form_height = form
                .items
                .iter()
                .fold(0, |a, i| a + i.read().unwrap().height());

            let mut area = layout[1].inner(Margin::new(1, 1));
            area.height = area.height.min(form_height + form.items.len() as u16 + 1);
            area.y = area.y.max(layout[1].y + layout[1].height - area.height) + 1;

            form.clone().render(area, buf);
        }
    }
}

impl Popup<'_> {
    pub fn handle_input(&mut self, k: KeyCode) -> bool {
        if let Some(form) = self.form.as_mut() {
            form.handle_input(k)
        } else {
            false
        }
    }
}

//

#[allow(unused_variables)]
pub trait FormItem: Send + Sync {
    fn focus(&mut self);
    fn blur(&mut self);
    fn ren(&self, a: Rect, b: &mut Buffer);
    fn name(&self) -> String;
    fn input(&mut self, k: KeyCode) {}
    fn submit(&self, f: &Form) -> bool {
        false
    }
    fn value(&self) -> Option<String> {
        None
    }
    fn should_prevent_q(&self) -> bool {
        false
    }
    fn should_prevent_nav(&self, k: KeyCode) -> bool {
        false
    }
    fn height(&self) -> u16 {
        3
    }
}

#[derive(Default, Clone)]
pub struct Form {
    pub items: Vec<Arc<RwLock<Box<dyn FormItem>>>>,
    pub focused: usize,
    pub prevent_q: bool,
    pub title: Option<String>,
    pub borders: bool,
}

impl Form {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, item: impl FormItem + 'static) {
        self.items.push(Arc::new(RwLock::new(Box::new(item))));
        self.focus(self.items.len());
    }

    pub fn focus(&mut self, i: usize) {
        if self.items.is_empty() {
            return;
        }

        let i = i % self.items.len();
        self.items.iter().for_each(|i| i.write().unwrap().blur());
        self.items[i].write().unwrap().focus();
    }

    pub fn handle_input(&mut self, k: KeyCode) -> bool {
        if self.items.is_empty() {
            return false;
        }

        if self.items[self.focused]
            .read()
            .unwrap()
            .should_prevent_nav(k)
        {
            self.items[self.focused].write().unwrap().input(k);
            return false;
        }

        match k {
            KeyCode::Enter => {
                if self.items[self.focused].read().unwrap().submit(self) {
                    return true;
                }

                self.items[self.focused].write().unwrap().input(k);
            }
            KeyCode::Tab | KeyCode::Down => {
                self.items[self.focused].write().unwrap().blur();
                self.focused = (self.focused + 1) % self.items.len();
                self.items[self.focused].write().unwrap().focus();
                self.prevent_q = self.items[self.focused].read().unwrap().should_prevent_q();
            }
            KeyCode::BackTab | KeyCode::Up => {
                self.items[self.focused].write().unwrap().blur();
                self.focused = (self.focused + self.items.len() - 1) % self.items.len();
                self.items[self.focused].write().unwrap().focus();
                self.prevent_q = self.items[self.focused].read().unwrap().should_prevent_q();
            }
            _ => {
                self.items[self.focused].write().unwrap().input(k);
                self.prevent_q = self.items[self.focused].read().unwrap().should_prevent_q();
            }
        }

        false
    }

    pub fn slurp(&self) -> Vec<(String, String)> {
        self.items
            .iter()
            .map(|i| {
                let i = i.read().unwrap();
                (i.name(), i.value().unwrap_or_default())
            })
            .collect::<Vec<_>>()
    }

    pub fn ren(&self, area: Rect, buf: &mut Buffer) {
        let heights = self
            .items
            .iter()
            .map(|i| i.clone().read().unwrap().height());
        let height = heights.clone().sum::<u16>() + (self.items.len()).saturating_sub(1) as u16;

        Block::new()
            .borders(match self.borders {
                true => Borders::ALL,
                false => Borders::NONE,
            })
            .title(self.title.clone().unwrap_or_default())
            .render(area, buf);

        let border_offset = match self.borders {
            true => (1, 1),
            false => (0, 0),
        };

        let max_size = Rect {
            x: 0,
            y: 1,
            width: area.width - 2,
            height,
        };

        let mut scroll = Scroll::new(max_size).page_size(area);
        let mut scroll_offset = 0;
        let layout = layout::Layout::new(
            layout::Direction::Vertical,
            self.items
                .iter()
                .map(|i| Constraint::Length(i.clone().read().unwrap().height()))
                .collect::<Vec<_>>(),
        )
        .spacing(1)
        .flex(Flex::SpaceBetween)
        .split(max_size);

        for (i, item) in self.items.iter().enumerate() {
            let widget = item.read().unwrap();
            if self.focused == i {
                let feet_y =
                    heights.clone().take(i).sum::<u16>() + widget.height() as u16 + i as u16 + 3;
                if feet_y > area.height {
                    scroll_offset = feet_y - area.height;
                }
            }

            widget.ren(layout[i], scroll.buffer_mut());
        }

        scroll.set_offset(Position {
            y: scroll_offset,
            x: 0,
        });

        scroll.render(
            Rect {
                x: area.x + border_offset.0,
                y: area.y + border_offset.1,
                width: area.width - (border_offset.0 * 2),
                height: area.height - (border_offset.1 * 2),
            },
            buf,
        );
    }
}

impl Widget for Form {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        self.ren(area, buf);
    }
}

//

pub struct Titled {
    pub title: String,
    pub widget: Box<dyn Fn(Rect, &mut Buffer)>,
}

impl Titled {
    pub fn new(title: &str, maker: impl Fn(Rect, &mut Buffer) + 'static) -> Self {
        Self {
            title: title.to_string(),
            widget: Box::new(maker),
        }
    }

    pub fn ephemeral(title: &str, widget: impl Widget + Sized, area: Rect, buf: &mut Buffer) {
        let l0 = Layout::new(
            layout::Direction::Horizontal,
            vec![
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(1),
            ],
        )
        .split(area);

        let l1 = Layout::new(
            layout::Direction::Vertical,
            vec![
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(1),
            ],
        )
        .split(l0[1]);

        Block::new()
            .title(title)
            .borders(Borders::ALL)
            .render(area, buf);

        widget.render(l1[1], buf);
    }
}

impl Widget for Titled {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let l0 = Layout::new(
            layout::Direction::Horizontal,
            vec![
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(1),
            ],
        )
        .split(area);

        let l1 = Layout::new(
            layout::Direction::Vertical,
            vec![
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(1),
            ],
        )
        .split(l0[1]);

        Block::new()
            .title(self.title)
            .borders(Borders::ALL)
            .render(area, buf);

        (self.widget)(l1[1], buf);
    }
}
