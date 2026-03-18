use std::{cmp::Reverse, iter};

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
use tracing::debug;
use unicode_width::UnicodeWidthStr;

use crate::app::App;

impl Widget for &App {
    // TODO: Better code
    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Some(proto) = &self.cur_frame {
            let image = Image::new(proto);
            image.render(area, buf);
        }

        if let Some(location) = &self.location {
            let content = format!("{}, {}", location.neighborhood, location.country);
            let content_width = content.len() as u16;
            let content_rect = if area.width > 92 {
                Rect::new(0, 0, area.width, 5)
                    .centered_horizontally(Constraint::Max(content_width + 4))
            } else {
                Rect::new(0, 3, area.width, 3)
                    .centered_horizontally(Constraint::Max(content_width + 4))
            };

            let town_name = Paragraph::new(content)
                .style(Style::default().bg(Color::Rgb(0, 132, 48)).fg(Color::White))
                .centered()
                .bold()
                .block(
                    Block::bordered()
                        .border_type(BorderType::Rounded)
                        .padding(Padding::vertical(if area.width > 92 { 1 } else { 0 })),
                );

            Clear.render(content_rect, buf);

            town_name.render(content_rect, buf);

            let content_rect = Rect::new(0, content_rect.bottom(), area.width, 3)
                .centered_horizontally(Constraint::Max(location.road.len() as u16 + 2));

            let street_name = Paragraph::new(location.road.clone())
                .style(Style::default().bg(Color::White).fg(Color::Black))
                .centered()
                .block(Block::bordered().border_type(BorderType::Rounded));
            street_name.render(content_rect, buf);
        }

        let content: Line = vec![
            "● ".red(),
            format!("{} drivers online", self.users_online).black(),
        ]
        .into();
        let content_width = content.width() as u16 + 4;
        let content_rect = Rect::new(area.width - content_width, 0, content_width, 3);
        Clear.render(content_rect, buf);
        let drivers_online = Paragraph::new(content)
            .style(Style::default().bg(Color::Rgb(255, 242, 2)))
            .centered()
            .block(Block::bordered().border_type(BorderType::Rounded).black())
            .bold();
        drivers_online.render(content_rect, buf);

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
            let content_rect = Rect::new(area.width - content_width, 4, content_width, 18);
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
                .map(|(idx, count)| **count)
                .sum::<u16>()
                .max(1);

            debug!("Total votes: {total}");

            let counts_rect = inner_rect.offset(Offset::new(0, 3));

            for (offset, (idx, count)) in vote_counts.iter().take(4).enumerate() {
                let mut emoji = match idx {
                    -1 => "⏭",
                    -2 => "📢",
                    0.. => {
                        let aro_heading = self.vote_options[**idx as usize].heading;
                        let heading_diff = aro_heading - heading;
                        match heading_diff.round() as i16 {
                            -102..-67 => "⬅", // TODO: Better emoji support
                            -67..-22 => "↖",
                            -22..23 => "⬆",
                            23..68 => "↗",
                            68..102 => "➡︎",
                            _ => "",
                        }
                    }
                    _ => "",
                }
                .to_string();
                let ratio = **count as f64 / total as f64;
                debug!("Padding .{emoji}. Width: {}", emoji.width());
                emoji.extend(iter::repeat(" ").take(2 - emoji.width()));
                debug!("To .{emoji}.");

                let text = format!("{emoji} {count} votes");
                let percentage = format!("{}%", (ratio * 100.0).round());

                debug!("Text: {}", &text);

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
                );
                let gauge_rect = Rect::new(
                    text_rect.x + 2,
                    text_rect.y + 1,
                    text_rect.width - 2,
                    text_rect.height,
                );

                debug!("Text rect: {text_rect:?}");
                text.render(text_rect, buf);
                percent.render(text_rect, buf);
                gauge.render(gauge_rect, buf);
            }
        }
    }
}
