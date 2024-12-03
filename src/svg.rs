use crate::{
    utils::{find_all, get_note_coords, Palette},
    Chord, GuitarString,
};

pub fn svg_draw_bg(use_background: bool, palette: &Palette) -> String {
    if use_background {
        format!(
            "<rect fill=\"{}\" width=\"300\" height=\"310\" rx=\"10\" />",
            palette.bg
        )
    } else {
        "".into()
    }
}

pub fn svg_draw_finger(
    finger: &str,
    i: GuitarString,
    string_space: &i32,
    palette: &Palette,
) -> String {
    let x = 50 + (i as i32 * string_space);
    let y = if finger == "0" || finger == "x" {
        35
    } else {
        265
    };

    format!(
        "<text x=\"{}\" y=\"{}\" class=\"text\" dominant-baseline=\"middle\" text-anchor=\"middle\" font-size=\"16\" fill=\"{}\" font-weight=\"400\">{}</text>",
        x, y, palette.fg, finger
    )
}

pub fn svg_draw_min_fret(min_fret: &i32, string_space: &i32, palette: &Palette) -> String {
    let offset_top = 50;

    let x = 32;
    let y = string_space * 2 + offset_top - (string_space / 2);
    format!(
        "<text x=\"{}\" y=\"{}\" class=\"text\" dominant-baseline=\"middle\" text-anchor=\"end\" font-size=\"16\" fill=\"{}\" font-weight=\"400\">{}</text>",
        x, y, palette.fg, min_fret
    )
}

pub fn svg_draw_note(
    note: &i32,
    string: GuitarString,
    string_space: &i32,
    min_fret: &i32,
    palette: &Palette,
) -> String {
    if note <= &0 {
        return "".to_string();
    }
    let radius = 13;

    let (x, y) = get_note_coords(note, string, string_space, min_fret);
    format!(
        "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\" />",
        x, y, radius, palette.fg
    )
}

pub fn svg_draw_barres(
    barre_fret: &i32,
    frets: &[i32],
    string_space: &i32,
    min_fret: &i32,
    palette: &Palette,
) -> String {
    let strings = find_all(frets, barre_fret);
    if strings.len() < 2 {
        return String::from("");
    }

    let first = get_note_coords(
        barre_fret,
        (*strings.first().unwrap_or(&0)).into(),
        string_space,
        min_fret,
    );
    let last = get_note_coords(
        barre_fret,
        (*strings.last().unwrap_or(&0)).into(),
        string_space,
        min_fret,
    );

    // move curve out of centre of frets
    let y_offset = if barre_fret == &1 { 27 } else { 23 };

    // amount to move the controls
    // controls the angle of curve
    let control_y_offset = y_offset + 10;
    let control_x_offset = 8;

    let origin_control = (first.0 + control_x_offset, first.1 - control_y_offset);
    let end_control = (last.0 - control_x_offset, last.1 - control_y_offset);

    format!(
        "<path d=\"M {} {} C {} {}, {} {}, {} {}\" stroke=\"{}\" stroke-width=\"3\" fill=\"transparent\" stroke-linecap=\"round\" />",
        first.0,
        first.1 - y_offset,
        origin_control.0,
        origin_control.1,
        end_control.0,
        end_control.1,
        last.0,
        last.1 - y_offset,
        palette.fg
    )
}

pub fn svg_draw_title(chord_settings: &Chord, palette: &Palette) -> String {
    match (chord_settings.title, chord_settings.suffix) {
        (Some(title), Some(suffix)) => format!(
            "<text x=\"150px\" y=\"{}\" class=\"text\" dominant-baseline=\"middle\"
        text-anchor=\"middle\" font-size=\"24\" fill=\"{}\" font-weight=\"400\">{}<tspan font-size=\"18\" fill=\"{}\" font-weight=\"300\">{}</tspan></text>",
            18,
            palette.fg,
            title,
            palette.fg,
            suffix
        ),
        (Some(title), None) => format!(
            "<text x=\"150px\" y=\"{}\" class=\"text\" dominant-baseline=\"middle\"
  text-anchor=\"middle\" font-size=\"24\" fill=\"{}\" font-weight=\"400\">{}</text>",
            18,
            palette.fg,
            title,
        ),
        _ => String::from(""),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        svg::{svg_draw_barres, svg_draw_note},
        utils::Palette,
        Chord,
    };

    use super::svg_draw_title;

    #[test]
    fn should_render_note() {
        let palette = Palette {
            fg: "#fff",
            bg: "#111",
        };
        let note = svg_draw_note(&6, crate::GuitarString::D, &10, &0, &palette);
        let expected = "<circle cx=\"70\" cy=\"105\" r=\"13\" fill=\"#fff\" />";
        assert_eq!(note, expected);

        let note = svg_draw_note(&2, crate::GuitarString::E, &12, &1, &palette);
        let expected = "<circle cx=\"50\" cy=\"68\" r=\"13\" fill=\"#fff\" />";
        assert_eq!(note, expected);

        let note = svg_draw_note(&7, crate::GuitarString::A, &14, &2, &palette);
        let expected = "<circle cx=\"64\" cy=\"141\" r=\"13\" fill=\"#fff\" />";
        assert_eq!(note, expected);

        let note = svg_draw_note(&4, crate::GuitarString::G, &20, &3, &palette);
        let expected = "<circle cx=\"110\" cy=\"100\" r=\"13\" fill=\"#fff\" />";
        assert_eq!(note, expected);

        let note = svg_draw_note(&9, crate::GuitarString::B, &30, &5, &palette);
        let expected = "<circle cx=\"170\" cy=\"215\" r=\"13\" fill=\"#fff\" />";
        assert_eq!(note, expected);

        let note = svg_draw_note(&12, crate::GuitarString::HighE, &32, &10, &palette);
        let expected = "<circle cx=\"210\" cy=\"162\" r=\"13\" fill=\"#fff\" />";
        assert_eq!(note, expected);

        let palette = Palette {
            fg: "#111",
            bg: "#333",
        };
        let note = svg_draw_note(&6, crate::GuitarString::D, &10, &0, &palette);
        let expected = "<circle cx=\"70\" cy=\"105\" r=\"13\" fill=\"#111\" />";
        assert_eq!(note, expected);

        let note = svg_draw_note(&2, crate::GuitarString::E, &12, &1, &palette);
        let expected = "<circle cx=\"50\" cy=\"68\" r=\"13\" fill=\"#111\" />";
        assert_eq!(note, expected);

        let note = svg_draw_note(&7, crate::GuitarString::A, &14, &2, &palette);
        let expected = "<circle cx=\"64\" cy=\"141\" r=\"13\" fill=\"#111\" />";
        assert_eq!(note, expected);

        let note = svg_draw_note(&4, crate::GuitarString::G, &20, &3, &palette);
        let expected = "<circle cx=\"110\" cy=\"100\" r=\"13\" fill=\"#111\" />";
        assert_eq!(note, expected);

        let note = svg_draw_note(&9, crate::GuitarString::B, &30, &5, &palette);
        let expected = "<circle cx=\"170\" cy=\"215\" r=\"13\" fill=\"#111\" />";
        assert_eq!(note, expected);

        let note = svg_draw_note(&12, crate::GuitarString::HighE, &32, &10, &palette);
        let expected = "<circle cx=\"210\" cy=\"162\" r=\"13\" fill=\"#111\" />";
        assert_eq!(note, expected);
    }

    #[test]
    fn should_draw_barre() {
        let palette = Palette {
            fg: "#efe",
            bg: "#333",
        };

        let barre = svg_draw_barres(&5, &vec![5, 7, 7, 6, 5, -1], &40, &5, &palette);
        let expected = "<path d=\"M 50 87 C 58 77, 202 77, 210 87\" stroke=\"#efe\" stroke-width=\"3\" fill=\"transparent\" stroke-linecap=\"round\" />";
        assert_eq!(barre, expected);
    }

    #[test]
    fn should_draw_titke() {
        let palette = Palette {
            fg: "#efe",
            bg: "#333",
        };

        let title = String::from("C");
        let suffix = String::from("aug9");

        let chord = Chord {
            title: Some(&title),
            ..Default::default()
        };
        // no suffix
        assert_eq!(
            svg_draw_title(&chord, &palette),
            "<text x=\"150px\" y=\"18\" class=\"text\" dominant-baseline=\"middle\"
  text-anchor=\"middle\" font-size=\"24\" fill=\"#efe\" font-weight=\"400\">C</text>",
        );

        //  with suffix
        let chord = Chord {
            title: Some(&title),
            suffix: Some(&suffix),
            ..Default::default()
        };
        assert_eq!(svg_draw_title(&chord, &palette), "<text x=\"150px\" y=\"18\" class=\"text\" dominant-baseline=\"middle\"
        text-anchor=\"middle\" font-size=\"24\" fill=\"#efe\" font-weight=\"400\">C<tspan font-size=\"18\" fill=\"#efe\" font-weight=\"300\">aug9</tspan></text>");
    }
}
