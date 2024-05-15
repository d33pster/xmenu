use std::io::Write;

use xmenu::{Xmenu, Colour};

fn main() {
    let mut xm = Xmenu::new();
    println!("Use Up and Down arrows to navigate. Press Enter to choose. Press Esc to Exit.");
    xm.add("Show demo without colour.");
    xm.add("Show demo.");
    xm.add("Fix Windows-Terminal-Colour Problem.");
    let result = xm.run(Colour::Default);
    
    if result == "Abort".to_string() {
        std::process::exit(0);
    } else if result == "Show demo without colour.".to_string() {
        println!("\nDemo without colour");
        let mut xm = Xmenu::new();
        xm.add("https");
        xm.add("ssh");
        let demo_res = xm.run(Colour::Default);
        println!("You chose: {}", demo_res);
    } else if result == "Show demo.".to_string() {
        println!("\nDemo with color:");
        let mut xm = Xmenu::new();
        xm.add("https");
        xm.add("ssh");
        let demo_res = xm.run(Colour::Blue);
        println!("You chose: {}", demo_res);
    } else if result == "Fix Windows-Terminal-Colour Problem.".to_string() {
        let os = std::env::consts::OS;

        if os!="windows" {
            eprintln!("This feature is only for Windows.");
            std::process::exit(0);
        }else {
            match make_dot_bat_file() {
                Ok(()) => {println!("Wrote -> fix-terminal.bat")},
                Err(e) => {eprintln!("Error: {}", e.to_string())},
            }

            let _output = std::process::Command::new("cmd")
                .args(&["/C", "fix-terminal.bat"])
                .output()
                .expect("Error running fix-terminal.bat. Double-Click on it. Its in the current working directory.");

            println!("Fixed");
        }
    }
}

fn make_dot_bat_file() -> std::io::Result<()> {
    let filepath = "fix-terminal.bat";
    let mut file = std::fs::File::create(filepath)?;

    file.write_all(r###"@echo off
:: BatchGotAdmin
:--------------------------
@Rem --> Check for Permission
>nul 2>&1 "%SYSTEMROOT%\system32\cacls.exe" "%SYSTEMROOT%\system32\config\system"

@Rem if error flag set, we do not have admin.
if %errorlevel% NEQ 0 (
    goto UACPrompt
) else ( goto gotAdmin )

:UACPrompt
    echo Set UAC = CreateObject^("Shell.Application"^) > "%temp%\getadmin.vbs"
    echo UAC.ShellExecute "%~s0", "", "", "runas", 1 >> "%temp%\getadmin.vbs"

    "%temp%\getadmin.vbs"
    exit /B

:gotAdmin
    if exist "%temp%\getadmin.vbs" ( del "%temp%\getadmin.vbs" )
    pushd "%CD%"
    CD /D "%~dp0"
:-----------------------------------

@Rem check registry for ansi colour code
echo [REGISTRY CHECK]
reg query HKCU\Console\ | find /I "VirtualTerminalLevel" > nul
if %errorlevel% NEQ 0 (
    @Rem ANSI settings not found.
    reg add HKCU\Console /v VirtualTerminalLevel /t REG_DWORD /d 1
    echo added registry for ANSI escape sequence
) else (
    echo found existing registry for ANSI escape sequence. Skipping..
)

echo.
echo FIXED TERMINAL COLOUR PROBLEM
setlocal

set /p choke=

endlocal"###.as_bytes())?;

    file.flush()?;

    Ok(())
}