use std::ops::{Bound, RangeBounds};

pub trait SliceByChar {
    fn slice_by_char(&self, range: impl RangeBounds<usize>) -> String;

    fn skip_by_char(&self, count: usize) -> String;

    fn take_by_char(&self, count: usize) -> String;
}

impl SliceByChar for String {
    fn slice_by_char(&self, range: impl RangeBounds<usize>) -> String {
        let start = match range.start_bound() {
            Bound::Unbounded => 0usize,
            Bound::Excluded(val) => *val + 1,
            Bound::Included(val) => *val,
        };
        let end = match range.end_bound() {
            Bound::Unbounded => self.len(),
            Bound::Excluded(val) => *val - 1,
            Bound::Included(val) => *val,
        };
        let chars_count = self.chars().count();
        if !cfg!(slice_by_char_mode = "unsafe") && chars_count <= start {
            return "".into();
        }
        self.chars()
            .skip(start)
            .take(if cfg!(slice_by_char_mode = "unsafe") {
                end - start
            } else {
                std::cmp::max(std::cmp::min(end, chars_count), start) - start
            })
            .collect()
    }

    fn skip_by_char(&self, count: usize) -> String {
        self.slice_by_char(count..)
    }

    fn take_by_char(&self, count: usize) -> String {
        self.slice_by_char(0..=count)
    }
}

impl SliceByChar for &'_ str {
    fn slice_by_char(&self, range: impl RangeBounds<usize>) -> String {
        self.to_string().slice_by_char(range)
    }

    fn skip_by_char(&self, count: usize) -> String {
        self.to_string().skip_by_char(count)
    }

    fn take_by_char(&self, count: usize) -> String {
        self.to_string().take_by_char(count)
    }
}
