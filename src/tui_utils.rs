use crate::tui_scrolllist;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::{
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Text, Widget},
    Frame,
};
use tui_scrolllist::ScrollableList;

/// use layouts to create a rects that
/// centers inside `r` and sizes `percent_x`/`percent_x` of `r`
pub fn centered_rect(
    percent_x: u16,
    percent_y: u16,
    r: Rect,
) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

pub fn draw_list<'b, B: Backend, L>(
    f: &mut Frame<B>,
    r: Rect,
    title: &'b String,
    items: L,
    select: Option<usize>,
    selected: bool,
) where
    L: Iterator<Item = Text<'b>>,
{
    let mut style_border = Style::default();
    let mut style_title = Style::default();
    if selected {
        style_border = style_border.fg(Color::Green);
        style_title = style_title.modifier(Modifier::BOLD);
    }
    ScrollableList::new(items)
        .block(
            Block::default()
                .title(title.as_str())
                .borders(Borders::ALL)
                .title_style(style_title)
                .border_style(style_border),
        )
        .scroll(select.unwrap_or_default())
        .style(Style::default().fg(Color::White))
        .render(f, r);
}