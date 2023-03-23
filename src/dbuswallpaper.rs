use dbus::{blocking::Connection, Error};
use std::path::Path;
use std::time::Duration;

const DBUS_PROXY_TIMEOUT: u64 = 5000;

pub struct DbusWallpaper {
    pub connection: Connection,
    pub filename: String,
}

impl DbusWallpaper {
    pub fn new(filename: String) -> Self {
        Self {
            connection: Connection::new_session().expect("D-Bus connection failed"),
            filename,
        }
    }

    pub fn set_wallpaper(self) {
        let proxy = self.connection.with_proxy(
            "org.kde.plasmashell",
            "/PlasmaShell",
            Duration::from_millis(DBUS_PROXY_TIMEOUT),
        );

        let jscript_get_wallpaper = concat!(
            "var allDesktops = desktops();",
            "d = allDesktops[0];",
            "d.wallpaperPlugin = \"org.kde.image\";",
            "d.currentConfigGroup = Array(\"Wallpaper\",\"org.kde.image\",\"General\");",
            "print(d.readConfig(\"Image\"));",
        );

        let (proxy_result,): (String,) = proxy
            .method_call(
                "org.kde.PlasmaShell",
                "evaluateScript",
                (jscript_get_wallpaper,),
            )
            .unwrap();

        let mut is_set: bool = false;
        if proxy_result.len() > 0 && proxy_result.contains("file://") {
            match Path::new(&proxy_result).file_name() {
                Some(file) => {
                    if file.to_str().unwrap() == Path::new(&self.filename).file_name().unwrap() {
                        is_set = true;
                    }
                }
                None => (),
            }
        }

        if !is_set {
            println!(":: Changing wallpaper.");
            let jscript = concat!(
                "var allDesktops = desktops();",
                "print (allDesktops);",
                "for (i=0;i<allDesktops.length;i++) {",
                "d = allDesktops[i];",
                "d.wallpaperPlugin = \"org.kde.image\";",
                "d.currentConfigGroup = Array(\"Wallpaper\", \"%s\", \"General\");",
                "d.writeConfig(\"Image\", \"file://{FILENAME}\")",
                "}",
            );
           
            let proxy_result: Result<(), Error> = proxy.method_call(
                "org.kde.PlasmaShell",
                "evaluateScript",
                (jscript.replace("{FILENAME}", &self.filename),),
            );

            match proxy_result {
                Ok(_) => println!(":: Wallpaper set."),
                Err(e) => panic!("{}", e),
            }
        } else {
            println!(":: Skipping wallpaper change, already set.");
        }
    }
}
