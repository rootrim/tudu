use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, HighlightSpacing, List, Paragraph, StatefulWidget, Widget, Wrap},
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
                Line::from(vec![
                    Span::raw(if todo.todo_state == TodoState::Completed {
                        "[x] "
                    } else {
                        "[ ] "
                    }),
                    Span::styled(
                        todo.title.clone(),
                        Style::default().add_modifier(if todo.todo_state == TodoState::Completed {
                            Modifier::CROSSED_OUT | Modifier::DIM
                        } else {
                            Modifier::empty()
                        }),
                    ),
                ])
            })
            .collect();

        let list = List::new(list_items)
            .block(list_block)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(&list, list_area, buf, &mut self.todos.state);

        Paragraph::new(
            if self.is_editing {
                "Type the name and press Enter/ESC to save."
            } else {
            "Use j/k to move, ESC to unselect, m/SPACE to toggle complete, d to delete selected, a to add a new todo, r to rename selected, q to quit." } 
            )
            .centered()
            .wrap(Wrap { trim: true })
            .render(footer_area, buf);

        Paragraph::new("Absolute TUDU")
            .bold()
            .centered()
            .render(header_area, buf);
    }
}
