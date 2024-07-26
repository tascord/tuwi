use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};

use super::{add_cursor, FormItem};

#[derive(Clone)]
pub enum ListCursor {
    Item(usize),
    Input(usize),
}

#[derive(Clone)]
pub struct List {
    pub name: String,
    pub title: String,
    pub values: Vec<String>,
    pub focused: bool,
    buffer: String,
    cursor: ListCursor,
}

impl List {
    pub fn new(name: &str, title: &str, values: Vec<String>) -> Self {
        Self {
            values,
            name: name.to_string(),
            title: title.to_string(),
            focused: false,
            cursor: ListCursor::Input(0),
            buffer: String::new(),
        }
    }

    fn set_editing_cursor_position(&mut self, i: usize) {
        self.cursor = ListCursor::Input(i);
    }
}

impl FormItem for List {
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
        matches!(self.cursor, ListCursor::Input(_))
    }

    fn should_prevent_nav(&self, k: KeyCode) -> bool {
        match self.cursor {
            ListCursor::Item(i) => (i != 0 && KeyCode::Up == k) || KeyCode::Down == k,
            _ => KeyCode::Up == k,
        }
    }

    fn height(&self) -> u16 {
        self.values.len() as u16 + 4 // Border + Input + Spacer
    }

    fn input(&mut self, k: KeyCode) {
        match self.cursor {
            ListCursor::Item(i) => match k {
                KeyCode::Up => {
                    if i > 0 {
                        self.cursor = ListCursor::Item(i - 1);
                    }
                }
                KeyCode::Down => {
                    if i < self.values.len().saturating_sub(1) {
                        self.cursor = ListCursor::Item(i + 1);
                    } else {
                        self.cursor = ListCursor::Input(self.buffer.len());
                    }
                }
                KeyCode::Backspace | KeyCode::Delete => {
                    if i < self.values.len() {
                        self.values.remove(i);
                        self.cursor = match self.values.len() {
                            0 => ListCursor::Input(0),
                            _ => ListCursor::Item(i.saturating_sub(1)),
                        };
                    }
                }
                _ => {}
            },
            ListCursor::Input(i) => match k {
                KeyCode::Up => {
                    self.cursor = ListCursor::Item(self.values.len().saturating_sub(1));
                }
                KeyCode::Backspace => {
                    if i > 0 {
                        self.buffer.remove(i as usize - 1);
                        self.set_editing_cursor_position(i - 1);
                    }
                }
                KeyCode::Left => {
                    if i > 0 {
                        self.set_editing_cursor_position(i - 1);
                    }
                }
                KeyCode::Right => {
                    if i < self.buffer.len() {
                        self.set_editing_cursor_position(i + 1);
                    }
                }
                KeyCode::Home => {
                    self.set_editing_cursor_position(0);
                }
                KeyCode::End => {
                    self.set_editing_cursor_position(self.buffer.len());
                }
                KeyCode::Delete => {
                    self.buffer.remove(i);
                }
                KeyCode::Char(c) => {
                    self.buffer.insert(i, c);
                    self.set_editing_cursor_position(i + 1);
                }
                KeyCode::Enter => {
                    self.values.push(self.buffer.clone());
                    self.buffer.clear();
                    self.cursor = ListCursor::Item(self.values.len() - 1);
                }
                _ => {}
            },
        }
    }
}

impl Widget for List
where
    Self: FormItem,
{
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        // Border

        Block::default()
            .title(self.title)
            .borders(Borders::ALL)
            .border_style(match self.focused {
                true => ratatui::style::Style::default().fg(ratatui::style::Color::Yellow),
                false => ratatui::style::Style::default(),
            })
            .render(area, buf);

        // Values

        for (i, value) in self.values.iter().enumerate() {
            let is_selected = matches!(self.cursor, ListCursor::Item(j) if i == j);
            Paragraph::new(value.to_string())
                .style(match is_selected {
                    true => Style::default()
                        .fg(ratatui::style::Color::White)
                        .bg(ratatui::style::Color::Yellow),
                    false => Style::default()
                        .fg(ratatui::style::Color::Black)
                        .bg(ratatui::style::Color::White),
                })
                .alignment(Alignment::Center)
                .render(
                    Rect {
                        x: area.x + 1,
                        y: area.y + 1 + i as u16,
                        width: area.width - 2,
                        height: 1,
                    },
                    buf,
                );
        }

        // Spacer
        Paragraph::new("---")
            .style(Style::default().fg(ratatui::style::Color::Gray))
            .alignment(Alignment::Center)
            .render(
                Rect {
                    x: area.x + 1,
                    y: area.y + 1 + self.values.len() as u16,
                    width: area.width - 2,
                    height: 1,
                },
                buf,
            );

        // Input

        let input_area = Rect {
            x: area.x + 1,
            y: area.y + area.height - 2,
            width: area.width - 2,
            height: 1,
        };

        let mut offset = 0;
        let input_cursor = match self.cursor {
            ListCursor::Item(_) => self.buffer.len(),
            ListCursor::Input(i) => i,
        };

        let slice = match area.columns().count() < self.buffer.len() + 3
            && input_cursor > area.columns().count()
        {
            false => self.buffer.clone(),
            true => {
                // slice with the cursor at the end
                offset = input_cursor - area.columns().count() + 3;
                self.buffer[offset..].to_string()
            }
        };

        Paragraph::new(
            match self.focused && matches!(self.cursor, ListCursor::Input(_)) {
                true => add_cursor(slice, input_cursor - offset),
                false => Line::raw(self.buffer.clone()),
            },
        )
        .render(input_area, buf);
    }
}
