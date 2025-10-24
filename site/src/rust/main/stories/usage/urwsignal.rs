//! Section describing URwSignal
//!

use forge::Section;

/// URwSignal description
const URW_SIGNAL: &str = r############"
# URwSignal

Generic purpose read-write signal witch allows mapping over value. It's useful 
when creating and testing components. It can be used in place of the `leptos::RwSignal`.

## Usage

Value transformation

```rust

struct MyStruct{
    x: i32,
}

let my_struct: URwSignal<MyStruct> = URwSignal::new(MyStruct{x: 5});

let value: URwSignal<i32> = my_struct.map(
    |value| value.x,
    |value, new_x| {
        *value.x = new_x;
    }
);

assert!(value.get() == 5);
value.set(7);
assert!(my_struct.get().x == 7);

```

URwSignal supports

- `IntoProperty`
- `Render`
- `RenderHtml`
- `AddAnyAttr`
- `AttributeValue`
- `IntoSplitSignal`
- `From<Subfield<_,_,T>>` to `URwSignal<T>`
- `From<URwSignal<T>>` to `Signal<T>`
- `From<URwSignal<T>>` to `SignalSetter<T>`

"############;

/// Section describing [URwSignal]
#[derive(Debug, Default, Copy, Clone)]
pub struct URwSignalSection;

impl Section for URwSignalSection {
    fn description(&self) -> &'static str {
        URW_SIGNAL
    }
}
