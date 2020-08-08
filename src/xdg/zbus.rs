#[cfg(all(unix, not(target_os = "macos")))]
pub(crate) fn _show(notification: &Notification, id: u32, connection: &Connection) -> Result<u32> {
    let connection = zbus::Connection::new_session()?;

    let proxy = NotificationsProxy::new(&connection)?;
    let reply = proxy.notify(
        &self.appname,
        id,
        &self.icon,
        &self.summary,
        &self.body,
        &[],            // actions
        HashMap::new(), // hints
        self.timeout.into(),
    )?;
    dbg!(reply);

    Ok(reply)
}
#[dbus_proxy]
trait Notifications {
    fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: &[&str],
        hints: HashMap<&str, &Value>,
        expire_timeout: i32,
    ) -> zbus::Result<u32>;
}

pub fn connect_and_send_notification(notification: &Notification) -> Result<NotificationHandle> {
    let connection = zbus::Connection::new_session()?;

    let reply: u32 = 
    connection.call_method(
        Some(crate::xdg::NOTIFICATION_NAMESPACE),
        crate::xdg::NOTIFICATION_OBJECTPATH,
        Some(crate::xdg::NOTIFICATION_NAMESPACE),
        "Notify",
        &(
            &self.appname,
            id,
            &self.icon,
            &self.summary,
            &self.body,
            &self.actions,
            crate::hints::hints_to_map(&self.hints),
            self.timeout.into_i32(),
        )

    )?.body().unwrap();
    dbg!(reply)
}