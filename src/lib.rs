use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Event {
    Request,
    Response,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ComponentError {
    #[error("This is an example error {0}")]
    Example(u32)
}

pub trait Component
{
    type Item;
    fn handle_event(&mut self, event: Self::Item) -> Result<Self::Item, ComponentError>;
}

pub struct Server<C> {
    components: Vec<C>,
}

impl<C> Server<C>
where
    C: Component<Item=Event>
{
    pub fn new() -> Server<C> {
        Server {
            components: Vec::new(),
        }
    }

    pub fn add_component(&mut self, component: C) {
        self.components.push(component);
    }

    pub fn event(&mut self, event: Event) -> Result<Vec<Event>, ComponentError> {
        let mut events = Vec::new();
        for c in self.components.iter_mut() {
            let reply = c.handle_event(event)?;
            events.push(reply);
        }
        Ok(events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_handle_event() {
        let mut server = Server::<Comp>::new();

        // TODO: pass this test by implementing Component, etc.
        //

        assert_eq!(server.event(Event::Request), Ok(vec![Event::Response, Event::Response]));
        assert_eq!(server.event(Event::Response), Err(ComponentError::Example(42)));
    }
}
