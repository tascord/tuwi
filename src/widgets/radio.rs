use std::ops::{Add, Sub};

use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::{self, Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use super::FormItem;

#[derive(Clone)]
pub struct Radio {
    pub title: String,
    pub name: String,
    pub value: String,
    pub options: Vec<(String, String)>,
    pub focused: bool,
}

impl Radio {
    /// (Label, Value)
    pub fn new(name: &str, title: &str, options: Vec<(&str, &str)>, value: &str) -> Self {
        Self {
            name: name.to_string(),
            title: title.to_string(),
            options: options
                .iter()
                .map(|a| (a.0.to_string(), a.1.to_string()))
                .collect::<Vec<_>>(),
            value: value.to_string(),
            focused: false,
        }
    }

    fn get_index_of_selected(&self) -> isize {
        self.options
            .iter()
            .position(|(_, a)| a == &self.value)
            .unwrap_or(0) as isize
    }
}

impl FormItem for Radio {
    fn focus(&mut self) {
        self.focused = true;
    }

    fn name(&self) -> String {
        self.name.clone()
    }
    fn blur(&mut self) {
        self.focused = false;
    }

    fn ren(&self, a: Rect, b: &mut Buffer) {
        self.clone().render(a, b);
    }

    fn input(&mut self, k: KeyCode) {
        if self.options.is_empty() {
            return;
        }

        match k {
            KeyCode::Left => {
                self.value = self.options[(self.get_index_of_selected().sub(1)
                    % self.options.len() as isize)
                    .unsigned_abs()]
                .1
                .clone()
            }
            KeyCode::Right => {
                self.value = self.options
                    [(self.get_index_of_selected().add(1) % self.options.len() as isize) as usize]
                    .1
                    .clone()
            }
            KeyCode::Home => self.value = self.options.first().unwrap().1.clone(),
            KeyCode::End => self.value = self.options.last().unwrap().1.clone(),
            _ => {}
        }
    }

    fn value(&self) -> Option<String> {
        Some(self.value.clone())
    }
}

impl Widget for Radio {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let inner_0 = Layout::new(
            layout::Direction::Vertical,
            vec![
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(1),
            ],
        )
        .split(area);

        let inner_1 = Layout::new(
            layout::Direction::Horizontal,
            self.options.iter().map(|_| Constraint::Fill(1)),
        )
        .flex(layout::Flex::SpaceBetween)
        .split(inner_0[1]);

        Block::new()
            .title(self.title)
            .borders(Borders::ALL)
            .border_style(match self.focused {
                true => ratatui::style::Style::default().fg(ratatui::style::Color::Yellow),
                false => ratatui::style::Style::default(),
            })
            .render(area, buf);

        self.options.iter().enumerate().for_each(|(i, (l, v))| {
            Paragraph::new(Line::from(vec![Span::styled(l.clone(), {
                let st = Style::new();
                match self.value == v.to_string() {
                    true => st.bold().fg(Color::Yellow),
                    false => st,
                }
            })]))
            .alignment(Alignment::Center)
            .render(inner_1[i], buf);
        });
    }
}
