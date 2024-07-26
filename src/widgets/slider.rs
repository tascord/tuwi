use std::ops::{Add, Sub};

use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use super::FormItem;

pub fn map_range(value: i32, in_range: (i32, i32), out_range: (i32, i32)) -> i32 {
    let (in_min, in_max) = in_range;
    let (out_min, out_max) = out_range;
    let value = value.max(in_min).min(in_max);
    let mapped_value = (value - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;
    mapped_value
}

#[derive(Clone)]
pub struct Slider {
    pub name: String,
    pub title: String,
    pub value: i32,
    pub range: (i32, i32),
    pub focused: bool,
    pub units: Option<String>,
}

impl Slider {
    pub fn new(
        name: &str,
        title: &str,
        range: (i32, i32),
        value: i32,
        units: Option<&str>,
    ) -> Self {
        Self {
            name: name.to_string(),
            range,
            value,
            focused: false,
            title: title.to_string(),
            units: units.map(|s| s.to_string()),
        }
    }
}

impl FormItem for Slider {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn focus(&mut self) {
        self.focused = true;
    }

    fn blur(&mut self) {
        self.focused = false;
    }

    fn ren(&self, a: Rect, b: &mut Buffer) {
        self.clone().render(a, b);
    }

    fn value(&self) -> Option<String> {
        Some(self.value.to_string())
    }

    fn input(&mut self, k: KeyCode) {
        match k {
            KeyCode::Left => self.value = self.value.sub(1).max(self.range.0),
            KeyCode::Right => self.value = self.value.add(1).min(self.range.1),
            KeyCode::Home => self.value = self.range.0,
            KeyCode::End => self.value = self.range.1,
            _ => {}
        }
    }
}

impl Widget for Slider {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let char_area = area.right() - area.left();
        let filled_proportion = map_range(self.value, self.range, (0, char_area as i32));

        let bar = vec![
            Span::styled(
                " ".repeat(filled_proportion as usize),
                Style::new().bg(Color::Yellow),
            ),
            Span::styled(
                " ".repeat(char_area as usize - filled_proportion as usize),
                Style::new().bg(Color::White),
            ),
        ];

        Paragraph::new(Line::from(bar))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title(format!(
                        "{}: {}{}",
                        self.title,
                        self.value,
                        self.units.unwrap_or_default()
                    ))
                    .borders(Borders::ALL)
                    .border_style(match self.focused {
                        true => ratatui::style::Style::default().fg(ratatui::style::Color::Yellow),
                        false => ratatui::style::Style::default(),
                    }),
            )
            .render(area, buf);
    }
}
