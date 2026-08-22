// //! Generic implementation of the [Option<RwSignal>][reactive_graph::signal::RwSignal] to make it more useful for writing applications
// //!
// //! 
// //! 
// //! Author of leptos thinks RwSignal can be confusing, I say it it's a developer responsibility to not make it
// //! confusing by:
// //!
// //! 1. Naming stuff correctly
// //! 2. Writing documentation that explains what the signal does and how to use it
// //! 3. Believing  in the intelligence of the user (that might be somewhat questionable from time to time, I agree)
// //!

// use std::panic::Location;

// use leptos::attr::Attribute;
// use leptos::attr::AttributeValue;
// use leptos::attr::any_attribute::AnyAttribute;
// use leptos::leptos_dom::logging::console_log;
// use leptos::prelude::MaybeProp;
// use leptos::prelude::Track;
// use leptos::prelude::guards::ReadGuard;
// use leptos::tachys::html::property::IntoProperty;
// use leptos::tachys::hydration::Cursor;
// use leptos::tachys::reactive_graph::RenderEffectState;
// use leptos::tachys::reactive_graph::bind::IntoSplitSignal;
// use leptos::tachys::renderer::types::Element;
// use leptos::tachys::ssr::StreamBuilder;
// use leptos::tachys::view::Mountable;
// use leptos::tachys::view::Position;
// use leptos::tachys::view::PositionState;
// use leptos::tachys::view::Render;
// use leptos::tachys::view::RenderHtml;
// use leptos::tachys::view::add_attr::AddAnyAttr;

// use serde::*;

// use reactive_stores::Field;
// use reactive_stores::Store;
// use reactive_stores::StoreField;
// use reactive_stores::Subfield;

// use reactive_graph::effect::RenderEffect;
// use reactive_graph::owner::SyncStorage;
// use reactive_graph::traits::DefinedAt;
// use reactive_graph::traits::Dispose;
// use reactive_graph::traits::Get;
// use reactive_graph::traits::ReadUntracked;
// use reactive_graph::traits::Set;
// use reactive_graph::wrappers::read::Signal;
// use reactive_graph::wrappers::read::SignalReadGuard;
// use reactive_graph::wrappers::write::SignalSetter;
// use utils::prelude::ThreadSafe;

// use crate::signal::URwSignal;

// use std::sync::atomic::AtomicUsize;
// use std::sync::atomic::Ordering;

// static NEXT_MAYBE_SIGNAL: AtomicUsize = AtomicUsize::new(0);

// /// Signal which allows reading and writing a value
// ///
// /// You should never, ever create an RwSignal which reads from one value and writes to the other. I consider
// /// you to be warned.
// #[derive(Debug, Serialize)]
// pub struct MaybeURwSignal<T>(Option<URwSignal<T>>, usize)
// where
//     T: ThreadSafe
// ;


// impl<T> MaybeURwSignal<T>
// where
//     T: ThreadSafe
// {
//     #[track_caller]
//     fn new_instance(inner: Option<URwSignal<T>>) -> Self {
//         let id = NEXT_MAYBE_SIGNAL.fetch_add(1, Ordering::Relaxed);
//         let location = Location::caller();
//         console_log(&format!(
//                 "creating maybe urw signal: {} @ {:?} from {:?}", 
//                 id, 
//                 inner.map(|s| format!("{:?}", s.defined_at())),
//                 location,
//             ));
//         Self(
//             inner,
//             id,
//         )
//     }
// }

// impl<T> Clone for MaybeURwSignal<T>
// where
//     T: ThreadSafe,
// {
//     #[track_caller]
//     fn clone(&self) -> Self {
//         Self::new_instance(self.0.clone())
//     }
// }


// impl<T> MaybeURwSignal<T>
// where
//     T: ThreadSafe,
// {
//     /// Creates a new MaybeURwSignal with initial value `initial`
//     #[track_caller]
//     pub fn new(initial: T) -> Self {
//         Self::new_instance(Some(URwSignal::new(initial)))
//     }

//     /// Creates an empty MayURwSignal
//     #[track_caller]
//     pub fn empty() -> Self {
//         Self::new_instance(None)
//     }
// }

// impl<T> Default for MaybeURwSignal<T>
// where
//     T: ThreadSafe,
// {
//     fn default() -> Self {
//         Self::empty()
//     }
// }

// impl<T> MaybeURwSignal<T>
// where
//     T: ThreadSafe + Clone,
// {
//     /// Returns a read only part of the RwSignal
//     pub fn read_only(&self) -> Option<Signal<T>> {
//         self.0.map(|s| s.read_only() )
//     }

//     /// Returns a write only part of the RwSignal
//     pub fn write_only(&self) -> Option<SignalSetter<T>> {
//         self.0.map(|s| s.write_only() )
//     }

//     /// Transforms signal of T into signal of A
//     ///
//     /// Allows consistent read/write with the derived URwSignal
//     #[track_caller]
//     pub fn map<A>(
//         &self,
//         towards: impl Fn(&T) -> A + Send + Sync + 'static,
//         from: impl Fn(&mut T, A) + Send + Sync + 'static,
//     ) -> MaybeURwSignal<A>
//     where
//         A: ThreadSafe,
//     {
//         let inner: Option<URwSignal<A>> = self.0.map(|s| s.map(towards, from));

//         MaybeURwSignal::new_instance(inner)
//     }
// }

// impl<T> Dispose for MaybeURwSignal<T>
// where
//     T: ThreadSafe,
// {
//     fn dispose(mut self) {
//         console_log(&format!("Disposing signal {} defined at {:?}", self.1, self.0.defined_at()));
//         if let Some(s) = self.0 {
//             s.dispose()
//         }

//         self.0 = None;
//     }
// }

// impl<T> DefinedAt for MaybeURwSignal<T>
// where
//     T: ThreadSafe,
// {
//     fn defined_at(&self) -> Option<&'static Location<'static>> {
//         self.0.map(|s| s.defined_at()).flatten()
//     }
// }

// impl<T> PartialEq for MaybeURwSignal<T>
// where
//     T: ThreadSafe,
// {
//     fn eq(&self, other: &Self) -> bool {
//         match (self.0, other.0) {
//             (None, None) => true,
//             (None, _) => false,
//             (_, None) => false,
//             (Some(me), Some(other)) => me == other
//         }
//     }
// }

// impl<T> Eq for MaybeURwSignal<T> where T: ThreadSafe {}

// impl<T> ReadUntracked for MaybeURwSignal<T>
// where
//     T: ThreadSafe + Clone,
// {
//     type Value = ReadGuard<T, SignalReadGuard<T, SyncStorage>>;

//     fn try_read_untracked(&self) -> Option<Self::Value> {
//         self.0.map(|s| s.try_read_untracked()).flatten()
//     }
// }

// impl<T> Set for MaybeURwSignal<T>
// where
//     T: ThreadSafe,
// {
//     type Value = Option<T>;

//     fn set(&self, new_value: Self::Value) {
//         self.0.map(|s| s.set(new_value));
//     }

//     fn try_set(&self, value: Self::Value) -> Option<Self::Value> {
//         if let Some(s) = self.0 {
//             s.try_set(value)
//         }
//         else {
//             Some(value)
//         }
//     }
// }

// //
// //
// // Code below was copied from tachys::reactive_graph::reactive_impl macro
// //
// //


// impl<T> Render for MaybeURwSignal<T>
// where
//     T: Render + Clone + Send + Sync + 'static,
//     <T as Render>::State: 'static,
//     MaybeURwSignal<T>: Get<Value = T>,
//     T: ThreadSafe
// {
//     type State = RenderEffectState<<T as Render>::State>;

//     #[track_caller]
//     fn build(self) -> Self::State {
//         (move || self.get()).build()
//     }

//     #[track_caller]
//     fn rebuild(self, state: &mut Self::State) {
//         let new = self.build();
//         let mut old = std::mem::replace(state, new);
//         old.insert_before_this(state);
//         old.unmount();
//     }
// }

// impl<T> AddAnyAttr for MaybeURwSignal<T>
// where
//     T: RenderHtml + Clone + Send + Sync + 'static,
//     <T as Render>::State: 'static,
//     MaybeURwSignal<T>: Get<Value = T>,
//     T: ThreadSafe
// {
//     type Output<SomeNewAttr: Attribute> = Self;

//     fn add_any_attr<NewAttr: Attribute>(
//         self,
//         _attr: NewAttr,
//     ) -> Self::Output<NewAttr> {
//         todo!()
//     }
// }

// impl<T> RenderHtml for MaybeURwSignal<T>
// where
//     T: RenderHtml + Clone + Send + Sync + 'static,
//     <T as Render>::State: 'static,
//     MaybeURwSignal<T>: Get<Value = T>,
//     T: ThreadSafe
// {
//     type AsyncOutput = Self;
//     type Owned = Self;

//     const MIN_LENGTH: usize = 0;

//     fn dry_resolve(&mut self) {
//         if true {
//             _ = self.get();
//         }
//     }

//     async fn resolve(self) -> Self::AsyncOutput {
//         self
//     }

//     fn html_len(&self) -> usize {
//         <Option<T>>::MIN_LENGTH
//     }

//     fn to_html_with_buf(
//         self,
//         buf: &mut String,
//         position: &mut Position,
//         escape: bool,
//         mark_branches: bool,
//         extra_attrs: Vec<AnyAttribute>,
//     ) {
//         let value = self.get();
//         value.to_html_with_buf(
//             buf,
//             position,
//             escape,
//             mark_branches,
//             extra_attrs,
//         )
//     }

//     fn to_html_async_with_buf<const OUT_OF_ORDER: bool>(
//         self,
//         buf: &mut StreamBuilder,
//         position: &mut Position,
//         escape: bool,
//         mark_branches: bool,
//         extra_attrs: Vec<AnyAttribute>,
//     ) where
//         Self: Sized,
//     {
//         let value = self.get();
//         value.to_html_async_with_buf::<OUT_OF_ORDER>(
//             buf,
//             position,
//             escape,
//             mark_branches,
//             extra_attrs,
//         );
//     }

//     fn hydrate<const FROM_SERVER: bool>(
//         self,
//         cursor: &Cursor,
//         position: &PositionState,
//     ) -> Self::State {
//         (move || self.get())
//             .hydrate::<FROM_SERVER>(cursor, position)
//     }

//     fn into_owned(self) -> Self::Owned {
//         self
//     }
// }

// impl<T> AttributeValue for MaybeURwSignal<T>
// where
//     T: AttributeValue + Send + Sync + Clone + 'static,
//     <T as AttributeValue>::State: 'static,
//     MaybeURwSignal<T>: Get<Value = T>,
//     T: ThreadSafe
// {
//     type AsyncOutput = Self;
//     type State = RenderEffect<<T as AttributeValue>::State>;
//     type Cloneable = Self;
//     type CloneableOwned = Self;

//     fn html_len(&self) -> usize {
//         0
//     }

//     fn to_html(self, key: &str, buf: &mut String) {
//         let value = self.get();
//         value.to_html(key, buf);
//     }

//     fn to_template(_key: &str, _buf: &mut String) {}

//     fn hydrate<const FROM_SERVER: bool>(
//         self,
//         key: &str,
//         el: &Element,
//     ) -> Self::State {
//         (move || self.get()).hydrate::<FROM_SERVER>(key, el)
//     }

//     fn build(
//         self,
//         el: &Element,
//         key: &str,
//     ) -> Self::State {
//         (
//             move || {
//                 console_log(&format!(
//                     "Building: {} from {:?}", 
//                     self.1, 
//                     self.0.map(|s| format!("{:?}", s.defined_at()))
//                 ));
//                 self.get()
//             }
//         ).build(el, key)
//     }

//     fn rebuild(self, key: &str, state: &mut Self::State) {
//         (move || self.get()).rebuild(key, state)
//     }

//     fn into_cloneable(self) -> Self::Cloneable {
//         self
//     }

//     fn into_cloneable_owned(self) -> Self::CloneableOwned {
//         self
//     }

//     fn dry_resolve(&mut self) {}

//     async fn resolve(self) -> Self::AsyncOutput {
//         self
//     }
// }


// //
// // End of copied code
// //

// impl<T> IntoProperty for MaybeURwSignal<T>
// where
//     T: 'static + IntoProperty + Clone + Send + Sync,
//     <T as IntoProperty>::State: 'static,
//     MaybeURwSignal<T>: Get<Value = T> + Clone,
// {
//     type State = RenderEffect<<T as IntoProperty>::State>;
//     type Cloneable = Self;
//     type CloneableOwned = Self;

//     fn build(self, el: &Element, key: &str) -> Self::State {
//         (move || {
//             if let Some(s) = self.0 {
//                 let location = s.defined_at();

//                 if let Some(location) = location {
//                     console_log(&format!("panic occurred in file '{}' at line {}", location.file(), location.line()));
//                 } else {
//                     console_log ("panic occurred but can't get location information...");
//                 }
//             }
//             else {
//                 console_log("Dunno");
//             }

//             console_log(&format!(
//                 "Building property: {} from {:?}", 
//                 self.1, 
//                 self.0.map(|s| format!("{:?}", s.defined_at()))
//             ));
//             self.get()
//         }).build(el, key)
//     }

//     fn hydrate<const FROM_SERVER: bool>(self, el: &Element, key: &str) -> Self::State {
//         (move || self.get()).hydrate::<FROM_SERVER>(el, key)
//     }

//     fn rebuild(self, state: &mut Self::State, key: &str) {
//         (move || self.get()).rebuild(state, key)
//     }

//     fn into_cloneable(self) -> Self::Cloneable {
//         self
//     }

//     fn into_cloneable_owned(self) -> Self::CloneableOwned {
//         self
//     }
// }

// impl<T> From<T> for MaybeURwSignal<T>
// where
//     T: ThreadSafe,
// {
//     #[track_caller]
//     fn from(value: T) -> Self {
//         Self::new(value)
//     }
// }

// impl<T> From<Option<T>> for MaybeURwSignal<T> 
// where
//     T: ThreadSafe
// {
//     #[track_caller]
//     fn from(value: Option<T>) -> Self {
//         if let Some(t) = value {
//             Self::new(t)
//         }
//         else {
//             Self::empty()
//         }
//     }
// }

// impl<T> From<MaybeURwSignal<T>> for Signal<Option<T>>
// where
//     T: ThreadSafe,
//     MaybeURwSignal<T>: Get<Value = Option<T>>
// {
//     #[track_caller]
//     fn from(val: MaybeURwSignal<T>) -> Self {
//         Signal::derive(move || val.get() )
//     }
// }

// impl<T> From<MaybeURwSignal<T>> for Signal<T>
// where
//     T: ThreadSafe,
//     MaybeURwSignal<T>: Get<Value = T>
// {
//     #[track_caller]
//     fn from(val: MaybeURwSignal<T>) -> Self {
//         Signal::derive(move || {
//             val.get() 
//         })
//     }
// }

// impl<T> IntoSplitSignal for MaybeURwSignal<T>
// where
//     T: ThreadSafe + Clone,
// {
//     type Value = T;
//     type Read = Signal<T>;
//     type Write = SignalSetter<T>;

//     fn into_split_signal(self) -> (Self::Read, Self::Write) {
//         (self.clone().into(), self.into())
//     }
// }

// impl<Inner, Prev, T> From<Subfield<Inner, Prev, T>> for MaybeURwSignal<T>
// where
//     Inner: StoreField<Value = Prev> + Track + ThreadSafe + Clone,
//     Prev: 'static,
//     T: Send + Sync + Clone + 'static,
// {
//     #[track_caller]
//     fn from(value: Subfield<Inner, Prev, T>) -> Self {
//         Self::new_instance(Some(value.into()))
//     }
// }

// impl<T> From<Field<T>> for MaybeURwSignal<T> 
// where
//     T: ThreadSafe + Clone,
// {
//     #[track_caller]
//     fn from(value: Field<T>) -> Self {
//         Self::new_instance(Some(value.into()))
//     }
// }

// impl<T> From<MaybeURwSignal<T>> for SignalSetter<T>
// where
//     T: Send + Sync + 'static,
// {
//     #[track_caller]
//     fn from(value: MaybeURwSignal<T>) -> Self {
//         SignalSetter::map(move |t| {
//             value.0.map(|s| s.set(t));
//         })

//     }
// }

// impl<T> From<MaybeURwSignal<T>> for SignalSetter<Option<T>>
// where
//     T: Send + Sync + 'static,
// {
//     #[track_caller]
//     fn from(value: MaybeURwSignal<T>) -> Self {
//         SignalSetter::map(move |t| {
//             value.0.map(|s| {
//                 if let Some(t) = t {
//                     s.set(t)
//                 }
//             });
//         })
//     }
// }

// impl<T> From<MaybeURwSignal<T>> for MaybeURwSignal<Option<T>> 
// where
//     T: ThreadSafe + Clone {

//     #[track_caller]
//     fn from(value: MaybeURwSignal<T>) -> Self {
//         Self::new_instance(
//             value.0.map(|s| s.into()),
//         )
//     }
// }

// impl<T> From<MaybeURwSignal<T>> for Signal<Option<T>> 
// where
//     T: ThreadSafe + Clone 
// {
//     #[track_caller]
//     fn from(value: MaybeURwSignal<T>) -> Self {
//         value.into()
//     }
// }

// impl<T> From<MaybeURwSignal<T>> for MaybeProp<T> 
// where
//     T: ThreadSafe + Clone 
// {
//     #[track_caller]
//     fn from(value: MaybeURwSignal<T>) -> Self {
//         let value: Signal<Option<T>> = value.into();
//         value.into()
//     }
// }

// impl<T> MaybeURwSignal<Option<T>> 
// where
//     T: ThreadSafe + Clone
// {
//     /// Unwrap implementation for a signal of `Option<T>`.
//     /// 
//     /// # Panics!
//     /// 
//     /// It will panic any time the signal has None as a value
//     pub fn unwrap(&self) -> MaybeURwSignal<T> {
//         self.map(
//             |v| {
//                 v.as_ref().unwrap().clone()
//             },
//             |v, n| {
//                 v.replace(n);
//             }
//         )
//     }
// }

// impl<T> From<Store<T>> for MaybeURwSignal<T> 
// where
//     T: ThreadSafe + Clone
// {
//     #[track_caller]
//     fn from(value: Store<T>) -> Self {
//         let value: Field<T> = value.into();

//         value.into()
//     }
// }

// impl<T> From<URwSignal<T>> for MaybeURwSignal<T>
// where
//     T: ThreadSafe + Clone
// {
//     #[track_caller]
//     fn from(value: URwSignal<T>) -> Self {
//         Self::new_instance(Some(value))
//     }
// }