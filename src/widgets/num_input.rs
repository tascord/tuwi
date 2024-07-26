use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};

use super::{add_cursor, FormItem};

#[derive(Clone)]
pub struct NumInput {
    pub name: String,
    pub title: String,
    pub value: String,
    pub focused: bool,
    pub min: i64,
    pub max: i64,
    pub step: i64,
    cursor: usize,
}

impl NumInput {
    pub fn new(name: &str, title: &str, value: i64, range: (i64, i64), step: i64) -> Self {
        Self {
            title: title.to_string(),
            value: value.to_string(),
            focused: false,
            min: range.0,
            max: range.1,
            step,
            cursor: 0,
            name: name.to_string(),
        }
    }
}

impl FormItem for NumInput {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn ren(&self, a: Rect, b: &mut Buffer) {
        self.clone().render(a, b);
    }

    fn focus(&mut self) {
        self.focused = true;
    }

    fn blur(&mut self) {
        self.focused = false;
        self.value = self
            .value
            .trim()
            .parse::<i64>()
            .unwrap_or(0)
            .max(self.min)
            .min(self.max)
            .to_string();
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
                if c.is_digit(10) {
                    self.value.insert(self.cursor, c);
                    self.cursor += 1;
                }

                if c == '-' && self.min < 0 {
                    self.value = match self.value.starts_with('-') {
                        true => {
                            self.cursor = self.cursor.saturating_sub(1);
                            self.value[1..].to_string()
                        }
                        false => {
                            self.cursor = self.cursor.saturating_add(1).min(self.value.len() + 1);
                            format!("-{}", self.value)
                        }
                    };
                }
            }
            _ => {}
        }
    }
}

impl Widget for NumInput
where
    Self: FormItem,
{
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let mut offset = 0;
        let slice = match area.columns().count() < self.value.len() + 3
            && self.cursor > area.columns().count()
        {
            false => self.value.clone(),
            true => {
                // slice with the cursor at the end
                offset = self.cursor - area.columns().count() + 3;
                self.value[offset..].to_string()
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
            true => add_cursor(slice, self.cursor - offset),
            false => Line::raw(self.value.clone()),
        })
        .block(block)
        .render(area, buf);
    }
}
