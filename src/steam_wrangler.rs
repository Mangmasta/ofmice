//! Finds the Source SDK 2013 MP
//! and ensures Team Fortress 2 is installed.
use std::path::PathBuf;
use steamworks::{Client, AppId};

/// A user-friendly **actionable** error
#[derive(Debug, Clone, Copy)]
pub enum WranglerError{
    SteamNotRunning,
    SSDKNotInstalled,
    TF2NotInstalled,
}

const TF2_APPID: AppId = AppId(440);
#[allow(clippy::unreadable_literal)]
const SSDK_APPID: AppId = AppId(243750);

/// Checks if TF2 and SSDK 2013 are both installed,
/// then returns the path to the SSDK 2013.
///
/// Future work: Pass the TF2 path to OF?
pub fn wrangle_steam_and_get_ssdk_path() -> Result<PathBuf, WranglerError>{
    // Init SteamWorks, and replace any error with a SteamNotRunning error.
    let client = Client::init().map_err(|_| WranglerError::SteamNotRunning)?;

    // Note: Every steam user 'owns' these, as they're free.
    // Otherwise, we'd check here, and add to struct Error.

    // TF2 must be installed.
    let apps = client.0.apps();
    if !apps.is_app_installed(TF2_APPID){
        return Err(WranglerError::TF2NotInstalled);
    }

    // SSDK must be installed.
    if !apps.is_app_installed(SSDK_APPID){
        return Err(WranglerError::SSDKNotInstalled);
    }

    // Return the SSDK install path
    Ok(apps.app_install_dir(SSDK_APPID).into())
}

//TODO: Automatically installing TF2 and SSDK might be a good idea.
