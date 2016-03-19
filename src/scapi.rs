//! Sciter C API interface.

#![allow(non_snake_case, non_camel_case_types)]

use sctypes::*;
use scdef::*;
use scdom::*;
use scvalue::*;
use sctiscript::{HVM, tiscript_value, tiscript_native_interface};
use scbehavior::*;
use scgraphics::{SciterGraphicsAPI};
use screquest::{SciterRequestAPI};
use utf::*;

/// Sciter API functions.
#[repr(C)]
pub struct ISciterAPI
{
	pub version: UINT,

	pub SciterClassName: extern "stdcall" fn () -> LPCWSTR,
	pub SciterVersion: extern "stdcall" fn (major: bool) -> UINT,
	pub SciterDataReady: extern "stdcall" fn (hwnd: HWINDOW, uri: LPCWSTR, data: LPCBYTE, dataLength: UINT) -> BOOL,
	pub SciterDataReadyAsync: extern "stdcall" fn (hwnd: HWINDOW, uri: LPCWSTR, data: LPCBYTE, dataLength: UINT, requestId: LPVOID) -> BOOL,

  // #ifdef WINDOWS
  #[cfg(windows)]
	pub SciterProc: extern "stdcall" fn (hwnd: HWINDOW, msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT,
	#[cfg(windows)]
	pub SciterProcND: extern "stdcall" fn (hwnd: HWINDOW, msg: UINT, wParam: WPARAM, lParam: LPARAM, pbHandled: * mut BOOL) -> LRESULT,
  // #endif

	pub SciterLoadFile: extern "stdcall" fn (hWndSciter: HWINDOW, filename: LPCWSTR) -> BOOL,

	pub SciterLoadHtml: extern "stdcall" fn (hWndSciter: HWINDOW, html: LPCBYTE, htmlSize: UINT, baseUrl: LPCWSTR) -> BOOL,
	pub SciterSetCallback: extern "stdcall" fn (hWndSciter: HWINDOW, cb: SciterHostCallback, cbParam: LPVOID) -> VOID,
	pub SciterSetMasterCSS: extern "stdcall" fn (utf8: LPCBYTE, numBytes: UINT) -> BOOL,
	pub SciterAppendMasterCSS: extern "stdcall" fn (utf8: LPCBYTE, numBytes: UINT) -> BOOL,
	pub SciterSetCSS: extern "stdcall" fn (hWndSciter: HWINDOW, utf8: LPCBYTE, numBytes: UINT, baseUrl: LPCWSTR, mediaType: LPCWSTR) -> BOOL,
	pub SciterSetMediaType: extern "stdcall" fn (hWndSciter: HWINDOW, mediaType: LPCWSTR) -> BOOL,
	pub SciterSetMediaVars: extern "stdcall" fn (hWndSciter: HWINDOW, mediaVars: * const VALUE) -> BOOL,
	pub SciterGetMinWidth: extern "stdcall" fn (hWndSciter: HWINDOW) -> UINT,
	pub SciterGetMinHeight: extern "stdcall" fn (hWndSciter: HWINDOW, width: UINT) -> UINT,
	pub SciterCall: extern "stdcall" fn (hWnd: HWINDOW, functionName: LPCSTR, argc: UINT, argv: * const VALUE, retval: * mut VALUE) -> BOOL,
	pub SciterEval: extern "stdcall" fn (hwnd: HWINDOW, script: LPCWSTR, scriptLength: UINT, pretval: * mut VALUE) -> BOOL,
	pub SciterUpdateWindow: extern "stdcall" fn (hwnd: HWINDOW) -> VOID,

  // #ifdef WINDOWS
  #[cfg(windows)]
	pub SciterTranslateMessage: extern "stdcall" fn (lpMsg: * mut MSG) -> BOOL,
  // #endif

	pub SciterSetOption: extern "stdcall" fn (hWnd: HWINDOW, option: SCITER_RT_OPTIONS, value: UINT_PTR) -> BOOL,
	pub SciterGetPPI: extern "stdcall" fn (hWndSciter: HWINDOW, px: * mut UINT, py: * mut UINT) -> VOID,
	pub SciterGetViewExpando: extern "stdcall" fn (hwnd: HWINDOW, pval: * mut VALUE) -> BOOL,

  // #ifdef WINDOWS
  #[cfg(windows)]
	pub SciterRenderD2D: extern "stdcall" fn (hWndSciter: HWINDOW, prt: * mut ID2D1RenderTarget) -> BOOL,
	#[cfg(windows)]
	pub SciterD2DFactory: extern "stdcall" fn (ppf: * mut* mut ID2D1Factory) -> BOOL,
	#[cfg(windows)]
	pub SciterDWFactory: extern "stdcall" fn (ppf: * mut* mut IDWriteFactory) -> BOOL,
  // #endif

	pub SciterGraphicsCaps: extern "stdcall" fn (pcaps: LPUINT) -> BOOL,
	pub SciterSetHomeURL: extern "stdcall" fn (hWndSciter: HWINDOW, baseUrl: LPCWSTR) -> BOOL,

  // #if defined(OSX)
  #[cfg(osx)]
	pub SciterCreateNSView: extern "stdcall" fn (frame: LPRECT) -> HWINDOW, // returns NSView*
  // #endif

  // #if defined(LINUX)
  #[cfg(linux)]
	pub SciterCreateWidget: extern "stdcall" fn (frame: LPRECT) -> HWINDOW, // returns GtkWidget
  // #endif

	pub SciterCreateWindow: extern "stdcall" fn (creationFlags: UINT, frame: LPCRECT, delegate: * const SciterWindowDelegate, delegateParam: LPVOID, parent: HWINDOW) -> HWINDOW,
	pub SciterSetupDebugOutput: extern "stdcall" fn (hwndOrNull: HWINDOW, param: LPVOID, pfOutput: DEBUG_OUTPUT_PROC),

	//|
	//| DOM Element API
	//|
	pub Sciter_UseElement: extern "stdcall" fn (he: HELEMENT) -> SCDOM_RESULT,
	pub Sciter_UnuseElement: extern "stdcall" fn (he: HELEMENT) -> SCDOM_RESULT,
	pub SciterGetRootElement: extern "stdcall" fn (hwnd: HWINDOW, phe: * mut HELEMENT) -> SCDOM_RESULT,
	pub SciterGetFocusElement: extern "stdcall" fn (hwnd: HWINDOW, phe: * mut HELEMENT) -> SCDOM_RESULT,
	pub SciterFindElement: extern "stdcall" fn (hwnd: HWINDOW, pt: POINT, phe: * mut HELEMENT) -> SCDOM_RESULT,
	pub SciterGetChildrenCount: extern "stdcall" fn (he: HELEMENT, count: * mut UINT) -> SCDOM_RESULT,
	pub SciterGetNthChild: extern "stdcall" fn (he: HELEMENT, n: UINT, phe: * mut HELEMENT) -> SCDOM_RESULT,
	pub SciterGetParentElement: extern "stdcall" fn (he: HELEMENT, p_parent_he: * mut HELEMENT) -> SCDOM_RESULT,
	pub SciterGetElementHtmlCB: extern "stdcall" fn (he: HELEMENT, outer: BOOL, rcv: * mut LPCBYTE_RECEIVER, rcv_param: LPVOID) -> SCDOM_RESULT,
	pub SciterGetElementTextCB: extern "stdcall" fn (he: HELEMENT, rcv: * mut LPCWSTR_RECEIVER, rcv_param: LPVOID) -> SCDOM_RESULT,
	pub SciterSetElementText: extern "stdcall" fn (he: HELEMENT, utf16: LPCWSTR, length: UINT) -> SCDOM_RESULT,
	pub SciterGetAttributeCount: extern "stdcall" fn (he: HELEMENT, p_count: LPUINT) -> SCDOM_RESULT,
	pub SciterGetNthAttributeNameCB: extern "stdcall" fn (he: HELEMENT, n: UINT, rcv: * mut LPCSTR_RECEIVER, rcv_param: LPVOID) -> SCDOM_RESULT,
	pub SciterGetNthAttributeValueCB: extern "stdcall" fn (he: HELEMENT, n: UINT, rcv: * mut LPCWSTR_RECEIVER, rcv_param: LPVOID) -> SCDOM_RESULT,
	pub SciterGetAttributeByNameCB: extern "stdcall" fn (he: HELEMENT, name: LPCSTR, rcv: * mut LPCWSTR_RECEIVER, rcv_param: LPVOID) -> SCDOM_RESULT,
	pub SciterSetAttributeByName: extern "stdcall" fn (he: HELEMENT, name: LPCSTR, value: LPCWSTR) -> SCDOM_RESULT,
	pub SciterClearAttributes: extern "stdcall" fn (he: HELEMENT) -> SCDOM_RESULT,
	pub SciterGetElementIndex: extern "stdcall" fn (he: HELEMENT, p_index: LPUINT) -> SCDOM_RESULT,
	pub SciterGetElementType: extern "stdcall" fn (he: HELEMENT, p_type: * mut LPCSTR) -> SCDOM_RESULT,
	pub SciterGetElementTypeCB: extern "stdcall" fn (he: HELEMENT, rcv: * mut LPCSTR_RECEIVER, rcv_param: LPVOID) -> SCDOM_RESULT,
	pub SciterGetStyleAttributeCB: extern "stdcall" fn (he: HELEMENT, name: LPCSTR, rcv: * mut LPCWSTR_RECEIVER, rcv_param: LPVOID) -> SCDOM_RESULT,
	pub SciterSetStyleAttribute: extern "stdcall" fn (he: HELEMENT, name: LPCSTR, value: LPCWSTR) -> SCDOM_RESULT,
	pub SciterGetElementLocation: extern "stdcall" fn (he: HELEMENT, p_location: LPRECT, areas: UINT /*ELEMENT_AREAS*/) -> SCDOM_RESULT,
	pub SciterScrollToView: extern "stdcall" fn (he: HELEMENT, SciterScrollFlags: UINT) -> SCDOM_RESULT,
	pub SciterUpdateElement: extern "stdcall" fn (he: HELEMENT, andForceRender: BOOL) -> SCDOM_RESULT,
	pub SciterRefreshElementArea: extern "stdcall" fn (he: HELEMENT, rc: RECT) -> SCDOM_RESULT,
	pub SciterSetCapture: extern "stdcall" fn (he: HELEMENT) -> SCDOM_RESULT,
	pub SciterReleaseCapture: extern "stdcall" fn (he: HELEMENT) -> SCDOM_RESULT,
	pub SciterGetElementHwnd: extern "stdcall" fn (he: HELEMENT, p_hwnd: * mut HWINDOW, rootWindow: BOOL) -> SCDOM_RESULT,
	pub SciterCombineURL: extern "stdcall" fn (he: HELEMENT, szUrlBuffer: LPWSTR, UrlBufferSize: UINT) -> SCDOM_RESULT,
	pub SciterSelectElements: extern "stdcall" fn (he: HELEMENT, CSS_selectors: LPCSTR, callback: * mut SciterElementCallback, param: LPVOID) -> SCDOM_RESULT,
	pub SciterSelectElementsW: extern "stdcall" fn (he: HELEMENT, CSS_selectors: LPCWSTR, callback: * mut SciterElementCallback, param: LPVOID) -> SCDOM_RESULT,
	pub SciterSelectParent: extern "stdcall" fn (he: HELEMENT, selector: LPCSTR, depth: UINT, heFound: * mut HELEMENT) -> SCDOM_RESULT,
	pub SciterSelectParentW: extern "stdcall" fn (he: HELEMENT, selector: LPCWSTR, depth: UINT, heFound: * mut HELEMENT) -> SCDOM_RESULT,
	pub SciterSetElementHtml: extern "stdcall" fn (he: HELEMENT, html: * const BYTE, htmlLength: UINT, how: UINT) -> SCDOM_RESULT,
	pub SciterGetElementUID: extern "stdcall" fn (he: HELEMENT, puid: * mut UINT) -> SCDOM_RESULT,
	pub SciterGetElementByUID: extern "stdcall" fn (hwnd: HWINDOW, uid: UINT, phe: * mut HELEMENT) -> SCDOM_RESULT,
	pub SciterShowPopup: extern "stdcall" fn (hePopup: HELEMENT, heAnchor: HELEMENT, placement: UINT) -> SCDOM_RESULT,
	pub SciterShowPopupAt: extern "stdcall" fn (hePopup: HELEMENT, pos: POINT, animate: BOOL) -> SCDOM_RESULT,
	pub SciterHidePopup: extern "stdcall" fn (he: HELEMENT) -> SCDOM_RESULT,
	pub SciterGetElementState: extern "stdcall" fn (he: HELEMENT, pstateBits: * mut UINT) -> SCDOM_RESULT,
	pub SciterSetElementState: extern "stdcall" fn (he: HELEMENT, stateBitsToSet: UINT, stateBitsToClear: UINT, updateView: BOOL) -> SCDOM_RESULT,
	pub SciterCreateElement: extern "stdcall" fn (tagname: LPCSTR, textOrNull: LPCWSTR, /*out*/ phe: * mut HELEMENT) -> SCDOM_RESULT,
	pub SciterCloneElement: extern "stdcall" fn (he: HELEMENT, /*out*/ phe: * mut HELEMENT) -> SCDOM_RESULT,
	pub SciterInsertElement: extern "stdcall" fn (he: HELEMENT, hparent: HELEMENT, index: UINT) -> SCDOM_RESULT,
	pub SciterDetachElement: extern "stdcall" fn (he: HELEMENT) -> SCDOM_RESULT,
	pub SciterDeleteElement: extern "stdcall" fn (he: HELEMENT) -> SCDOM_RESULT,
	pub SciterSetTimer: extern "stdcall" fn (he: HELEMENT, milliseconds: UINT, timer_id: UINT_PTR) -> SCDOM_RESULT,
	pub SciterDetachEventHandler: extern "stdcall" fn (he: HELEMENT, pep: ElementEventProc, tag: LPVOID) -> SCDOM_RESULT,
	pub SciterAttachEventHandler: extern "stdcall" fn (he: HELEMENT, pep: ElementEventProc, tag: LPVOID) -> SCDOM_RESULT,
	pub SciterWindowAttachEventHandler: extern "stdcall" fn (hwndLayout: HWINDOW, pep: ElementEventProc, tag: LPVOID, subscription: UINT) -> SCDOM_RESULT,
	pub SciterWindowDetachEventHandler: extern "stdcall" fn (hwndLayout: HWINDOW, pep: ElementEventProc, tag: LPVOID) -> SCDOM_RESULT,
	pub SciterSendEvent: extern "stdcall" fn (he: HELEMENT, appEventCode: UINT, heSource: HELEMENT, reason: UINT_PTR, /*out*/ handled: * mut BOOL) -> SCDOM_RESULT,
	pub SciterPostEvent: extern "stdcall" fn (he: HELEMENT, appEventCode: UINT, heSource: HELEMENT, reason: UINT_PTR) -> SCDOM_RESULT,
	pub SciterCallBehaviorMethod: extern "stdcall" fn (he: HELEMENT, params: * const METHOD_PARAMS) -> SCDOM_RESULT,
	pub SciterRequestElementData: extern "stdcall" fn (he: HELEMENT, url: LPCWSTR, dataType: UINT, initiator: HELEMENT) -> SCDOM_RESULT,
	pub SciterHttpRequest: extern "stdcall" fn (he: HELEMENT, url: LPCWSTR, dataType: UINT, requestType: UINT, requestParams: * const REQUEST_PARAM, nParams: UINT),
	pub SciterGetScrollInfo: extern "stdcall" fn (he: HELEMENT, scrollPos: LPPOINT, viewRect: LPRECT, contentSize: LPSIZE) -> SCDOM_RESULT,
	pub SciterSetScrollPos: extern "stdcall" fn (he: HELEMENT, scrollPos: POINT, smooth: BOOL) -> SCDOM_RESULT,
	pub SciterGetElementIntrinsicWidths: extern "stdcall" fn (he: HELEMENT, pMinWidth: * mut INT, pMaxWidth: * mut INT) -> SCDOM_RESULT,
	pub SciterGetElementIntrinsicHeight: extern "stdcall" fn (he: HELEMENT, forWidth: INT, pHeight: * mut INT) -> SCDOM_RESULT,
	pub SciterIsElementVisible: extern "stdcall" fn (he: HELEMENT, pVisible: * mut BOOL) -> SCDOM_RESULT,
	pub SciterIsElementEnabled: extern "stdcall" fn (he: HELEMENT, pEnabled: * mut BOOL) -> SCDOM_RESULT,
	pub SciterSortElements: extern "stdcall" fn (he: HELEMENT, firstIndex: UINT, lastIndex: UINT, cmpFunc: * mut ELEMENT_COMPARATOR, cmpFuncParam: LPVOID) -> SCDOM_RESULT,
	pub SciterSwapElements: extern "stdcall" fn (he1: HELEMENT, he2: HELEMENT) -> SCDOM_RESULT,
	pub SciterTraverseUIEvent: extern "stdcall" fn (evt: UINT, eventCtlStruct: LPVOID, bOutProcessed: * mut BOOL) -> SCDOM_RESULT,
	pub SciterCallScriptingMethod: extern "stdcall" fn (he: HELEMENT, name: LPCSTR, argv: * const VALUE, argc: UINT, retval: * mut VALUE) -> SCDOM_RESULT,
	pub SciterCallScriptingFunction: extern "stdcall" fn (he: HELEMENT, name: LPCSTR, argv: * const VALUE, argc: UINT, retval: * mut VALUE) -> SCDOM_RESULT,
	pub SciterEvalElementScript: extern "stdcall" fn (he: HELEMENT, script: LPCWSTR, scriptLength: UINT, retval: * mut VALUE) -> SCDOM_RESULT,
	pub SciterAttachHwndToElement: extern "stdcall" fn (he: HELEMENT, hwnd: HWINDOW) -> SCDOM_RESULT,
	pub SciterControlGetType: extern "stdcall" fn (he: HELEMENT, /*CTL_TYPE*/ pType: * mut UINT) -> SCDOM_RESULT,
	pub SciterGetValue: extern "stdcall" fn (he: HELEMENT, pval: * mut VALUE) -> SCDOM_RESULT,
	pub SciterSetValue: extern "stdcall" fn (he: HELEMENT, pval: * const VALUE) -> SCDOM_RESULT,
	pub SciterGetExpando: extern "stdcall" fn (he: HELEMENT, pval: * mut VALUE, forceCreation: BOOL) -> SCDOM_RESULT,
	pub SciterGetObject: extern "stdcall" fn (he: HELEMENT, pval: * mut tiscript_value, forceCreation: BOOL) -> SCDOM_RESULT,
	pub SciterGetElementNamespace: extern "stdcall" fn (he: HELEMENT, pval: * mut tiscript_value) -> SCDOM_RESULT,
	pub SciterGetHighlightedElement: extern "stdcall" fn (hwnd: HWINDOW, phe: * mut HELEMENT) -> SCDOM_RESULT,
	pub SciterSetHighlightedElement: extern "stdcall" fn (hwnd: HWINDOW, he: HELEMENT) -> SCDOM_RESULT,
	//|
	//| DOM Node API
	//|
	pub SciterNodeAddRef: extern "stdcall" fn (hn: HNODE) -> SCDOM_RESULT,
	pub SciterNodeRelease: extern "stdcall" fn (hn: HNODE) -> SCDOM_RESULT,
	pub SciterNodeCastFromElement: extern "stdcall" fn (he: HELEMENT, phn: * mut HNODE) -> SCDOM_RESULT,
	pub SciterNodeCastToElement: extern "stdcall" fn (hn: HNODE, he: * mut HELEMENT) -> SCDOM_RESULT,
	pub SciterNodeFirstChild: extern "stdcall" fn (hn: HNODE, phn: * mut HNODE) -> SCDOM_RESULT,
	pub SciterNodeLastChild: extern "stdcall" fn (hn: HNODE, phn: * mut HNODE) -> SCDOM_RESULT,
	pub SciterNodeNextSibling: extern "stdcall" fn (hn: HNODE, phn: * mut HNODE) -> SCDOM_RESULT,
	pub SciterNodePrevSibling: extern "stdcall" fn (hn: HNODE, phn: * mut HNODE) -> SCDOM_RESULT,
	pub SciterNodeParent: extern "stdcall" fn (hnode: HNODE, pheParent: * mut HELEMENT) -> SCDOM_RESULT,
	pub SciterNodeNthChild: extern "stdcall" fn (hnode: HNODE, n: UINT, phn: * mut HNODE) -> SCDOM_RESULT,
	pub SciterNodeChildrenCount: extern "stdcall" fn (hnode: HNODE, pn: * mut UINT) -> SCDOM_RESULT,
	pub SciterNodeType: extern "stdcall" fn (hnode: HNODE, pNodeType: * mut UINT /*NODE_TYPE*/) -> SCDOM_RESULT,
	pub SciterNodeGetText: extern "stdcall" fn (hnode: HNODE, rcv: * mut LPCWSTR_RECEIVER, rcv_param: LPVOID) -> SCDOM_RESULT,
	pub SciterNodeSetText: extern "stdcall" fn (hnode: HNODE, text: LPCWSTR, textLength: UINT) -> SCDOM_RESULT,
	pub SciterNodeInsert: extern "stdcall" fn (hnode: HNODE, how: UINT /*NODE_INS_TARGET*/, what: HNODE) -> SCDOM_RESULT,
	pub SciterNodeRemove: extern "stdcall" fn (hnode: HNODE, finalize: BOOL) -> SCDOM_RESULT,
	pub SciterCreateTextNode: extern "stdcall" fn (text: LPCWSTR, textLength: UINT, phnode: * mut HNODE) -> SCDOM_RESULT,
	pub SciterCreateCommentNode: extern "stdcall" fn (text: LPCWSTR, textLength: UINT, phnode: * mut HNODE) -> SCDOM_RESULT,
	//|
	//| Value API
	//|
	pub ValueInit: extern "stdcall" fn (pval: * mut VALUE) -> VALUE_RESULT,
	pub ValueClear: extern "stdcall" fn (pval: * mut VALUE) -> VALUE_RESULT,
	pub ValueCompare: extern "stdcall" fn (pval1: * const VALUE, pval2: * const VALUE) -> VALUE_RESULT,
	pub ValueCopy: extern "stdcall" fn (pdst: * mut VALUE, psrc: * const VALUE) -> VALUE_RESULT,
	pub ValueIsolate: extern "stdcall" fn (pdst: * mut VALUE) -> VALUE_RESULT,
	pub ValueType: extern "stdcall" fn (pval: * const VALUE, pType: * mut UINT, pUnits: * mut UINT) -> VALUE_RESULT,
	pub ValueStringData: extern "stdcall" fn (pval: * const VALUE, pChars: * mut LPCWSTR, pNumChars: * mut UINT) -> VALUE_RESULT,
	pub ValueStringDataSet: extern "stdcall" fn (pval: * mut VALUE, chars: LPCWSTR, numChars: UINT, units: UINT) -> VALUE_RESULT,
	pub ValueIntData: extern "stdcall" fn (pval: * const VALUE, pData: * mut INT) -> VALUE_RESULT,
	pub ValueIntDataSet: extern "stdcall" fn (pval: * mut VALUE, data: INT, vtype: UINT, units: UINT) -> VALUE_RESULT,
	pub ValueInt64Data: extern "stdcall" fn (pval: * const VALUE, pData: * mut INT64) -> VALUE_RESULT,
	pub ValueInt64DataSet: extern "stdcall" fn (pval: * mut VALUE, data: INT64, vtype: UINT, units: UINT) -> VALUE_RESULT,
	pub ValueFloatData: extern "stdcall" fn (pval: * const VALUE, pData: * mut FLOAT_VALUE) -> VALUE_RESULT,
	pub ValueFloatDataSet: extern "stdcall" fn (pval: * mut VALUE, data: FLOAT_VALUE, vtype: UINT, units: UINT) -> VALUE_RESULT,
	pub ValueBinaryData: extern "stdcall" fn (pval: * const VALUE, pBytes: * mut LPCBYTE, pnBytes: * mut UINT) -> VALUE_RESULT,
	pub ValueBinaryDataSet: extern "stdcall" fn (pval: * mut VALUE, pBytes: LPCBYTE, nBytes: UINT, vtype: UINT, units: UINT) -> VALUE_RESULT,
	pub ValueElementsCount: extern "stdcall" fn (pval: * const VALUE, pn: * mut INT) -> VALUE_RESULT,
	pub ValueNthElementValue: extern "stdcall" fn (pval: * const VALUE, n: INT, pretval: * mut VALUE) -> VALUE_RESULT,
	pub ValueNthElementValueSet: extern "stdcall" fn (pval: * mut VALUE, n: INT, pval_to_set: * const VALUE) -> VALUE_RESULT,
	pub ValueNthElementKey: extern "stdcall" fn (pval: * const VALUE, n: INT, pretval: * mut VALUE) -> VALUE_RESULT,
	pub ValueEnumElements: extern "stdcall" fn (pval: * mut VALUE, penum: * mut KeyValueCallback, param: LPVOID) -> VALUE_RESULT,
	pub ValueSetValueToKey: extern "stdcall" fn (pval: * mut VALUE, pkey: * const VALUE, pval_to_set: * const VALUE) -> VALUE_RESULT,
	pub ValueGetValueOfKey: extern "stdcall" fn (pval: * const VALUE, pkey: * const VALUE, pretval: * mut VALUE) -> VALUE_RESULT,
	pub ValueToString: extern "stdcall" fn (pval: * mut VALUE, how: VALUE_STRING_CVT_TYPE) -> VALUE_RESULT,
	pub ValueFromString: extern "stdcall" fn (pval: * mut VALUE, str: LPCWSTR, strLength: UINT, how: VALUE_STRING_CVT_TYPE) -> UINT,
	pub ValueInvoke: extern "stdcall" fn (pval: * mut VALUE, pthis: * mut VALUE, argc: UINT, argv: * const VALUE, pretval: * mut VALUE, url: LPCWSTR) -> VALUE_RESULT,
	pub ValueNativeFunctorSet: extern "stdcall" fn (pval: * mut VALUE, pinvoke: * mut NATIVE_FUNCTOR_INVOKE, prelease: * mut NATIVE_FUNCTOR_RELEASE, tag: * mut VOID) -> VALUE_RESULT,
	pub ValueIsNativeFunctor: extern "stdcall" fn (pval: * const VALUE) -> BOOL,

	  // tiscript VM API
	pub TIScriptAPI: extern "stdcall" fn () -> * mut tiscript_native_interface,

	pub SciterGetVM: extern "stdcall" fn (hwnd: HWINDOW) -> HVM,

	pub Sciter_v2V: extern "stdcall" fn (vm: HVM, script_value: tiscript_value, value: * mut VALUE, isolate: BOOL) -> BOOL,
	pub Sciter_V2v: extern "stdcall" fn (vm: HVM, valuev: * const VALUE, script_value: * mut tiscript_value) -> BOOL,

	pub SciterOpenArchive: extern "stdcall" fn (archiveData: LPCBYTE, archiveDataLength: UINT) -> HSARCHIVE,
	pub SciterGetArchiveItem: extern "stdcall" fn (harc: HSARCHIVE, path: LPCWSTR, pdata: * mut LPCBYTE, pdataLength: * mut UINT) -> BOOL,
	pub SciterCloseArchive: extern "stdcall" fn (harc: HSARCHIVE) -> BOOL,

	pub SciterFireEvent: extern "stdcall" fn (evt: * const BEHAVIOR_EVENT_PARAMS, post: BOOL, handled: * mut BOOL) -> SCDOM_RESULT,

	pub SciterGetCallbackParam: extern "stdcall" fn (hwnd: HWINDOW) -> LPVOID,
	pub SciterPostCallback: extern "stdcall" fn (hwnd: HWINDOW, wparam: UINT_PTR, lparam: UINT_PTR, timeoutms: UINT) -> UINT_PTR,

	pub GetSciterGraphicsAPI: extern "stdcall" fn () -> * const SciterGraphicsAPI,
	pub GetSciterRequestAPI: extern "stdcall" fn () -> * const SciterRequestAPI,

  // #ifdef WINDOWS
  #[cfg(windows)]
	pub SciterCreateOnDirectXWindow: extern "stdcall" fn (hwnd: HWINDOW, pSwapChain: * mut IDXGISwapChain) -> BOOL,
	#[cfg(windows)]
	pub SciterRenderOnDirectXWindow: extern "stdcall" fn (hwnd: HWINDOW, elementToRenderOrNull: HELEMENT, frontLayer: BOOL) -> BOOL,
	#[cfg(windows)]
	pub SciterRenderOnDirectXTexture: extern "stdcall" fn (hwnd: HWINDOW, elementToRenderOrNull: HELEMENT, surface: * mut IDXGISurface) -> BOOL,
  // #endif
}

impl ISciterAPI {
	pub fn SciterClassName(&self) -> String {
		return w2s((self.SciterClassName)());
	}
}
