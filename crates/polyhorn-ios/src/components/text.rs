use polyhorn_core::CommandBuffer;
use polyhorn_ios_sys::coregraphics::{CGRect, CGSize};
use polyhorn_ios_sys::foundation::{NSAttributedString, NSMutableAttributedString};
use polyhorn_ios_sys::polykit::{PLYLabel, PLYView};
use polyhorn_ui::geometry::{Dimension, Size};
use polyhorn_ui::layout::{Algorithm, MeasureFunc};
use polyhorn_ui::styles::TextStyle;

use crate::prelude::*;
use crate::raw::{attributed_string, Builtin, Container, OpaqueContainer};
use crate::Key;

impl Container for PLYLabel {
    fn mount(&mut self, _child: &mut OpaqueContainer) {
        unimplemented!("PLYLabel cannot mount children.")
    }

    fn unmount(&mut self) {
        PLYLabel::to_view(self).remove_from_superview()
    }

    fn to_view(&self) -> Option<PLYView> {
        Some(PLYLabel::to_view(self))
    }
}

struct TextSegment {
    value: String,
    style: TextStyle,
}

fn collect_texts(style: &TextStyle, children: &Element) -> Vec<TextSegment> {
    fn collect_texts(style: &TextStyle, element: &Element, results: &mut Vec<TextSegment>) {
        match element {
            Element::Component(component) => {
                let opaque = &component.component;

                if let Some(text) = opaque.as_ref().as_any().downcast_ref::<Text>() {
                    collect_texts(&text.style, &component.children, results);
                }
            }
            Element::String(string) => results.push(TextSegment {
                value: string.clone(),
                style: style.clone(),
            }),
            Element::Fragment(fragment) => {
                for element in &fragment.elements {
                    collect_texts(style, element, results);
                }
            }
            _ => unimplemented!(),
        }
    }

    let mut segments = vec![];

    collect_texts(style, children, &mut segments);

    segments
}

fn transform_texts(texts: Vec<TextSegment>) -> NSAttributedString {
    let mut string = NSMutableAttributedString::new();

    for text in texts {
        string.append_attributed_string(&attributed_string(&text.value, &text.style));
    }

    string.into()
}

impl Component for Text {
    fn render(&self, manager: &mut Manager) -> Element {
        let label_ref = use_reference!(manager, None);

        let texts = collect_texts(&self.style, &manager.children());

        use_effect!(manager, move |link, buffer| {
            let id = match label_ref.apply(link, |label| label.to_owned()) {
                Some(id) => id,
                None => return,
            };

            buffer.mutate(&[id], move |containers, _| {
                let container = &mut containers[0];

                let attributed_string = transform_texts(texts);

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
                            MeasureFunc::Boxed(Box::new(move |size| {
                                let min_size = CGSize {
                                    width: match size.width {
                                        Dimension::Points(width) => width as _,
                                        _ => 0.0,
                                    },
                                    height: match size.height {
                                        Dimension::Points(height) => height as _,
                                        _ => 0.0,
                                    },
                                };

                                let target =
                                    attributed_string.bounding_rect_with_size(min_size).size;

                                let result = Size {
                                    width: target.width.ceil() as _,
                                    height: target.height.ceil() as _,
                                };

                                result
                            })),
                        );
                }

                if let Some(view) = container.downcast_mut::<PLYLabel>() {
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
            Some(label_ref.weak(manager)),
        )
    }
}
