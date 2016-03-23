//! C interface for behaviors support (a.k.a windowless controls).

#![allow(non_camel_case_types, non_snake_case)]
#![allow(dead_code)]

use sctypes::*;
use scdom::*;
use scvalue::{VALUE};

#[repr(C)]
pub struct BEHAVIOR_EVENT_PARAMS
{
	pub cmd: UINT,
	pub heTarget: HELEMENT,

	pub he: HELEMENT,
	pub reason: UINT_PTR,

	pub data:   VALUE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum INITIALIZATION_EVENTS
{
	BEHAVIOR_DETACH = 0,
	BEHAVIOR_ATTACH = 1,
}

#[repr(C)]
pub struct INITIALIZATION_PARAMS
{
	pub cmd: INITIALIZATION_EVENTS,
}

#[repr(C)]
pub struct SCRIPTING_METHOD_PARAMS
{
	pub name: LPCSTR,
	pub argv: *const VALUE,
	pub argc: UINT,
	pub result: VALUE,
}


#[repr(C)]
#[derive(Copy, Clone)]
pub enum EVENT_GROUPS
{
	HANDLE_INITIALIZATION = 0x0000, /** attached/detached */
	HANDLE_MOUSE = 0x0001,          /** mouse events */
	HANDLE_KEY = 0x0002,            /** key events */
	HANDLE_FOCUS = 0x0004,          /** focus events, if this flag is set it also means that element it attached to is focusable */
	HANDLE_SCROLL = 0x0008,         /** scroll events */
	HANDLE_TIMER = 0x0010,          /** timer event */
	HANDLE_SIZE = 0x0020,           /** size changed event */
	HANDLE_DRAW = 0x0040,           /** drawing request (event) */
	HANDLE_DATA_ARRIVED = 0x080,    /** requested data () has been delivered */
	HANDLE_BEHAVIOR_EVENT        = 0x0100, /** logical, synthetic events:
	                                           BUTTON_CLICK, HYPERLINK_CLICK, etc.,
	                                           a.k.a. notifications from intrinsic behaviors */
	HANDLE_METHOD_CALL           = 0x0200, /** behavior specific methods */
	HANDLE_SCRIPTING_METHOD_CALL = 0x0400, /** behavior specific methods */
	HANDLE_TISCRIPT_METHOD_CALL  = 0x0800, /** behavior specific methods using direct tiscript::value's */

	HANDLE_EXCHANGE              = 0x1000, /** system drag-n-drop */
	HANDLE_GESTURE               = 0x2000, /** touch input events */

	HANDLE_ALL                   = 0xFFFF, /* all of them */
		
		/** special value for getting subscription flags */
	SUBSCRIPTIONS_REQUEST        = -1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum PHASE_MASK
{
	BUBBLING = 0,      // bubbling (emersion) phase
	SINKING  = 0x8000,  // capture (immersion) phase, this flag is or'ed with EVENTS codes below
	HANDLED  = 0x10000
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum EVENT_REASON
{
	BY_MOUSE_CLICK,
	BY_KEY_CLICK,
	SYNTHESIZED, // synthesized, programmatically generated.
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum EDIT_CHANGED_REASON
{
	BY_INS_CHAR,  // single char insertion
	BY_INS_CHARS, // character range insertion, clipboard
	BY_DEL_CHAR,  // single char deletion
	BY_DEL_CHARS, // character range deletion (selection)
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum BEHAVIOR_EVENTS
{
	BUTTON_CLICK = 0,              // click on button
	BUTTON_PRESS = 1,              // mouse down or key down in button
	BUTTON_STATE_CHANGED = 2,      // checkbox/radio/slider changed its state/value
	EDIT_VALUE_CHANGING = 3,       // before text change
	EDIT_VALUE_CHANGED = 4,        // after text change
	SELECT_SELECTION_CHANGED = 5,  // selection in <select> changed
	SELECT_STATE_CHANGED = 6,      // node in select expanded/collapsed, heTarget is the node

	POPUP_REQUEST   = 7,           // request to show popup just received,
	                               //     here DOM of popup element can be modifed.
	POPUP_READY     = 8,           // popup element has been measured and ready to be shown on screen,
	                               //     here you can use functions like ScrollToView.
	POPUP_DISMISSED = 9,           // popup element is closed,
	                               //     here DOM of popup element can be modifed again - e.g. some items can be removed
	                               //     to free memory.

	MENU_ITEM_ACTIVE = 0xA,        // menu item activated by mouse hover or by keyboard,
	MENU_ITEM_CLICK = 0xB,         // menu item click,
	                               //   BEHAVIOR_EVENT_PARAMS structure layout
	                               //   BEHAVIOR_EVENT_PARAMS.cmd - MENU_ITEM_CLICK/MENU_ITEM_ACTIVE
	                               //   BEHAVIOR_EVENT_PARAMS.heTarget - owner(anchor) of the menu
	                               //   BEHAVIOR_EVENT_PARAMS.he - the menu item, presumably <li> element
	                               //   BEHAVIOR_EVENT_PARAMS.reason - BY_MOUSE_CLICK | BY_KEY_CLICK


	CONTEXT_MENU_REQUEST = 0x10,   // "right-click", BEHAVIOR_EVENT_PARAMS::he is current popup menu HELEMENT being processed or NULL.
	                               // application can provide its own HELEMENT here (if it is NULL) or modify current menu element.

	VISIUAL_STATUS_CHANGED = 0x11, // broadcast notification, sent to all elements of some container being shown or hidden
	DISABLED_STATUS_CHANGED = 0x12,// broadcast notification, sent to all elements of some container that got new value of :disabled state

	POPUP_DISMISSING = 0x13,       // popup is about to be closed

	CONTENT_CHANGED = 0x15,        // content has been changed, is posted to the element that gets content changed,  reason is combination of CONTENT_CHANGE_BITS.
	                               // target == NULL means the window got new document and this event is dispatched only to the window.

	CLICK = 0x16,                  // generic click
	CHANGE = 0x17,                 // generic change

	// "grey" event codes  - notfications from behaviors from this SDK
	HYPERLINK_CLICK = 0x80,        // hyperlink click

	//TABLE_HEADER_CLICK,            // click on some cell in table header,
	//                               //     target = the cell,
	//                               //     reason = index of the cell (column number, 0..n)
	//TABLE_ROW_CLICK,               // click on data row in the table, target is the row
	//                               //     target = the row,
	//                               //     reason = index of the row (fixed_rows..n)
	//TABLE_ROW_DBL_CLICK,           // mouse dbl click on data row in the table, target is the row
	//                               //     target = the row,
	//                               //     reason = index of the row (fixed_rows..n)

	ELEMENT_COLLAPSED = 0x90,      // element was collapsed, so far only behavior:tabs is sending these two to the panels
	ELEMENT_EXPANDED,              // element was expanded,

	ACTIVATE_CHILD,                // activate (select) child,
	                               // used for example by accesskeys behaviors to send activation request, e.g. tab on behavior:tabs.

	//DO_SWITCH_TAB = ACTIVATE_CHILD,// command to switch tab programmatically, handled by behavior:tabs
	//                               // use it as SciterPostEvent(tabsElementOrItsChild, DO_SWITCH_TAB, tabElementToShow, 0);

	INIT_DATA_VIEW,                // request to virtual grid to initialize its view

	ROWS_DATA_REQUEST,             // request from virtual grid to data source behavior to fill data in the table
	                               // parameters passed throug DATA_ROWS_PARAMS structure.

	UI_STATE_CHANGED,              // ui state changed, observers shall update their visual states.
	                               // is sent for example by behavior:richtext when caret position/selection has changed.

	FORM_SUBMIT,                   // behavior:form detected submission event. BEHAVIOR_EVENT_PARAMS::data field contains data to be posted.
	                               // BEHAVIOR_EVENT_PARAMS::data is of type T_MAP in this case key/value pairs of data that is about 
	                               // to be submitted. You can modify the data or discard submission by returning true from the handler.
	FORM_RESET,                    // behavior:form detected reset event (from button type=reset). BEHAVIOR_EVENT_PARAMS::data field contains data to be reset.
	                               // BEHAVIOR_EVENT_PARAMS::data is of type T_MAP in this case key/value pairs of data that is about 
	                               // to be rest. You can modify the data or discard reset by returning true from the handler.

	DOCUMENT_COMPLETE,             // document in behavior:frame or root document is complete.

	HISTORY_PUSH,                  // requests to behavior:history (commands)
	HISTORY_DROP,                     
	HISTORY_PRIOR,
	HISTORY_NEXT,
	HISTORY_STATE_CHANGED,         // behavior:history notification - history stack has changed

	CLOSE_POPUP,                   // close popup request,
	REQUEST_TOOLTIP,               // request tooltip, evt.source <- is the tooltip element.

	ANIMATION         = 0xA0,      // animation started (reason=1) or ended(reason=0) on the element.

	DOCUMENT_CREATED  = 0xC0,      // document created, script namespace initialized. target -> the document
	DOCUMENT_CLOSE_REQUEST = 0xC1, // document is about to be closed, to cancel closing do: evt.data = sciter::value("cancel");
	DOCUMENT_CLOSE    = 0xC2,      // last notification before document removal from the DOM
	DOCUMENT_READY    = 0xC3,      // document has got DOM structure, styles and behaviors of DOM elements. Script loading run is complete at this moment. 

	VIDEO_INITIALIZED = 0xD1,      // <video> "ready" notification   
	VIDEO_STARTED     = 0xD2,      // <video> playback started notification   
	VIDEO_STOPPED     = 0xD3,      // <video> playback stoped/paused notification   
	VIDEO_BIND_RQ     = 0xD4,      // <video> request for frame source binding, 
	                               //   If you want to provide your own video frames source for the given target <video> element do the following:
	                               //   1. Handle and consume this VIDEO_BIND_RQ request 
	                               //   2. You will receive second VIDEO_BIND_RQ request/event for the same <video> element
	                               //      but this time with the 'reason' field set to an instance of sciter::video_destination interface.
	                               //   3. add_ref() it and store it for example in worker thread producing video frames.
	                               //   4. call sciter::video_destination::start_streaming(...) providing needed parameters
	                               //      call sciter::video_destination::render_frame(...) as soon as they are available
	                               //      call sciter::video_destination::stop_streaming() to stop the rendering (a.k.a. end of movie reached)

	PAGINATION_STARTS  = 0xE0,     // behavior:pager starts pagination
	PAGINATION_PAGE    = 0xE1,     // behavior:pager paginated page no, reason -> page no
	PAGINATION_ENDS    = 0xE2,     // behavior:pager end pagination, reason -> total pages

	FIRST_APPLICATION_EVENT_CODE = 0x100,
	// all custom event codes shall be greater
	// than this number. All codes below this will be used
	// solely by application - Sciter will not intrepret it
	// and will do just dispatching.
	// To send event notifications with  these codes use
	// SciterSend/PostEvent API.
}


impl ::std::ops::BitOr for EVENT_GROUPS {
  type Output = EVENT_GROUPS;
  fn bitor(self, rhs: Self::Output) -> Self::Output {
    let rn = (self as UINT) | (rhs as UINT);
    unsafe { ::std::mem::transmute(rn) }
  }
}

