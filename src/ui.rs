use std::cmp::{self, Reverse};

use chrono::Utc;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Offset, Rect},
    style::{Color, Style, Stylize},
    symbols,
    text::Line,
    widgets::{Block, BorderType, Clear, LineGauge, Padding, Paragraph, Widget, Wrap},
};
use ratatui_image::Image;
use unicode_width::UnicodeWidthStr;

const WIDE_BREAK: u16 = 92;

use crate::app::App;

// Compute min_width of a piece of text (kinda like css min-width I think)
fn compute_min_width(content: &str, wrap: bool) -> u16 {
    if wrap {
        // If we can wrap, calculate the longest word
        content
            .split_whitespace()
            .fold(0, |max, word| cmp::max(max, word.width())) as u16
    } else {
        content.width() as u16
    }
}

impl App {
    fn render_frame(&self, area: Rect, buf: &mut Buffer) {
        // Display the current streetview frame
        if let Some(proto) = &self.cur_frame {
            let image = Image::new(proto);
            image.render(area, buf);
        }
    }

    fn render_location(&self, area: Rect, buf: &mut Buffer) {
        if let Some(location) = &self.location {
            let content = format!("{}, {}", location.neighborhood, location.country);
            let is_wide = area.width > WIDE_BREAK;

            let (town_box, padding) = self.compute_town_box_layout(area, &content, is_wide);
            self.render_town_box(buf, town_box, &content, is_wide, padding);

            let street_box = self.compute_street_box_layout(area, town_box, &location.road);
            self.render_street_box(buf, street_box, &location.road);
        }
    }

    fn compute_town_box_layout(&self, area: Rect, content: &str, is_wide: bool) -> (Rect, Padding) {
        let padding = if is_wide {
            Padding::uniform(1)
        } else {
            Padding::ZERO
        };
        let box_width = Self::calculate_box_width(content, is_wide, padding, area.width);
        let box_height = if content.width() as u16 + padding.left + padding.right + 2 <= box_width {
            1 + padding.top + padding.bottom + 2
        } else {
            2 + padding.top + padding.bottom + 2
        };

        let box_rect = if is_wide {
            Rect::new(0, 0, area.width, box_height)
                .centered_horizontally(Constraint::Max(box_width))
        } else {
            Rect::new(0, 4, area.width, box_height)
                .centered_horizontally(Constraint::Max(box_width))
        };

        (box_rect, padding)
    }

    fn calculate_box_width(content: &str, is_wide: bool, padding: Padding, area_width: u16) -> u16 {
        let content_width = content.width() as u16;
        let min_content_width = compute_min_width(content, is_wide);
        let preferred = content_width + padding.right + padding.left + 2;
        let minimum = min_content_width + padding.left + padding.right + 2;

        cmp::min(cmp::max(minimum, area_width / 2), preferred)
    }

    fn render_town_box(
        &self,
        buf: &mut Buffer,
        box_rect: Rect,
        content: &str,
        is_wide: bool,
        padding: Padding,
    ) {
        let mut town_name = Paragraph::new(content)
            .style(Style::default().bg(Color::Rgb(0, 132, 48)).fg(Color::White))
            .centered()
            .bold()
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .padding(padding),
            );

        if is_wide {
            town_name = town_name.wrap(Wrap { trim: true });
        }

        Clear.render(box_rect, buf);
        town_name.render(box_rect, buf);
    }

    fn compute_street_box_layout(&self, area: Rect, town_box: Rect, road: &str) -> Rect {
        Rect::new(0, town_box.bottom(), area.width, 3)
            .centered_horizontally(Constraint::Max(road.len() as u16 + 2))
    }

    fn render_street_box(&self, buf: &mut Buffer, street_box: Rect, road: &str) {
        let street_name = Paragraph::new(road)
            .style(Style::default().bg(Color::White).fg(Color::Black))
            .centered()
            .block(Block::bordered().border_type(BorderType::Rounded));
        street_name.render(street_box, buf);
    }

    /// Render the box with the current drivers count in the top right corner of the rect
    fn render_drivers_online(&self, area: Rect, buf: &mut Buffer) {
        let content: Line = vec![
            "● ".red(),
            format!("{} drivers online", self.users_online).black(),
        ]
        .into();
        let content_width = content.width() as u16 + 4;
        let content_rect = Rect::new(
            area.width.saturating_sub(content_width),
            0,
            content_width,
            3,
        )
        .clamp(area);
        Clear.render(content_rect, buf);
        let drivers_online = Paragraph::new(content)
            .style(Style::default().bg(Color::Rgb(255, 242, 2)))
            .centered()
            .block(Block::bordered().border_type(BorderType::Rounded).black())
            .bold();
        drivers_online.render(content_rect, buf);
    }

    fn render_vote_counts(&self, area: Rect, buf: &mut Buffer) {
        // Render the vote counts box
        if let Some(end_time) = self.vote_ends
            && let Some((_, heading)) = &self.current_pano
        {
            let secs_until = end_time.signed_duration_since(Utc::now()).num_seconds();
            let picking_in = if secs_until > 0 {
                format!("Picking option in {secs_until} seconds...")
            } else {
                "Picking option...".to_string()
            };
            let content_width = 21;
            let content_rect = Rect::new(
                area.width.saturating_sub(content_width),
                4,
                content_width,
                18,
            )
            .clamp(area);
            let block = Block::bordered()
                .border_type(BorderType::Rounded)
                .bg(Color::Rgb(255, 255, 255))
                .black();
            let inner_rect = block.inner(content_rect);

            Clear.render(content_rect, buf);
            block.render(content_rect, buf);

            Paragraph::new(picking_in)
                .black()
                .wrap(Wrap { trim: false })
                .render(inner_rect, buf);

            let mut vote_counts: Vec<_> = self.vote_counts.iter().collect();
            vote_counts.sort_by_key(|(idx, count)| Reverse((**count, **idx)));
            let total = vote_counts
                .iter()
                .map(|(_, count)| **count)
                .sum::<u16>()
                .max(1);

            let counts_rect = inner_rect.offset(Offset::new(0, 3));

            for (offset, (idx, count)) in vote_counts.iter().take(4).enumerate() {
                let mut emoji = match idx {
                    -1 => "⏭",
                    -2 => "📢",
                    0.. => {
                        let aro_heading = self.vote_options[**idx as usize].heading;
                        let heading_diff = aro_heading - heading;
                        match heading_diff.round() as i16 {
                            -102..-67 => "⬅️", // TODO: Better emoji support
                            -67..-22 => "↖️",
                            -22..23 => "⬆️",
                            23..68 => "↗️",
                            68..102 => "➡️",
                            _ => "",
                        }
                    }
                    _ => "",
                }
                .to_string();
                let ratio = **count as f64 / total as f64;
                emoji.extend(std::iter::repeat_n(" ", 2 - emoji.width()));

                let text = format!("{emoji} {count} votes");
                let percentage = format!("{}%", (ratio * 100.0).round());

                let text = Paragraph::new(text).left_aligned().black();
                let percent = Paragraph::new(percentage).right_aligned().black();

                let gauge = LineGauge::default()
                    .filled_symbol(symbols::line::THICK_HORIZONTAL)
                    .unfilled_symbol(" ")
                    .black()
                    .label(Line::default())
                    .ratio(ratio);

                let text_rect = Rect::new(
                    counts_rect.x,
                    counts_rect.y + (offset as u16 * 2),
                    counts_rect.width,
                    1,
                )
                .clamp(area);
                let gauge_rect = Rect::new(
                    text_rect.x + 2,
                    text_rect.y + 1,
                    text_rect.width - 2,
                    text_rect.height,
                )
                .clamp(area);

                text.render(text_rect, buf);
                percent.render(text_rect, buf);
                gauge.render(gauge_rect, buf);
            }
        }
    }
}

impl Widget for &App {
    /// Render the whole UI.
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_frame(area, buf);
        self.render_location(area, buf);
        self.render_drivers_online(area, buf);
        self.render_vote_counts(area, buf);
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use crate::{event::EventHandler, roadtrip::Location};

    use pretty_assertions::assert_eq;

    use super::*;

    /// check if the area and content (raw text) of two buffers are the same
    pub fn assert_buffer_eq(buffer: &ratatui::buffer::Buffer, expected: &ratatui::buffer::Buffer) {
        // if this is false, the test passes
        if buffer.area() != expected.area()
            || !buffer
                .content()
                .iter()
                .zip(expected.content().iter())
                .all(|(a, b)| a.symbol() == b.symbol())
        {
            // otherwise, let's "assert" that they are the same, simply so that `pretty_assertions::assert_eq` will print the diff
            pretty_assertions::assert_eq!(buffer, expected);
        }
    }

    #[test]
    fn test_full_render() {
        let (tx, _) = tokio::sync::mpsc::channel(100);
        let app = App::new(EventHandler::new_deterministic(), tx);

        let area = Rect::new(0, 0, 100, 50);
        let mut buf = Buffer::empty(area);
        app.render(area, &mut buf);

        // Narrow
        let area = Rect::new(0, 0, 20, 70);
        let mut buf = Buffer::empty(area);
        app.render(area, &mut buf);

        // Very small
        let area = Rect::new(0, 0, 5, 5);
        let mut buf = Buffer::empty(area);
        app.render(area, &mut buf);

        // Very wide
        let area = Rect::new(0, 0, 300, 50);
        let mut buf = Buffer::empty(area);
        app.render(area, &mut buf);
    }

    #[test]
    fn test_render_drivers_online() {
        let (tx, _) = tokio::sync::mpsc::channel(100);
        let mut app = App::new(EventHandler::new_deterministic(), tx);
        app.users_online = 100;

        let area = Rect::new(0, 0, 30, 5);
        let mut buf = Buffer::empty(area);
        app.render_drivers_online(area, &mut buf);

        assert_buffer_eq(
            &buf,
            &Buffer::with_lines(vec![
                "      ╭──────────────────────╮",
                "      │ ● 100 drivers online │",
                "      ╰──────────────────────╯",
                "                              ",
                "                              ",
            ]),
        );

        // Text is clipped if the rect is too narrow
        let area = Rect::new(0, 0, 10, 5);
        let mut buf = Buffer::empty(area);
        app.render_drivers_online(area, &mut buf);

        assert_buffer_eq(
            &buf,
            &Buffer::with_lines(vec![
                "╭────────╮",
                "│● 100 dr│",
                "╰────────╯",
                "          ",
                "          ",
            ]),
        );
    }

    #[test]
    fn test_location_render() {
        let (tx, _) = tokio::sync::mpsc::channel(100);
        let mut app = App::new(EventHandler::new_deterministic(), tx);

        app.location = Some(Location {
            neighborhood: "Town of East Hampton".to_string(), // Wide text for testing
            country: "United States of America".to_string(),
            road: "Main Street".to_string(),
            county: "Suffolk County".to_string(), // Random
            state: "New York".to_string(),        // Random
        });

        let area = Rect::new(0, 0, 100, 10); // Wide layout
        let mut buf = Buffer::empty(area);
        app.render_location(area, &mut buf);

        assert_buffer_eq(
            &buf,
            &Buffer::with_lines(vec![
                //         1         2.        3.        4         5.        6.        7.        8.        9.        0
                //1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890
                "                         ╭────────────────────────────────────────────────╮                         ",
                "                         │                                                │                         ",
                "                         │ Town of East Hampton, United States of America │                         ",
                "                         │                                                │                         ",
                "                         ╰────────────────────────────────────────────────╯                         ",
                "                                            ╭───────────╮                                           ",
                "                                            │Main Street│                                           ",
                "                                            ╰───────────╯                                           ",
                "                                                                                                    ",
                "                                                                                                    ",
            ]),
        );
    }

    #[test]
    fn test_min_width() {
        assert_eq!(compute_min_width("123 1234", true), 4);
        assert_eq!(
            compute_min_width("Town of East Hampton, United States of America", true),
            8
        );
    }

    #[test]
    fn test_min_width_no_wrap() {
        assert_eq!(compute_min_width("123 1234", false), 8);
    }

    #[test]
    fn test_min_width_empty() {
        assert_eq!(compute_min_width("", true), 0);
    }

    #[test]
    fn test_min_width_single_word() {
        assert_eq!(compute_min_width("hello", true), 5);
    }

    #[test]
    fn test_min_width_unicode() {
        // "é" = width 1, "界" = width 2
        assert_eq!(compute_min_width("éé é", true), 2);
        assert_eq!(compute_min_width("hello 世界", true), 5);
    }
}
