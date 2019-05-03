use sm::{AsEnum, Event, InitialState, Initializer, Machine, NoneEvent, State, Transition};
use amethyst::core::nalgebra as na;

// --------
// MACHINE
// --------

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MouseDrag<S: State, E: Event>(S, Option<E>);

impl<S: State, E: Event> Machine for MouseDrag<S, E> {
    type Event = E;
    type State = S;

    fn state(&self) -> Self::State {
        self.0.clone()
    }

    fn trigger(&self) -> Option<Self::Event> {
        self.1.clone()
    }
}

impl<S: InitialState> Initializer<S> for MouseDrag<S, NoneEvent> {
    type Machine = MouseDrag<S, NoneEvent>;

    fn new(state: S) -> Self::Machine {
        MouseDrag(state, None)
    }
}

// -------
// STATES
// -------

// NotDragging
// ===========
#[derive(Clone, Debug, Eq)]
pub struct NotDragging;

impl State for NotDragging {}

impl InitialState for NotDragging {}

// StartDragging
// =============
#[derive(Clone, Debug)]
pub struct StartDragging {
    pub start_position: na::Point2<f64>,
}

impl State for StartDragging {}

impl Eq for StartDragging {}

// Dragging
// ========
#[derive(Clone, Debug)]
pub struct Dragging {
    pub start_position: na::Point2<f64>,
    pub delta_start_position: na::Point2<f64>,
    pub delta_end_position: na::Point2<f64>,
}

impl State for Dragging {}

impl Eq for Dragging {}

// FinishDragging
// ==============
#[derive(Clone, Debug)]
pub struct FinishDragging {
    pub start_position: na::Point2<f64>,
    pub delta_start_position: na::Point2<f64>,
    pub end_position: na::Point2<f64>,
}

impl State for FinishDragging {}

impl Eq for FinishDragging {}

// --------------
// STATE VARIANT
// --------------
#[derive(Clone, Debug)]
pub enum StateVariant {
    NotDragging(NotDragging),
    StartDragging(StartDragging),
    Dragging(Dragging),
    FinishDragging(FinishDragging),
}

impl AsEnum for NotDragging {
    type Enum = StateVariant;

    fn as_enum(self) -> Self::Enum {
        StateVariant::NotDragging(self)
    }
}

impl AsEnum for StartDragging {
    type Enum = StateVariant;

    fn as_enum(self) -> Self::Enum {
        StateVariant::StartDragging(self)
    }
}

impl AsEnum for Dragging {
    type Enum = StateVariant;

    fn as_enum(self) -> Self::Enum {
        StateVariant::Dragging(self)
    }
}

impl AsEnum for FinishDragging {
    type Enum = StateVariant;

    fn as_enum(self) -> Self::Enum {
        StateVariant::FinishDragging(self)
    }
}

// ---------
// STATE EQ
// ---------

// NotDragging
// ===========
impl PartialEq<NotDragging> for NotDragging {
    fn eq(&self, _: &NotDragging) -> bool {
        true
    }
}

impl PartialEq<StartDragging> for NotDragging {
    fn eq(&self, _: &StartDragging) -> bool {
        false
    }
}

impl PartialEq<Dragging> for NotDragging {
    fn eq(&self, _: &Dragging) -> bool {
        false
    }
}

impl PartialEq<FinishDragging> for NotDragging {
    fn eq(&self, _: &FinishDragging) -> bool {
        false
    }
}

// StartDragging
// =============
impl PartialEq<StartDragging> for StartDragging {
    fn eq(&self, _: &StartDragging) -> bool {
        true
    }
}

impl PartialEq<Dragging> for StartDragging {
    fn eq(&self, _: &Dragging) -> bool {
        false
    }
}

impl PartialEq<FinishDragging> for StartDragging {
    fn eq(&self, _: &FinishDragging) -> bool {
        false
    }
}

// Dragging
// ========
impl PartialEq<Dragging> for Dragging {
    fn eq(&self, _: &Dragging) -> bool {
        true
    }
}

impl PartialEq<FinishDragging> for Dragging {
    fn eq(&self, _: &FinishDragging) -> bool {
        false
    }
}

// Finish
// ========
impl PartialEq<FinishDragging> for FinishDragging {
    fn eq(&self, _: &FinishDragging) -> bool {
        true
    }
}

// -------
// EVENTS
// -------

// Clicked
// =======
#[derive(Clone, Debug)]
pub struct Clicked {
    pub position: na::Point2<f64>,
}

impl Event for Clicked {}

impl Eq for Clicked {}

// NotClicked
// ==========
#[derive(Clone, Debug)]
pub struct NotClicked {
    pub position: na::Point2<f64>,
}

impl Event for NotClicked {}

impl Eq for NotClicked {}

// --------------
// EVENT VARIANT
// --------------

#[derive(Clone, Debug)]
pub enum EventVariant {
    Clicked(Clicked),
    NotClicked(NotClicked),
}

impl AsEnum for Clicked {
    type Enum = EventVariant;

    fn as_enum(self) -> Self::Enum {
        EventVariant::Clicked(self)
    }
}

impl AsEnum for NotClicked {
    type Enum = EventVariant;

    fn as_enum(self) -> Self::Enum {
        EventVariant::NotClicked(self)
    }
}

// ---------
// EVENT EQ
// ---------
impl PartialEq<NotClicked> for NotClicked {
    fn eq(&self, _: &NotClicked) -> bool {
        true
    }
}

impl PartialEq<Clicked> for Clicked {
    fn eq(&self, _: &Clicked) -> bool {
        true
    }
}

impl PartialEq<Clicked> for NotClicked {
    fn eq(&self, _: &Clicked) -> bool {
        false
    }
}

// ------------
// TRANSITIONS
// ------------

impl<E: Event> Transition<Clicked> for MouseDrag<NotDragging, E> {
    type Machine = MouseDrag<StartDragging, Clicked>;

    fn transition(self, event: Clicked) -> Self::Machine {
        MouseDrag(
            StartDragging {
                start_position: event.position,
            },
            Some(event),
        )
    }
}

impl<E: Event> Transition<NotClicked> for MouseDrag<NotDragging, E> {
    type Machine = MouseDrag<NotDragging, NotClicked>;

    fn transition(self, event: NotClicked) -> Self::Machine {
        MouseDrag(NotDragging, Some(event))
    }
}

impl<E: Event> Transition<Clicked> for MouseDrag<StartDragging, E> {
    type Machine = MouseDrag<Dragging, Clicked>;

    fn transition(self, event: Clicked) -> Self::Machine {
        MouseDrag(
            Dragging {
                start_position: self.state().start_position,
                delta_start_position: self.state().start_position,
                delta_end_position: event.position,
            },
            Some(event),
        )
    }
}

impl<E: Event> Transition<NotClicked> for MouseDrag<StartDragging, E> {
    type Machine = MouseDrag<FinishDragging, NotClicked>;

    fn transition(self, event: NotClicked) -> Self::Machine {
        MouseDrag(
            FinishDragging {
                start_position: self.state().start_position,
                delta_start_position: self.state().start_position,
                end_position: event.position,
            },
            Some(event),
        )
    }
}

impl<E: Event> Transition<Clicked> for MouseDrag<Dragging, E> {
    type Machine = MouseDrag<Dragging, Clicked>;

    fn transition(self, event: Clicked) -> Self::Machine {
        MouseDrag(
            Dragging {
                start_position: self.state().start_position,
                delta_start_position: self.state().delta_end_position,
                delta_end_position: event.position,
            },
            Some(event),
        )
    }
}

impl<E: Event> Transition<NotClicked> for MouseDrag<Dragging, E> {
    type Machine = MouseDrag<FinishDragging, NotClicked>;

    fn transition(self, event: NotClicked) -> Self::Machine {
        MouseDrag(
            FinishDragging {
                start_position: self.state().start_position,
                delta_start_position: self.state().delta_end_position,
                end_position: event.position,
            },
            Some(event),
        )
    }
}

impl<E: Event> Transition<Clicked> for MouseDrag<FinishDragging, E> {
    type Machine = MouseDrag<StartDragging, Clicked>;

    fn transition(self, event: Clicked) -> Self::Machine {
        MouseDrag(
            StartDragging {
                start_position: event.position,
            },
            Some(event),
        )
    }
}

impl<E: Event> Transition<NotClicked> for MouseDrag<FinishDragging, E> {
    type Machine = MouseDrag<NotDragging, NotClicked>;

    /// FinishDragging -> NotDragging
    fn transition(self, event: NotClicked) -> Self::Machine {
        MouseDrag(NotDragging, Some(event))
    }
}

// --------
// VARIANT
// --------
#[derive(Clone, Debug)]
pub enum Variant {
    InitialNotDragging(MouseDrag<NotDragging, NoneEvent>),
    NotDraggingByNotClicked(MouseDrag<NotDragging, NotClicked>),
    StartDraggingByClicked(MouseDrag<StartDragging, Clicked>),
    DraggingByClicked(MouseDrag<Dragging, Clicked>),
    FinishDraggingByNotClicked(MouseDrag<FinishDragging, NotClicked>),
}

impl Default for Variant {
    fn default() -> Self {
        MouseDrag::new(NotDragging).as_enum()
    }
}

impl AsEnum for MouseDrag<NotDragging, NoneEvent> {
    type Enum = Variant;

    fn as_enum(self) -> Self::Enum {
        Variant::InitialNotDragging(self)
    }
}

impl AsEnum for MouseDrag<NotDragging, NotClicked> {
    type Enum = Variant;

    fn as_enum(self) -> Self::Enum {
        Variant::NotDraggingByNotClicked(self)
    }
}

impl AsEnum for MouseDrag<StartDragging, Clicked> {
    type Enum = Variant;

    fn as_enum(self) -> Self::Enum {
        Variant::StartDraggingByClicked(self)
    }
}

impl AsEnum for MouseDrag<Dragging, Clicked> {
    type Enum = Variant;
    fn as_enum(self) -> Self::Enum {
        Variant::DraggingByClicked(self)
    }
}

impl AsEnum for MouseDrag<FinishDragging, NotClicked> {
    type Enum = Variant;

    fn as_enum(self) -> Self::Enum {
        Variant::FinishDraggingByNotClicked(self)
    }
}
