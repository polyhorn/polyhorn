use polyhorn_ui::geometry::{Dimension, Point, Size};
use polyhorn_ui::layout::LayoutAxisX;
use polyhorn_ui::styles::{Position, ViewStyle};
use std::cell::RefCell;
use std::collections::HashMap;

use super::Algorithm;
use crate::{Layout, MeasureFunc};

mod convert;

use convert::IntoYoga;

pub struct Flexbox {
    counter: usize,
    nodes: HashMap<usize, RefCell<yoga::Node>>,
}

impl Flexbox {
    fn next_id(&mut self) -> usize {
        let id = self.counter;
        self.counter += 1;
        id
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct Node(usize);

impl Algorithm for Flexbox {
    type Node = Node;

    fn new() -> Self {
        Flexbox {
            counter: 0,
            nodes: Default::default(),
        }
    }

    fn new_node(&mut self, style: ViewStyle, children: &[Self::Node]) -> Self::Node {
        let id = self.next_id();
        let mut node = yoga::Node::new();

        for (i, child) in children.iter().enumerate() {
            node.insert_child(
                &mut self.nodes.get(&child.0).unwrap().borrow_mut(),
                i as u32,
            );
        }

        self.nodes.insert(id, RefCell::new(node));

        let node = Node(id);

        self.set_style(node, style);

        node
    }

    fn new_leaf(&mut self, style: ViewStyle, measure: MeasureFunc) -> Self::Node {
        let id = self.next_id();
        let node = yoga::Node::new();

        self.nodes.insert(id, RefCell::new(node));

        let node = Node(id);

        self.set_style(node, style);
        self.set_measure(node, measure);

        node
    }

    fn add_child(&mut self, parent: Self::Node, child: Self::Node) {
        let mut parent = self.nodes.get(&parent.0).unwrap().borrow_mut();
        let mut child = self.nodes.get(&child.0).unwrap().borrow_mut();
        let child_count = parent.child_count();
        parent.insert_child(&mut child, child_count);
    }

    fn remove_child(&mut self, parent: Self::Node, child: Self::Node) {
        let mut parent = self.nodes.get(&parent.0).unwrap().borrow_mut();
        let mut child = self.nodes.get(&child.0).unwrap().borrow_mut();
        parent.remove_child(&mut child);
    }

    fn child_count(&self, parent: Self::Node) -> usize {
        self.nodes.get(&parent.0).unwrap().borrow().child_count() as usize
    }

    fn remove(&mut self, node: Self::Node) {
        let _ = self.nodes.remove(&node.0);
    }

    fn set_style(&mut self, node: Self::Node, style: ViewStyle) {
        let mut node = self.nodes.get(&node.0).unwrap().borrow_mut();

        match style.position {
            Position::Absolute(absolute) => {
                node.set_position_type(yoga::PositionType::Absolute);

                node.set_position(yoga::Edge::Top, absolute.distances.vertical.top.into_yoga());
                node.set_position(
                    yoga::Edge::Bottom,
                    absolute.distances.vertical.bottom.into_yoga(),
                );

                match absolute.distances.horizontal {
                    LayoutAxisX::DirectionDependent { leading, trailing } => {
                        node.set_position(yoga::Edge::Start, leading.into_yoga());
                        node.set_position(yoga::Edge::End, trailing.into_yoga());
                    }
                    LayoutAxisX::DirectionIndependent { left, right } => {
                        node.set_position(yoga::Edge::Left, left.into_yoga());
                        node.set_position(yoga::Edge::Right, right.into_yoga());
                    }
                }
            }
            Position::Relative(relative) => {
                node.set_position_type(yoga::PositionType::Relative);
                node.set_flex_basis(relative.flex_basis.into_yoga());
                node.set_flex_grow(relative.flex_grow);
                node.set_flex_shrink(relative.flex_shrink);
            }
        };

        node.set_flex_direction(style.flex_direction.into_yoga());
        node.set_align_items(style.align_items.into_yoga());
        node.set_justify_content(style.justify_content.into_yoga());

        node.set_min_width(style.min_size.width.into_yoga());
        node.set_width(style.size.width.into_yoga());
        node.set_max_width(style.max_size.width.into_yoga());

        node.set_min_height(style.min_size.height.into_yoga());
        node.set_height(style.size.height.into_yoga());
        node.set_max_height(style.max_size.height.into_yoga());

        node.set_padding(yoga::Edge::Top, style.padding.vertical.top.into_yoga());
        node.set_padding(
            yoga::Edge::Bottom,
            style.padding.vertical.bottom.into_yoga(),
        );

        match style.padding.horizontal {
            LayoutAxisX::DirectionDependent { leading, trailing } => {
                node.set_padding(yoga::Edge::Start, leading.into_yoga());
                node.set_padding(yoga::Edge::End, trailing.into_yoga());
            }
            LayoutAxisX::DirectionIndependent { left, right } => {
                node.set_padding(yoga::Edge::Left, left.into_yoga());
                node.set_padding(yoga::Edge::Right, right.into_yoga());
            }
        }

        node.set_margin(yoga::Edge::Top, style.margin.vertical.top.into_yoga());
        node.set_margin(yoga::Edge::Bottom, style.margin.vertical.bottom.into_yoga());

        match style.margin.horizontal {
            LayoutAxisX::DirectionDependent { leading, trailing } => {
                node.set_margin(yoga::Edge::Start, leading.into_yoga());
                node.set_margin(yoga::Edge::End, trailing.into_yoga());
            }
            LayoutAxisX::DirectionIndependent { left, right } => {
                node.set_margin(yoga::Edge::Left, left.into_yoga());
                node.set_margin(yoga::Edge::Right, right.into_yoga());
            }
        }

        node.set_overflow(style.overflow.into_yoga());
    }

    fn set_measure(&mut self, node: Self::Node, measure: MeasureFunc) {
        let mut node = self.nodes.get(&node.0).unwrap().borrow_mut();

        node.set_context(Some(yoga::Context::new(measure)));

        extern "C" fn measure_fn(
            node: yoga::NodeRef,
            width: f32,
            _width_mode: yoga::MeasureMode,
            height: f32,
            _height_mode: yoga::MeasureMode,
        ) -> yoga::Size {
            let measure = yoga::Node::get_context(&node)
                .unwrap()
                .downcast_ref::<MeasureFunc>()
                .unwrap();

            match measure {
                MeasureFunc::Boxed(boxed) => {
                    let result = boxed(Size {
                        width: Dimension::Points(width),
                        height: Dimension::Points(height),
                    });

                    yoga::Size {
                        width: result.width,
                        height: result.height,
                    }
                }
            }
        }

        node.set_measure_func(Some(measure_fn))
    }

    fn compute_layout(&mut self, node: Self::Node, size: Size<Dimension<f32>>) {
        let mut node = self.nodes.get(&node.0).unwrap().borrow_mut();

        node.calculate_layout(
            match size.width {
                Dimension::Points(width) => width,
                _ => 0.0,
            },
            match size.height {
                Dimension::Points(height) => height,
                _ => 0.0,
            },
            yoga::Direction::LTR,
        );
    }

    fn layout(&self, node: Self::Node) -> Layout {
        let node = self.nodes.get(&node.0).unwrap().borrow();

        Layout {
            origin: Point {
                x: node.get_layout_left(),
                y: node.get_layout_top(),
            },
            size: Size {
                width: node.get_layout_width(),
                height: node.get_layout_height(),
            },
        }
    }
}
