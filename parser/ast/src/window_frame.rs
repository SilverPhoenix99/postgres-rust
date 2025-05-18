#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowFrame {
    kind: WindowFrameKind,
    frame_extent: FrameExtent,
    window_exclusion: Option<WindowExclusion>,
}

impl WindowFrame {
    pub fn new(kind: WindowFrameKind, frame_extent: FrameExtent, window_exclusion: Option<WindowExclusion>) -> Self {
        Self { kind, frame_extent, window_exclusion }
    }

    pub fn kind(&self) -> WindowFrameKind {
        self.kind
    }

    pub fn frame_extent(&self) -> &FrameExtent {
        &self.frame_extent
    }

    pub fn window_exclusion(&self) -> Option<WindowExclusion> {
        self.window_exclusion
    }
}

impl Default for WindowFrame {
    fn default() -> Self {
        Self {
            kind: WindowFrameKind::Range,
            frame_extent: FrameExtent::Unbounded { end: Some(PrecedingEnd::CurrentRow) },
            window_exclusion: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowFrameKind {
    Range,
    Rows,
    Groups,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowExclusion {
    CurrentRow,
    Group,
    Ties,
}

use crate::FrameExtent;
use crate::PrecedingEnd;
