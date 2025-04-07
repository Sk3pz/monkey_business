use macroquad::prelude::*;
use std::f32::consts::PI;
use crate::assets::GlobalAssets;

const DEFAULT_COLOR: Color = WHITE;

fn generate_rounded_perimeter(pos: Vec2, size: Vec2, radius: f32, segments_per_corner: u16) -> Vec<Vec2> {
    let mut points = Vec::new();
    let segments = segments_per_corner.max(1);

    let r = radius.min(size.x / 2.0).min(size.y / 2.0).max(0.0);

    // Early exit for sharp rectangle
    if r < 0.01 {
        return vec![
            pos,
            pos + vec2(size.x, 0.0),
            pos + vec2(size.x, size.y),
            pos + vec2(0.0, size.y),
        ];
    }

    let angle_step = PI / 2.0 / segments as f32;

    let centers = [
        vec2(pos.x + size.x - r, pos.y + r), // Top-right
        vec2(pos.x + size.x - r, pos.y + size.y - r), // Bottom-right
        vec2(pos.x + r, pos.y + size.y - r), // Bottom-left
        vec2(pos.x + r, pos.y + r), // Top-left
    ];

    let start_angles = [1.5 * PI, 0.0, 0.5 * PI, PI];

    for (center, start_angle) in centers.iter().zip(start_angles.iter()) {
        for i in 0..segments {
            let angle = start_angle + angle_step * i as f32;
            let x = center.x + r * angle.cos();
            let y = center.y + r * angle.sin();
            points.push(vec2(x.round(), y.round())); // << Rounds to nearest pixel
        }
    }

    points
}

pub fn draw_rounded_rect(
    pos: Vec2,
    size: Vec2,
    radius: f32,
    fill_color: Color,
    border: bool,
    border_color: Option<Color>,
) {
    let segments = 12;
    let thickness = if border { 1.0 } else { 0.0 };
    let bcolor = border_color.unwrap_or(BLACK);

    let outer = generate_rounded_perimeter(pos, size, radius, segments);

    if fill_color.a > 0.0 {
        let center = pos + size / 2.0;
        let mut verts = vec![Vertex {
            position: center.extend(0.0),
            uv: vec2(0.5, 0.5),
            color: fill_color.into(),
            normal: Default::default(),
        }];

        verts.extend(outer.iter().map(|&p| Vertex {
            position: p.extend(0.0),
            uv: vec2((p.x - pos.x) / size.x, (p.y - pos.y) / size.y),
            color: fill_color.into(),
            normal: Default::default(),
        }));

        let mut indices = Vec::new();
        for i in 1..=outer.len() as u16 {
            indices.extend_from_slice(&[
                0,
                i,
                if i == outer.len() as u16 { 1 } else { i + 1 },
            ]);
        }

        draw_mesh(&Mesh {
            vertices: verts,
            indices,
            texture: None,
        });
    }

    if thickness > 0.0 && size.x > 2.0 * thickness && size.y > 2.0 * thickness {
        let inner_pos = pos + vec2(thickness, thickness);
        let inner_size = size - vec2(2.0 * thickness, 2.0 * thickness);
        let inner = generate_rounded_perimeter(inner_pos, inner_size, (radius - thickness).max(0.0), segments);

        if inner.len() == outer.len() {
            let mut verts = Vec::new();
            verts.extend(outer.iter().map(|&p| Vertex {
                position: p.extend(0.0),
                uv: vec2(0.0, 0.0),
                color: bcolor.into(),
                normal: Default::default(),
            }));
            verts.extend(inner.iter().map(|&p| Vertex {
                position: p.extend(0.0),
                uv: vec2(1.0, 1.0),
                color: bcolor.into(),
                normal: Default::default(),
            }));

            let n = outer.len() as u16;
            let mut indices = Vec::new();
            for i in 0..n {
                let next = (i + 1) % n;
                indices.extend_from_slice(&[
                    i, next, n + i,
                    next, n + next, n + i,
                ]);
            }

            draw_mesh(&Mesh {
                vertices: verts,
                indices,
                texture: None,
            });
        }
    }
}

#[derive(Debug)]
struct TextSpan {
    text: String,
    color: Color,
}

/// Parses ANSI escape codes into colored spans
pub fn parse_ansi(input: &str) -> Vec<Vec<TextSpan>> {
    let mut spans: Vec<Vec<TextSpan>> = Vec::new();
    let mut current_line: Vec<TextSpan> = Vec::new();
    let mut current_color = DEFAULT_COLOR;
    let mut buf = String::new();

    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == 0x1B && i + 1 < bytes.len() && bytes[i + 1] == b'[' {
            // Flush existing text
            if !buf.is_empty() {
                for ch in buf.chars() {
                    if ch == '\n' {
                        spans.push(std::mem::take(&mut current_line));
                    } else {
                        current_line.push(TextSpan {
                            text: ch.to_string(),
                            color: current_color,
                        });
                    }
                }
                buf.clear();
            }

            // Find 'm' that ends the ANSI code
            i += 2; // Skip \x1b[
            let start = i;
            while i < bytes.len() && bytes[i] != b'm' {
                i += 1;
            }

            if i >= bytes.len() {
                break; // Malformed ANSI sequence
            }

            let code_str = std::str::from_utf8(&bytes[start..i]).unwrap_or("");
            i += 1; // Skip 'm'

            let codes = code_str
                .split(';')
                .filter_map(|s| s.parse::<u8>().ok())
                .collect::<Vec<_>>();

            let mut j = 0;
            while j < codes.len() {
                match codes[j] {
                    0 => current_color = WHITE,
                    30 => current_color = Color::from_rgba(0x00, 0x00, 0x00, 255),
                    31 => current_color = Color::from_rgba(0xAA, 0x00, 0x00, 255),
                    32 => current_color = Color::from_rgba(0x00, 0xAA, 0x00, 255),
                    33 => current_color = Color::from_rgba(0xFF, 0xAA, 0x00, 255),
                    34 => current_color = Color::from_rgba(0x55, 0x55, 0xFF, 255),
                    35 => current_color = Color::from_rgba(0xAA, 0x00, 0xAA, 255),
                    36 => current_color = Color::from_rgba(0x00, 0xAA, 0xAA, 255),
                    37 => current_color = Color::from_rgba(0xAA, 0xAA, 0xAA, 255),
                    90 => current_color = Color::from_rgba(0x55, 0x55, 0x55, 255),
                    91 => current_color = Color::from_rgba(0xFF, 0x55, 0x55, 255),
                    92 => current_color = Color::from_rgba(0x55, 0xFF, 0x55, 255),
                    93 => current_color = Color::from_rgba(0xFF, 0xFF, 0x55, 255),
                    94 => current_color = Color::from_rgba(0x00, 0x00, 0xAA, 255),
                    95 => current_color = Color::from_rgba(0xFF, 0x55, 0xFF, 255),
                    96 => current_color = Color::from_rgba(0x55, 0xFF, 0xFF, 255),
                    97 => current_color = Color::from_rgba(0xFF, 0xFF, 0xFF, 255),
                    38 => {
                        if j + 4 < codes.len() && codes[j + 1] == 2 {
                            let r = codes[j + 2];
                            let g = codes[j + 3];
                            let b = codes[j + 4];
                            current_color = Color::from_rgba(r, g, b, 255);
                            j += 4;
                        }
                    }
                    _ => {}
                }
                j += 1;
            }
        } else {
            buf.push(input[i..=i].chars().next().unwrap());
            i += input[i..].chars().next().unwrap().len_utf8();
        }
    }

    // Flush trailing buffer
    if !buf.is_empty() {
        for ch in buf.chars() {
            if ch == '\n' {
                spans.push(std::mem::take(&mut current_line));
            } else {
                current_line.push(TextSpan {
                    text: ch.to_string(),
                    color: current_color,
                });
            }
        }
    }

    if !current_line.is_empty() {
        spans.push(current_line);
    }

    spans
}

/// Draw parsed ANSI text using your custom font
pub fn draw_ansi_text(text: &str, position: Vec2, assets: &GlobalAssets, font_size: u16, line_spacing: f32) {
    let lines = parse_ansi(text);
    let mut y = position.y;

    for line in lines {
        let mut x = position.x;

        for span in line {
            let params = TextParams {
                font: Some(&assets.font),
                font_size,
                color: span.color,
                ..Default::default()
            };
            draw_text_ex(&span.text, x, y, params);

            let measure = measure_text(&span.text, Some(&assets.font), font_size, 1.0);
            x += measure.width;
        }

        y += font_size as f32 + line_spacing;
    }
}

pub fn remove_ansii_escape_codes(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == 0x1B && i + 1 < bytes.len() && bytes[i + 1] == b'[' {
            // Found the start of an ANSI escape sequence
            i += 2;
            while i < bytes.len() {
                if (bytes[i] >= b'a' && bytes[i] <= b'z') || (bytes[i] >= b'A' && bytes[i] <= b'Z') {
                    // ANSI sequence typically ends with a letter (e.g. 'm', 'K', etc.)
                    i += 1;
                    break;
                }
                i += 1;
            }
        } else {
            // Normal character
            output.push(input[i..].chars().next().unwrap());
            i += input[i..].chars().next().unwrap().len_utf8();
        }
    }

    output
}