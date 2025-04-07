use macroquad::input::mouse_position;
use macroquad::prelude::*;
use crate::assets::GlobalAssets;
use crate::util::{draw_ansi_text, draw_rounded_rect, remove_ansii_escape_codes};

// draw a tooltip at the mouse position
pub fn tooltip<S: Into<String>>(text: S, global_assets: &GlobalAssets) {
    // get the mouse position
    let mouse_pos = mouse_position();
    // get the tooltip text
    let tooltip_text = text.into();
    // let tooltip_text = format!("{}Press {}E{}!",
    //                            better_term::Color::BrightWhite,
    //                            better_term::Color::BrightYellow,
    //                            better_term::Color::BrightWhite);
    let raw_text = remove_ansii_escape_codes(&tooltip_text);
    // get the font size
    let font_size = 8.0;
    // get the text size
    let text_size = measure_text(&raw_text, Some(&global_assets.font), font_size as u16, 1.0);
    // draw the tooltip background
    // draw_rectangle_ex(mouse_pos.0 + 8.0, mouse_pos.1 - 11.0, text_size.width + 4.0, text_size.height + 5.0, DrawRectangleParams {
    //     offset: Default::default(),
    //     rotation: 0.0,
    //     color: Color::from_rgba(0, 0, 0, 80),
    // });
    // draw_rectangle_lines_ex(mouse_pos.0 + 8.0, mouse_pos.1 - 11.0, text_size.width + 4.0, text_size.height + 5.0, 1.0, DrawRectangleParams {
    //     offset: Default::default(),
    //     rotation: 0.0,
    //     color: Color::from_rgba(0, 0, 0, 255),
    // });
    draw_rounded_rect(
        vec2(mouse_pos.0 + 8.0, mouse_pos.1 - 11.0),
        vec2(text_size.width + 4.0, text_size.height + 4.0),
        2.0,
        Color::from_rgba(0, 0, 0, 150),
        true,
        Some(Color::from_rgba(0, 0, 0, 255)));

    draw_ansi_text(&tooltip_text, vec2(mouse_pos.0 + 10.0, mouse_pos.1), &global_assets, font_size as u16, 4.0);

    // draw the tooltip text
    // draw_text_ex(&tooltip_text,
    //           mouse_pos.0 + 10.0,
    //           mouse_pos.1,
    //           TextParams {
    //                 font: Some(&global_assets.font),
    //                 font_size: font_size as u16,
    //                 color: WHITE,
    //                 ..Default::default()
    //           });
}

#[derive(Clone, Debug)]
pub struct ToolTipCard {
    pub title: String,
    pub lines: Vec<String>,
}

impl ToolTipCard {
    pub fn new<S: Into<String>, S2: Into<String>>(title: S, lines: Vec<S2>) -> Self {
        let title = title.into();
        let lines = lines.into_iter().map(|s| s.into()).collect();
        ToolTipCard { title, lines }
    }

    pub fn from_string<S: Into<String>>(full_string: S) -> Self {
        // split by newlines, first is title rest are lines
        let full_string = full_string.into();
        let mut lines = full_string.split('\n');
        let title = lines.next().unwrap_or("").to_string();
        let lines = lines.map(|s| s.to_string()).collect();
        ToolTipCard { title, lines }
    }
}

pub fn tooltip_card(card: ToolTipCard, global_assets: &GlobalAssets) {
    let mouse_pos = mouse_position();
    let title_text = card.title;
    let line_texts: Vec<String> = card.lines;

    let title_font_size = 11.0;
    let line_font_size = title_font_size * 0.8;
    let padding = 4.0;
    let line_spacing = 2.0;

    // Measure title
    let raw_title = remove_ansii_escape_codes(&title_text);
    let title_size = measure_text(&raw_title, Some(&global_assets.font), title_font_size as u16, 1.0);

    // Measure lines
    let mut max_width = title_size.width;
    let mut total_height = title_size.height;

    let mut measured_lines = Vec::new();
    for line in &line_texts {
        let raw_line = remove_ansii_escape_codes(line);
        let size = measure_text(&raw_line, Some(&global_assets.font), line_font_size as u16, 1.0);
        max_width = max_width.max(size.width);
        measured_lines.push(size);
    }

    // Calculate total height correctly (spacing only between lines)
    if !measured_lines.is_empty() {
        let lines_height: f32 = measured_lines.iter().map(|s| s.height).sum();
        let spacing_total = line_spacing * (measured_lines.len() as f32);
        total_height += lines_height + spacing_total;
    }

    // Draw background with border
    let bg_pos = vec2(mouse_pos.0 + 12.0, mouse_pos.1 - 30.0);
    let bg_size = vec2(max_width + padding * 2.0, total_height + padding * 2.0);

    draw_rounded_rect(
        bg_pos,
        bg_size,
        2.0,
        Color::from_rgba(0, 0, 0, 150),
        true,
        Some(Color::from_rgba(0, 0, 0, 255)),
    );

    // Draw title
    let mut draw_pos = vec2(bg_pos.x + padding, bg_pos.y + padding + title_size.height);
    draw_ansi_text(&title_text, draw_pos, global_assets, title_font_size as u16, 4.0);

    // Draw lines
    for (i, line) in line_texts.iter().enumerate() {
        draw_pos.y += measured_lines[i].height + line_spacing;
        draw_ansi_text(line, draw_pos, global_assets, line_font_size as u16, 4.0);
    }
}