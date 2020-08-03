use apk_tools_sys::*;
use std::convert::TryInto;
use std::convert::TryFrom;
use std::error::Error;
use std::ffi::*;
use zbus::{dbus_interface, fdo};
use serde::*;
use serde::ser::*;

pub struct ApkDatabase {
    db: apk_database,
    db_options: apk_db_options,
    additional_repos: apk_repository_list,
}

/*
    ApkDatbase - Implementation doesn't matter much for lifetime issues
*/
impl ApkDatabase {
    pub fn new() -> ApkDatabase {
        let mut apk_db = ApkDatabase {
            db: apk_database::default(),
            db_options: apk_db_options::default(),
            additional_repos: apk_repository_list::default(),
        };
        unsafe {
            apk_db_init(&mut apk_db.db);
            apk_db.db_options.repository_list.next = &mut apk_db.db_options.repository_list;
            apk_db.db_options.repository_list.prev = &mut apk_db.db_options.repository_list;
        }
        apk_db.db_options.open_flags = (APK_OPENF_READ | APK_OPENF_NO_AUTOUPDATE) as u64;

        let res = unsafe { apk_db_open(&mut apk_db.db, &mut apk_db.db_options) };

        if res != 0 {
            panic!(
                "Failed to open the apk database due to error"
            );
        }
        apk_db
    }

        pub fn search_file_owner(&mut self, path: &str) -> Option<ApkPackage> {
        let blob = apk_blob_t {
            ptr: CString::new(path).unwrap().into_raw(),
            len: path.len() as i64,
        };
        if let Some(pkg) = unsafe { apk_db_get_file_owner(&mut self.db, blob).as_ref() } {
            return Some(ApkPackage::new(pkg, None, true));
        }
        None
    }
}

/*
    ApkPackage that only lifes as long as the apk_package pointer it's
    created from (which lifes as long as the ApkDatabase it comes from).
    This is because it's pulling the data from the apk_package pointer to
    avoid copying this data again.
*/
pub struct ApkPackage<'a> {
    apk_package: &'a apk_package,
    is_installed: bool,
    old_version: Option<&'a str>,
}

impl<'a> ApkPackage<'a> {
    pub fn new(
        new_package: &'a apk_package,
        old_package: Option<&apk_package>,
        is_installed: bool,
    ) -> ApkPackage<'a> {
        ApkPackage {
            apk_package: new_package,
            old_version: old_package
                .map(|p| unsafe { CStr::from_ptr((*p.name).name).to_str().unwrap() }),
            is_installed,
        }
    }

    /*
        Pulls name from the ptr
    */
    pub fn name(&self) -> &str {
        unsafe {
            CStr::from_ptr((*self.apk_package.name).name)
                .to_str()
                .unwrap()
        }
    }
}


impl<'a> Serialize for ApkPackage<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ApkPackage", 1)?;
        state.serialize_field("name", &self.name())?;
        /*
            Normally many more fields
        */
        state.end()
    }
}

impl<'a> zvariant::Type for ApkPackage<'a> {
    fn signature() -> zvariant::Signature<'static> {
        zvariant::Signature::try_from("(s)").unwrap()
    }
}

struct DBusServer;
#[dbus_interface(name = "dev.Cogitri.apkPolkit1")]
impl DBusServer {
    fn search_file_owner(&self, path: &str) -> fdo::Result<ApkPackage> {
        /*
          Create db and drop it after the function so we don't hold the
          db for too long
        */
        let mut db = ApkDatabase::new();
        match db.search_file_owner(path) {
            /*
              This doesn't work since pkg's lifetime is attached to ApkDatabase.
              With GDBus I would've serialised the ApkPackage into a GVariant here
              and returned that.
            */
            Some(pkg) => Ok(pkg),
            None => Err(fdo::Error::Failed("No such package".to_string())),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let connection = zbus::Connection::new_system()?;
    fdo::DBusProxy::new(&connection)
        .unwrap()
        .request_name(
            "dev.Cogitri.apkPolkit1",
            fdo::RequestNameFlags::ReplaceExisting.into(),
        )
        .unwrap();

    let mut object_server = zbus::ObjectServer::new(&connection);
    object_server
        .at(
            &"/dev/Cogitri/apkPolkit1".try_into().unwrap(),
            DBusServer,
        )
        .unwrap();
    loop {
        if let Err(err) = object_server.try_handle_next() {
            eprintln!("{}", err);
        }
    }
}
