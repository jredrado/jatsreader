
mod element;
mod selector;
mod range;
mod locator;

pub use element::ElementRef;
pub use selector::Selector;

pub use range::Range;

pub use locator::DOMRange;
pub use locator::DOMIndex;
pub use locator::TextContext;

pub use locator::Locator;
pub use locator::Location;
pub use locator::SimplifiedLocator;