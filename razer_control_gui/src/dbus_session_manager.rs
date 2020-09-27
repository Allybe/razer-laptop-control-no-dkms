// This code was autogenerated with `dbus-codegen-rust -d org.gnome.SessionManager -p /org/gnome/SessionManager -m None`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
use dbus::arg;
use dbus::blocking;

pub trait OrgFreedesktopDBusProperties {
    fn get(&self, interface_name: &str, property_name: &str) -> Result<arg::Variant<Box<dyn arg::RefArg + 'static>>, dbus::Error>;
    fn get_all(&self, interface_name: &str) -> Result<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, dbus::Error>;
    fn set(&self, interface_name: &str, property_name: &str, value: arg::Variant<Box<dyn arg::RefArg>>) -> Result<(), dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target=blocking::Connection>> OrgFreedesktopDBusProperties for blocking::Proxy<'a, C> {

    fn get(&self, interface_name: &str, property_name: &str) -> Result<arg::Variant<Box<dyn arg::RefArg + 'static>>, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "Get", (interface_name, property_name, ))
            .and_then(|r: (arg::Variant<Box<dyn arg::RefArg + 'static>>, )| Ok(r.0, ))
    }

    fn get_all(&self, interface_name: &str) -> Result<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "GetAll", (interface_name, ))
            .and_then(|r: (::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, )| Ok(r.0, ))
    }

    fn set(&self, interface_name: &str, property_name: &str, value: arg::Variant<Box<dyn arg::RefArg>>) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "Set", (interface_name, property_name, value, ))
    }
}

#[derive(Debug)]
pub struct OrgFreedesktopDBusPropertiesPropertiesChanged {
    pub interface_name: String,
    pub changed_properties: ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
    pub invalidated_properties: Vec<String>,
}

impl arg::AppendAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.interface_name, i);
        arg::RefArg::append(&self.changed_properties, i);
        arg::RefArg::append(&self.invalidated_properties, i);
    }
}

impl arg::ReadAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopDBusPropertiesPropertiesChanged {
            interface_name: i.read()?,
            changed_properties: i.read()?,
            invalidated_properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopDBusPropertiesPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
}

pub trait OrgFreedesktopDBusIntrospectable {
    fn introspect(&self) -> Result<String, dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target=blocking::Connection>> OrgFreedesktopDBusIntrospectable for blocking::Proxy<'a, C> {

    fn introspect(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
            .and_then(|r: (String, )| Ok(r.0, ))
    }
}

pub trait OrgFreedesktopDBusPeer {
    fn ping(&self) -> Result<(), dbus::Error>;
    fn get_machine_id(&self) -> Result<String, dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target=blocking::Connection>> OrgFreedesktopDBusPeer for blocking::Proxy<'a, C> {

    fn ping(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.DBus.Peer", "Ping", ())
    }

    fn get_machine_id(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Peer", "GetMachineId", ())
            .and_then(|r: (String, )| Ok(r.0, ))
    }
}

pub trait OrgGnomeSessionManager {
    fn setenv(&self, variable: &str, value: &str) -> Result<(), dbus::Error>;
    fn get_locale(&self, category: i32) -> Result<String, dbus::Error>;
    fn initialization_error(&self, message: &str, fatal: bool) -> Result<(), dbus::Error>;
    fn initialized(&self) -> Result<(), dbus::Error>;
    fn register_client(&self, app_id: &str, client_startup_id: &str) -> Result<dbus::Path<'static>, dbus::Error>;
    fn unregister_client(&self, client_id: dbus::Path) -> Result<(), dbus::Error>;
    fn inhibit(&self, app_id: &str, toplevel_xid: u32, reason: &str, flags: u32) -> Result<u32, dbus::Error>;
    fn uninhibit(&self, inhibit_cookie: u32) -> Result<(), dbus::Error>;
    fn is_inhibited(&self, flags: u32) -> Result<bool, dbus::Error>;
    fn get_clients(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
    fn get_inhibitors(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
    fn is_autostart_condition_handled(&self, condition: &str) -> Result<bool, dbus::Error>;
    fn shutdown(&self) -> Result<(), dbus::Error>;
    fn reboot(&self) -> Result<(), dbus::Error>;
    fn can_shutdown(&self) -> Result<bool, dbus::Error>;
    fn set_reboot_to_firmware_setup(&self, enable: bool) -> Result<(), dbus::Error>;
    fn can_reboot_to_firmware_setup(&self) -> Result<bool, dbus::Error>;
    fn logout(&self, mode: u32) -> Result<(), dbus::Error>;
    fn is_session_running(&self) -> Result<bool, dbus::Error>;
    fn session_name(&self) -> Result<String, dbus::Error>;
    fn renderer(&self) -> Result<String, dbus::Error>;
    fn session_is_active(&self) -> Result<bool, dbus::Error>;
    fn inhibited_actions(&self) -> Result<u32, dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target=blocking::Connection>> OrgGnomeSessionManager for blocking::Proxy<'a, C> {

    fn setenv(&self, variable: &str, value: &str) -> Result<(), dbus::Error> {
        self.method_call("org.gnome.SessionManager", "Setenv", (variable, value, ))
    }

    fn get_locale(&self, category: i32) -> Result<String, dbus::Error> {
        self.method_call("org.gnome.SessionManager", "GetLocale", (category, ))
            .and_then(|r: (String, )| Ok(r.0, ))
    }

    fn initialization_error(&self, message: &str, fatal: bool) -> Result<(), dbus::Error> {
        self.method_call("org.gnome.SessionManager", "InitializationError", (message, fatal, ))
    }

    fn initialized(&self) -> Result<(), dbus::Error> {
        self.method_call("org.gnome.SessionManager", "Initialized", ())
    }

    fn register_client(&self, app_id: &str, client_startup_id: &str) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call("org.gnome.SessionManager", "RegisterClient", (app_id, client_startup_id, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn unregister_client(&self, client_id: dbus::Path) -> Result<(), dbus::Error> {
        self.method_call("org.gnome.SessionManager", "UnregisterClient", (client_id, ))
    }

    fn inhibit(&self, app_id: &str, toplevel_xid: u32, reason: &str, flags: u32) -> Result<u32, dbus::Error> {
        self.method_call("org.gnome.SessionManager", "Inhibit", (app_id, toplevel_xid, reason, flags, ))
            .and_then(|r: (u32, )| Ok(r.0, ))
    }

    fn uninhibit(&self, inhibit_cookie: u32) -> Result<(), dbus::Error> {
        self.method_call("org.gnome.SessionManager", "Uninhibit", (inhibit_cookie, ))
    }

    fn is_inhibited(&self, flags: u32) -> Result<bool, dbus::Error> {
        self.method_call("org.gnome.SessionManager", "IsInhibited", (flags, ))
            .and_then(|r: (bool, )| Ok(r.0, ))
    }

    fn get_clients(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
        self.method_call("org.gnome.SessionManager", "GetClients", ())
            .and_then(|r: (Vec<dbus::Path<'static>>, )| Ok(r.0, ))
    }

    fn get_inhibitors(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
        self.method_call("org.gnome.SessionManager", "GetInhibitors", ())
            .and_then(|r: (Vec<dbus::Path<'static>>, )| Ok(r.0, ))
    }

    fn is_autostart_condition_handled(&self, condition: &str) -> Result<bool, dbus::Error> {
        self.method_call("org.gnome.SessionManager", "IsAutostartConditionHandled", (condition, ))
            .and_then(|r: (bool, )| Ok(r.0, ))
    }

    fn shutdown(&self) -> Result<(), dbus::Error> {
        self.method_call("org.gnome.SessionManager", "Shutdown", ())
    }

    fn reboot(&self) -> Result<(), dbus::Error> {
        self.method_call("org.gnome.SessionManager", "Reboot", ())
    }

    fn can_shutdown(&self) -> Result<bool, dbus::Error> {
        self.method_call("org.gnome.SessionManager", "CanShutdown", ())
            .and_then(|r: (bool, )| Ok(r.0, ))
    }

    fn set_reboot_to_firmware_setup(&self, enable: bool) -> Result<(), dbus::Error> {
        self.method_call("org.gnome.SessionManager", "SetRebootToFirmwareSetup", (enable, ))
    }

    fn can_reboot_to_firmware_setup(&self) -> Result<bool, dbus::Error> {
        self.method_call("org.gnome.SessionManager", "CanRebootToFirmwareSetup", ())
            .and_then(|r: (bool, )| Ok(r.0, ))
    }

    fn logout(&self, mode: u32) -> Result<(), dbus::Error> {
        self.method_call("org.gnome.SessionManager", "Logout", (mode, ))
    }

    fn is_session_running(&self) -> Result<bool, dbus::Error> {
        self.method_call("org.gnome.SessionManager", "IsSessionRunning", ())
            .and_then(|r: (bool, )| Ok(r.0, ))
    }

    fn session_name(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.gnome.SessionManager", "SessionName")
    }

    fn renderer(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.gnome.SessionManager", "Renderer")
    }

    fn session_is_active(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.gnome.SessionManager", "SessionIsActive")
    }

    fn inhibited_actions(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.gnome.SessionManager", "InhibitedActions")
    }
}

#[derive(Debug)]
pub struct OrgGnomeSessionManagerClientAdded {
    pub id: dbus::Path<'static>,
}

impl arg::AppendAll for OrgGnomeSessionManagerClientAdded {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.id, i);
    }
}

impl arg::ReadAll for OrgGnomeSessionManagerClientAdded {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgGnomeSessionManagerClientAdded {
            id: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgGnomeSessionManagerClientAdded {
    const NAME: &'static str = "ClientAdded";
    const INTERFACE: &'static str = "org.gnome.SessionManager";
}

#[derive(Debug)]
pub struct OrgGnomeSessionManagerClientRemoved {
    pub id: dbus::Path<'static>,
}

impl arg::AppendAll for OrgGnomeSessionManagerClientRemoved {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.id, i);
    }
}

impl arg::ReadAll for OrgGnomeSessionManagerClientRemoved {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgGnomeSessionManagerClientRemoved {
            id: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgGnomeSessionManagerClientRemoved {
    const NAME: &'static str = "ClientRemoved";
    const INTERFACE: &'static str = "org.gnome.SessionManager";
}

#[derive(Debug)]
pub struct OrgGnomeSessionManagerInhibitorAdded {
    pub id: dbus::Path<'static>,
}

impl arg::AppendAll for OrgGnomeSessionManagerInhibitorAdded {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.id, i);
    }
}

impl arg::ReadAll for OrgGnomeSessionManagerInhibitorAdded {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgGnomeSessionManagerInhibitorAdded {
            id: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgGnomeSessionManagerInhibitorAdded {
    const NAME: &'static str = "InhibitorAdded";
    const INTERFACE: &'static str = "org.gnome.SessionManager";
}

#[derive(Debug)]
pub struct OrgGnomeSessionManagerInhibitorRemoved {
    pub id: dbus::Path<'static>,
}

impl arg::AppendAll for OrgGnomeSessionManagerInhibitorRemoved {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.id, i);
    }
}

impl arg::ReadAll for OrgGnomeSessionManagerInhibitorRemoved {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgGnomeSessionManagerInhibitorRemoved {
            id: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgGnomeSessionManagerInhibitorRemoved {
    const NAME: &'static str = "InhibitorRemoved";
    const INTERFACE: &'static str = "org.gnome.SessionManager";
}

#[derive(Debug)]
pub struct OrgGnomeSessionManagerSessionRunning {
}

impl arg::AppendAll for OrgGnomeSessionManagerSessionRunning {
    fn append(&self, _: &mut arg::IterAppend) {
    }
}

impl arg::ReadAll for OrgGnomeSessionManagerSessionRunning {
    fn read(_: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgGnomeSessionManagerSessionRunning {
        })
    }
}

impl dbus::message::SignalArgs for OrgGnomeSessionManagerSessionRunning {
    const NAME: &'static str = "SessionRunning";
    const INTERFACE: &'static str = "org.gnome.SessionManager";
}

#[derive(Debug)]
pub struct OrgGnomeSessionManagerSessionOver {
}

impl arg::AppendAll for OrgGnomeSessionManagerSessionOver {
    fn append(&self, _: &mut arg::IterAppend) {
    }
}

impl arg::ReadAll for OrgGnomeSessionManagerSessionOver {
    fn read(_: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgGnomeSessionManagerSessionOver {
        })
    }
}

impl dbus::message::SignalArgs for OrgGnomeSessionManagerSessionOver {
    const NAME: &'static str = "SessionOver";
    const INTERFACE: &'static str = "org.gnome.SessionManager";
}