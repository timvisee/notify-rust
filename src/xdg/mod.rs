//! This module contains XDG and DBus specific code.
//!
//! it should not be available under any platform other than `(unix, not(target_os = "macos"))`

use dbus::ffidisp::Connection as DbusConnection;

use crate::{error::*, notification::Notification};

use std::ops::{Deref, DerefMut};

mod dbus_rs;

#[cfg(not(feature = "debug_namespace"))]
pub static NOTIFICATION_NAMESPACE: &str = "org.freedesktop.Notifications";
#[cfg(not(feature = "debug_namespace"))]
pub static NOTIFICATION_OBJECTPATH: &str = "/org/freedesktop/Notifications";

#[cfg(feature = "debug_namespace")]
pub static NOTIFICATION_NAMESPACE: &str = "de.hoodie.Notifications";
#[cfg(feature = "debug_namespace")]
pub static NOTIFICATION_OBJECTPATH: &str = "/de/hoodie/Notifications";

/// A handle to a shown notification.
///
/// This keeps a connection alive to ensure actions work on certain desktops.
#[derive(Debug)]
pub struct NotificationHandle {
    inner: dbus_rs::NotificationHandle,
}

impl NotificationHandle {
    pub(crate) fn new(id: u32, connection: DbusConnection, notification: Notification) -> NotificationHandle {
        NotificationHandle {
            inner: dbus_rs::NotificationHandle::new(id, connection, notification),
        }
    }

    /// Waits for the user to act on a notification and then calls
    /// `invocation_closure` with the name of the corresponding action.
    pub fn wait_for_action<F>(self, invocation_closure: F)
    where
        F: FnOnce(&str),
    {
        self.inner.wait_for_action(invocation_closure)
    }

    /// Manually close the notification
    ///
    /// # Example
    /// see
    /// ```no_run
    /// let handle: notify_rust::NotificationHandle = Notification::new()
    ///     .summary("oh no")
    ///     .hint(notify_rust::Hint::Transient(true))
    ///     .body("I'll be here till you close me!")
    ///     .hint(Hint::Resident(true)) // does not work on kde
    ///     .timeout(Timeout::Never) // works on kde and gnome
    ///     .show()
    ///     .unwrap();
    /// // ... and then later
    /// handle.close();
    /// ```
    pub fn close(self) {
        self.inner.close()
    }

    /// Executes a closure after the notification has closed.
    /// ## Example
    /// ```no_run
    /// # use notify_rust::Notification;
    /// Notification::new().summary("Time is running out")
    ///                    .body("This will go away.")
    ///                    .icon("clock")
    ///                    .show()
    ///                    .unwrap()
    ///                    .on_close(|| println!("closed"));
    /// ```
    pub fn on_close<F>(self, closure: F)
    where
        F: FnOnce(),
    {
        self.inner.wait_for_action(|action| {
            if action == "__closed" {
                closure();
            }
        });
    }

    /// Replace the original notification with an updated version
    /// ## Example
    /// ```no_run
    /// # use notify_rust::Notification;
    /// let mut notification = Notification::new().summary("Latest News")
    ///                                           .body("Bayern Dortmund 3:2")
    ///                                           .show()
    ///                                           .unwrap();
    ///
    /// std::thread::sleep_ms(1_500);
    ///
    /// notification.summary("Latest News (Correction)")
    ///             .body("Bayern Dortmund 3:3");
    ///
    /// notification.update();
    /// ```
    /// Watch out for different implementations of the
    /// notification server! On plasma5 for instance, you should also change the appname, so the old
    /// message is really replaced and not just amended. Xfce behaves well, all others have not
    /// been tested by the developer.
    pub fn update(&mut self) {
        self.inner.update()
    }

    /// Returns the Handle's id.
    pub fn id(&self) -> u32 {
        self.inner.id
    }
}

/// Required for `DerefMut`
impl Deref for NotificationHandle {
    type Target = Notification;

    fn deref(&self) -> &Notification {
        &self.inner.notification
    }
}

/// Allow you to easily modify notification properties
impl DerefMut for NotificationHandle {
    fn deref_mut(&mut self) -> &mut Notification {
        &mut self.inner.notification
    }
}

impl From<dbus_rs::NotificationHandle> for NotificationHandle {
    fn from(inner: dbus_rs::NotificationHandle) -> NotificationHandle {
        NotificationHandle { inner }
    }
}

pub(crate) fn show_notification(notification: &Notification) -> Result<NotificationHandle> {
    dbus_rs::connect_and_send_notification(notification).map(Into::into)
}

// here be public functions

/// Get list of all capabilities of the running notification server.
pub fn get_capabilities() -> Result<Vec<String>> {
    dbus_rs::get_capabilities()
}

/// Returns a struct containing `ServerInformation`.
///
/// This struct contains `name`, `vendor`, `version` and `spec_version` of the notification server
/// running.
/// TODO dbus stuff module!!!
pub fn get_server_information() -> Result<ServerInformation> {
    dbus_rs::get_server_information()
}

/// Return value of `get_server_information()`.
#[derive(Debug)]
pub struct ServerInformation {
    /// The product name of the server.
    pub name: String,
    /// The vendor name.
    pub vendor: String,
    /// The server's version string.
    pub version: String,
    /// The specification version the server is compliant with.
    pub spec_version: String,
}

/// Strictly internal.
/// The NotificationServer implemented here exposes a "Stop" function.
/// stops the notification server
#[cfg(all(feature = "server", unix, not(target_os = "macos")))]
#[doc(hidden)]
pub fn stop_server() {
    dbus_rs::stop_server()
}

/// Listens for the `ActionInvoked(UInt32, String)` Signal.
///
/// No need to use this, check out `Notification::show_and_wait_for_action(FnOnce(action:&str))`
pub fn handle_action<F>(id: u32, func: F)
where
    F: FnOnce(&str),
{
    dbus_rs::handle_action(id, func)
}
