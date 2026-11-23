use ratatui::{
    style::Style,
    text::{Line, Span},
};

use crate::theme::color::Color;

const MARKER_START: &str = "&({";
const MARKER_END: &str = "})";

pub fn parse(art: &'static str, base_color: Color) -> anyhow::Result<Vec<Line<'static>>> {
    let mut lines = Vec::new();
    let mut current_color = base_color;

    for line in art.lines() {
        let mut spans = Vec::new();
        let mut rest = line;

        while let Some(start) = rest.find(MARKER_START) {
            let after_start = &rest[start + MARKER_START.len()..];

            let Some(end) = after_start.find(MARKER_END) else {
                anyhow::bail!("Unclosed logo color marker");
            };

            let text = &rest[..start];

            if !text.is_empty() {
                spans.push(Span::styled(
                    text,
                    Style::default().fg(current_color.into()),
                ));
            }

            let color = &after_start[..end];

            current_color = if color.is_empty() || color.eq_ignore_ascii_case("reset") {
                base_color
            } else {
                Color::try_from(color.to_string())?
            };

            rest = &after_start[end + MARKER_END.len()..];
        }

        if !rest.is_empty() {
            spans.push(Span::styled(
                rest,
                Style::default().fg(current_color.into()),
            ));
        }

        lines.push(Line::from(spans));
    }

    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn color(value: &str) -> Color {
        Color::try_from(value.to_string()).unwrap()
    }

    #[test]
    fn strips_markers_and_resets_to_base_color() {
        let lines = parse(
            "&({#ff0000})red&({#00ff00})green&({})base",
            color("#0000ff"),
        )
        .unwrap();

        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].width(), "redgreenbase".len());
        assert_eq!(lines[0].spans.len(), 3);
    }

    #[test]
    fn rejects_unclosed_marker() {
        assert!(parse("&({#ff0000red", color("#0000ff")).is_err());
    }

    #[test]
    fn all_logos_parse() {
        let logos = [
            include_str!("../../assets/almalinux.txt"),
            include_str!("../../assets/alpine.txt"),
            include_str!("../../assets/arch.txt"),
            include_str!("../../assets/centos.txt"),
            include_str!("../../assets/debian.txt"),
            include_str!("../../assets/endeavouros.txt"),
            include_str!("../../assets/fedora.txt"),
            include_str!("../../assets/freebsd.txt"),
            include_str!("../../assets/gentoo.txt"),
            include_str!("../../assets/kali.txt"),
            include_str!("../../assets/macos.txt"),
            include_str!("../../assets/manjaro.txt"),
            include_str!("../../assets/mint.txt"),
            include_str!("../../assets/nixos.txt"),
            include_str!("../../assets/opensuse.txt"),
            include_str!("../../assets/popos.txt"),
            include_str!("../../assets/redhat.txt"),
            include_str!("../../assets/rocky.txt"),
            include_str!("../../assets/tux.txt"),
            include_str!("../../assets/ubuntu.txt"),
            include_str!("../../assets/void.txt"),
            include_str!("../../assets/windows.txt"),
        ];

        for logo in logos {
            let (color, art) = logo.split_once('\n').unwrap();
            let color = Color::try_from(color.to_string()).unwrap();

            parse(art, color).unwrap();
        }
    }
}
