@echo off
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

endlocal