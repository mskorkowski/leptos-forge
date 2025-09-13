//! Module contains the drawer on the right side of the work area

use leptos::prelude::*;

/// Shows a collapsible pane on the right hand side
#[component]
pub fn Drawer() -> impl IntoView {
    view! {
      <div class="w-96 flex-none flex flex-col bg-forgegray-100">
        Drawer
        <div class="my-2 mx-2 border-y border-forgegray-300">
          <div class="font-bold">Transakcja</div>
          <div class="">Opis zmian</div>
        </div>
        <div class="my-2 mx-2 border-y border-forgegray-300">
          <div class="font-bold">Transakcja</div>
          <div class="">Opis zmian</div>
        </div>
        <div class="my-2 mx-2 border-y border-forgegray-300">
          <div class="font-bold">Transakcja</div>
          <div class="">Opis zmian</div>
        </div>
        <div class="my-2 mx-2 border-y border-forgegray-300">
          <div class="font-bold">Transakcja</div>
          <div class="">Opis zmian</div>
        </div>
      </div>
    }
}
