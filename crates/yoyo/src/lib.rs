pub use yoyo_macros::yoyo;

pub mod components;
mod style;
mod transition;
pub mod utils;
mod variants;

pub use components::TouchableOpacity;
pub use components::View;
pub use components::{AnimatePresence, Presence};
pub use style::{Style, Transitions};
pub use transition::{Easing, Spring, TransformTransition, Transition, Tween};
pub use variants::Variants;

pub mod hooks {
    pub use super::components::UsePresence;
}
