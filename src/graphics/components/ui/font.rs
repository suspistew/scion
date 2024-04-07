/// [`Font`] represents the different fonts available in `Scion`
#[derive(Clone)]
pub enum Font {
    /// Texture based font
    Bitmap {
        /// Path to the texture of this font, PNG only.
        texture_path: String,
        /// List of characters available in the font, in the right order
        chars: String,
        /// Character width in pixel
        width: f32,
        /// Character height in pixel
        height: f32,
        /// Number of column in the font's texture
        texture_columns: f32,
        /// Number of lines in the font's texture
        texture_lines: f32,
    },
    TrueType {
        font_path: String
    }
}

impl Font {
    pub(crate) fn find_line_and_column(
        chars: &String,
        texture_column: f32,
        character: char,
    ) -> (f32, f32) {
        let index_of_char = chars
            .find(character)
            .expect("A character is not a part of the bitmap font but has been added to an UiText");

        (
            (index_of_char / texture_column as usize) as f32,
            (index_of_char % texture_column as usize) as f32,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_indexes() {
        assert_eq!((1., 0.), Font::find_line_and_column(&"abcdef".to_string(), 3., 'd'))
    }
}
