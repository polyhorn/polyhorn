use super::element::{ElementBuiltin, ElementComponent, ElementContext, ElementFragment};
use super::{
    CommandBuffer, Component, Compositor, Disposable, Effect, EffectLink, Element, EventLoop,
    Instance, LayoutEffect, Manager, Platform,
};
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

pub struct Render<P>
where
    P: Platform + ?Sized,
{
    renderer: Rc<Renderer<P>>,
    buffer: P::CommandBuffer,
    layout_effects: Vec<LayoutEffect<P>>,
    effects: Vec<Effect<P>>,
}

impl<P> Render<P>
where
    P: Platform + ?Sized,
{
    fn new(renderer: Rc<Renderer<P>>) -> Render<P> {
        let buffer = renderer
            .compositor
            .try_borrow_mut()
            .expect("Couldn't acquire new command buffer from busy compositor.")
            .buffer();

        Render {
            renderer,
            buffer,
            layout_effects: vec![],
            effects: vec![],
        }
    }

    fn rerender_builtin(&mut self, instance: &Rc<Instance<P>>, element: ElementBuiltin<P>) {
        let container = instance.container();
        let builtin = element.builtin;

        self.buffer
            .mutate(&[container], move |containers, environment| {
                builtin.update(containers[0], environment);
            });

        self.rerender_edges(instance, vec![*element.children]);
    }

    fn rerender_component(&mut self, instance: &Rc<Instance<P>>, element: ElementComponent<P>) {
        let (edges, (effects, layout_effects)) = {
            let mut memory = instance.memory_mut();
            let compositor = self
                .renderer
                .compositor
                .try_borrow()
                .expect("Couldn't borrow compositor.");
            let bus = self
                .renderer
                .bus
                .try_borrow()
                .expect("Couldn't borrow bus.");
            let mut manager = Manager::new(
                &*compositor,
                &*bus,
                &mut memory,
                instance.context(),
                *element.children,
                &instance,
            );
            (
                vec![element.component.render(&mut manager)],
                manager.into_effects(),
            )
        };

        self.rerender_edges(instance, edges);

        self.effects.extend(effects);
        self.layout_effects.extend(layout_effects);
    }

    fn rerender_context(&mut self, instance: &Rc<Instance<P>>, element: ElementContext<P>) {
        instance.context().insert_raw(element.value);

        self.rerender_edges(instance, vec![*element.children])
    }

    fn rerender_fragment(&mut self, instance: &Rc<Instance<P>>, element: ElementFragment<P>) {
        self.rerender_edges(instance, element.elements)
    }

    fn rerender_edges(&mut self, instance: &Rc<Instance<P>>, edges: Vec<Element<P>>) {
        let mut topology = instance.topology_mut();
        let topology = topology.deref_mut();

        // Re-rendering looks a bit like mark and sweep. We start by collecting
        // the set of keys of edges.
        let mut keys = topology.keys();

        for element in edges {
            let key = element.key();

            keys.remove(key);

            if let Some(existing) = topology.edge(key) {
                // The edge already exists. We replace its element and issue a
                // re-render.
                existing.topology_mut().deref_mut().update(element);
                self.rerender(existing)
            } else {
                // The edge does not yet exist. We issue a fresh render and store
                // the resulting instance in the topology of this instance.
                let key = key.clone();
                let instance = self.render(
                    Some(instance.clone()),
                    element,
                    instance.container().clone(),
                );
                topology.add_edge(key, instance);
            }
        }

        // Finally, we unmount all instances that correspond to edges that are
        // no longer present.
        for key in keys {
            if let Some(instance) = topology.remove_edge(&key) {
                self.unmount(&instance);
            }
        }
    }

    fn unmount(&mut self, instance: &Rc<Instance<P>>) {
        for edge in instance.topology_mut().edges() {
            self.unmount(&edge);
        }

        match instance.topology_mut().deref_mut().element() {
            Element::Builtin(_) => {
                self.buffer.unmount(instance.container());
            }
            _ => {}
        }
    }

    /// This function is called when re-rendering an existing instance.
    pub fn rerender(&mut self, instance: &Rc<Instance<P>>) {
        let element = instance.topology_mut().element().clone();

        match element {
            Element::Builtin(element) => self.rerender_builtin(instance, element),
            Element::Component(element) => self.rerender_component(instance, element),
            Element::Context(element) => self.rerender_context(instance, element),
            Element::Fragment(element) => self.rerender_fragment(instance, element),
            Element::String(_text) => unimplemented!("Can't render string element directly."),
        }
    }

    /// This function is called when rendering an element into a container for
    /// the first time.
    pub fn render(
        &mut self,
        parent: Option<Rc<Instance<P>>>,
        element: Element<P>,
        in_container: P::ContainerID,
    ) -> Rc<Instance<P>> {
        // We start by figuring out if we need to create a new container for this
        // element or not.
        let container = match &element {
            Element::Builtin(element) => {
                let builtin = element.builtin.clone();
                let container = self.buffer.mount(in_container, move |parent, environment| {
                    builtin.instantiate(parent, environment)
                });

                if let Some(reference) = &element.reference {
                    reference.replace(Some(container));
                }

                container
            }
            _ => in_container,
        };

        let renderer = self.renderer.clone();

        // Then, we create an instance for this element.
        let instance = Rc::new(Instance::new(renderer, parent, element, container));

        // Finally, we pretend that this is simply a re-render.
        self.rerender(&instance);

        instance
    }

    pub fn finish(mut self) {
        self.buffer.layout();

        // Finally, we apply the effects and we're done!
        for effect in self.layout_effects.into_iter() {
            let instance = effect.instance().clone();
            let memory = instance.memory();
            let link = EffectLink::new(&instance, &memory);

            effect.invoke(&link, &mut self.buffer);
        }

        self.buffer.commit();

        let effects = self.effects;

        self.renderer.bus.borrow().queue_retain(async move {
            for effect in effects.into_iter() {
                let instance = effect.instance().clone();
                let memory = instance.memory();
                let link = EffectLink::new(&instance, &memory);

                effect.invoke(&link);
            }
        });
    }
}

pub struct Renderer<P>
where
    P: Platform + ?Sized,
{
    compositor: RefCell<P::Compositor>,
    bus: RefCell<EventLoop>,
}

impl<P> Renderer<P>
where
    P: Platform + ?Sized,
{
    /// This function returns a new reference counted renderer with the given
    /// compositor.
    pub fn new(compositor: P::Compositor, bus: EventLoop) -> Rc<Renderer<P>> {
        Rc::new(Renderer {
            compositor: RefCell::new(compositor),
            bus: RefCell::new(bus),
        })
    }

    pub fn queue_rerender(self: &Rc<Self>, instance: &Rc<Instance<P>>) {
        let renderer = self.clone();
        let instance = instance.clone();

        self.bus.borrow().queue_retain(async move {
            let mut render = Render::new(renderer);
            render.rerender(&instance);
            render.finish();
        });
    }

    pub fn render(
        self: &Rc<Self>,
        element: Element<P>,
        container: P::ContainerID,
    ) -> Rc<Instance<P>> {
        let mut render = Render::new(self.clone());
        let instance = render.render(None, element, container);
        render.finish();

        instance
    }
}

/// This is the entry point of Polyhorn. This function renders an element into
/// the given container. The returned instance must be retained. Once the
/// returned is dropped, all UI will be unmounted.
pub fn render<F, P>(element: F, container: P::Container) -> Disposable
where
    F: FnOnce() -> Element<P> + Send + 'static,
    P: Platform + ?Sized,
{
    P::with_compositor(container, move |container_id, compositor, bus| {
        // We've now switched to the render thread.
        let renderer = Renderer::new(compositor, bus);
        Disposable::new(renderer.render(element(), container_id))
    })
}
