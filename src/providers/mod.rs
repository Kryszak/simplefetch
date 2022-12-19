pub(crate) mod os_information;
pub(crate) mod os_installation_date;
pub(crate) mod os_name;
pub(crate) mod os_wm;
pub(crate) mod os_kernel_version;
pub(crate) mod os_shell;
pub(crate) mod os_terminal;

pub(crate) use os_installation_date::OsInstallationDate;
pub(crate) use os_information::OsInformation;
pub(crate) use os_name::OsName;
pub(crate) use os_wm::OsWm;
pub(crate) use os_kernel_version::OsKernelVersion;
pub(crate) use os_shell::OsShell;
pub(crate) use os_terminal::OsTerminal;