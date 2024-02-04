use std::borrow::Cow;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PushLayerError {
    #[error("Can't push on top of empty layer №{}", .0)]
    EmptyLayer(usize),
}

#[derive(Debug, Error)]
pub enum PopLayerError {
    #[error("Can't pop layer when already at root")]
    AtRoot,
}

#[derive(Debug, Clone)]
pub struct TreeDisplay {
    layer: usize,
    columns: Vec<Vec<(String, usize)>>,
}

impl Default for TreeDisplay {
    fn default() -> Self {
        Self {
            layer: 0,
            columns: vec![Vec::new()],
        }
    }
}

impl TreeDisplay {
    pub fn push_layer(&mut self) -> Result<(), PushLayerError> {
        if self.columns[self.layer].is_empty() {
            return Err(PushLayerError::EmptyLayer(self.layer));
        }
        self.layer += 1;
        if self.layer == self.columns.len() {
            self.columns.push(Default::default())
        }
        Ok(())
    }

    pub fn pop_layer(&mut self) -> Result<(), PopLayerError> {
        if self.layer == 0 {
            return Err(PopLayerError::AtRoot);
        }
        self.layer -= 1;
        Ok(())
    }

    pub fn add_child(&mut self, text: String) {
        self.columns[self.layer].push((text, 0));
        if self.layer != 0 {
            let rows = &mut self.columns[self.layer - 1];
            let last = rows.len() - 1;
            rows[last].1 += 1;
        }
    }

    pub fn lines(&self) -> Vec<String> {
        let mut cols: Vec<_> = self.columns.iter().map(|e| e.as_slice()).collect();
        let mut output = Vec::new();
        TreeDisplay::build_display_inner(&mut Vec::new(), cols.as_mut_slice(), &mut output);
        output
    }

    fn build_display_inner<'a>(
        prefix: &mut Vec<Cow<'static, str>>,
        cols: &mut [&[(String, usize)]],
        output: &mut Vec<String>,
    ) {
        let [col, rest @ ..] = cols else {
            return;
        };
        let mut iter = col.iter().peekable();
        while let Some((text, children_count)) = iter.next() {
            output.push(format!(
                "{}{}",
                prefix
                    .split_last()
                    .map(|e| e.1.join("")
                        + if iter.peek().is_some() {
                            "├── "
                        } else {
                            "└── "
                        })
                    .unwrap_or_default(),
                text
            ));
            if iter.peek().is_some() {
                prefix.push(Cow::Borrowed("|   "));
            } else {
                prefix.push(Cow::Borrowed("    "));
            }
            let (next, leftover) = rest[0].split_at(*children_count);
            rest[0] = &next;
            TreeDisplay::build_display_inner(prefix, rest, output);
            rest[0] = &leftover;
            prefix.pop();
        }
    }
}
