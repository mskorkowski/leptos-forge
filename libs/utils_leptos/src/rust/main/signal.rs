//! Generic implementation of the [RwSignal] to make it more useful for writing applications
//!
//! Author of leptos thinks RwSignal can be confusing, I say it it's a developer responsibility to not make it
//! confusing by:
//!
//! 1. Naming stuff correctly
//! 2. Writing documentation that explains what the signal does and how to use it
//! 3. Believing  in the intelligence of the user (that might be somewhat questionable from time to time, I agree)
//!

use std::panic::Location;

use leptos::attr::Attribute;
use leptos::attr::AttributeValue;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::Track;
use leptos::prelude::guards::ReadGuard;
use leptos::tachys::html::property::IntoProperty;
use leptos::tachys::hydration::Cursor;
use leptos::tachys::reactive_graph::RenderEffectState;
use leptos::tachys::reactive_graph::bind::IntoSplitSignal;
use leptos::tachys::renderer::types::Element;
use leptos::tachys::ssr::StreamBuilder;
use leptos::tachys::view::Mountable;
use leptos::tachys::view::Position;
use leptos::tachys::view::PositionState;
use leptos::tachys::view::Render;
use leptos::tachys::view::RenderHtml;
use leptos::tachys::view::add_attr::AddAnyAttr;

use leptos::server_fn::serde::Serialize;
use leptos::server_fn::serde::Serializer;
use reactive_stores::StoreField;
use reactive_stores::Subfield;

use reactive_graph::effect::RenderEffect;
use reactive_graph::owner::SyncStorage;
use reactive_graph::signal::signal;
use reactive_graph::traits::DefinedAt;
use reactive_graph::traits::Dispose;
use reactive_graph::traits::Get;
use reactive_graph::traits::ReadUntracked;
use reactive_graph::traits::Set;
use reactive_graph::traits::Update;
use reactive_graph::traits::With;
use reactive_graph::wrappers::read::Signal;
use reactive_graph::wrappers::read::SignalReadGuard;
use reactive_graph::wrappers::write::SignalSetter;
use utils::prelude::ThreadSafe;

/// Signal which allows reading and writing a value
///
/// You should never, ever create an RwSignal which reads from one value and writes to the other. I consider
/// you to be warned.
#[derive(Debug)]
pub struct URwSignal<T>
where
    T: ThreadSafe,
{
    /// Location where the URwSignal was created
    defined_at: &'static Location<'static>,
    /// signal from which we can read the value
    read_signal: Signal<T>,
    /// signal from where we can write the value to
    write_signal: SignalSetter<T>,
}

impl<T> Clone for URwSignal<T>
where
    T: ThreadSafe,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for URwSignal<T> where T: ThreadSafe {}

impl<T> URwSignal<T>
where
    T: ThreadSafe,
{
    /// Creates a new RwSignal with initial value `initial`
    #[track_caller]
    pub fn new(initial: T) -> Self {
        let (read, write) = signal(initial);
        Self {
            defined_at: Location::caller(),
            read_signal: read.into(),
            write_signal: write.into(),
        }
    }
}

impl<T> URwSignal<T>
where
    T: ThreadSafe + Clone,
{
    /// Returns a read only part of the RwSignal
    pub fn read_only(&self) -> Signal<T> {
        self.read_signal
    }

    /// Returns a write only part of the RwSignal
    pub fn write_only(&self) -> SignalSetter<T> {
        self.write_signal
    }

    /// Transforms signal of T into signal of A
    ///
    /// Allows consistent read/write with the derived URwSignal
    #[track_caller]
    pub fn map<A>(
        &self,
        towards: impl Fn(&T) -> A + Send + Sync + 'static,
        from: impl Fn(&mut T, A) + Send + Sync + 'static,
    ) -> URwSignal<A>
    where
        A: ThreadSafe,
    {
        let read: Signal<T> = self.read_signal;
        let new_read: Signal<A> = Signal::derive(move || read.with(|t| towards(t)));

        let write: SignalSetter<T> = self.write_signal;
        let new_write = SignalSetter::map(move |a: A| {
            let mut t = read.get();
            from(&mut t, a);
            write.set(t);
        });

        URwSignal {
            defined_at: Location::caller(),
            read_signal: new_read,
            write_signal: new_write,
        }
    }
}

impl<T> Dispose for URwSignal<T>
where
    T: ThreadSafe,
{
    fn dispose(self) {
        self.read_signal.dispose();
    }
}

impl<T> DefinedAt for URwSignal<T>
where
    T: ThreadSafe,
{
    fn defined_at(&self) -> Option<&'static Location<'static>> {
        { Some(self.defined_at) }
    }
}

impl<T> PartialEq for URwSignal<T>
where
    T: ThreadSafe,
{
    fn eq(&self, other: &Self) -> bool {
        self.read_signal == other.read_signal
    }
}

impl<T> Eq for URwSignal<T> where T: ThreadSafe {}

impl<T> ReadUntracked for URwSignal<T>
where
    T: ThreadSafe + Clone,
{
    type Value = ReadGuard<T, SignalReadGuard<T, SyncStorage>>;

    fn try_read_untracked(&self) -> Option<Self::Value> {
        self.read_signal.try_read_untracked()
    }
}

impl<T> Set for URwSignal<T>
where
    T: ThreadSafe,
{
    type Value = T;

    fn set(&self, new_value: Self::Value) {
        self.write_signal.set(new_value);
    }

    fn try_set(&self, value: Self::Value) -> Option<Self::Value> {
        self.write_signal.try_set(value)
    }
}

impl<T> Get for URwSignal<T>
where
    T: ThreadSafe + Clone,
{
    type Value = T;

    fn try_get(&self) -> Option<Self::Value> {
        self.read_signal.try_get()
    }
}

impl<T> IntoProperty for URwSignal<T>
where
    T: 'static + IntoProperty + Clone + Send + Sync,
    <T as IntoProperty>::State: 'static,
    URwSignal<T>: Get<Value = T> + Clone,
{
    type State = RenderEffect<<T as IntoProperty>::State>;
    type Cloneable = Self;
    type CloneableOwned = Self;

    fn build(self, el: &Element, key: &str) -> Self::State {
        (move || self.get()).build(el, key)
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &Element, key: &str) -> Self::State {
        (move || self.get()).hydrate::<FROM_SERVER>(el, key)
    }

    fn rebuild(self, state: &mut Self::State, key: &str) {
        (move || self.get()).rebuild(state, key)
    }

    fn into_cloneable(self) -> Self::Cloneable {
        self
    }

    fn into_cloneable_owned(self) -> Self::CloneableOwned {
        self
    }
}

impl<T> From<T> for URwSignal<T>
where
    T: ThreadSafe,
{
    #[track_caller]
    fn from(value: T) -> Self {
        let (read, write) = signal(value);

        URwSignal {
            defined_at: Location::caller(),
            read_signal: read.into(),
            write_signal: write.into(),
        }
    }
}

impl<T> Render for URwSignal<T>
where
    T: ThreadSafe + Clone + Render,
    <T as Render>::State: 'static,
    URwSignal<T>: Get<Value = T>,
{
    type State = RenderEffectState<<T as Render>::State>;

    fn build(self) -> Self::State {
        (move || self.get()).build()
    }

    fn rebuild(self, state: &mut Self::State) {
        let new = self.build();
        let mut old = std::mem::replace(state, new);
        old.insert_before_this(state);
        old.unmount();
    }
}

impl<T> RenderHtml for URwSignal<T>
where
    T: ThreadSafe + Clone + RenderHtml,
    <T as Render>::State: 'static,
    URwSignal<T>: Get<Value = T>,
{
    type AsyncOutput = Self;
    type Owned = Self;

    const MIN_LENGTH: usize = 0;

    fn dry_resolve(&mut self) {}

    async fn resolve(self) -> Self::AsyncOutput {
        self
    }

    fn html_len(&self) -> usize {
        T::MIN_LENGTH
    }

    fn to_html_with_buf(
        self,
        buf: &mut String,
        position: &mut Position,
        escape: bool,
        mark_branches: bool,
        extra_attrs: Vec<AnyAttribute>,
    ) {
        let value = self.get();
        value.to_html_with_buf(buf, position, escape, mark_branches, extra_attrs)
    }

    fn to_html_async_with_buf<const OUT_OF_ORDER: bool>(
        self,
        buf: &mut StreamBuilder,
        position: &mut Position,
        escape: bool,
        mark_branches: bool,
        extra_attrs: Vec<AnyAttribute>,
    ) where
        Self: Sized,
    {
        let value = self.get();
        value.to_html_async_with_buf::<OUT_OF_ORDER>(
            buf,
            position,
            escape,
            mark_branches,
            extra_attrs,
        );
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor,
        position: &PositionState,
    ) -> Self::State {
        (move || self.get()).hydrate::<FROM_SERVER>(cursor, position)
    }

    fn into_owned(self) -> Self::Owned {
        self
    }
}

impl<T> AddAnyAttr for URwSignal<T>
where
    T: ThreadSafe + Clone + RenderHtml,
    <T as Render>::State: 'static,
    URwSignal<T>: Get<Value = T>,
{
    type Output<SomeNewAttr: Attribute> = Self;

    fn add_any_attr<NewAttr: Attribute>(self, _attr: NewAttr) -> Self::Output<NewAttr>
    where
        Self::Output<NewAttr>: RenderHtml,
    {
        todo!()
    }
}

impl<T> AttributeValue for URwSignal<T>
where
    T: ThreadSafe + Clone + AttributeValue,
    <T as AttributeValue>::State: 'static,
    URwSignal<T>: Get<Value = T>,
{
    type AsyncOutput = Self;
    type State = RenderEffect<<T as AttributeValue>::State>;
    type Cloneable = Self;
    type CloneableOwned = Self;

    fn html_len(&self) -> usize {
        0
    }

    fn to_html(self, key: &str, buf: &mut String) {
        let value = self.get();
        value.to_html(key, buf);
    }

    fn to_template(_key: &str, _buf: &mut String) {}

    fn hydrate<const FROM_SERVER: bool>(self, key: &str, el: &Element) -> Self::State {
        (move || self.get()).hydrate::<FROM_SERVER>(key, el)
    }

    fn build(self, el: &Element, key: &str) -> Self::State {
        (move || self.get()).build(el, key)
    }

    fn rebuild(self, key: &str, state: &mut Self::State) {
        (move || self.get()).rebuild(key, state)
    }

    fn into_cloneable(self) -> Self::Cloneable {
        self
    }

    fn into_cloneable_owned(self) -> Self::CloneableOwned {
        self
    }

    fn dry_resolve(&mut self) {}

    async fn resolve(self) -> Self::AsyncOutput {
        self
    }
}

impl<T> Serialize for URwSignal<T>
where
    T: ThreadSafe + Serialize,
{
    fn serialize<Ser>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error>
    where
        Ser: Serializer,
    {
        self.read_signal.serialize(serializer)
    }
}

impl<T> From<URwSignal<T>> for Signal<T>
where
    T: ThreadSafe,
{
    #[track_caller]
    fn from(val: URwSignal<T>) -> Self {
        val.read_signal
    }
}

impl<T> IntoSplitSignal for URwSignal<T>
where
    T: ThreadSafe + Clone,
{
    type Value = T;
    type Read = Signal<T>;
    type Write = SignalSetter<T>;

    fn into_split_signal(self) -> (Self::Read, Self::Write) {
        (self.read_signal, self.write_signal)
    }
}

impl<Inner, Prev, T> From<Subfield<Inner, Prev, T>> for URwSignal<T>
where
    Inner: StoreField<Value = Prev> + Track + ThreadSafe + Clone,
    Prev: 'static,
    T: Send + Sync + Clone + 'static,
{
    #[track_caller]
    fn from(value: Subfield<Inner, Prev, T>) -> Self {
        let r: Signal<T> = value.clone().into();
        let w: SignalSetter<T> = SignalSetter::map(move |t| {
            value.update(|v| {
                *v = t;
            });
        });

        URwSignal {
            defined_at: Location::caller(),
            read_signal: r,
            write_signal: w,
        }
    }
}

impl<T> From<URwSignal<T>> for SignalSetter<T>
where
    T: Send + Sync + 'static,
{
    fn from(value: URwSignal<T>) -> Self {
        value.write_signal
    }
}

impl<T: ThreadSafe + Default> Default for URwSignal<T> {
    fn default() -> Self {
        URwSignal::new(T::default())
    }
}
