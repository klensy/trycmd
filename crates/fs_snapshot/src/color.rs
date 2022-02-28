#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
pub struct Palette {
    pub(crate) info: styled::Style,
    pub(crate) error: styled::Style,
    pub(crate) hint: styled::Style,
}

impl Palette {
    #[cfg(feature = "color")]
    pub fn always() -> Self {
        Self {
            info: styled::Style(yansi::Style::new(yansi::Color::Green)),
            error: styled::Style(yansi::Style::new(yansi::Color::Red)),
            hint: styled::Style(yansi::Style::new(yansi::Color::Unset).dimmed()),
        }
    }

    #[cfg(not(feature = "color"))]
    pub fn always() -> Self {
        Self::never()
    }

    pub fn never() -> Self {
        Self {
            info: Default::default(),
            error: Default::default(),
            hint: Default::default(),
        }
    }

    pub fn auto() -> Self {
        if is_colored() {
            Self::always()
        } else {
            Self::never()
        }
    }
}

fn is_colored() -> bool {
    #[cfg(feature = "color")]
    {
        concolor::get(concolor::Stream::Either).ansi_color()
    }

    #[cfg(not(feature = "color"))]
    {
        false
    }
}

#[cfg(feature = "color")]
mod styled {
    #[derive(Copy, Clone, Debug, Default)]
    pub(crate) struct Style(pub(crate) yansi::Style);

    impl Style {
        pub(crate) fn paint<T: std::fmt::Display>(self, item: T) -> impl std::fmt::Display {
            self.0.paint(item)
        }
    }
}

#[cfg(not(feature = "color"))]
mod styled {
    #[derive(Copy, Clone, Debug, Default)]
    pub(crate) struct Style;

    impl Style {
        pub(crate) fn paint<T: std::fmt::Display>(self, item: T) -> impl std::fmt::Display {
            item
        }
    }
}
