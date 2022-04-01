use docx_rs::{Docx, RunFonts, Style, StyleType, Styles};

pub const NORMAL: &str = "Normal";
pub const RUBRIC: &str = "Rubric";
pub const HEADING_1: &str = "Heading 1";
pub const HEADING_2: &str = "Heading 2";
pub const HEADING_3: &str = "Heading 3";
pub const HEADING_4: &str = "Heading 4";
pub const HEADING_5: &str = "Heading 5";
pub const DATE: &str = "Date";
pub const DAY: &str = "Day";
pub const RESPONSE: &str = "Response";
pub const ANTIPHON: &str = "Antiphon";
pub const ERROR: &str = "Error";
pub const PSALM_OR_CANTICLE: &str = "Psalm/Canticle";

pub trait StyledDocument
where
    Self: Sized,
{
    fn inject_styles(self) -> Self;
}

impl StyledDocument for Docx {
    fn inject_styles(self) -> Self {
        self.styles(
            Styles::new()
                .default_fonts(RunFonts::new().ascii("Garamond"))
                .default_spacing(0)
                .default_size(24)
                .add_style(Style::new(NORMAL, StyleType::Paragraph).name(NORMAL))
                .add_style(
                    Style::new(RUBRIC, StyleType::Paragraph)
                        .name(RUBRIC)
                        .color("red")
                        .italic(),
                )
                .add_style(
                    Style::new(RESPONSE, StyleType::Character)
                        .name(RESPONSE)
                        .bold(),
                )
                .add_style(
                    Style::new(RESPONSE, StyleType::Paragraph)
                        .name(RESPONSE)
                        .bold(),
                )
                .add_style(
                    Style::new(ANTIPHON, StyleType::Paragraph)
                        .name(ANTIPHON)
                        .italic(),
                )
                .add_style(
                    Style::new(ERROR, StyleType::Paragraph)
                        .name(ERROR)
                        .color("red")
                        .bold(),
                )
                .add_style(
                    Style::new(HEADING_1, StyleType::Paragraph)
                        .name(HEADING_1)
                        .based_on(NORMAL)
                        .size(72)
                        .bold(),
                )
                .add_style(
                    Style::new(HEADING_2, StyleType::Paragraph)
                        .name(HEADING_2)
                        .based_on(NORMAL)
                        .size(48)
                        .bold(),
                )
                .add_style(
                    Style::new(HEADING_3, StyleType::Paragraph)
                        .name(HEADING_3)
                        .based_on(NORMAL)
                        .size(32)
                        .bold(),
                )
                .add_style(
                    Style::new(HEADING_4, StyleType::Paragraph)
                        .name(HEADING_4)
                        .based_on(NORMAL)
                        .size(12)
                        .bold(),
                )
                .add_style(
                    Style::new(HEADING_5, StyleType::Paragraph)
                        .name(HEADING_5)
                        .based_on(NORMAL),
                )
                .add_style(
                    Style::new(DATE, StyleType::Paragraph)
                        .name(DATE)
                        .based_on(NORMAL)
                        .italic(),
                )
                .add_style(
                    Style::new(DAY, StyleType::Paragraph)
                        .name(DAY)
                        .based_on(NORMAL)
                        .italic(),
                )
                .add_style(
                    Style::new(PSALM_OR_CANTICLE, StyleType::Paragraph)
                        .name(PSALM_OR_CANTICLE)
                        .based_on(NORMAL), //.indent(Some(240), Some(SpecialIndentType::Hanging(240)), None, None),
                ),
        )
    }
}
