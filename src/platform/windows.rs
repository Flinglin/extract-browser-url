use crate::error::BrowserError;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use uiautomation::UIAutomation;
use uiautomation::controls::ControlType;
use uiautomation::types::UIProperty;
use windows::Win32::Foundation::{HWND, LPARAM};
use windows::Win32::System::ProcessStatus::K32GetModuleBaseNameW;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetForegroundWindow, GetWindowThreadProcessId, IsIconic, IsWindowVisible,
};
use windows::core::BOOL;

pub(crate) fn get_foreground_window() -> Result<HWND, BrowserError> {
    unsafe {
        let hwnd: HWND = GetForegroundWindow();
        if hwnd.0 == std::ptr::null_mut() {
            return Err(BrowserError::FailedFindBrowser);
        }
        Ok(hwnd)
    }
}

pub(crate) fn get_visible_windows() -> Result<Vec<HWND>, BrowserError> {
    let mut hwnds = Vec::new();
    unsafe {
        if let Err(_) = EnumWindows(
            Some(enum_windows_callback),
            LPARAM(&mut hwnds as *mut Vec<HWND> as isize),
        ) {
            return Err(BrowserError::FailedEnumWindow);
        }
    }
    Ok(hwnds)
}
unsafe extern "system" fn enum_windows_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    unsafe {
        let hwnds = &mut *(lparam.0 as *mut Vec<HWND>);
        if IsWindowVisible(hwnd).as_bool() && !IsIconic(hwnd).as_bool() {
            hwnds.push(hwnd);
        }
        BOOL::from(true)
    }
}

pub enum BrowserType {
    Firefox,
    Chrome,
    Edge,
}

pub(crate) fn classify_browser(hwnd: HWND) -> Result<(BrowserType, u32), BrowserError> {
    unsafe {
        let mut pid = 0u32;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));
        let process_handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid)
            .map_err(|_| BrowserError::FailedFindBrowser)?;
        let mut name_buf = vec![0u16; 256];
        let len = K32GetModuleBaseNameW(process_handle, None, &mut name_buf);
        let name = OsString::from_wide(&name_buf[0..len as usize])
            .to_string_lossy()
            .into_owned();
        if name == "msedge.exe" {
            Ok((BrowserType::Edge, pid))
        } else if name == "chrome.exe" {
            Ok((BrowserType::Chrome, pid))
        } else if name == "firefox.exe" {
            Ok((BrowserType::Firefox, pid))
        } else {
            Err(BrowserError::FailedFindBrowser)
        }
    }
}

pub(crate) fn get_browser_window_info(
    only_foreground: bool,
) -> Result<(BrowserType, u32), BrowserError> {
    let visible_windows = if only_foreground {
        vec![get_foreground_window()?]
    } else {
        get_visible_windows()?
    };
    for hwnd in visible_windows {
        return classify_browser(hwnd);
    }
    Err(BrowserError::FailedFindBrowser)
}


///
/// extract the browser active tab url
/// - timeout: Sets the time in millionseconds for matching url element. Recommended setting: 3000 milliseconds.
/// - only_foreground: Search only for browser windows that are in the foreground. Otherwise, search for the browser window that was last used and is currently visible.
/// ## Example:
/// ```rust
/// use extract_browser_url::extract_url;
/// fn main(){
///     let url=extract_url(3000,false).unwrap();
///     println!("the url is: {}",url)
/// }
/// ```
pub fn extract_url(timeout: u64, only_foreground: bool) -> Result<String, BrowserError> {
    let (browser_type, pid) = get_browser_window_info(only_foreground)?;

    if let Ok(automation) = UIAutomation::new()
        && let Ok(root) = automation.get_root_element()
    {
        return match browser_type {
            BrowserType::Firefox => {
                if let Ok(browser) = automation
                    .create_matcher()
                    .from(root)
                    .timeout(timeout)
                    .process_id(pid)
                    .find_first()
                    && let Ok(ele) = automation
                        .create_matcher()
                        .from(browser)
                        .timeout(timeout)
                        .control_type(ControlType::Edit)
                        .find_first()
                {
                    let url_variant = ele
                        .get_property_value(UIProperty::ValueValue)
                        .unwrap_or_default();
                    let url = url_variant.get_string().unwrap_or_default();
                    if url == "" {
                        return Err(BrowserError::FailedExtractUrl);
                    }
                    return Ok(url);
                }
                Err(BrowserError::FailedFindUrlUI)
            }
            BrowserType::Chrome => {
                if let Ok(browser) = automation
                    .create_matcher()
                    .from(root)
                    .timeout(timeout)
                    .process_id(pid)
                    .find_first()
                    && let Ok(toolbar) = automation
                        .create_matcher()
                        .from(browser)
                        .timeout(timeout)
                        .classname("ToolbarView")
                        .find_first()
                    && let Ok(address_bar) = automation
                        .create_matcher()
                        .from(toolbar)
                        .timeout(timeout)
                        .classname("LocationBarView")
                        .find_first()
                    && let Ok(ele) = automation
                        .create_matcher()
                        .from(address_bar)
                        .timeout(timeout)
                        .control_type(ControlType::Edit)
                        .find_first()
                {
                    let url_variant = ele
                        .get_property_value(UIProperty::ValueValue)
                        .unwrap_or_default();
                    let url = url_variant.get_string().unwrap_or_default();
                    if url == "" {
                        return Err(BrowserError::FailedExtractUrl);
                    }
                    return Ok(url);
                }
                Err(BrowserError::FailedFindUrlUI)
            }
            BrowserType::Edge => {
                if let Ok(browser) = automation
                    .create_matcher()
                    .from(root)
                    .timeout(timeout)
                    .process_id(pid)
                    .find_first()
                    && let Ok(toolbar) = automation
                        .create_matcher()
                        .from(browser)
                        .timeout(timeout)
                        .classname("EdgeToolbarView")
                        .find_first()
                    && let Ok(address_bar) = automation
                        .create_matcher()
                        .from(toolbar)
                        .timeout(timeout)
                        .classname("LocationBarView")
                        .find_first()
                    && let Ok(ele) = automation
                        .create_matcher()
                        .from(address_bar)
                        .timeout(timeout)
                        .control_type(ControlType::Edit)
                        .find_first()
                {
                    let url_variant = ele
                        .get_property_value(UIProperty::ValueValue)
                        .unwrap_or_default();
                    let url = url_variant.get_string().unwrap_or_default();
                    if url == "" {
                        return Err(BrowserError::FailedExtractUrl);
                    }
                    return Ok(url);
                }
                Err(BrowserError::FailedFindUrlUI)
            }
        };
    }
    Err(BrowserError::FailedFindUrlUI)
}
