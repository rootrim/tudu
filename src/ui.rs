use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Borders, List, Paragraph, StatefulWidget, Widget},
};

use crate::{app::App, types::TodoState};

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, list_area, footer_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        let list_block = Block::default()
            .title(Line::raw("TODO List").centered())
            .borders(Borders::ALL);

        let list_items: Vec<Line> = self
            .todos
            .items
            .iter()
            .map(|todo| {
                let status = if todo.todo_state == TodoState::Completed {
                    "[x] "
                } else {
                    "[ ] "
                };
                Line::raw(format!("{}{}", status, todo.title))
            })
            .collect();

        let list = List::new(list_items)
            .block(list_block)
            .highlight_symbol(">")
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

        StatefulWidget::render(&list, list_area, buf, &mut self.todos.state);

        Paragraph::new("Use j/k to move, ESC to unselect, m to toggle complete, d to delete selected, a to add a new todo.")
            .centered()
            .render(footer_area, buf);

        Paragraph::new("Absolute TUDU")
            .bold()
            .centered()
            .render(header_area, buf);
    }
}
