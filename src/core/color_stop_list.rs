#[derive(Debug, Clone)]
pub struct ColorStopListItem {
    pub color: crate::color::Color,
    pub offset: f32,
}

#[derive(Debug, Clone)]
pub struct ColorStopList(Vec<ColorStopListItem>);

impl ColorStopList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, color: crate::color::Color, offset: f32) {
        self.0.push(ColorStopListItem { color, offset });
    }

    pub fn iter(&self) -> std::slice::Iter<'_, ColorStopListItem> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, ColorStopListItem> {
        self.0.iter_mut()
    }

    pub fn into(self) -> Vec<peniko::ColorStop> {
        self.0
            .into_iter()
            .map(|item| peniko::ColorStop {
                color: item.color.get().into(),
                offset: item.offset,
            })
            .collect()
    }

    pub fn set_alpha(&self, alpha: f64) -> Self {
        let mut new_list = self.clone();
        for item in new_list.iter_mut() {
            item.color.set_alpha_mut(alpha);
        }
        new_list
    }

    pub fn from_vec(vec: Vec<(crate::color::Color, f32)>) -> Self {
        let mut list = Self::new();
        for (color, offset) in vec {
            list.push(color, offset);
        }
        list
    }
}

// Make ColorStopList slicable
impl AsRef<[ColorStopListItem]> for ColorStopList {
    fn as_ref(&self) -> &[ColorStopListItem] {
        &self.0
    }
}

impl std::ops::Deref for ColorStopList {
    type Target = [ColorStopListItem];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Macro to initialize ColorStopList with colors
///
/// # Examples
/// ```ignore
/// // Auto-distributed offsets (0.0, 0.5, 1.0)
/// let colors = color_stop_list![Color::RED, Color::GREEN, Color::BLUE];
///
/// // Explicit offsets
/// let colors = color_stop_list![explicit:
///     Color::RED, 0.0,
///     Color::GREEN, 0.5,
///     Color::BLUE, 1.0,
/// ];
/// ```
#[macro_export]
macro_rules! color_stop_list {
    // With explicit offsets: color_stop_list![explicit: (color, offset), ...]
    (explicit: $($color:expr, $offset:expr),* $(,)?) => {
        {
            let mut list = $crate::ColorStopList::new();
            $(
                list.push($color, $offset);
            )*
            list
        }
    };

    // Auto-distributed: color_stop_list![color1, color2, color3, ...]
    ($($color:expr),+ $(,)?) => {
        {
            let colors = vec![$($color),+];
            let count = colors.len() as f32;
            let mut list = $crate::ColorStopList::new();
            for (i, color) in colors.into_iter().enumerate() {
                let offset = if count > 1.0 {
                    i as f32 / (count - 1.0)
                } else {
                    0.0
                };
                list.push(color, offset);
            }
            list
        }
    };
}
