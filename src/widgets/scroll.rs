use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    widgets::Widget,
};

#[derive(Clone)]
pub struct Scroll {
    page_size: Rect,
    max_size: Rect,
    buffer: Buffer,
    offset: Position,
}

impl Scroll {
    pub fn new(max_size: Rect) -> Self {
        Self {
            max_size,
            page_size: Rect::default(),
            buffer: Buffer::empty(max_size),
            offset: Position::default(),
        }
    }

    pub fn page_size(&mut self, page_size: Rect) -> Self {
        self.page_size = page_size;
        self.clone()
    }

    pub fn render_widget(&mut self, widget: impl Widget, area: Rect) {
        widget.render(area, &mut self.buffer);
    }

    pub fn area(&self) -> Rect {
        self.max_size
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }

    pub fn offset(&self) -> Position {
        self.offset
    }

    pub fn set_offset(&mut self, offset: Position) -> Self {
        self.offset = offset;
        self.clone()
    }
}

impl Widget for Scroll {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let selection = Rect {
            x: self.offset.x.max(0),
            y: self.offset.y.max(0),
            width: self.page_size.width,
            height: self.page_size.height,
        };
        copy_area(&self.buffer, selection, buf, area);
    }
}

fn copy_area(src: &Buffer, src_area: Rect, dst: &mut Buffer, dst_area: Rect) {
    let src_area = src_area.intersection(*src.area());
    let dst_area = dst_area.intersection(*dst.area());
    for y in 0..dst_area.height {
        for x in 0..dst_area.width {
            let src_x = src_area.x + x;
            let src_y = src_area.y + y;

            if src_area.width <= x || src_area.height <= y {
                continue;
            }

            let dst_x = dst_area.x + x;
            let dst_y = dst_area.y + y;
            let src_idx = src.index_of(src_x, src_y);
            let dst_idx = dst.index_of(dst_x, dst_y);
            dst.content[dst_idx] = src.content.get(src_idx).unwrap().clone();
        }
    }
}
