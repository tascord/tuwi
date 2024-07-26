use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use super::FormItem;

pub fn add_reveal_cursor<'a>(s: String, c: usize, cc: char) -> Line<'a> {
    Line::from(vec![
        Span::raw(s[..c].to_string()),
        Span::styled(cc.to_string(), Style::new().bg(Color::Yellow)),
        Span::raw(match c == s.len() {
            true => String::new(),
            false => s[c + 1..].to_string(),
        }),
    ])
}

#[derive(Clone)]
pub struct Secret {
    pub title: String,
    pub name: String,
    pub value: String,
    pub focused: bool,
    cursor: usize,
}

impl Secret {
    pub fn new(name: &str, title: &str, value: &str) -> Self {
        Self {
            title: title.to_string(),
            name: name.to_string(),
            value: value.to_string(),
            focused: false,
            cursor: value.len(),
        }
    }
}

impl FormItem for Secret {
    fn ren(&self, a: Rect, b: &mut Buffer) {
        self.clone().render(a, b);
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn focus(&mut self) {
        self.focused = true;
    }

    fn blur(&mut self) {
        self.focused = false;
    }

    fn should_prevent_q(&self) -> bool {
        true
    }

    fn value(&self) -> Option<String> {
        Some(self.value.clone())
    }

    fn input(&mut self, k: KeyCode) {
        match k {
            KeyCode::Backspace => {
                if self.cursor > 0 {
                    self.value.remove(self.cursor - 1);
                    self.cursor -= 1;
                }
            }
            KeyCode::Left => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
            }
            KeyCode::Right => {
                if self.cursor < self.value.len() {
                    self.cursor += 1;
                }
            }
            KeyCode::Home => {
                self.cursor = 0;
            }
            KeyCode::End => {
                self.cursor = self.value.len();
            }
            KeyCode::Delete => {
                self.value.remove(self.cursor);
            }
            KeyCode::Char(c) => {
                self.value.insert(self.cursor, c);
                self.cursor += 1;
            }
            _ => {}
        }
    }
}

impl Widget for Secret
where
    Self: FormItem,
{
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let val = "*".repeat(self.value.len());

        let mut offset = 0;
        let slice: String =
            match area.columns().count() < val.len() + 3 && self.cursor > area.columns().count() {
                false => val.clone(),
                true => {
                    // slice with the cursor at the end
                    offset = self.cursor - area.columns().count() + 3;
                    val[offset..].to_string()
                }
            };

        let block = Block::default()
            .title(self.title)
            .borders(Borders::ALL)
            .border_style(match self.focused {
                true => ratatui::style::Style::default().fg(ratatui::style::Color::Yellow),
                false => ratatui::style::Style::default(),
            });

        Paragraph::new(match self.focused {
            true => add_reveal_cursor(
                slice,
                self.cursor - offset,
                self.value
                    .clone()
                    .chars()
                    .nth(self.cursor - offset)
                    .unwrap_or(' '),
            ),
            false => Line::raw(val.clone()),
        })
        .block(block)
        .render(area, buf);
    }
}
