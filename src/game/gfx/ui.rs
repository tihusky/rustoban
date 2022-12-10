use crate::prelude::*;

// Wrapper around a TextFragment that sets a margin on the text
pub struct TextBlock {
    pub content: TextFragment,
    pub margin_top: f32,
    pub margin_right: f32,
    pub margin_bottom: f32,
    pub margin_left: f32,
    pub text_align: TextAlign,
}

impl TextBlock {
    pub fn new(content: TextFragment, margin: (f32, f32, f32, f32), text_align: TextAlign) -> Self {
        Self {
            content,
            margin_top: margin.0,
            margin_right: margin.1,
            margin_bottom: margin.2,
            margin_left: margin.3,
            text_align,
        }
    }
}

pub fn print_spaced(ctx: &Context, canvas: &mut Canvas, blocks: &Vec<TextBlock>, origin: Point2D) {
    let x = origin.x as f32;
    let mut current_y = origin.y as f32;

    for block in blocks {
        current_y += block.margin_top;

        let mut text = Text::new(block.content.clone());
        text.set_layout(TextLayout {
            h_align: block.text_align,
            v_align: TextAlign::Begin,
        });
        canvas.draw(&text, DrawParam::default().dest([x, current_y]));

        current_y += text.dimensions(ctx).unwrap().h + block.margin_bottom;
    }
}
