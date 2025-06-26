use iced_widget::markdown::Catalog;

use super::{Theme, container};

impl Catalog for Theme {
    fn code_block<'a>() -> <Self as iced_widget::container::Catalog>::Class<'a>
    {
        Box::new(container::surface_container_highest)
    }
}
