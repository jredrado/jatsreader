mod element;
mod cfi;
mod range;
mod locator;

pub use element::ElementRef;
pub use cfi::CFIComponent;
pub use cfi::CFIComponentList;

pub use range::Range;

pub use locator::DOMRange;
pub use locator::DOMIndex;
pub use locator::TextContext;

pub use locator::Locator;
pub use locator::Location;
pub use locator::SimplifiedLocator;