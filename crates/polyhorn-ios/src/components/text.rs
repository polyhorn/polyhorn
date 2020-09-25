use polyhorn_core::CommandBuffer;
use polyhorn_ios_sys::foundation::{NSAttributedString, NSMutableAttributedString};
use polyhorn_ios_sys::{CGRect, CGSize, UILabel, UIView};
use polyhorn_layout::{self as layout, Algorithm};

use crate::*;

impl Container for UILabel {
    fn mount(&mut self, _child: &mut OpaqueContainer) {
        unimplemented!("UILabel cannot mount children.")
    }

    fn unmount(&mut self) {
        UILabel::to_view(self).remove_from_superview()
    }

    fn to_view(&self) -> Option<UIView> {
        Some(UILabel::to_view(self))
    }
}

#[derive(Clone)]
pub struct Text {
    pub style: TextStyle,
}

struct TextSegment {
    value: String,
    style: TextStyle,
}

impl Text {
    fn collect_texts(&self, children: &Element) -> Vec<TextSegment> {
        fn collect_texts(style: &TextStyle, element: &Element, results: &mut Vec<TextSegment>) {
            match element {
                polyhorn_core::Element::<Platform>::Component(component) => {
                    let opaque = &component.component;

                    if let Some(text) = opaque.as_ref().as_any().downcast_ref::<Text>() {
                        collect_texts(&text.style, &component.children, results);
                    }
                }
                polyhorn_core::Element::<Platform>::String(string) => results.push(TextSegment {
                    value: string.clone(),
                    style: style.clone(),
                }),
                polyhorn_core::Element::<Platform>::Fragment(fragment) => {
                    for element in &fragment.elements {
                        collect_texts(style, element, results);
                    }
                }
                _ => unimplemented!(),
            }
        }

        let mut segments = vec![];

        collect_texts(&self.style, children, &mut segments);

        segments
    }

    fn transform_texts(texts: Vec<TextSegment>) -> NSAttributedString {
        let mut string = NSMutableAttributedString::new();

        for text in texts {
            string.append_attributed_string(&markup::attributed_string(&text.value, &text.style));
        }

        string.into()
    }
}

impl Component for Text {
    fn render(&self, manager: &mut Manager) -> Element {
        let label_ref = use_reference!(manager);

        let label_ref_effect = label_ref.clone();

        let texts = self.collect_texts(&manager.children());

        use_effect!(manager, move |buffer| {
            let id = match label_ref_effect.as_copy() {
                Some(id) => id,
                None => return,
            };

            buffer.mutate(&[id], move |containers| {
                let container = &mut containers[0];

                let attributed_string = Text::transform_texts(texts);

                let layout = match container.layout() {
                    Some(layout) => layout.clone(),
                    None => return,
                };

                {
                    let attributed_string = attributed_string.clone();

                    layout
                        .layouter()
                        .write()
                        .unwrap()
                        .flexbox_mut()
                        .set_measure(
                            layout.node(),
                            layout::MeasureFunc::Boxed(Box::new(move |size| {
                                let min_size = CGSize {
                                    width: match size.width {
                                        Dimension::Pixels(width) => width as _,
                                        _ => 0.0,
                                    },
                                    height: match size.height {
                                        Dimension::Pixels(height) => height as _,
                                        _ => 0.0,
                                    },
                                };

                                let target =
                                    attributed_string.bounding_rect_with_size(min_size).size;

                                let result = layout::Size {
                                    width: target.width.ceil() as _,
                                    height: target.height.ceil() as _,
                                };

                                result
                            })),
                        );
                }

                if let Some(view) = container.downcast_mut::<UILabel>() {
                    view.set_attributed_text(&attributed_string);

                    view.to_view().set_layout(move || {
                        let current = layout.current();

                        CGRect::new(
                            current.origin.x as _,
                            current.origin.y as _,
                            current.size.width as _,
                            current.size.height as _,
                        )
                    });
                }
            });
        });

        Element::builtin(
            Key::new(()),
            Builtin::Label,
            Element::fragment(Key::new(()), vec![]),
            Some(label_ref),
        )
    }
}
