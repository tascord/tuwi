use std::sync::Arc;

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};

use super::{Form, FormItem};
use crate::ab;

#[derive(Clone)]
pub struct Button<'a> {
    pub text: String,
    pub focused: bool,
    pub handler: Arc<Box<dyn Fn(&Form) + Send + Sync + 'a>>,
    pub name: String,
}

impl<'a> Button<'a> {
    pub fn new(name: &str, text: &str, handler: impl Fn(&Form) + Send + Sync + 'a) -> Self {
        Self {
            text: text.to_string(),
            focused: false,
            handler: ab!(handler),
            name: name.to_string(),
        }
    }
}

impl FormItem for Button<'_> {
    fn focus(&mut self) {
        self.focused = true;
    }

    fn blur(&mut self) {
        self.focused = false;
    }

    fn ren(&self, a: Rect, b: &mut Buffer) {
        self.clone().render(a, b);
    }

    fn submit(&self, f: &Form) -> bool {
        (self.handler)(f);
        true
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Widget for Button<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Paragraph::new(Line::from(self.text.clone()).style(Style::new().bold()))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(match self.focused {
                        true => ratatui::style::Style::default().fg(ratatui::style::Color::Yellow),
                        false => ratatui::style::Style::default(),
                    }),
            )
            .render(area, buf);
    }
}
