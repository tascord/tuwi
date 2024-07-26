use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use super::FormItem;

pub fn add_cursor<'a>(s: String, c: usize) -> Line<'a> {
    Line::from(vec![
        Span::raw(s[..c].to_string()),
        Span::styled(
            s.clone().chars().nth(c).unwrap_or(' ').to_string(),
            Style::new().bg(Color::Yellow),
        ),
        Span::raw(match c == s.len() {
            true => String::new(),
            false => s[c + 1..].to_string(),
        }),
    ])
}

#[derive(Clone)]
pub struct Input {
    pub name: String,
    pub title: String,
    pub value: String,
    pub focused: bool,
    cursor: usize,
}

impl Input {
    pub fn new(name: &str, title: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            title: title.to_string(),
            value: value.to_string(),
            focused: false,
            cursor: value.len(),
        }
    }
}

impl FormItem for Input {
    fn ren(&self, a: Rect, b: &mut Buffer) {
        self.clone().render(a, b);
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

    fn name(&self) -> String {
        self.name.clone()
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

impl Widget for Input
where
    Self: FormItem,
{
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
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
