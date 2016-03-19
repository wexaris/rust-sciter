//! C interface for behaviors support (a.k.a windowless controls).

#![allow(non_camel_case_types, non_snake_case)]

use sctypes::*;
use scdom::*;
use scvalue::{VALUE};

#[repr(C)]
pub struct BEHAVIOR_EVENT_PARAMS
{
  pub cmd: UINT, // BEHAVIOR_EVENTS
  pub heTarget: HELEMENT,    // target element handler, in MENU_ITEM_CLICK this is owner element that caused this menu - e.g. context menu owner
                         // In scripting this field named as Event.owner
  pub he: HELEMENT,          // source element e.g. in SELECTION_CHANGED it is new selected <option>, in MENU_ITEM_CLICK it is menu item (LI) element
  pub reason: UINT_PTR,      // EVENT_REASON or EDIT_CHANGED_REASON - UI action causing change.
                         // In case of custom event notifications this may be any
                         // application specific value.
  pub data:   VALUE,  // auxiliary data accompanied with the event. E.g. FORM_SUBMIT event is using this field to pass collection of values.
}
