@echo off
if /I not "%~1"=="__KCC_RUN" (
    start "Komodo Control Center V18" cmd.exe /D /K call "%~f0" __KCC_RUN
    exit /b 0
)
setlocal EnableExtensions EnableDelayedExpansion

rem ============================================================================
rem  KOMODO CONTROL CENTER V18 - WINDOWS
rem  ASCII ONLY / NO BOM / CMD-SAFE
rem  Target repository:
rem  H:\REPOSITORIOS GITHUB\komodo-main
rem ============================================================================

set "KOMODO_ROOT=H:\REPOSITORIOS GITHUB\komodo-main"
set "INSTALL_BAT=%KOMODO_ROOT%\KOMODO_CONTROL_CENTER.bat"
set "PROJECT_NAME=komodo"
set "DASHBOARD_URL=http://localhost:9120"
set "LOG_DIR=%KOMODO_ROOT%\.komodo-windows"
set "STATE_DIR=%LOG_DIR%"
set "LOG_FILE=%LOG_DIR%\control-center.log"
set "ERROR_LOG=%LOG_DIR%\errors.log"
set "NOTIFY_FLAG=%LOG_DIR%\notifications.enabled"
set "CONTROL_ICON=%LOG_DIR%\komodo-control-center.ico"
set "NOTIFY_PS1=%LOG_DIR%\notify-error.ps1"

rem --- CMD appearance ----------------------------------------------------------
title KOMODO CONTROL CENTER V18 - WINDOWS
mode con: cols=118 lines=64 >nul 2>&1
chcp 65001 >nul 2>&1

rem --- ANSI colors. No Unicode characters are used in the source. -------------
for /F "delims=" %%E in ('echo prompt $E^| cmd') do set "ESC=%%E"
set "RESET=%ESC%[0m"
set "TAG_ADMIN=%ESC%[30;103m"
set "TAG_MEMBER=%ESC%[30;106m"
set "BOLD=%ESC%[1m"
set "DIM=%ESC%[2m"
set "RED=%ESC%[91m"
set "GREEN=%ESC%[92m"
set "YELLOW=%ESC%[93m"
set "BLUE=%ESC%[94m"
set "MAGENTA=%ESC%[95m"
set "CYAN=%ESC%[96m"
set "WHITE=%ESC%[97m"
set "GRAY=%ESC%[90m"

rem --- Validate repository -----------------------------------------------------
if not exist "%KOMODO_ROOT%\" goto ROOT_ERROR
if not exist "%LOG_DIR%\" mkdir "%LOG_DIR%" >nul 2>&1
if not exist "%NOTIFY_FLAG%" >"%NOTIFY_FLAG%" echo(ON
for %%Z in ("%NOTIFY_FLAG%") do if %%~zZ EQU 0 >"%NOTIFY_FLAG%" echo(ON
findstr /I /X /C:"ON" /C:"OFF" "%NOTIFY_FLAG%" >nul 2>&1
if errorlevel 1 >"%NOTIFY_FLAG%" echo(ON
call :EnsureSupportFiles
call :BeginSession

rem --- Install a permanent copy in repository automatically -------------------
if defined KCC_MASTER (
    if exist "%KCC_MASTER%" copy /Y "%KCC_MASTER%" "%INSTALL_BAT%" >nul 2>&1
) else (
    if /I not "%~f0"=="%INSTALL_BAT%" copy /Y "%~f0" "%INSTALL_BAT%" >nul 2>&1
)

call :DetectCompose
call :Log "Control Center started"

goto MAIN

:MAIN
@echo off
call :DetectCompose
call :QuickState
cls
call :Header
@echo off
echo %CYAN%%BOLD%   KOMODO CONTROL CENTER V18 - COMPLETE VERTICAL MENU%RESET%
echo.
echo    %GREEN%[ 1 ]%RESET%  Start Komodo
echo    %YELLOW%[ 2 ]%RESET%  Stop Komodo
echo    %YELLOW%[ 3 ]%RESET%  Restart Komodo
echo    %CYAN%[ 4 ]%RESET%  Open dashboard in browser
echo    %BLUE%[ 5 ]%RESET%  Service status
echo    %BLUE%[ 6 ]%RESET%  Live Core logs
echo    %BLUE%[ 7 ]%RESET%  All service logs
echo    %BLUE%[ 8 ]%RESET%  Intelligent diagnostics
echo    %BLUE%[ 9 ]%RESET%  HTTP health check
echo    %BLUE%[10 ]%RESET%  Port 9120 and processes
echo    %MAGENTA%[11 ]%RESET%  Update Docker images
echo    %MAGENTA%[12 ]%RESET%  Update Git repository
echo    %MAGENTA%[13 ]%RESET%  Edit environment file
echo    %MAGENTA%[14 ]%RESET%  Security audit
echo    %CYAN%[15 ]%RESET%  Open repository folder
echo    %CYAN%[16 ]%RESET%  Open PowerShell in repository
echo    %CYAN%[17 ]%RESET%  Open VS Code
echo    %CYAN%[18 ]%RESET%  Environment information
echo    %CYAN%[19 ]%RESET%  Open official Komodo GitHub
echo    %CYAN%[20 ]%RESET%  Quick help
echo    %GREEN%[21 ]%RESET%  Create / repair Desktop shortcut
echo    %GREEN%[22 ]%RESET%  Start Control Center with Windows
echo    %GREEN%[23 ]%RESET%  Remove Windows startup
echo    %GREEN%[24 ]%RESET%  View initial login / admin
echo    %GREEN%[25 ]%RESET%  Change initial admin / password
echo    %GREEN%[26 ]%RESET%  Download official Komodo update
echo    %GREEN%[27 ]%RESET%  Reset existing Komodo user password
echo    %GREEN%[28 ]%RESET%  Promote user to Super Admin
echo    %GREEN%[29 ]%RESET%  List real Komodo users
echo    %GREEN%[30 ]%RESET%  Enable local registration
echo    %MAGENTA%[31 ]%RESET%  TRANSLATE KOMODO LANGUAGES - LIVE
echo    %MAGENTA%[32 ]%RESET%  Restore original English Komodo interface
echo    %BLUE%[33 ]%RESET%  Logs / reports / notifications center
echo    %CYAN%[34 ]%RESET%  Complete Komodo guide - course 0 to 23
echo    %MAGENTA%[35 ]%RESET%  Publish Control Center via Fork + Pull Request
echo    %CYAN%[36 ]%RESET%  Control Center language
echo    %CYAN%[37 ]%RESET%  Documentation inside the menu
echo    %CYAN%[38 ]%RESET%  Export safe Support Bundle
echo    %CYAN%[39 ]%RESET%  Safe configuration backup
echo    %CYAN%[40 ]%RESET%  Quick assistant - What do you want to do?
echo    %GREEN%[41 ]%RESET%  KOMODO AUTONOMOUS CENTER - manage Komodo from this menu
echo    %GREEN%[42 ]%RESET%  Komodo API credentials / autonomous connection
echo    %GREEN%[43 ]%RESET%  Universal Komodo API runner - read / write / execute
echo    %GREEN%[44 ]%RESET%  Complete resource overview
echo    %RED%[90 ]%RESET%  Remove containers - preserve volumes
echo    %RED%[91 ]%RESET%  TOTAL reset including volumes
echo.
echo %GRAY%   [R] Refresh   [B] Back   [C] Cancel   [D] Docker Desktop   [0] Exit%RESET%
echo.
set "OP="
set /p "OP=%WHITE%%BOLD%   Select an option: %RESET%"
call :LogAction "MENU selected: %OP%"

if /I "%OP%"=="R" goto MAIN
if /I "%OP%"=="B" goto MAIN
if /I "%OP%"=="C" goto MAIN
if /I "%OP%"=="D" goto DOCKER_DESKTOP
if "%OP%"=="0" goto EXIT_MENU
if "%OP%"=="1" goto START
if "%OP%"=="2" goto STOP
if "%OP%"=="3" goto RESTART
if "%OP%"=="4" goto OPEN_WEB
if "%OP%"=="5" goto STATUS
if "%OP%"=="6" goto LOG_CORE
if "%OP%"=="7" goto LOG_ALL
if "%OP%"=="8" goto DIAG
if "%OP%"=="9" goto HEALTH
if "%OP%"=="10" goto PORT
if "%OP%"=="11" goto PULL_IMAGES
if "%OP%"=="12" goto GIT_PULL
if "%OP%"=="13" goto EDIT_ENV
if "%OP%"=="14" goto SECURITY
if "%OP%"=="15" goto EXPLORER
if "%OP%"=="16" goto POWERSHELL
if "%OP%"=="17" goto VSCODE
if "%OP%"=="18" goto INFO
if "%OP%"=="19" goto GITHUB
if "%OP%"=="20" goto HELP
if "%OP%"=="21" goto SHORTCUT
if "%OP%"=="22" goto AUTOSTART_ON
if "%OP%"=="23" goto AUTOSTART_OFF
if "%OP%"=="24" goto AUTH_INFO
if "%OP%"=="25" goto AUTH_EDIT
if "%OP%"=="26" goto UPDATE_FROM_GITHUB
if "%OP%"=="27" goto RESET_EXISTING_PASSWORD
if "%OP%"=="28" goto MAKE_SUPER_ADMIN
if "%OP%"=="29" goto LIST_REAL_USERS
if "%OP%"=="30" goto ENABLE_LOCAL_SIGNUP
if "%OP%"=="31" goto KOMODO_LANGUAGE_CENTER
if "%OP%"=="32" goto RESTORE_ENGLISH
if "%OP%"=="33" goto LOG_CENTER
if "%OP%"=="34" goto GUIDE_MAIN
if "%OP%"=="35" goto GITHUB_PUBLISH_CENTER
if "%OP%"=="36" goto CONTROL_CENTER_LANGUAGE
if "%OP%"=="37" goto DOCUMENTATION_CENTER
if "%OP%"=="38" goto SUPPORT_BUNDLE
if "%OP%"=="39" goto SAFE_CONFIG_BACKUP
if "%OP%"=="40" goto QUICK_WIZARD
if "%OP%"=="41" goto AUTONOMOUS_CENTER
if "%OP%"=="42" goto API_CREDENTIALS
if "%OP%"=="43" goto UNIVERSAL_API
if "%OP%"=="44" goto RESOURCE_OVERVIEW
if "%OP%"=="90" goto REMOVE_CONTAINERS
if "%OP%"=="91" goto FACTORY_RESET

call :MsgError "Invalid option: %OP%"
goto MAIN

rem ============================================================================
rem  OPERATIONS
rem ============================================================================

:START
@echo off
cls
call :Header
echo %GREEN%%BOLD%   [^>] start KOMODO%RESET%
echo.
call :Preflight
if errorlevel 1 goto PAUSE_MAIN

call :EnsureDocker
if errorlevel 1 goto PAUSE_MAIN

echo %CYAN%Stack detected:%RESET% %COMPOSE_FILE%
if defined ENV_FILE echo %CYAN%Ambiente:%RESET% %ENV_FILE%
echo.
echo %GRAY%Iniciando containers...%RESET%
call :Compose up -d
if errorlevel 1 (
    call :MsgError "Failed to start o stack."
    echo.
    echo run a option 8 for diagnose.
    goto PAUSE_MAIN
)

call :Log "Komodo started"
echo.
call :WaitHttp 25
echo.
call :Compose ps
echo.
call :MsgOk "command of startup completed."
call :ApplyPtBrSilent
echo %CYAN%dashboard esperado:%RESET% %DASHBOARD_URL%
echo.
choice /C SBC /N /M "Open dashboard? [S] Yes  [B] Back  [C] Cancel: "
if errorlevel 3 goto MAIN
if errorlevel 2 goto MAIN
start "" "%DASHBOARD_URL%"
goto PAUSE_MAIN

:STOP
@echo off
cls
call :Header
echo %YELLOW%%BOLD%   [[] stop KOMODO%RESET%
echo.
call :Preflight
if errorlevel 1 goto PAUSE_MAIN
call :EnsureDocker
if errorlevel 1 goto PAUSE_MAIN
call :Compose stop
if errorlevel 1 (
    call :MsgError "Could not stop all os services."
) else (
    call :Log "Komodo stopped"
    call :MsgOk "services parados. volumes preservados."
)
goto PAUSE_MAIN

:RESTART
@echo off
cls
call :Header
echo %YELLOW%%BOLD%   [@] restart KOMODO%RESET%
echo.
call :Preflight
if errorlevel 1 goto PAUSE_MAIN
call :EnsureDocker
if errorlevel 1 goto PAUSE_MAIN
call :Compose restart
if errorlevel 1 (
    call :MsgError "Failed to restart."
) else (
    call :WaitHttp 20
    call :Log "Komodo restarted"
    call :ApplyPtBrSilent
    call :MsgOk "restart completed."
)
goto PAUSE_MAIN

:OPEN_WEB
start "" "%DASHBOARD_URL%"
goto MAIN

:STATUS
cls
call :Header
echo %BLUE%%BOLD%   [#] STATUS of the services%RESET%
echo.
call :Preflight
if errorlevel 1 goto PAUSE_MAIN
call :EnsureDocker
if errorlevel 1 goto PAUSE_MAIN
call :Compose ps
echo.
echo %GRAY%containers with nome relacionado a Komodo:%RESET%
docker ps -a --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}" 2>nul | findstr /I /C:"NAMES" /C:"komodo"
goto PAUSE_MAIN

:LOG_CORE
cls
call :Header
echo %BLUE%%BOLD%   [=] logs of the CORE%RESET%
echo %GRAY%use CTRL+C for stop os logs and back.%RESET%
echo.
call :Preflight
if errorlevel 1 goto PAUSE_MAIN
call :EnsureDocker
if errorlevel 1 goto PAUSE_MAIN
call :Compose logs --tail=150 -f core
goto MAIN

:LOG_ALL
cls
call :Header
echo %BLUE%%BOLD%   [=] logs GERAIS%RESET%
echo %GRAY%use CTRL+C for stop os logs and back.%RESET%
echo.
call :Preflight
if errorlevel 1 goto PAUSE_MAIN
call :EnsureDocker
if errorlevel 1 goto PAUSE_MAIN
call :Compose logs --tail=120 -f
goto MAIN

:DIAG
@echo off
cls
call :Header
echo %BLUE%%BOLD%   [+] diagnostics intelligent - mode safe%RESET%
echo.
set /A ERRORS=0
set /A WARNINGS=0

if exist "%KOMODO_ROOT%\" (
    call :DiagOk "folder of the repository found."
) else (
    call :DiagFail "folder of the repository not found."
)

if defined COMPOSE_FILE (
    call :DiagOk "Compose detected: %COMPOSE_FILE%"
) else (
    call :DiagFail "in the compose suportado was detected."
)

if defined ENV_FILE (
    call :DiagOk "File of environment: %ENV_FILE%"
) else (
    call :DiagWarn "in the compose.env/.env detected. may be normal for this stack."
)

where docker >nul 2>&1
if errorlevel 1 (
    call :DiagFail "Docker CLI not found in the PATH."
) else (
    for /F "delims=" %%V in ('docker --version 2^>nul') of the call :DiagOk "%%V"
)

docker info >nul 2>&1
if errorlevel 1 (
    call :DiagFail "Docker Engine not is respondendo."
) else (
    call :DiagOk "Docker Engine online."
)

docker compose version >nul 2>&1
if errorlevel 1 (
    call :DiagFail "Docker Compose v2 not found."
) else (
    for /F "delims=" %%V in ('docker compose version 2^>nul') of the call :DiagOk "%%V"
)

where git >nul 2>&1
if errorlevel 1 (
    call :DiagWarn "Git not found in the PATH."
) else (
    for /F "delims=" %%V in ('git --version 2^>nul') of the call :DiagOk "%%V"
)

if exist "%KOMODO_ROOT%\.git\" (
    call :DiagOk "Repository possui .git and accepted Git Pull."
) else (
    call :DiagWarn "folder without .git. probably was downloaded how ZIP."
)

netstat -ano 2>nul | findstr /R /C:":9120 .*LISTENING" >nul 2>&1
if errorlevel 1 (
    call :DiagWarn "port 9120 not is in LISTENING."
) else (
    call :DiagOk "port 9120 in LISTENING."
)

where curl >nul 2>&1
if errorlevel 1 (
    call :DiagWarn "curl.exe not found; health HTTP not testado."
) else (
    curl.exe -fsS --max-time 3 "%DASHBOARD_URL%" >nul 2>&1
    if errorlevel 1 (
        call :DiagWarn "dashboard HTTP not respondeu in %DASHBOARD_URL%."
    ) else (
        call :DiagOk "dashboard HTTP respondeu in %DASHBOARD_URL%."
    )
)

where rustc >nul 2>&1
if errorlevel 1 (
    call :DiagWarn "Rust not installed. required only for development/build local."
) else (
    for /F "delims=" %%V in ('rustc --version 2^>nul') of the call :DiagOk "%%V"
)

where node >nul 2>&1
if errorlevel 1 (
    call :DiagWarn "Node.js not installed. may be required for development of the UI."
) else (
    for /F "delims=" %%V in ('node --version 2^>nul') of the call :DiagOk "Node.js %%V"
)

echo.
echo %WHITE%%BOLD%   RESULTADO%RESET%
echo %GRAY%   ------------------------------------------------------------------------%RESET%
echo    Errors  : %RED%!ERRORRS!%RESET%
echo    warnings : %YELLOW%!WARNINGS!%RESET%
echo.
if !ERRORS! EQU 0 (
    call :MsgOk "Base operacional aprovada."
) else (
    call :MsgError "Existem problems bloqueando o environment."
)
call :Log "Diagnosis: !ERRORS! errors / !WARNINGS! warnings"
goto PAUSE_MAIN

:HEALTH
cls
call :Header
echo %BLUE%%BOLD%   [+] HEALTH CHECK HTTP%RESET%
echo.
where curl >nul 2>&1
if errorlevel 1 (
    call :MsgError "curl.exe not found."
    goto PAUSE_MAIN
)
set "HTTP_CODE="
set "HTTP_TIME="
for /F "tokens=1,2" %%A in ('curl.exe -sS -o NUL -w "%%{http_code} %%{time_total}" --max-time 8 "%DASHBOARD_URL%" 2^>nul') do (
    set "HTTP_CODE=%%A"
    set "HTTP_TIME=%%B"
)
if not defined HTTP_CODE set "HTTP_CODE=000"
if "%HTTP_CODE%"=="000" (
    call :MsgError "dashboard HTTP not respondeu."
    echo %YELLOW%use a option 8 for diagnostics and a option 7 for logs.%RESET%
) else (
    echo %GREEN%[OK] HTTP %HTTP_CODE%  Tempo: %HTTP_TIME%s  %DASHBOARD_URL%%RESET%
)
goto PAUSE_MAIN

:PORT
cls
call :Header
echo %BLUE%%BOLD%   [#] port 9120 and processes%RESET%
echo.
netstat -ano 2>nul | findstr ":9120"
if errorlevel 1 echo %YELLOW%Nenhum processo encontrado usando a porta 9120.%RESET%
echo.
echo %GRAY%containers relacionados:%RESET%
docker ps -a --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}" 2>nul | findstr /I /C:"NAMES" /C:"komodo"
goto PAUSE_MAIN

:PULL_IMAGES
cls
call :Header
echo %MAGENTA%%BOLD%   [~] update images DOCKER%RESET%
echo.
call :Preflight
if errorlevel 1 goto PAUSE_MAIN
call :EnsureDocker
if errorlevel 1 goto PAUSE_MAIN
call :Compose pull
if errorlevel 1 (
    call :MsgError "Failed to download images."
    goto PAUSE_MAIN
)
echo.
choice /C SBC /N /M "Recriar containers? [S] Yes  [B] Back  [C] Cancel: "
if errorlevel 3 goto MAIN
if errorlevel 2 goto MAIN
call :Compose up -d
if errorlevel 1 (
    call :MsgError "Failed to recriar containers."
) else (
    call :Log "Docker images updated"
    call :MsgOk "update completed."
)
goto PAUSE_MAIN

:GIT_PULL
cls
call :Header
echo %MAGENTA%%BOLD%   [v] update repository GIT%RESET%
echo.
where git >nul 2>&1
if errorlevel 1 (
    call :MsgError "Git not found."
    goto PAUSE_MAIN
)
if not exist "%KOMODO_ROOT%\.git\" (
    call :MsgError "is folder not possui .git."
    echo.
    echo %YELLOW%it probably was downloaded how ZIP. Git Pull not works in ZIP.%RESET%
    echo O menu continues funcionando normalmente.
    goto PAUSE_MAIN
)
pushd "%KOMODO_ROOT%"
git status --short --branch
echo.
git pull --ff-only
set "RC=!errorlevel!"
popd
if not "!RC!"=="0" (
    call :MsgError "Git Pull interrompido. in the merge forcado was realizado."
) else (
    call :Log "Repository updated with git pull"
    call :MsgOk "Repository updated."
)
goto PAUSE_MAIN

:EDIT_ENV
cls
call :Header
echo %MAGENTA%%BOLD%   [#] EDITAR environment%RESET%
echo.
if not defined ENV_FILE (
    call :MsgError "in the compose.env or .env was detected."
    goto PAUSE_MAIN
)
start "" notepad.exe "%ENV_FILE%"
echo Editor aberto:
echo %ENV_FILE%
goto PAUSE_MAIN

:SECURITY
cls
call :Header
echo %MAGENTA%%BOLD%   [!] AUDITORIA of security%RESET%
echo.
if not defined ENV_FILE (
    call :MsgError "in the file of environment was detected."
    goto PAUSE_MAIN
)

set /A SECWARN=0
call :SecurityCheck "changeme" "value default changeme found"
call :SecurityCheck "a_random_secret" "Webhook secret default found"
call :SecurityCheck "a_random_jwt_secret" "JWT secret default found"
call :SecurityCheck "PASSWORD=admin" "Password admin default found"
call :SecurityCheck "PASSWORD=password" "Password password default found"

echo.
if !SECWARN! EQU 0 (
    call :MsgOk "in the value default conhecido was found."
) else (
    echo %YELLOW%%BOLD%!SECWARN! alert(s) of security found(s).%RESET%
    echo.
    echo Not expose o dashboard for a Internet using credentials default.
    echo.
    choice /C SBC /N /M "open file? [S] Yes  [B] Back  [C] Cancel: "
    if errorlevel 3 goto MAIN
    if errorlevel 2 goto MAIN
    start "" notepad.exe "%ENV_FILE%"
)
goto PAUSE_MAIN

:EXPLORER
start "" explorer.exe "%KOMODO_ROOT%"
goto MAIN

:POWERSHELL
start "Komodo PowerShell" powershell.exe -NoExit -ExecutionPolicy Bypass -Command "Set-Location -LiteralPath '%KOMODO_ROOT%'"
goto MAIN

:VSCODE
where code >nul 2>&1
if errorlevel 1 (
    cls
    call :Header
    call :MsgError "VS Code CLI 'code' not found in the PATH."
    if exist "%KOMODO_ROOT%\komodo.code-workspace" (
        echo.
        echo trying open o workspace by the associacao of the Windows...
        start "" "%KOMODO_ROOT%\komodo.code-workspace"
    )
    goto PAUSE_MAIN
)
if exist "%KOMODO_ROOT%\komodo.code-workspace" (
    start "" code "%KOMODO_ROOT%\komodo.code-workspace"
) else (
    start "" code "%KOMODO_ROOT%"
)
goto MAIN

:INFO
cls
call :Header
echo %CYAN%%BOLD%   [i] information of the environment%RESET%
echo.
echo    Repository : %KOMODO_ROOT%
echo    Launcher    : %INSTALL_BAT%
echo    Compose     : %COMPOSE_FILE%
if defined ENV_FILE (echo    Ambiente    : %ENV_FILE%) else (echo    Ambiente    : nao detectado)
echo    Dashboard   : %DASHBOARD_URL%
echo    log local   : %LOG_FILE%
echo.
echo %GRAY%   ------------------------------------------------------------------------%RESET%
ver
where docker >nul 2>&1 && docker --version
docker compose version 2>nul
where git >nul 2>&1 && git --version
where rustc >nul 2>&1 && rustc --version
where node >nul 2>&1 && node --version
echo.
if exist "%KOMODO_ROOT%\.git\" (
    pushd "%KOMODO_ROOT%"
    git remote -v
    git status --short --branch
    popd
) else (
    echo %YELLOW%without diretorio .git: copy probably obtida by download ZIP.%RESET%
)
goto PAUSE_MAIN

:GITHUB
start "" "https://github.com/moghtech/komodo"
goto MAIN

:HELP
cls
call :Header
echo %CYAN%%BOLD%   [?] AJUDA quick%RESET%
echo.
echo    1   Inicia o stack detected automatically.
echo    2   for os services without delete volumes.
echo    3   Reinicia os services.
echo    8   Diagnostica Docker, Compose, Git, HTTP, port and dependencias.
echo   11   Atualiza images Docker.
echo   12   runs Git Pull only if a folder have .git.
echo   14   Procura values inseguros common in the file of environment.
echo   21   Cria or repara o shortcut of the Area of Trabalho.
echo   22   Cria shortcut in the startup of the Windows.
echo   24   shows user/password initial configurados.
echo   25   Altera admin/password automatically and recria containers.
echo   26   Baixa a versao more recente direto of the GitHub, with backup.
echo   27   Reseta a password of a user that already EXISTE in the database.
echo   28   promotes a user existing for Super Admin.
echo   29   Lista users real with tag ADMIN or member.
echo   30   Habilita registration local without editar compose.env manualmente.
echo   31   Traduz interface Komodo for PT-BR to the live. after just F5.
echo   32   removes a translation and restores a interface original in English.
echo   33   Central of logs: report, errors, warnings and notifications ON/OFF.
echo   34   course complete 0 a 23. in each screen use [P] next step.
echo   35   Publica/atualiza o Control Center in the GitHub via fork + Pull Request.
echo   90   removes containers/network, preservando volumes.
echo   91   removes containers and volumes. deletes data.
echo.
echo %WHITE%%BOLD%   NAVEGACAO%RESET%
echo    B   returns for a screen/menu previous.
echo    C   Cancela a operation current and returns to the menu.
echo.
echo %WHITE%%BOLD%   PROTECOES%RESET%
echo    - O file uses only characters ASCII for avoid corruption of the CMD.
echo    - Not runs PowerShell automatically to the open.
echo    - Not depende of cd /d of the PowerShell.
echo    - Detecta automatically o file compose.
echo    - tries start o Docker Desktop when required.
echo    - Errors voltam to the menu in vez of close a window.
echo.
goto PAUSE_MAIN

:SHORTCUT
cls
call :Header
echo %GREEN%%BOLD%   [+] create / REPARAR shortcut%RESET%
echo.
call :CreateShortcut
@echo off
goto PAUSE_MAIN

:AUTOSTART_ON
cls
call :Header
echo %GREEN%%BOLD%   [+] start with O WINDOWS%RESET%
echo.
call :InstallAutostart
goto PAUSE_MAIN

:AUTOSTART_OFF
cls
call :Header
echo %GREEN%%BOLD%   [-] removes INICIO with WINDOWS%RESET%
echo.
call :RemoveAutostart
goto PAUSE_MAIN

:DOCKER_DESKTOP
call :StartDockerDesktop
goto MAIN


:AUTH_INFO
@echo off
cls
call :Header
echo %GREEN%%BOLD%   [@] LOGIN / ADMIN initial%RESET%
echo.
call :ShowUsersPanel
if errorlevel 1 goto PAUSE_MAIN
echo.
echo.
if not defined ENV_FILE (
    call :MsgError "in the file of environment was detected."
    goto PAUSE_MAIN
)
echo File:
echo   %ENV_FILE%
echo.
set "ADMIN_USER="
set "ADMIN_PASS="
for /F "tokens=1,* delims==" %%A in ('findstr /B /I "KOMODO_INIT_ADMIN_USERNAME=" "%ENV_FILE%" 2^>nul') do set "ADMIN_USER=%%B"
for /F "tokens=1,* delims==" %%A in ('findstr /B /I "KOMODO_INIT_ADMIN_PASSWORD=" "%ENV_FILE%" 2^>nul') do set "ADMIN_PASS=%%B"
if defined ADMIN_USER (
    echo %WHITE%User initial:%RESET% !ADMIN_USER!
) else (
    echo %YELLOW%User initial not definido.%RESET%
)
if defined ADMIN_PASS (
    echo %WHITE%Password initial:%RESET% !ADMIN_PASS!
) else (
    echo %YELLOW%Password initial not definida.%RESET%
)
echo.
echo %YELLOW%important:%RESET%
echo - these credentials are used for create o first admin in the first startup.
echo - if o user already was created in the database, changing o file may not change a password existing.
echo - in the compose official current, o default and admin / changeme.
echo.
goto PAUSE_MAIN

:AUTH_EDIT
@echo off
cls
call :Header
echo %GREEN%%BOLD%   [#] change ADMIN / password automatically%RESET%
echo.
call :ShowUsersPanel
if errorlevel 1 goto PAUSE_MAIN
echo.
echo.
if not defined ENV_FILE (
    call :MsgError "in the file of environment was detected."
    goto PAUSE_MAIN
)

echo is rotina faz tudo sozinha:
echo   1. Faz backup of the compose.env
echo   2. Altera user and password without open editor
echo   3. Recria os containers
echo   4. confirms a configuration applied
echo.
echo %YELLOW%important:%RESET% KOMODO_INIT_ADMIN_* cria o first admin.
echo if a account already existir in the database, these variaveis NOT redefinem
echo a password of the account existing.
echo for a account existing, use a option [27].
echo.

setlocal DisableDelayedExpansion
set "NEWADMIN="
set "NEWPASS="
set /p "NEWADMIN=New admin username (B/C cancela): "
if /I "%NEWADMIN%"=="B" (
    endlocal
    goto MAIN
)
if /I "%NEWADMIN%"=="C" (
    endlocal
    goto MAIN
)
if not defined NEWADMIN (
    endlocal
    call :MsgError "User vazio. Operation cancelled."
    goto PAUSE_MAIN
)
set /p "NEWPASS=New password (B/C cancela): "
if /I "%NEWPASS%"=="B" (
    endlocal
    goto MAIN
)
if /I "%NEWPASS%"=="C" (
    endlocal
    goto MAIN
)
if not defined NEWPASS (
    endlocal
    call :MsgError "Password vazia. Operation cancelled."
    goto PAUSE_MAIN
)

set "ENV_BACKUP=%ENV_FILE%.backup"
copy /Y "%ENV_FILE%" "%ENV_BACKUP%" >nul 2>&1
if errorlevel 1 (
    endlocal
    call :MsgError "Could not create backup."
    goto PAUSE_MAIN
)

set "VBS=%TEMP%\komodo_auth_%RANDOM%.vbs"
> "%VBS%" echo On Error Resume Next
>>"%VBS%" echo Set sh = CreateObject("WScript.Shell")
>>"%VBS%" echo envPath = sh.Environment("PROCESS")("ENV_FILE")
>>"%VBS%" echo newUser = sh.Environment("PROCESS")("NEWADMIN")
>>"%VBS%" echo newPass = sh.Environment("PROCESS")("NEWPASS")
>>"%VBS%" echo Set stm = CreateObject("ADODB.Stream")
>>"%VBS%" echo stm.Type = 2
>>"%VBS%" echo stm.Charset = "utf-8"
>>"%VBS%" echo stm.Open
>>"%VBS%" echo stm.LoadFromFile envPath
>>"%VBS%" echo txt = stm.ReadText
>>"%VBS%" echo stm.Close
>>"%VBS%" echo Set re = New RegExp
>>"%VBS%" echo re.Global = True
>>"%VBS%" echo re.MultiLine = True
>>"%VBS%" echo re.Pattern = "^KOMODO_INIT_ADMIN_USERNAME=.*$"
>>"%VBS%" echo If re.Test(txt) Then
>>"%VBS%" echo   txt = re.Replace(txt, "KOMODO_INIT_ADMIN_USERNAME=" ^& newUser)
>>"%VBS%" echo Else
>>"%VBS%" echo   txt = txt ^& vbCrLf ^& "KOMODO_INIT_ADMIN_USERNAME=" ^& newUser
>>"%VBS%" echo End If
>>"%VBS%" echo re.Pattern = "^KOMODO_INIT_ADMIN_PASSWORD=.*$"
>>"%VBS%" echo If re.Test(txt) Then
>>"%VBS%" echo   txt = re.Replace(txt, "KOMODO_INIT_ADMIN_PASSWORD=" ^& newPass)
>>"%VBS%" echo Else
>>"%VBS%" echo   txt = txt ^& vbCrLf ^& "KOMODO_INIT_ADMIN_PASSWORD=" ^& newPass
>>"%VBS%" echo End If
>>"%VBS%" echo Set out = CreateObject("ADODB.Stream")
>>"%VBS%" echo out.Type = 2
>>"%VBS%" echo out.Charset = "utf-8"
>>"%VBS%" echo out.Open
>>"%VBS%" echo out.WriteText txt
>>"%VBS%" echo out.Position = 0
>>"%VBS%" echo out.SaveToFile envPath, 2
>>"%VBS%" echo out.Close
>>"%VBS%" echo If Err.Number ^<^> 0 Then WScript.Quit 1 Else WScript.Quit 0

cscript //nologo "%VBS%" >nul 2>&1
set "AUTH_RC=%errorlevel%"
del /Q "%VBS%" >nul 2>&1

if not "%AUTH_RC%"=="0" (
    copy /Y "%ENV_BACKUP%" "%ENV_FILE%" >nul 2>&1
    endlocal
    call :MsgError "Failed to change credentials. backup restored."
    goto PAUSE_MAIN
)

endlocal
echo.
call :MsgOk "File of environment updated."
echo backup created in:
echo   %ENV_FILE%.backup
echo.
echo recreating containers for load a new configuration...
call :Preflight
if errorlevel 1 goto AUTH_DONE
call :EnsureDocker
if errorlevel 1 goto AUTH_DONE
call :Compose up -d --force-recreate
if errorlevel 1 (
    call :MsgError "File updated, mas os containers not foram recriados."
    goto AUTH_DONE
)
call :Log "Initial admin credentials changed automatically"
call :MsgOk "containers recriados."

:AUTH_DONE
echo.
call :DetectCompose
set "SHOW_USER="
for /F "tokens=1,* delims==" %%A in ('findstr /B /I "KOMODO_INIT_ADMIN_USERNAME=" "%ENV_FILE%" 2^>nul') do set "SHOW_USER=%%B"
echo %WHITE%User configured:%RESET% !SHOW_USER!
echo %WHITE%Password:%RESET% updated automatically in the compose.env
echo.
echo %YELLOW%in the file was aberto for editing manual.%RESET%
goto PAUSE_MAIN

:UPDATE_FROM_GITHUB
cls
call :Header
echo %GREEN%%BOLD%   [+] download update official of the GITHUB%RESET%
echo.
echo Repository official:
echo   https://github.with/moghtech/komodo
echo.
echo %YELLOW%is option works same if your folder veio of ZIP and not possui .git.%RESET%
echo it cria a backup before of copy a new versao.
echo.
choice /C SBC /N /M "Continue? [S] Yes  [B] Back  [C] Cancel: "
if errorlevel 3 goto MAIN
if errorlevel 2 goto MAIN

set "UPD_ROOT=%TEMP%\komodo_update_%RANDOM%%RANDOM%"
set "UPD_ZIP=%UPD_ROOT%\komodo-main.zip"
set "UPD_EXTRACT=%UPD_ROOT%\extract"
set "BACKUP_DIR=%KOMODO_ROOT%_backup_%RANDOM%%RANDOM%"

mkdir "%UPD_ROOT%" >nul 2>&1
mkdir "%UPD_EXTRACT%" >nul 2>&1

echo.
echo %CYAN%1/4 Baixando versao more recente...%RESET%
where curl >nul 2>&1
if errorlevel 1 (
    call :MsgError "curl.exe not found."
    goto UPDATE_CLEANUP
)
curl.exe -L --fail --silent --show-error "https://github.com/moghtech/komodo/archive/refs/heads/main.zip" -o "%UPD_ZIP%"
if errorlevel 1 (
    call :MsgError "Failed to download a update."
    goto UPDATE_CLEANUP
)

echo %CYAN%2/4 Extraindo pacote...%RESET%
where tar >nul 2>&1
if errorlevel 1 (
    call :MsgError "tar.exe not found."
    goto UPDATE_CLEANUP
)
tar -xf "%UPD_ZIP%" -C "%UPD_EXTRACT%"
if errorlevel 1 (
    call :MsgError "Failed to extrair o pacote."
    goto UPDATE_CLEANUP
)

if not exist "%UPD_EXTRACT%\komodo-main\" (
    call :MsgError "Estrutura inesperada in the pacote baixado."
    goto UPDATE_CLEANUP
)

echo %CYAN%3/4 creating backup of the versao current...%RESET%
robocopy "%KOMODO_ROOT%" "%BACKUP_DIR%" /E /XD ".komodo-windows" /XF "KOMODO_CONTROL_CENTER.bat" >nul
if errorlevel 8 (
    call :MsgError "Failed to create backup. update cancelled."
    goto UPDATE_CLEANUP
)

echo %CYAN%4/4 Aplicando files new...%RESET%
robocopy "%UPD_EXTRACT%\komodo-main" "%KOMODO_ROOT%" /E /XD ".git" ".komodo-windows" /XF "KOMODO_CONTROL_CENTER.bat" >nul
if errorlevel 8 (
    call :MsgError "Failure durante a copy of the files."
    echo backup disponivel in:
    echo   %BACKUP_DIR%
    goto UPDATE_CLEANUP
)

call :Log "Repository updated from official GitHub ZIP"
echo.
call :MsgOk "update official applied."
echo backup of the versao previous:
echo   %BACKUP_DIR%
echo.
echo %YELLOW%O launcher was preservado and not was sobrescrito.%RESET%
echo.
choice /C SBC /N /M "Update Docker images? [S] Yes  [B] Back  [C] Cancel: "
if errorlevel 3 goto UPDATE_CLEANUP
if errorlevel 2 goto UPDATE_CLEANUP
call :DetectCompose
call :Preflight
if errorlevel 1 goto UPDATE_CLEANUP
call :EnsureDocker
if errorlevel 1 goto UPDATE_CLEANUP
call :Compose pull
call :Compose up -d

:UPDATE_CLEANUP
rmdir /S /Q "%UPD_ROOT%" >nul 2>&1
goto PAUSE_MAIN



:LoadDbEnv
set "DB_USER="
set "DB_PASS="
set "DB_NAME=komodo"
if defined ENV_FILE (
    for /F "tokens=1,* delims==" %%A in ('findstr /B /I "KOMODO_DATABASE_USERNAME=" "%ENV_FILE%" 2^>nul') do set "DB_USER=%%B"
    for /F "tokens=1,* delims==" %%A in ('findstr /B /I "KOMODO_DATABASE_PASSWORD=" "%ENV_FILE%" 2^>nul') do set "DB_PASS=%%B"
    for /F "tokens=1,* delims==" %%A in ('findstr /B /I "KOMODO_DATABASE_DB_NAME=" "%ENV_FILE%" 2^>nul') do set "DB_NAME=%%B"
)
if not defined DB_USER set "DB_USER=admin"
if not defined DB_PASS set "DB_PASS=admin"
if not defined DB_NAME set "DB_NAME=komodo"
exit /b 0

:ListUsersRaw
call :LoadDbEnv
set "USERS_TMP=%TEMP%\komodo_users_%RANDOM%.txt"
call :Compose exec -T mongo mongosh --quiet -u "%DB_USER%" -p "%DB_PASS%" --authenticationDatabase admin "%DB_NAME%" --eval "db.getCollectionNames().forEach(function(n){db.getCollection(n).find({username:{$exists:true}},{username:1,enabled:1,admin:1,super_admin:1}).forEach(function(u){print(u.username+'|'+u.enabled+'|'+u.admin+'|'+u.super_admin);});});" > "%USERS_TMP%" 2>nul
set "USERS_RC=%errorlevel%"
if not "%USERS_RC%"=="0" (
    if exist "%USERS_TMP%" del /Q "%USERS_TMP%" >nul 2>&1
    exit /b %USERS_RC%
)

set "USER_COUNT=0"
for /F "usebackq tokens=1-4 delims=|" %%A in ("%USERS_TMP%") do (
    set /A USER_COUNT+=1 >nul
    set "U_NAME=%%A"
    set "U_ENABLED=%%B"
    set "U_ADMIN=%%C"
    set "U_SUPER=%%D"

    set "U_ROLE=MEMBRO"
    set "U_TAG=%TAG_MEMBER% [ MEMBRO ] %RESET%"
    if /I "%%C"=="true" (
        set "U_ROLE=ADMIN"
        set "U_TAG=%TAG_ADMIN% [ ADMIN ] %RESET%"
    )
    if /I "%%D"=="true" (
        set "U_ROLE=ADMIN"
        set "U_TAG=%TAG_ADMIN% [ ADMIN ] %RESET%"
    )

    echo   !U_TAG!  !U_NAME!   %GRAY%enabled=!U_ENABLED!%RESET%
)

if exist "%USERS_TMP%" del /Q "%USERS_TMP%" >nul 2>&1

if "%USER_COUNT%"=="0" (
    echo   %YELLOW%in the user found.%RESET%
)
exit /b 0

:ShowUsersPanel
call :Preflight >nul 2>&1
if errorlevel 1 exit /b 1
call :EnsureDocker >nul 2>&1
if errorlevel 1 exit /b 1
echo %CYAN%%BOLD%   users real of the KOMODO%RESET%
echo %GRAY%   ------------------------------------------------------------%RESET%
call :ListUsersRaw
set "SHOW_USERS_RC=%errorlevel%"
echo %GRAY%   ------------------------------------------------------------%RESET%
if not "%SHOW_USERS_RC%"=="0" (
    call :MsgError "Could not consultar os users real of the Komodo."
    exit /b 1
)
exit /b 0

:RESET_EXISTING_PASSWORD
@echo off
cls
call :Header
echo %GREEN%%BOLD%   [*] RESETAR password of user existing%RESET%
echo.
echo first vou consultar o database and show os users that REALMENTE existem.
echo.
call :Preflight
if errorlevel 1 goto PAUSE_MAIN
call :EnsureDocker
if errorlevel 1 goto PAUSE_MAIN
echo.
call :ShowUsersPanel
if errorlevel 1 goto PAUSE_MAIN

echo.
echo %YELLOW%important:%RESET% choose exatamente a username mostrado acima.
echo Digitar a nome that not aparece in the lista vai falhar.
echo.
setlocal DisableDelayedExpansion
set "RESET_USER="
set "RESET_PASS="
set /p "RESET_USER=User existing [admin] (B/C cancela): "
if /I "%RESET_USER%"=="B" (
    endlocal
    goto MAIN
)
if /I "%RESET_USER%"=="C" (
    endlocal
    goto MAIN
)
if not defined RESET_USER set "RESET_USER=admin"

set /p "RESET_PASS=New password (B/C cancela): "
if /I "%RESET_PASS%"=="B" (
    endlocal
    goto MAIN
)
if /I "%RESET_PASS%"=="C" (
    endlocal
    goto MAIN
)
if not defined RESET_PASS (
    endlocal
    call :MsgError "Password vazia. Operation cancelled."
    goto PAUSE_MAIN
)

echo.
echo User: %RESET_USER%
choice /C SBC /N /M "Confirmar redefinicao? [S] Yes  [B] Back  [C] Cancel: "
if errorlevel 3 (
    endlocal
    goto MAIN
)
if errorlevel 2 (
    endlocal
    goto MAIN
)

echo.
call :Compose exec -T core km set user "%RESET_USER%" password "%RESET_PASS%" -y
set "RESET_RC=%errorlevel%"

if not "%RESET_RC%"=="0" (
    echo.
    call :MsgError "O Komodo not conseguiu redefinir a password."
    echo O username informado probably not existe.
    endlocal
    goto PAUSE_MAIN
)

echo.
call :MsgOk "Password redefinida in the user existing."
echo User: %RESET_USER%
call :Log "Password reset for existing Komodo user"
endlocal
goto PAUSE_MAIN

:MAKE_SUPER_ADMIN
@echo off
cls
call :Header
echo %GREEN%%BOLD%   [+] TORNAR user SUPER ADMIN%RESET%
echo.
call :Preflight
if errorlevel 1 goto PAUSE_MAIN
call :EnsureDocker
if errorlevel 1 goto PAUSE_MAIN

call :ShowUsersPanel
if errorlevel 1 goto PAUSE_MAIN

echo.
echo %YELLOW%choose exatamente a username listado acima.%RESET%
setlocal DisableDelayedExpansion
set "ADMIN_TARGET="
set /p "ADMIN_TARGET=User existing [admin] (B/C cancela): "
if /I "%ADMIN_TARGET%"=="B" (
    endlocal
    goto MAIN
)
if /I "%ADMIN_TARGET%"=="C" (
    endlocal
    goto MAIN
)
if not defined ADMIN_TARGET set "ADMIN_TARGET=admin"

echo.
choice /C SBC /N /M "Confirmar promocao? [S] Yes  [B] Back  [C] Cancel: "
if errorlevel 3 (
    endlocal
    goto MAIN
)
if errorlevel 2 (
    endlocal
    goto MAIN
)

echo.
call :Compose exec -T core km set user "%ADMIN_TARGET%" super-admin true -y
set "ADMIN_RC=%errorlevel%"

if not "%ADMIN_RC%"=="0" (
    echo.
    call :MsgError "Could not elevar o user."
    echo O username informado needs existir in the database.
    endlocal
    goto PAUSE_MAIN
)

call :MsgOk "User now and Super Admin."
call :Log "Existing Komodo user elevated to Super Admin"
endlocal
goto PAUSE_MAIN

:LIST_REAL_USERS
@echo off
cls
call :Header
echo %GREEN%%BOLD%   [@] users real of the KOMODO%RESET%
echo.
call :Preflight
if errorlevel 1 goto PAUSE_MAIN
call :EnsureDocker
if errorlevel 1 goto PAUSE_MAIN
echo Consultando directly o MongoDB usado by the Komodo...
echo.
call :ShowUsersPanel
if errorlevel 1 (
    goto PAUSE_MAIN
)
echo.
echo %GREEN%use only usernames mostrados acima in the options 27 and 28.%RESET%
echo %YELLOW%if lowdrus NOT aparecer here, o registration ainda not was created.%RESET%
goto PAUSE_MAIN

:ENABLE_LOCAL_SIGNUP
@echo off
cls
call :Header
echo %GREEN%%BOLD%   [+] enable registration local%RESET%
echo.
call :ShowUsersPanel
if errorlevel 1 goto PAUSE_MAIN
echo.
echo.
echo is option habilita o botao Sign Up / registration local of the Komodo.
echo it NOT deletes users existing.
echo.
echo configurations that will be aplicadas:
echo   KOMODO_LOCAL_AUTH=true
echo   KOMODO_DISABLE_USER_REGISTRATION=false
echo   KOMODO_DISABLE_LOCAL_USER_REGISTRATION=false
echo   KOMODO_ENABLE_NEW_USERS=true
echo.
choice /C SBC /N /M "Aplicar? [S] Yes  [B] Back  [C] Cancel: "
if errorlevel 3 goto MAIN
if errorlevel 2 goto MAIN

if not defined ENV_FILE (
    call :MsgError "compose.env not found."
    goto PAUSE_MAIN
)

copy /Y "%ENV_FILE%" "%ENV_FILE%.signup-backup" >nul 2>&1

set "TMPENV=%TEMP%\komodo_env_%RANDOM%.tmp"
findstr /V /B /I "KOMODO_LOCAL_AUTH= KOMODO_DISABLE_USER_REGISTRATION= KOMODO_DISABLE_LOCAL_USER_REGISTRATION= KOMODO_ENABLE_NEW_USERS=" "%ENV_FILE%" > "%TMPENV%"
>>"%TMPENV%" echo KOMODO_LOCAL_AUTH=true
>>"%TMPENV%" echo KOMODO_DISABLE_USER_REGISTRATION=false
>>"%TMPENV%" echo KOMODO_DISABLE_LOCAL_USER_REGISTRATION=false
>>"%TMPENV%" echo KOMODO_ENABLE_NEW_USERS=true
copy /Y "%TMPENV%" "%ENV_FILE%" >nul
del /Q "%TMPENV%" >nul 2>&1

echo.
echo recreating o Core with registration local habilitado...
call :Compose up -d --force-recreate core
if errorlevel 1 (
    call :MsgError "Failed to recriar o Core."
    goto PAUSE_MAIN
)

call :WaitHttp 25
echo.
call :MsgOk "registration local habilitado."
echo.
echo now o dashboard must exibir a option of registration local.
echo after of create o user desejado, use:
echo   [29] for confirmar that ele existe
echo   [28] for torna-lo Super Admin
echo.
start "" "%DASHBOARD_URL%"
goto PAUSE_MAIN


:FindCoreContainer
set "CORE_ID="
for /F "usebackq delims=" %%I in (`docker ps --filter "label=com.docker.compose.project=%PROJECT_NAME%" --filter "label=com.docker.compose.service=core" --format "{{.ID}}"`) do (
    if not defined CORE_ID set "CORE_ID=%%I"
)
if not defined CORE_ID exit /b 1
exit /b 0

:BuildPtBrPayload
if not exist "%STATE_DIR%\ptbr" mkdir "%STATE_DIR%\ptbr" >nul 2>&1
set "PTBR_JS=%STATE_DIR%\ptbr\komodo-ptbr.js"
set "PTBR_VBS=%STATE_DIR%\ptbr\patch-index.vbs"
> "%PTBR_JS%.b64" echo KCgpID0+IHsKICAidXNlIHN0cmljdCI7CiAgaWYgKHdpbmRvdy5fX0tPTU9ET19QVEJSX0xPQURF
>>"%PTBR_JS%.b64" echo RF9fKSByZXR1cm47CiAgd2luZG93Ll9fS09NT0RPX1BUQlJfTE9BREVEX18gPSB0cnVlOwoKICBj
>>"%PTBR_JS%.b64" echo b25zdCBQVCA9IHsiTG9nIEluIjoiRW50cmFyIiwiTG9naW4iOiJFbnRyYXIiLCJMb2cgT3V0Ijoi
>>"%PTBR_JS%.b64" echo U2FpciIsIkxvZ291dCI6IlNhaXIiLCJTaWduIFVwIjoiQ3JpYXIgY29udGEiLCJVc2VybmFtZSI6
>>"%PTBR_JS%.b64" echo IlVzdcOhcmlvIiwiUGFzc3dvcmQiOiJTZW5oYSIsIkVudGVyIHVzZXJuYW1lIjoiRGlnaXRlIG8g
>>"%PTBR_JS%.b64" echo dXN1w6FyaW8iLCJFbnRlciBwYXNzd29yZCI6IkRpZ2l0ZSBhIHNlbmhhIiwiSW52YWxpZCBsb2dp
>>"%PTBR_JS%.b64" echo biBjcmVkZW50aWFscyI6IkNyZWRlbmNpYWlzIGRlIGxvZ2luIGludsOhbGlkYXMiLCJZb3UgaGF2
>>"%PTBR_JS%.b64" echo ZSI6IlZvY8OqIHRlbSIsImF0dGVtcHRzIHJlbWFpbmluZyI6InRlbnRhdGl2YXMgcmVzdGFudGVz
>>"%PTBR_JS%.b64" echo IiwiU2VlIGNvbnNvbGUgZm9yIGRldGFpbHMiOiJWZWphIG8gY29uc29sZSBwYXJhIGRldGFsaGVz
>>"%PTBR_JS%.b64" echo IiwiUmVtZW1iZXIgbWUiOiJMZW1icmFyIGRlIG1pbSIsIkF1dGhlbnRpY2F0aW9uIjoiQXV0ZW50
>>"%PTBR_JS%.b64" echo aWNhw6fDo28iLCJBY2Nlc3MgRGVuaWVkIjoiQWNlc3NvIG5lZ2FkbyIsIlVuYXV0aG9yaXplZCI6
>>"%PTBR_JS%.b64" echo Ik7Do28gYXV0b3JpemFkbyIsIlVzZXIiOiJVc3XDoXJpbyIsIlVzZXJzIjoiVXN1w6FyaW9zIiwi
>>"%PTBR_JS%.b64" echo TWVtYmVyIjoiTWVtYnJvIiwiTWVtYmVycyI6Ik1lbWJyb3MiLCJBZG1pbiI6IkFkbWluaXN0cmFk
>>"%PTBR_JS%.b64" echo b3IiLCJTdXBlciBBZG1pbiI6IlN1cGVyIEFkbWluaXN0cmFkb3IiLCJFbmFibGVkIjoiQXRpdmFk
>>"%PTBR_JS%.b64" echo byIsIkRpc2FibGVkIjoiRGVzYXRpdmFkbyIsIkVuYWJsZSI6IkF0aXZhciIsIkRpc2FibGUiOiJE
>>"%PTBR_JS%.b64" echo ZXNhdGl2YXIiLCJDcmVhdGUgVXNlciI6IkNyaWFyIHVzdcOhcmlvIiwiVXBkYXRlIFVzZXIiOiJB
>>"%PTBR_JS%.b64" echo dHVhbGl6YXIgdXN1w6FyaW8iLCJEZWxldGUgVXNlciI6IkV4Y2x1aXIgdXN1w6FyaW8iLCJVc2Vy
>>"%PTBR_JS%.b64" echo IFNldHRpbmdzIjoiQ29uZmlndXJhw6fDtWVzIGRvIHVzdcOhcmlvIiwiSG9tZSI6Ikluw61jaW8i
>>"%PTBR_JS%.b64" echo LCJEYXNoYm9hcmQiOiJQYWluZWwiLCJPdmVydmlldyI6IlZpc8OjbyBnZXJhbCIsIlNldHRpbmdz
>>"%PTBR_JS%.b64" echo IjoiQ29uZmlndXJhw6fDtWVzIiwiU3lzdGVtIjoiU2lzdGVtYSIsIlJlc291cmNlcyI6IlJlY3Vy
>>"%PTBR_JS%.b64" echo c29zIiwiUmVzb3VyY2UiOiJSZWN1cnNvIiwiU2VhcmNoIjoiUGVzcXVpc2FyIiwiRmlsdGVyIjoi
>>"%PTBR_JS%.b64" echo RmlsdHJhciIsIkZpbHRlcnMiOiJGaWx0cm9zIiwiQ2xlYXIiOiJMaW1wYXIiLCJDbGVhciBBbGwi
>>"%PTBR_JS%.b64" echo OiJMaW1wYXIgdHVkbyIsIlJlc2V0IjoiUmVkZWZpbmlyIiwiUmVmcmVzaCI6IkF0dWFsaXphciIs
>>"%PTBR_JS%.b64" echo IlJlbG9hZCI6IlJlY2FycmVnYXIiLCJTYXZlIjoiU2FsdmFyIiwiU2F2ZSBDaGFuZ2VzIjoiU2Fs
>>"%PTBR_JS%.b64" echo dmFyIGFsdGVyYcOnw7VlcyIsIlNhdmluZyI6IlNhbHZhbmRvIiwiQ2FuY2VsIjoiQ2FuY2VsYXIi
>>"%PTBR_JS%.b64" echo LCJDbG9zZSI6IkZlY2hhciIsIkJhY2siOiJWb2x0YXIiLCJOZXh0IjoiUHLDs3hpbW8iLCJQcmV2
>>"%PTBR_JS%.b64" echo aW91cyI6IkFudGVyaW9yIiwiQ29udGludWUiOiJDb250aW51YXIiLCJDb25maXJtIjoiQ29uZmly
>>"%PTBR_JS%.b64" echo bWFyIiwiU3VibWl0IjoiRW52aWFyIiwiQXBwbHkiOiJBcGxpY2FyIiwiRWRpdCI6IkVkaXRhciIs
>>"%PTBR_JS%.b64" echo IlVwZGF0ZSI6IkF0dWFsaXphciIsIkNyZWF0ZSI6IkNyaWFyIiwiQWRkIjoiQWRpY2lvbmFyIiwi
>>"%PTBR_JS%.b64" echo UmVtb3ZlIjoiUmVtb3ZlciIsIkRlbGV0ZSI6IkV4Y2x1aXIiLCJDbG9uZSI6IkNsb25hciIsIkNv
>>"%PTBR_JS%.b64" echo cHkiOiJDb3BpYXIiLCJDb3BpZWQiOiJDb3BpYWRvIiwiRG93bmxvYWQiOiJCYWl4YXIiLCJVcGxv
>>"%PTBR_JS%.b64" echo YWQiOiJFbnZpYXIiLCJJbXBvcnQiOiJJbXBvcnRhciIsIkV4cG9ydCI6IkV4cG9ydGFyIiwiT3Bl
>>"%PTBR_JS%.b64" echo biI6IkFicmlyIiwiVmlldyI6IlZpc3VhbGl6YXIiLCJEZXRhaWxzIjoiRGV0YWxoZXMiLCJBY3Rp
>>"%PTBR_JS%.b64" echo b25zIjoiQcOnw7VlcyIsIkFjdGlvbiI6IkHDp8OjbyIsIk1vcmUiOiJNYWlzIiwiTmFtZSI6Ik5v
>>"%PTBR_JS%.b64" echo bWUiLCJEZXNjcmlwdGlvbiI6IkRlc2NyacOnw6NvIiwiU3RhdHVzIjoiU3RhdHVzIiwiSW5mbyI6
>>"%PTBR_JS%.b64" echo IkluZm9ybWHDp8O1ZXMiLCJJbmZvcm1hdGlvbiI6IkluZm9ybWHDp8O1ZXMiLCJUeXBlIjoiVGlw
>>"%PTBR_JS%.b64" echo byIsIlZlcnNpb24iOiJWZXJzw6NvIiwiQ3JlYXRlZCI6IkNyaWFkbyIsIlVwZGF0ZWQiOiJBdHVh
>>"%PTBR_JS%.b64" echo bGl6YWRvIiwiQ3JlYXRlZCBBdCI6IkNyaWFkbyBlbSIsIlVwZGF0ZWQgQXQiOiJBdHVhbGl6YWRv
>>"%PTBR_JS%.b64" echo IGVtIiwiTGFzdCBVcGRhdGVkIjoiw5psdGltYSBhdHVhbGl6YcOnw6NvIiwiTGFzdCBTZWVuIjoi
>>"%PTBR_JS%.b64" echo VmlzdG8gcG9yIMO6bHRpbW8iLCJOZXZlciI6Ik51bmNhIiwiWWVzIjoiU2ltIiwiTm8iOiJOw6Nv
>>"%PTBR_JS%.b64" echo IiwiTm9uZSI6Ik5lbmh1bSIsIkFsbCI6IlRvZG9zIiwiU2VsZWN0IEFsbCI6IlNlbGVjaW9uYXIg
>>"%PTBR_JS%.b64" echo dG9kb3MiLCJMb2FkaW5nIjoiQ2FycmVnYW5kbyIsIkxvYWRpbmcuLi4iOiJDYXJyZWdhbmRvLi4u
>>"%PTBR_JS%.b64" echo IiwiUGxlYXNlIHdhaXQiOiJBZ3VhcmRlIiwiVW5rbm93biI6IkRlc2NvbmhlY2lkbyIsIlN1Y2Nl
>>"%PTBR_JS%.b64" echo c3MiOiJTdWNlc3NvIiwiRmFpbGVkIjoiRmFsaG91IiwiRmFpbHVyZSI6IkZhbGhhIiwiRXJyb3Ii
>>"%PTBR_JS%.b64" echo OiJFcnJvIiwiV2FybmluZyI6IkF2aXNvIiwiSGVhbHRoeSI6IlNhdWTDoXZlbCIsIlVuaGVhbHRo
>>"%PTBR_JS%.b64" echo eSI6Ik7Do28gc2F1ZMOhdmVsIiwiT25saW5lIjoiT25saW5lIiwiT2ZmbGluZSI6Ik9mZmxpbmUi
>>"%PTBR_JS%.b64" echo LCJSdW5uaW5nIjoiRW0gZXhlY3XDp8OjbyIsIlN0b3BwZWQiOiJQYXJhZG8iLCJTdG9wcGluZyI6
>>"%PTBR_JS%.b64" echo IlBhcmFuZG8iLCJTdGFydGluZyI6IkluaWNpYW5kbyIsIlN0YXJ0IjoiSW5pY2lhciIsIlN0b3Ai
>>"%PTBR_JS%.b64" echo OiJQYXJhciIsIlJlc3RhcnQiOiJSZWluaWNpYXIiLCJQYXVzZSI6IlBhdXNhciIsIlJlc3VtZSI6
>>"%PTBR_JS%.b64" echo IlJldG9tYXIiLCJRdWV1ZWQiOiJOYSBmaWxhIiwiUGVuZGluZyI6IlBlbmRlbnRlIiwiQ29tcGxl
>>"%PTBR_JS%.b64" echo dGUiOiJDb25jbHXDrWRvIiwiQ29tcGxldGVkIjoiQ29uY2x1w61kbyIsIkFjdGl2ZSI6IkF0aXZv
>>"%PTBR_JS%.b64" echo IiwiSW5hY3RpdmUiOiJJbmF0aXZvIiwiQXZhaWxhYmxlIjoiRGlzcG9uw612ZWwiLCJVbmF2YWls
>>"%PTBR_JS%.b64" echo YWJsZSI6IkluZGlzcG9uw612ZWwiLCJTZXJ2ZXJzIjoiU2Vydmlkb3JlcyIsIlNlcnZlciI6IlNl
>>"%PTBR_JS%.b64" echo cnZpZG9yIiwiU3RhY2tzIjoiU3RhY2tzIiwiU3RhY2siOiJTdGFjayIsIkRlcGxveW1lbnRzIjoi
>>"%PTBR_JS%.b64" echo SW1wbGFudGHDp8O1ZXMiLCJEZXBsb3ltZW50IjoiSW1wbGFudGHDp8OjbyIsIkJ1aWxkcyI6IkJ1
>>"%PTBR_JS%.b64" echo aWxkcyIsIkJ1aWxkIjoiQnVpbGQiLCJSZXBvcyI6IlJlcG9zaXTDs3Jpb3MiLCJSZXBvIjoiUmVw
>>"%PTBR_JS%.b64" echo b3NpdMOzcmlvIiwiUmVwb3NpdG9yaWVzIjoiUmVwb3NpdMOzcmlvcyIsIlJlcG9zaXRvcnkiOiJS
>>"%PTBR_JS%.b64" echo ZXBvc2l0w7NyaW8iLCJQcm9jZWR1cmVzIjoiUHJvY2VkaW1lbnRvcyIsIlByb2NlZHVyZSI6IlBy
>>"%PTBR_JS%.b64" echo b2NlZGltZW50byIsIlJlc291cmNlIFN5bmNzIjoiU2luY3Jvbml6YcOnw7VlcyBkZSByZWN1cnNv
>>"%PTBR_JS%.b64" echo cyIsIlJlc291cmNlIFN5bmMiOiJTaW5jcm9uaXphw6fDo28gZGUgcmVjdXJzb3MiLCJTeW5jcyI6
>>"%PTBR_JS%.b64" echo IlNpbmNyb25pemHDp8O1ZXMiLCJTeW5jIjoiU2luY3Jvbml6YXIiLCJCdWlsZGVycyI6IkNvbnN0
>>"%PTBR_JS%.b64" echo cnV0b3JlcyIsIkJ1aWxkZXIiOiJDb25zdHJ1dG9yIiwiU3dhcm1zIjoiU3dhcm1zIiwiU3dhcm0i
>>"%PTBR_JS%.b64" echo OiJTd2FybSIsIkFsZXJ0ZXJzIjoiQWxlcnRhcyIsIkFsZXJ0ZXIiOiJBbGVydGEiLCJBbGVydHMi
>>"%PTBR_JS%.b64" echo OiJBbGVydGFzIiwiQWxlcnQiOiJBbGVydGEiLCJWYXJpYWJsZXMiOiJWYXJpw6F2ZWlzIiwiVmFy
>>"%PTBR_JS%.b64" echo aWFibGUiOiJWYXJpw6F2ZWwiLCJUYWdzIjoiVGFncyIsIlRhZyI6IlRhZyIsIkFwaSBLZXlzIjoi
>>"%PTBR_JS%.b64" echo Q2hhdmVzIGRlIEFQSSIsIkFQSSBLZXlzIjoiQ2hhdmVzIGRlIEFQSSIsIkFQSSBLZXkiOiJDaGF2
>>"%PTBR_JS%.b64" echo ZSBkZSBBUEkiLCJQcm92aWRlcnMiOiJQcm92ZWRvcmVzIiwiUHJvdmlkZXIiOiJQcm92ZWRvciIs
>>"%PTBR_JS%.b64" echo IkdpdCBQcm92aWRlcnMiOiJQcm92ZWRvcmVzIEdpdCIsIkltYWdlIFJlZ2lzdHJpZXMiOiJSZWdp
>>"%PTBR_JS%.b64" echo c3Ryb3MgZGUgaW1hZ2VucyIsIlJlZ2lzdHJ5IjoiUmVnaXN0cm8iLCJSZWdpc3RyaWVzIjoiUmVn
>>"%PTBR_JS%.b64" echo aXN0cm9zIiwiU2VydmVyIEluZm8iOiJJbmZvcm1hw6fDtWVzIGRvIHNlcnZpZG9yIiwiU2VydmVy
>>"%PTBR_JS%.b64" echo IFN0YXRzIjoiRXN0YXTDrXN0aWNhcyBkbyBzZXJ2aWRvciIsIlN5c3RlbSBTdGF0cyI6IkVzdGF0
>>"%PTBR_JS%.b64" echo w61zdGljYXMgZG8gc2lzdGVtYSIsIkNQVSI6IkNQVSIsIk1lbW9yeSI6Ik1lbcOzcmlhIiwiRGlz
>>"%PTBR_JS%.b64" echo ayI6IkRpc2NvIiwiTmV0d29yayI6IlJlZGUiLCJOZXR3b3JrcyI6IlJlZGVzIiwiUHJvY2Vzc2Vz
>>"%PTBR_JS%.b64" echo IjoiUHJvY2Vzc29zIiwiUHJvY2VzcyI6IlByb2Nlc3NvIiwiQ29udGFpbmVycyI6IkNvbnRhaW5l
>>"%PTBR_JS%.b64" echo cnMiLCJDb250YWluZXIiOiJDb250YWluZXIiLCJJbWFnZXMiOiJJbWFnZW5zIiwiSW1hZ2UiOiJJ
>>"%PTBR_JS%.b64" echo bWFnZW0iLCJWb2x1bWVzIjoiVm9sdW1lcyIsIlZvbHVtZSI6IlZvbHVtZSIsIlBvcnRzIjoiUG9y
>>"%PTBR_JS%.b64" echo dGFzIiwiUG9ydCI6IlBvcnRhIiwiSG9zdCI6Ikhvc3QiLCJBZGRyZXNzIjoiRW5kZXJlw6dvIiwi
>>"%PTBR_JS%.b64" echo UmVnaW9uIjoiUmVnacOjbyIsIkNvbm5lY3RlZCI6IkNvbmVjdGFkbyIsIkRpc2Nvbm5lY3RlZCI6
>>"%PTBR_JS%.b64" echo IkRlc2NvbmVjdGFkbyIsIkNvbm5lY3QiOiJDb25lY3RhciIsIkNvbm5lY3Rpb24iOiJDb25leMOj
>>"%PTBR_JS%.b64" echo byIsIlVwdGltZSI6IlRlbXBvIGF0aXZvIiwiSG9zdG5hbWUiOiJOb21lIGRvIGhvc3QiLCJEZXBs
>>"%PTBR_JS%.b64" echo b3kiOiJJbXBsYW50YXIiLCJEZXBsb3kgU3RhY2siOiJJbXBsYW50YXIgU3RhY2siLCJSZWRlcGxv
>>"%PTBR_JS%.b64" echo eSI6IlJlaW1wbGFudGFyIiwiUmVkZXBsb3kgU3RhY2siOiJSZWltcGxhbnRhciBTdGFjayIsIlB1
>>"%PTBR_JS%.b64" echo bGwiOiJCYWl4YXIiLCJQdWxsIEltYWdlcyI6IkJhaXhhciBpbWFnZW5zIiwiRG93biI6IkRlcnJ1
>>"%PTBR_JS%.b64" echo YmFyIiwiRGVzdHJveSI6IkRlc3RydWlyIiwiU2VydmljZXMiOiJTZXJ2acOnb3MiLCJTZXJ2aWNl
>>"%PTBR_JS%.b64" echo IjoiU2VydmnDp28iLCJDb21wb3NlIjoiQ29tcG9zZSIsIkNvbXBvc2UgRmlsZSI6IkFycXVpdm8g
>>"%PTBR_JS%.b64" echo Q29tcG9zZSIsIkVudmlyb25tZW50IjoiQW1iaWVudGUiLCJFbnZpcm9ubWVudCBWYXJpYWJsZXMi
>>"%PTBR_JS%.b64" echo OiJWYXJpw6F2ZWlzIGRlIGFtYmllbnRlIiwiRW52aXJvbm1lbnQgRmlsZSI6IkFycXVpdm8gZGUg
>>"%PTBR_JS%.b64" echo YW1iaWVudGUiLCJDb25maWciOiJDb25maWd1cmHDp8OjbyIsIkNvbmZpZ3VyYXRpb24iOiJDb25m
>>"%PTBR_JS%.b64" echo aWd1cmHDp8OjbyIsIkNvbmZpZ3MiOiJDb25maWd1cmHDp8O1ZXMiLCJGaWxlcyI6IkFycXVpdm9z
>>"%PTBR_JS%.b64" echo IiwiRmlsZSI6IkFycXVpdm8iLCJQYXRoIjoiQ2FtaW5obyIsIlBhdGhzIjoiQ2FtaW5ob3MiLCJQ
>>"%PTBR_JS%.b64" echo cmUgRGVwbG95IjoiUHLDqS1pbXBsYW50YcOnw6NvIiwiUG9zdCBEZXBsb3kiOiJQw7NzLWltcGxh
>>"%PTBR_JS%.b64" echo bnRhw6fDo28iLCJCcmFuY2giOiJCcmFuY2giLCJCcmFuY2hlcyI6IkJyYW5jaGVzIiwiQ29tbWl0
>>"%PTBR_JS%.b64" echo IjoiQ29tbWl0IiwiQ29tbWl0cyI6IkNvbW1pdHMiLCJMYXRlc3QgQ29tbWl0Ijoiw5psdGltbyBj
>>"%PTBR_JS%.b64" echo b21taXQiLCJHaXQgQWNjb3VudCI6IkNvbnRhIEdpdCIsIkdpdCBQcm92aWRlciI6IlByb3ZlZG9y
>>"%PTBR_JS%.b64" echo IEdpdCIsIkNsb25lIFJlcG8iOiJDbG9uYXIgcmVwb3NpdMOzcmlvIiwiUHVsbCBSZXBvIjoiQXR1
>>"%PTBR_JS%.b64" echo YWxpemFyIHJlcG9zaXTDs3JpbyIsIkJ1aWxkIEltYWdlIjoiQ29uc3RydWlyIGltYWdlbSIsIkJ1
>>"%PTBR_JS%.b64" echo aWxkIEFyZ3MiOiJBcmd1bWVudG9zIGRlIGJ1aWxkIiwiRG9ja2VyZmlsZSI6IkRvY2tlcmZpbGUi
>>"%PTBR_JS%.b64" echo LCJDb250ZXh0IjoiQ29udGV4dG8iLCJCdWlsZCBDb250ZXh0IjoiQ29udGV4dG8gZGUgYnVpbGQi
>>"%PTBR_JS%.b64" echo LCJCdWlsZCBQYXRoIjoiQ2FtaW5obyBkZSBidWlsZCIsIkJ1aWxkIEhpc3RvcnkiOiJIaXN0w7Ny
>>"%PTBR_JS%.b64" echo aWNvIGRlIGJ1aWxkcyIsIkJ1aWxkIExvZ3MiOiJMb2dzIGRlIGJ1aWxkIiwiVHJpZ2dlciBCdWls
>>"%PTBR_JS%.b64" echo ZCI6IkV4ZWN1dGFyIGJ1aWxkIiwiRXhlY3V0aW9ucyI6IkV4ZWN1w6fDtWVzIiwiRXhlY3V0aW9u
>>"%PTBR_JS%.b64" echo IjoiRXhlY3XDp8OjbyIsIlJ1biI6IkV4ZWN1dGFyIiwiUnVuIE5vdyI6IkV4ZWN1dGFyIGFnb3Jh
>>"%PTBR_JS%.b64" echo IiwiRXhlY3V0ZSI6IkV4ZWN1dGFyIiwiRXhlY3V0aW9uIEhpc3RvcnkiOiJIaXN0w7NyaWNvIGRl
>>"%PTBR_JS%.b64" echo IGV4ZWN1w6fDtWVzIiwiSGlzdG9yeSI6Ikhpc3TDs3JpY28iLCJMb2dzIjoiTG9ncyIsIkxvZyI6
>>"%PTBR_JS%.b64" echo IkxvZyIsIkxpdmUgTG9ncyI6IkxvZ3MgYW8gdml2byIsIkNvbnNvbGUiOiJDb25zb2xlIiwiVGVy
>>"%PTBR_JS%.b64" echo bWluYWwiOiJUZXJtaW5hbCIsIlNoZWxsIjoiU2hlbGwiLCJDb21tYW5kIjoiQ29tYW5kbyIsIkNv
>>"%PTBR_JS%.b64" echo bW1hbmRzIjoiQ29tYW5kb3MiLCJPdXRwdXQiOiJTYcOtZGEiLCJTdGFydGVkIjoiSW5pY2lhZG8i
>>"%PTBR_JS%.b64" echo LCJGaW5pc2hlZCI6IkZpbmFsaXphZG8iLCJEdXJhdGlvbiI6IkR1cmHDp8OjbyIsIk1lc3NhZ2Ui
>>"%PTBR_JS%.b64" echo OiJNZW5zYWdlbSIsIlJ1biBQcm9jZWR1cmUiOiJFeGVjdXRhciBwcm9jZWRpbWVudG8iLCJSdW4g
>>"%PTBR_JS%.b64" echo QWN0aW9uIjoiRXhlY3V0YXIgYcOnw6NvIiwiRXhlY3V0aW9uIFR5cGUiOiJUaXBvIGRlIGV4ZWN1
>>"%PTBR_JS%.b64" echo w6fDo28iLCJTdGFnZSI6IkV0YXBhIiwiU3RhZ2VzIjoiRXRhcGFzIiwiRW5hYmxlZCBFeGVjdXRp
>>"%PTBR_JS%.b64" echo b25zIjoiRXhlY3XDp8O1ZXMgaGFiaWxpdGFkYXMiLCJBZGQgU3RhZ2UiOiJBZGljaW9uYXIgZXRh
>>"%PTBR_JS%.b64" echo cGEiLCJBZGQgRXhlY3V0aW9uIjoiQWRpY2lvbmFyIGV4ZWN1w6fDo28iLCJTeW5jIFJlc291cmNl
>>"%PTBR_JS%.b64" echo cyI6IlNpbmNyb25pemFyIHJlY3Vyc29zIiwiU3luYyBSZXNvdXJjZXMgTm93IjoiU2luY3Jvbml6
>>"%PTBR_JS%.b64" echo YXIgcmVjdXJzb3MgYWdvcmEiLCJQZW5kaW5nIFVwZGF0ZXMiOiJBdHVhbGl6YcOnw7VlcyBwZW5k
>>"%PTBR_JS%.b64" echo ZW50ZXMiLCJNYW5hZ2VkIFJlc291cmNlcyI6IlJlY3Vyc29zIGdlcmVuY2lhZG9zIiwiUmVzb3Vy
>>"%PTBR_JS%.b64" echo Y2UgUGF0aCI6IkNhbWluaG8gZG8gcmVjdXJzbyIsIlN5bmMgRmlsZSI6IkFycXVpdm8gZGUgc2lu
>>"%PTBR_JS%.b64" echo Y3Jvbml6YcOnw6NvIiwiTm90aWZpY2F0aW9ucyI6Ik5vdGlmaWNhw6fDtWVzIiwiTm90aWZpY2F0
>>"%PTBR_JS%.b64" echo aW9uIjoiTm90aWZpY2HDp8OjbyIsIkRpc21pc3MiOiJEaXNwZW5zYXIiLCJNYXJrIGFzIHJlYWQi
>>"%PTBR_JS%.b64" echo OiJNYXJjYXIgY29tbyBsaWRvIiwiTWFyayBhbGwgYXMgcmVhZCI6Ik1hcmNhciB0dWRvIGNvbW8g
>>"%PTBR_JS%.b64" echo bGlkbyIsIkdlbmVyYWwiOiJHZXJhbCIsIlNlY3VyaXR5IjoiU2VndXJhbsOnYSIsIkFjY291bnQi
>>"%PTBR_JS%.b64" echo OiJDb250YSIsIlByb2ZpbGUiOiJQZXJmaWwiLCJQcmVmZXJlbmNlcyI6IlByZWZlcsOqbmNpYXMi
>>"%PTBR_JS%.b64" echo LCJUaGVtZSI6IlRlbWEiLCJEYXJrIjoiRXNjdXJvIiwiTGlnaHQiOiJDbGFybyIsIlN5c3RlbSBU
>>"%PTBR_JS%.b64" echo aGVtZSI6IlRlbWEgZG8gc2lzdGVtYSIsIkxhbmd1YWdlIjoiSWRpb21hIiwiQXBwZWFyYW5jZSI6
>>"%PTBR_JS%.b64" echo IkFwYXLDqm5jaWEiLCJQZXJtaXNzaW9ucyI6IlBlcm1pc3PDtWVzIiwiUGVybWlzc2lvbiI6IlBl
>>"%PTBR_JS%.b64" echo cm1pc3PDo28iLCJSZWFkIjoiTGVpdHVyYSIsIldyaXRlIjoiRXNjcml0YSIsIkZ1bGwiOiJDb21w
>>"%PTBR_JS%.b64" echo bGV0byIsIlJvbGUiOiJGdW7Dp8OjbyIsIlJvbGVzIjoiRnVuw6fDtWVzIiwiVXNlciBHcm91cHMi
>>"%PTBR_JS%.b64" echo OiJHcnVwb3MgZGUgdXN1w6FyaW9zIiwiVXNlciBHcm91cCI6IkdydXBvIGRlIHVzdcOhcmlvcyIs
>>"%PTBR_JS%.b64" echo Ik5ldyBVc2VyIjoiTm92byB1c3XDoXJpbyIsIk5ldyBVc2VycyI6Ik5vdm9zIHVzdcOhcmlvcyIs
>>"%PTBR_JS%.b64" echo IkNoYW5nZSBQYXNzd29yZCI6IkFsdGVyYXIgc2VuaGEiLCJDdXJyZW50IFBhc3N3b3JkIjoiU2Vu
>>"%PTBR_JS%.b64" echo aGEgYXR1YWwiLCJOZXcgUGFzc3dvcmQiOiJOb3ZhIHNlbmhhIiwiQ29uZmlybSBQYXNzd29yZCI6
>>"%PTBR_JS%.b64" echo IkNvbmZpcm1hciBzZW5oYSIsIlR3byBGYWN0b3IgQXV0aGVudGljYXRpb24iOiJBdXRlbnRpY2HD
>>"%PTBR_JS%.b64" echo p8OjbyBkZSBkb2lzIGZhdG9yZXMiLCJUd28tRmFjdG9yIEF1dGhlbnRpY2F0aW9uIjoiQXV0ZW50
>>"%PTBR_JS%.b64" echo aWNhw6fDo28gZGUgZG9pcyBmYXRvcmVzIiwiUGFzc2tleSI6IkNoYXZlIGRlIGFjZXNzbyIsIkFQ
>>"%PTBR_JS%.b64" echo SSI6IkFQSSIsIldlYmhvb2siOiJXZWJob29rIiwiV2ViaG9va3MiOiJXZWJob29rcyIsIlNlY3Jl
>>"%PTBR_JS%.b64" echo dCI6IlNlZ3JlZG8iLCJUb2tlbiI6IlRva2VuIiwiQ3JlYXRlIFNlcnZlciI6IkNyaWFyIHNlcnZp
>>"%PTBR_JS%.b64" echo ZG9yIiwiQ3JlYXRlIFN0YWNrIjoiQ3JpYXIgc3RhY2siLCJDcmVhdGUgRGVwbG95bWVudCI6IkNy
>>"%PTBR_JS%.b64" echo aWFyIGltcGxhbnRhw6fDo28iLCJDcmVhdGUgQnVpbGQiOiJDcmlhciBidWlsZCIsIkNyZWF0ZSBS
>>"%PTBR_JS%.b64" echo ZXBvIjoiQ3JpYXIgcmVwb3NpdMOzcmlvIiwiQ3JlYXRlIFByb2NlZHVyZSI6IkNyaWFyIHByb2Nl
>>"%PTBR_JS%.b64" echo ZGltZW50byIsIkNyZWF0ZSBBY3Rpb24iOiJDcmlhciBhw6fDo28iLCJDcmVhdGUgUmVzb3VyY2Ug
>>"%PTBR_JS%.b64" echo U3luYyI6IkNyaWFyIHNpbmNyb25pemHDp8OjbyBkZSByZWN1cnNvcyIsIkNyZWF0ZSBCdWlsZGVy
>>"%PTBR_JS%.b64" echo IjoiQ3JpYXIgY29uc3RydXRvciIsIkNyZWF0ZSBBbGVydGVyIjoiQ3JpYXIgYWxlcnRhIiwiQ3Jl
>>"%PTBR_JS%.b64" echo YXRlIFZhcmlhYmxlIjoiQ3JpYXIgdmFyacOhdmVsIiwiQ3JlYXRlIFRhZyI6IkNyaWFyIHRhZyIs
>>"%PTBR_JS%.b64" echo Ik5ldyBTZXJ2ZXIiOiJOb3ZvIHNlcnZpZG9yIiwiTmV3IFN0YWNrIjoiTm92YSBzdGFjayIsIk5l
>>"%PTBR_JS%.b64" echo dyBEZXBsb3ltZW50IjoiTm92YSBpbXBsYW50YcOnw6NvIiwiTmV3IEJ1aWxkIjoiTm92byBidWls
>>"%PTBR_JS%.b64" echo ZCIsIk5ldyBSZXBvIjoiTm92byByZXBvc2l0w7NyaW8iLCJOZXcgUHJvY2VkdXJlIjoiTm92byBw
>>"%PTBR_JS%.b64" echo cm9jZWRpbWVudG8iLCJOZXcgQWN0aW9uIjoiTm92YSBhw6fDo28iLCJSZXF1aXJlZCI6Ik9icmln
>>"%PTBR_JS%.b64" echo YXTDs3JpbyIsIk9wdGlvbmFsIjoiT3BjaW9uYWwiLCJEZWZhdWx0IjoiUGFkcsOjbyIsIkFkdmFu
>>"%PTBR_JS%.b64" echo Y2VkIjoiQXZhbsOnYWRvIiwiQmFzaWMiOiJCw6FzaWNvIiwiU291cmNlIjoiT3JpZ2VtIiwiVGFy
>>"%PTBR_JS%.b64" echo Z2V0IjoiRGVzdGlubyIsIkRlc3RpbmF0aW9uIjoiRGVzdGlubyIsIk1vZGUiOiJNb2RvIiwiTG9j
>>"%PTBR_JS%.b64" echo YWwiOiJMb2NhbCIsIlJlbW90ZSI6IlJlbW90byIsIlB1YmxpYyI6IlDDumJsaWNvIiwiUHJpdmF0
>>"%PTBR_JS%.b64" echo ZSI6IlByaXZhZG8iLCJBdXRvIjoiQXV0b23DoXRpY28iLCJBdXRvbWF0aWMiOiJBdXRvbcOhdGlj
>>"%PTBR_JS%.b64" echo byIsIk1hbnVhbCI6Ik1hbnVhbCIsIlNjaGVkdWxlIjoiQWdlbmRhbWVudG8iLCJTY2hlZHVsZSBG
>>"%PTBR_JS%.b64" echo b3JtYXQiOiJGb3JtYXRvIGRvIGFnZW5kYW1lbnRvIiwiVGltZXpvbmUiOiJGdXNvIGhvcsOhcmlv
>>"%PTBR_JS%.b64" echo IiwiRW5hYmxlZCBieSBkZWZhdWx0IjoiQXRpdmFkbyBwb3IgcGFkcsOjbyIsIlRpbWVvdXQiOiJU
>>"%PTBR_JS%.b64" echo ZW1wbyBsaW1pdGUiLCJSZXRyaWVzIjoiVGVudGF0aXZhcyIsIlJldHJ5IjoiVGVudGFyIG5vdmFt
>>"%PTBR_JS%.b64" echo ZW50ZSIsIkFyZSB5b3Ugc3VyZT8iOiJUZW0gY2VydGV6YT8iLCJBcmUgeW91IHN1cmUgeW91IHdh
>>"%PTBR_JS%.b64" echo bnQgdG8gZGVsZXRlIHRoaXMgcmVzb3VyY2U/IjoiVGVtIGNlcnRlemEgZGUgcXVlIGRlc2VqYSBl
>>"%PTBR_JS%.b64" echo eGNsdWlyIGVzdGUgcmVjdXJzbz8iLCJUaGlzIGFjdGlvbiBjYW5ub3QgYmUgdW5kb25lLiI6IkVz
>>"%PTBR_JS%.b64" echo dGEgYcOnw6NvIG7Do28gcG9kZSBzZXIgZGVzZmVpdGEuIiwiQ29uZmlybSBEZWxldGUiOiJDb25m
>>"%PTBR_JS%.b64" echo aXJtYXIgZXhjbHVzw6NvIiwiRGVsZXRlIFJlc291cmNlIjoiRXhjbHVpciByZWN1cnNvIiwiQ29u
>>"%PTBR_JS%.b64" echo ZmlybSBBY3Rpb24iOiJDb25maXJtYXIgYcOnw6NvIiwiQ29uZmlybSBEZXBsb3ltZW50IjoiQ29u
>>"%PTBR_JS%.b64" echo ZmlybWFyIGltcGxhbnRhw6fDo28iLCJObyBkYXRhIjoiU2VtIGRhZG9zIiwiTm8gcmVzdWx0cyI6
>>"%PTBR_JS%.b64" echo Ik5lbmh1bSByZXN1bHRhZG8iLCJObyByZXNvdXJjZXMgZm91bmQiOiJOZW5odW0gcmVjdXJzbyBl
>>"%PTBR_JS%.b64" echo bmNvbnRyYWRvIiwiUm93cyBwZXIgcGFnZSI6IkxpbmhhcyBwb3IgcMOhZ2luYSIsIlBhZ2UiOiJQ
>>"%PTBR_JS%.b64" echo w6FnaW5hIiwib2YiOiJkZSIsIlNob3dpbmciOiJNb3N0cmFuZG8iLCJTb3J0IjoiT3JkZW5hciIs
>>"%PTBR_JS%.b64" echo IkFzY2VuZGluZyI6IkNyZXNjZW50ZSIsIkRlc2NlbmRpbmciOiJEZWNyZXNjZW50ZSIsIkRvY3Mi
>>"%PTBR_JS%.b64" echo OiJEb2N1bWVudGHDp8OjbyIsIkRvY3VtZW50YXRpb24iOiJEb2N1bWVudGHDp8OjbyIsIkhlbHAi
>>"%PTBR_JS%.b64" echo OiJBanVkYSIsIkFib3V0IjoiU29icmUiLCJDaGFuZ2Vsb2ciOiJSZWdpc3RybyBkZSBhbHRlcmHD
>>"%PTBR_JS%.b64" echo p8O1ZXMiLCJVcGRhdGVzIjoiQXR1YWxpemHDp8O1ZXMiLCJVcGRhdGUgQXZhaWxhYmxlIjoiQXR1
>>"%PTBR_JS%.b64" echo YWxpemHDp8OjbyBkaXNwb27DrXZlbCIsIkNvcmUiOiJDb3JlIiwiUGVyaXBoZXJ5IjoiUGVyaXBo
>>"%PTBR_JS%.b64" echo ZXJ5IiwiRGF0YWJhc2UiOiJCYW5jbyBkZSBkYWRvcyIsIkJhY2t1cHMiOiJCYWNrdXBzIiwiQmFj
>>"%PTBR_JS%.b64" echo a3VwIjoiQmFja3VwIiwiUmVzdG9yZSI6IlJlc3RhdXJhciIsIlJlc3RvcmUgQmFja3VwIjoiUmVz
>>"%PTBR_JS%.b64" echo dGF1cmFyIGJhY2t1cCIsIkRhdGFiYXNlIEJhY2t1cCI6IkJhY2t1cCBkbyBiYW5jbyBkZSBkYWRv
>>"%PTBR_JS%.b64" echo cyIsIkNvcHkgRGF0YWJhc2UiOiJDb3BpYXIgYmFuY28gZGUgZGFkb3MiLCJNYWludGVuYW5jZSI6
>>"%PTBR_JS%.b64" echo Ik1hbnV0ZW7Dp8OjbyIsIldlYmhvb2sgU2VjcmV0IjoiU2VncmVkbyBkbyB3ZWJob29rIiwiSldU
>>"%PTBR_JS%.b64" echo IFNlY3JldCI6IlNlZ3JlZG8gSldUIiwiQ29weSB0byBjbGlwYm9hcmQiOiJDb3BpYXIgcGFyYSBh
>>"%PTBR_JS%.b64" echo IMOhcmVhIGRlIHRyYW5zZmVyw6puY2lhIiwiT3BlbiBpbiBuZXcgdGFiIjoiQWJyaXIgZW0gbm92
>>"%PTBR_JS%.b64" echo YSBhYmEiLCJFeHRlcm5hbCBMaW5rIjoiTGluayBleHRlcm5vIiwiRXhwYW5kIjoiRXhwYW5kaXIi
>>"%PTBR_JS%.b64" echo LCJDb2xsYXBzZSI6IlJlY29saGVyIiwiU2hvdyI6Ik1vc3RyYXIiLCJIaWRlIjoiT2N1bHRhciIs
>>"%PTBR_JS%.b64" echo IlNob3cgUGFzc3dvcmQiOiJNb3N0cmFyIHNlbmhhIiwiSGlkZSBQYXNzd29yZCI6Ik9jdWx0YXIg
>>"%PTBR_JS%.b64" echo c2VuaGEifTsKICBjb25zdCBQQVRURVJOUyA9IFtbIlxcYkNyZWF0ZSBOZXdcXGIiLCJDcmlhciBu
>>"%PTBR_JS%.b64" echo b3ZvIl0sWyJcXGJBZGQgTmV3XFxiIiwiQWRpY2lvbmFyIG5vdm8iXSxbIlxcYkxhc3QgdXBkYXRl
>>"%PTBR_JS%.b64" echo ZFxcYiIsIsOabHRpbWEgYXR1YWxpemHDp8OjbyJdLFsiXFxiTm8gKFtBLVphLXogXSspIGZvdW5k
>>"%PTBR_JS%.b64" echo XFxiIiwiTmVuaHVtIFxcMSBlbmNvbnRyYWRvIl0sWyJcXGJGYWlsZWQgdG9cXGIiLCJGYWxoYSBh
>>"%PTBR_JS%.b64" echo byJdLFsiXFxiVW5hYmxlIHRvXFxiIiwiTsOjbyBmb2kgcG9zc8OtdmVsIl0sWyJcXGJTdWNjZXNz
>>"%PTBR_JS%.b64" echo ZnVsbHlcXGIiLCJDb20gc3VjZXNzbyJdLFsiXFxiQ2xpY2sgdG9cXGIiLCJDbGlxdWUgcGFyYSJd
>>"%PTBR_JS%.b64" echo LFsiXFxiU2VsZWN0IGFcXGIiLCJTZWxlY2lvbmUgdW0iXSxbIlxcYlNlbGVjdCBhblxcYiIsIlNl
>>"%PTBR_JS%.b64" echo bGVjaW9uZSB1bSJdLFsiXFxiU2VsZWN0XFxiIiwiU2VsZWNpb25hciJdLFsiXFxiRW50ZXIgYVxc
>>"%PTBR_JS%.b64" echo YiIsIkRpZ2l0ZSB1bSJdLFsiXFxiRW50ZXJcXGIiLCJEaWdpdGUiXSxbIlxcYkNob29zZVxcYiIs
>>"%PTBR_JS%.b64" echo IkVzY29saGVyIl0sWyJcXGJDdXJyZW50XFxiIiwiQXR1YWwiXSxbIlxcYkxhdGVzdFxcYiIsIk1h
>>"%PTBR_JS%.b64" echo aXMgcmVjZW50ZSJdXS5tYXAoKFtwLCByXSkgPT4gW25ldyBSZWdFeHAocCwgImdpIiksIHJdKTsK
>>"%PTBR_JS%.b64" echo ICBjb25zdCBBVFRSUyA9IFsicGxhY2Vob2xkZXIiLCAidGl0bGUiLCAiYXJpYS1sYWJlbCIsICJh
>>"%PTBR_JS%.b64" echo bHQiXTsKICBjb25zdCBTS0lQID0gbmV3IFNldChbIlNDUklQVCIsIlNUWUxFIiwiQ09ERSIsIlBS
>>"%PTBR_JS%.b64" echo RSIsIlRFWFRBUkVBIiwiTk9TQ1JJUFQiLCJTVkciLCJQQVRIIl0pOwogIGNvbnN0IHByb2Nlc3Nl
>>"%PTBR_JS%.b64" echo ZCA9IG5ldyBXZWFrTWFwKCk7CgogIGZ1bmN0aW9uIGNsZWFuKHMpIHsKICAgIHJldHVybiBTdHJp
>>"%PTBR_JS%.b64" echo bmcocyA/PyAiIikucmVwbGFjZSgvXHMrL2csICIgIikudHJpbSgpOwogIH0KCiAgZnVuY3Rpb24g
>>"%PTBR_JS%.b64" echo cHJlc2VydmVXaGl0ZXNwYWNlKG9yaWdpbmFsLCB0cmFuc2xhdGVkKSB7CiAgICBjb25zdCBsZWFk
>>"%PTBR_JS%.b64" echo ID0gb3JpZ2luYWwubWF0Y2goL15ccyovKT8uWzBdIHx8ICIiOwogICAgY29uc3QgdGFpbCA9IG9y
>>"%PTBR_JS%.b64" echo aWdpbmFsLm1hdGNoKC9ccyokLyk/LlswXSB8fCAiIjsKICAgIHJldHVybiBsZWFkICsgdHJhbnNs
>>"%PTBR_JS%.b64" echo YXRlZCArIHRhaWw7CiAgfQoKICBmdW5jdGlvbiB0cmFuc2xhdGVTdHJpbmcocmF3KSB7CiAgICBp
>>"%PTBR_JS%.b64" echo ZiAoIXJhdyB8fCB0eXBlb2YgcmF3ICE9PSAic3RyaW5nIikgcmV0dXJuIHJhdzsKICAgIGNvbnN0
>>"%PTBR_JS%.b64" echo IHRyaW1tZWQgPSBjbGVhbihyYXcpOwogICAgaWYgKCF0cmltbWVkKSByZXR1cm4gcmF3OwoKICAg
>>"%PTBR_JS%.b64" echo IGlmIChPYmplY3QucHJvdG90eXBlLmhhc093blByb3BlcnR5LmNhbGwoUFQsIHRyaW1tZWQpKSB7
>>"%PTBR_JS%.b64" echo CiAgICAgIHJldHVybiBwcmVzZXJ2ZVdoaXRlc3BhY2UocmF3LCBQVFt0cmltbWVkXSk7CiAgICB9
>>"%PTBR_JS%.b64" echo CgogICAgLy8gSGFuZGxlIHB1bmN0dWF0aW9uIHZhcmlhbnRzIHNhZmVseS4KICAgIGNvbnN0IGNv
>>"%PTBR_JS%.b64" echo cmUgPSB0cmltbWVkLnJlcGxhY2UoL1suOiE/XSskLywgIiIpOwogICAgY29uc3Qgc3VmZml4ID0g
>>"%PTBR_JS%.b64" echo dHJpbW1lZC5zbGljZShjb3JlLmxlbmd0aCk7CiAgICBpZiAoT2JqZWN0LnByb3RvdHlwZS5oYXNP
>>"%PTBR_JS%.b64" echo d25Qcm9wZXJ0eS5jYWxsKFBULCBjb3JlKSkgewogICAgICByZXR1cm4gcHJlc2VydmVXaGl0ZXNw
>>"%PTBR_JS%.b64" echo YWNlKHJhdywgUFRbY29yZV0gKyBzdWZmaXgpOwogICAgfQoKICAgIC8vIE9ubHkgcGF0dGVybi10
>>"%PTBR_JS%.b64" echo cmFuc2xhdGUgc2hvcnQgVUkgdGV4dC4gTmV2ZXIgcmV3cml0ZSBsb25nIGxvZ3Mgb3IgY29kZS4K
>>"%PTBR_JS%.b64" echo ICAgIGlmICh0cmltbWVkLmxlbmd0aCA8PSAxMjAgJiYgIS9be31bXF08Pj0kXXxodHRwcz86XC9c
>>"%PTBR_JS%.b64" echo L3xbXFwvXVtBLVphLXowLTlfLi1dKy8udGVzdCh0cmltbWVkKSkgewogICAgICBsZXQgb3V0ID0g
>>"%PTBR_JS%.b64" echo dHJpbW1lZDsKICAgICAgZm9yIChjb25zdCBbcngsIHJlcGxhY2VtZW50XSBvZiBQQVRURVJOUykg
>>"%PTBR_JS%.b64" echo b3V0ID0gb3V0LnJlcGxhY2UocngsIHJlcGxhY2VtZW50KTsKICAgICAgaWYgKG91dCAhPT0gdHJp
>>"%PTBR_JS%.b64" echo bW1lZCkgcmV0dXJuIHByZXNlcnZlV2hpdGVzcGFjZShyYXcsIG91dCk7CiAgICB9CiAgICByZXR1
>>"%PTBR_JS%.b64" echo cm4gcmF3OwogIH0KCiAgZnVuY3Rpb24gaXNFZGl0YWJsZShlbCkgewogICAgcmV0dXJuIGVsICYm
>>"%PTBR_JS%.b64" echo IChlbC5pc0NvbnRlbnRFZGl0YWJsZSB8fCBlbC5jbG9zZXN0Py4oJ1tjb250ZW50ZWRpdGFibGU9
>>"%PTBR_JS%.b64" echo InRydWUiXScpKTsKICB9CgogIGZ1bmN0aW9uIHRyYW5zbGF0ZVRleHROb2RlKG5vZGUpIHsKICAg
>>"%PTBR_JS%.b64" echo IGNvbnN0IHBhcmVudCA9IG5vZGUucGFyZW50RWxlbWVudDsKICAgIGlmICghcGFyZW50IHx8IFNL
>>"%PTBR_JS%.b64" echo SVAuaGFzKHBhcmVudC50YWdOYW1lKSB8fCBpc0VkaXRhYmxlKHBhcmVudCkpIHJldHVybjsKICAg
>>"%PTBR_JS%.b64" echo IGNvbnN0IGJlZm9yZSA9IG5vZGUubm9kZVZhbHVlOwogICAgY29uc3QgYWZ0ZXIgPSB0cmFuc2xh
>>"%PTBR_JS%.b64" echo dGVTdHJpbmcoYmVmb3JlKTsKICAgIGlmIChhZnRlciAhPT0gYmVmb3JlKSBub2RlLm5vZGVWYWx1
>>"%PTBR_JS%.b64" echo ZSA9IGFmdGVyOwogIH0KCiAgZnVuY3Rpb24gdHJhbnNsYXRlRWxlbWVudChlbCkgewogICAgaWYg
>>"%PTBR_JS%.b64" echo KCEoZWwgaW5zdGFuY2VvZiBFbGVtZW50KSB8fCBTS0lQLmhhcyhlbC50YWdOYW1lKSkgcmV0dXJu
>>"%PTBR_JS%.b64" echo OwogICAgZm9yIChjb25zdCBhdHRyIG9mIEFUVFJTKSB7CiAgICAgIGlmICghZWwuaGFzQXR0cmli
>>"%PTBR_JS%.b64" echo dXRlKGF0dHIpKSBjb250aW51ZTsKICAgICAgY29uc3QgYmVmb3JlID0gZWwuZ2V0QXR0cmlidXRl
>>"%PTBR_JS%.b64" echo KGF0dHIpOwogICAgICBjb25zdCBhZnRlciA9IHRyYW5zbGF0ZVN0cmluZyhiZWZvcmUpOwogICAg
>>"%PTBR_JS%.b64" echo ICBpZiAoYWZ0ZXIgIT09IGJlZm9yZSkgZWwuc2V0QXR0cmlidXRlKGF0dHIsIGFmdGVyKTsKICAg
>>"%PTBR_JS%.b64" echo IH0KICAgIGlmIChlbCBpbnN0YW5jZW9mIEhUTUxJbnB1dEVsZW1lbnQgJiYgWyJidXR0b24iLCJz
>>"%PTBR_JS%.b64" echo dWJtaXQiLCJyZXNldCJdLmluY2x1ZGVzKGVsLnR5cGUpKSB7CiAgICAgIGNvbnN0IGFmdGVyID0g
>>"%PTBR_JS%.b64" echo dHJhbnNsYXRlU3RyaW5nKGVsLnZhbHVlKTsKICAgICAgaWYgKGFmdGVyICE9PSBlbC52YWx1ZSkg
>>"%PTBR_JS%.b64" echo ZWwudmFsdWUgPSBhZnRlcjsKICAgIH0KICB9CgogIGZ1bmN0aW9uIHdhbGsocm9vdCkgewogICAg
>>"%PTBR_JS%.b64" echo aWYgKCFyb290KSByZXR1cm47CiAgICBpZiAocm9vdC5ub2RlVHlwZSA9PT0gTm9kZS5URVhUX05P
>>"%PTBR_JS%.b64" echo REUpIHsKICAgICAgdHJhbnNsYXRlVGV4dE5vZGUocm9vdCk7CiAgICAgIHJldHVybjsKICAgIH0K
>>"%PTBR_JS%.b64" echo ICAgIGlmIChyb290Lm5vZGVUeXBlICE9PSBOb2RlLkVMRU1FTlRfTk9ERSAmJiByb290Lm5vZGVU
>>"%PTBR_JS%.b64" echo eXBlICE9PSBOb2RlLkRPQ1VNRU5UX05PREUgJiYKICAgICAgICByb290Lm5vZGVUeXBlICE9PSBO
>>"%PTBR_JS%.b64" echo b2RlLkRPQ1VNRU5UX0ZSQUdNRU5UX05PREUpIHJldHVybjsKCiAgICBpZiAocm9vdC5ub2RlVHlw
>>"%PTBR_JS%.b64" echo ZSA9PT0gTm9kZS5FTEVNRU5UX05PREUpIHRyYW5zbGF0ZUVsZW1lbnQocm9vdCk7CgogICAgY29u
>>"%PTBR_JS%.b64" echo c3Qgd2Fsa2VyID0gZG9jdW1lbnQuY3JlYXRlVHJlZVdhbGtlcigKICAgICAgcm9vdCwKICAgICAg
>>"%PTBR_JS%.b64" echo Tm9kZUZpbHRlci5TSE9XX0VMRU1FTlQgfCBOb2RlRmlsdGVyLlNIT1dfVEVYVCwKICAgICAgewog
>>"%PTBR_JS%.b64" echo ICAgICAgIGFjY2VwdE5vZGUobm9kZSkgewogICAgICAgICAgaWYgKG5vZGUubm9kZVR5cGUgPT09
>>"%PTBR_JS%.b64" echo IE5vZGUuRUxFTUVOVF9OT0RFICYmIFNLSVAuaGFzKG5vZGUudGFnTmFtZSkpIHsKICAgICAgICAg
>>"%PTBR_JS%.b64" echo ICAgcmV0dXJuIE5vZGVGaWx0ZXIuRklMVEVSX1JFSkVDVDsKICAgICAgICAgIH0KICAgICAgICAg
>>"%PTBR_JS%.b64" echo IHJldHVybiBOb2RlRmlsdGVyLkZJTFRFUl9BQ0NFUFQ7CiAgICAgICAgfQogICAgICB9CiAgICAp
>>"%PTBR_JS%.b64" echo OwogICAgbGV0IG47CiAgICB3aGlsZSAoKG4gPSB3YWxrZXIubmV4dE5vZGUoKSkpIHsKICAgICAg
>>"%PTBR_JS%.b64" echo aWYgKG4ubm9kZVR5cGUgPT09IE5vZGUuVEVYVF9OT0RFKSB0cmFuc2xhdGVUZXh0Tm9kZShuKTsK
>>"%PTBR_JS%.b64" echo ICAgICAgZWxzZSB0cmFuc2xhdGVFbGVtZW50KG4pOwogICAgICBpZiAobi5zaGFkb3dSb290KSB3
>>"%PTBR_JS%.b64" echo YWxrKG4uc2hhZG93Um9vdCk7CiAgICB9CiAgfQoKICBsZXQgc2NoZWR1bGVkID0gZmFsc2U7CiAg
>>"%PTBR_JS%.b64" echo Y29uc3QgcXVldWUgPSBuZXcgU2V0KCk7CiAgZnVuY3Rpb24gc2NoZWR1bGUobm9kZSkgewogICAg
>>"%PTBR_JS%.b64" echo cXVldWUuYWRkKG5vZGUpOwogICAgaWYgKHNjaGVkdWxlZCkgcmV0dXJuOwogICAgc2NoZWR1bGVk
>>"%PTBR_JS%.b64" echo ID0gdHJ1ZTsKICAgIHJlcXVlc3RBbmltYXRpb25GcmFtZSgoKSA9PiB7CiAgICAgIHNjaGVkdWxl
>>"%PTBR_JS%.b64" echo ZCA9IGZhbHNlOwogICAgICBmb3IgKGNvbnN0IGl0ZW0gb2YgcXVldWUpIHdhbGsoaXRlbSk7CiAg
>>"%PTBR_JS%.b64" echo ICAgIHF1ZXVlLmNsZWFyKCk7CiAgICB9KTsKICB9CgogIGNvbnN0IG9ic2VydmVyID0gbmV3IE11
>>"%PTBR_JS%.b64" echo dGF0aW9uT2JzZXJ2ZXIoKG11dGF0aW9ucykgPT4gewogICAgZm9yIChjb25zdCBtIG9mIG11dGF0
>>"%PTBR_JS%.b64" echo aW9ucykgewogICAgICBpZiAobS50eXBlID09PSAiY2hhcmFjdGVyRGF0YSIpIHNjaGVkdWxlKG0u
>>"%PTBR_JS%.b64" echo dGFyZ2V0KTsKICAgICAgZWxzZSBpZiAobS50eXBlID09PSAiYXR0cmlidXRlcyIpIHNjaGVkdWxl
>>"%PTBR_JS%.b64" echo KG0udGFyZ2V0KTsKICAgICAgZWxzZSBmb3IgKGNvbnN0IG4gb2YgbS5hZGRlZE5vZGVzKSBzY2hl
>>"%PTBR_JS%.b64" echo ZHVsZShuKTsKICAgIH0KICB9KTsKCiAgZnVuY3Rpb24gc3RhcnQoKSB7CiAgICBkb2N1bWVudC5k
>>"%PTBR_JS%.b64" echo b2N1bWVudEVsZW1lbnQubGFuZyA9ICJwdC1CUiI7CiAgICB3YWxrKGRvY3VtZW50KTsKICAgIG9i
>>"%PTBR_JS%.b64" echo c2VydmVyLm9ic2VydmUoZG9jdW1lbnQuZG9jdW1lbnRFbGVtZW50LCB7CiAgICAgIHN1YnRyZWU6
>>"%PTBR_JS%.b64" echo IHRydWUsCiAgICAgIGNoaWxkTGlzdDogdHJ1ZSwKICAgICAgY2hhcmFjdGVyRGF0YTogdHJ1ZSwK
>>"%PTBR_JS%.b64" echo ICAgICAgYXR0cmlidXRlczogdHJ1ZSwKICAgICAgYXR0cmlidXRlRmlsdGVyOiBBVFRSUwogICAg
>>"%PTBR_JS%.b64" echo fSk7CgogICAgLy8gU1BBcyBjYW4gdXBkYXRlIGNvbnRlbnQgYWZ0ZXIgYW5pbWF0aW9ucy9mZXRj
>>"%PTBR_JS%.b64" echo aGVzLiBBIGxpZ2h0IHBlcmlvZGljIHN3ZWVwCiAgICAvLyBjYXRjaGVzIHBvcnRhbC9kaWFsb2cg
>>"%PTBR_JS%.b64" echo cm9vdHMgd2l0aG91dCBjaGFuZ2luZyBhcHBsaWNhdGlvbiBzdGF0ZS4KICAgIHNldEludGVydmFs
>>"%PTBR_JS%.b64" echo KCgpID0+IHdhbGsoZG9jdW1lbnQuYm9keSksIDI1MDApOwoKICAgIC8vIE1hcmsgdGhlIHBhZ2Ug
>>"%PTBR_JS%.b64" echo c28gdGhlIENvbnRyb2wgQ2VudGVyIHRyYW5zbGF0aW9uIGNhbiBiZSB2ZXJpZmllZCB2aXN1YWxs
>>"%PTBR_JS%.b64" echo eS4KICAgIGRvY3VtZW50LmRvY3VtZW50RWxlbWVudC5kYXRhc2V0LmtvbW9kb0xhbmd1YWdlID0g
>>"%PTBR_JS%.b64" echo InB0LUJSIjsKICAgIGNvbnNvbGUuaW5mbygiW0tvbW9kbyBQVC1CUl0gVHJhZHXDp8OjbyBkaW7D
>>"%PTBR_JS%.b64" echo om1pY2EgY2FycmVnYWRhLiBJbnRlcmZhY2UgcHQtQlIgYXRpdmEuIik7CiAgfQoKICBpZiAoZG9j
>>"%PTBR_JS%.b64" echo dW1lbnQucmVhZHlTdGF0ZSA9PT0gImxvYWRpbmciKSB7CiAgICBkb2N1bWVudC5hZGRFdmVudExp
>>"%PTBR_JS%.b64" echo c3RlbmVyKCJET01Db250ZW50TG9hZGVkIiwgc3RhcnQsIHsgb25jZTogdHJ1ZSB9KTsKICB9IGVs
>>"%PTBR_JS%.b64" echo c2UgewogICAgc3RhcnQoKTsKICB9Cn0pKCk7Cg==
certutil -f -decode "%PTBR_JS%.b64" "%PTBR_JS%" >nul 2>&1
del /Q "%PTBR_JS%.b64" >nul 2>&1
if errorlevel 1 exit /b 1
> "%PTBR_VBS%.b64" echo T3B0aW9uIEV4cGxpY2l0CkRpbSBmLCBtb2RlLCB0b2tlbiwgc3RtLCBodG1sLCByZSwgdGFnCklm
>>"%PTBR_VBS%.b64" echo IFdTY3JpcHQuQXJndW1lbnRzLkNvdW50IDwgMiBUaGVuIFdTY3JpcHQuUXVpdCAyCmYgPSBXU2Ny
>>"%PTBR_VBS%.b64" echo aXB0LkFyZ3VtZW50cygwKQptb2RlID0gTENhc2UoV1NjcmlwdC5Bcmd1bWVudHMoMSkpCklmIFdT
>>"%PTBR_VBS%.b64" echo Y3JpcHQuQXJndW1lbnRzLkNvdW50ID49IDMgVGhlbiB0b2tlbiA9IFdTY3JpcHQuQXJndW1lbnRz
>>"%PTBR_VBS%.b64" echo KDIpIEVsc2UgdG9rZW4gPSAiMSIKClNldCBzdG0gPSBDcmVhdGVPYmplY3QoIkFET0RCLlN0cmVh
>>"%PTBR_VBS%.b64" echo bSIpCnN0bS5UeXBlID0gMgpzdG0uQ2hhcnNldCA9ICJ1dGYtOCIKc3RtLk9wZW4Kc3RtLkxvYWRG
>>"%PTBR_VBS%.b64" echo cm9tRmlsZSBmCmh0bWwgPSBzdG0uUmVhZFRleHQKc3RtLkNsb3NlCgpTZXQgcmUgPSBOZXcgUmVn
>>"%PTBR_VBS%.b64" echo RXhwCnJlLkdsb2JhbCA9IFRydWUKcmUuSWdub3JlQ2FzZSA9IFRydWUKcmUuTXVsdGlMaW5lID0g
>>"%PTBR_VBS%.b64" echo VHJ1ZQpyZS5QYXR0ZXJuID0gIjxzY3JpcHRbXj5dKmlkPVsiIidda29tb2RvLXB0YnItbG9hZGVy
>>"%PTBR_VBS%.b64" echo WyIiJ11bXj5dKj5ccyo8L3NjcmlwdD4iCmh0bWwgPSByZS5SZXBsYWNlKGh0bWwsICIiKQoKSWYg
>>"%PTBR_VBS%.b64" echo bW9kZSA9ICJhcHBseSIgVGhlbgogIHRhZyA9ICI8c2NyaXB0IGlkPSIia29tb2RvLXB0YnItbG9h
>>"%PTBR_VBS%.b64" echo ZGVyIiIgc3JjPSIiL2tvbW9kby1wdGJyLmpzP3Y9IiAmIHRva2VuICYgIiIiIGRlZmVyPjwvc2Ny
>>"%PTBR_VBS%.b64" echo aXB0PiIKICByZS5HbG9iYWwgPSBGYWxzZQogIHJlLlBhdHRlcm4gPSAiPC9ib2R5PiIKICBJZiBy
>>"%PTBR_VBS%.b64" echo ZS5UZXN0KGh0bWwpIFRoZW4KICAgIGh0bWwgPSByZS5SZXBsYWNlKGh0bWwsIHRhZyAmIHZiQ3JM
>>"%PTBR_VBS%.b64" echo ZiAmICI8L2JvZHk+IikKICBFbHNlCiAgICBodG1sID0gaHRtbCAmIHZiQ3JMZiAmIHRhZyAmIHZi
>>"%PTBR_VBS%.b64" echo Q3JMZgogIEVuZCBJZgpFbmQgSWYKClNldCBzdG0gPSBDcmVhdGVPYmplY3QoIkFET0RCLlN0cmVh
>>"%PTBR_VBS%.b64" echo bSIpCnN0bS5UeXBlID0gMgpzdG0uQ2hhcnNldCA9ICJ1dGYtOCIKc3RtLk9wZW4Kc3RtLldyaXRl
>>"%PTBR_VBS%.b64" echo VGV4dCBodG1sCnN0bS5Qb3NpdGlvbiA9IDAKc3RtLlNhdmVUb0ZpbGUgZiwgMgpzdG0uQ2xvc2UK
>>"%PTBR_VBS%.b64" echo V1NjcmlwdC5RdWl0IDAK
certutil -f -decode "%PTBR_VBS%.b64" "%PTBR_VBS%" >nul 2>&1
del /Q "%PTBR_VBS%.b64" >nul 2>&1
if errorlevel 1 exit /b 1
exit /b 0

:ApplyPtBr
call :Preflight
if errorlevel 1 exit /b 1
call :EnsureDocker
if errorlevel 1 exit /b 1
call :FindCoreContainer
if errorlevel 1 (
    call :MsgError "container Core active not found."
    exit /b 1
)
call :BuildPtBrPayload
if errorlevel 1 (
    call :MsgError "Failed to preparar o modulo PT-BR."
    exit /b 1
)

set "PTBR_INDEX=%STATE_DIR%\ptbr\index.html"
docker cp "%CORE_ID%:/app/ui/index.html" "%PTBR_INDEX%" >nul 2>&1
if errorlevel 1 (
    call :MsgError "Could not ler /app/ui/index.html of the Core."
    exit /b 1
)

for /F "tokens=1-3 delims=/:. " %%A in ("%TIME%") do set "PTBR_TOKEN=%RANDOM%%%A%%B%%C"
cscript //nologo "%PTBR_VBS%" "%PTBR_INDEX%" apply "%PTBR_TOKEN%" >nul 2>&1
if errorlevel 1 (
    call :MsgError "Failed to injetar o carregador PT-BR in the index.html."
    exit /b 1
)

docker cp "%PTBR_JS%" "%CORE_ID%:/app/ui/komodo-ptbr.js" >nul 2>&1
if errorlevel 1 (
    call :MsgError "Failed to copy o modulo PT-BR for o Core."
    exit /b 1
)
docker cp "%PTBR_INDEX%" "%CORE_ID%:/app/ui/index.html" >nul 2>&1
if errorlevel 1 (
    call :MsgError "Failed to aplicar o index.html traduzido."
    exit /b 1
)

> "%STATE_DIR%\ptbr.enabled" echo enabled
curl.exe -fsS --max-time 4 "%DASHBOARD_URL%/komodo-ptbr.js" >nul 2>&1
if errorlevel 1 (
    call :MsgError "O modulo was copiado, mas o Core ainda not o serviu via HTTP."
    exit /b 1
)
exit /b 0

:ApplyPtBrSilent
if not exist "%STATE_DIR%\ptbr.enabled" exit /b 0
call :ApplyPtBr >nul 2>&1
exit /b 0

:TRANSLATE_PTBR
@echo off
cls
call :Header
echo %MAGENTA%%BOLD%   [BR] translate all O KOMODO for PORTUGUES BR - to the live%RESET%
echo.
echo A translation will be applied directly in the interface of the Core in execucao.
echo.
echo %GREEN%NOT will be required:%RESET%
echo   - close this BAT
echo   - restart this BAT
echo   - restart Docker
echo   - restart Komodo
echo   - recompilar a interface
echo.
echo after of the mensagem of success:
echo   %YELLOW%va for o navegador and press only F5.%RESET%
echo.
echo O sistema traduz textos visiveis, botoes, menus, formularios,
echo placeholders, titulos, dialogs and elementos carregados after by the SPA.
echo Nomes criados by you, commands, logs and data tecnicos permanecem intactos.
echo.
choice /C SBC /N /M "Aplicar PT-BR now? [S] Yes  [B] Back  [C] Cancel: "
if errorlevel 3 goto MAIN
if errorlevel 2 goto MAIN

echo.
echo %CYAN%Applying PT-BR language module to active Core...%RESET%
call :RemoveGenericLanguageSilent
call :ApplyPtBr
if errorlevel 1 (
    echo.
    call :MsgError "A translation not was applied. in the restart was executed."
    goto PAUSE_MAIN
)

call :Log "PT-BR live UI translation applied"
echo.
call :MsgOk "translation PT-BR active."
echo.
echo %TAG_ADMIN% [ PT-BR ACTIVE ] %RESET%  interface preparada with success.
echo.
echo %WHITE%%BOLD%now faca only isto:%RESET%
echo   1. Volte for a aba of the Komodo.
echo   2. press F5.
echo   3. A interface carregara in Portugues of the Brasil.
echo.
echo A translation stays marcada for reaplicacao automatic caso o Core
echo seja recriado futuramente by the Control Center.
goto PAUSE_MAIN

:RESTORE_ENGLISH
@echo off
cls
call :Header
echo %MAGENTA%%BOLD%   [EN] restore interface original in English - to the live%RESET%
echo.
choice /C SBC /N /M "restore English now? [S] Yes  [B] Back  [C] Cancel: "
if errorlevel 3 goto MAIN
if errorlevel 2 goto MAIN

call :Preflight
if errorlevel 1 goto PAUSE_MAIN
call :EnsureDocker
if errorlevel 1 goto PAUSE_MAIN
call :FindCoreContainer
if errorlevel 1 (
    call :MsgError "container Core active not found."
    goto PAUSE_MAIN
)
call :BuildPtBrPayload
if errorlevel 1 goto PAUSE_MAIN

set "PTBR_INDEX=%STATE_DIR%\ptbr\index.html"
docker cp "%CORE_ID%:/app/ui/index.html" "%PTBR_INDEX%" >nul 2>&1
if errorlevel 1 (
    call :MsgError "Could not ler o index.html."
    goto PAUSE_MAIN
)
cscript //nologo "%PTBR_VBS%" "%PTBR_INDEX%" remove "0" >nul 2>&1
if errorlevel 1 (
    call :MsgError "Failed to removes o carregador PT-BR."
    goto PAUSE_MAIN
)
docker cp "%PTBR_INDEX%" "%CORE_ID%:/app/ui/index.html" >nul 2>&1
if errorlevel 1 (
    call :MsgError "Failed to restore o index.html."
    goto PAUSE_MAIN
)
docker exec "%CORE_ID%" sh -lc "rm -f /app/ui/komodo-ptbr.js" >nul 2>&1
powershell.exe -NoProfile -Command "$p='%PTBR_INDEX%';$s=[IO.File]::ReadAllText($p);$s=[regex]::Replace($s,'<script id="komodo-generic-lang-loader".*?</script>','','Singleline');[IO.File]::WriteAllText($p,$s,(New-Object Text.UTF8Encoding($false)))" >nul 2>&1
docker cp "%PTBR_INDEX%" "%CORE_ID%:/app/ui/index.html" >nul 2>&1
docker exec "%CORE_ID%" sh -lc "rm -f /app/ui/komodo-lang.js" >nul 2>&1
del /Q "%STATE_DIR%\generic-lang.enabled" >nul 2>&1
del /Q "%STATE_DIR%\ptbr.enabled" >nul 2>&1

call :Log "PT-BR live UI translation disabled"
call :MsgOk "interface original in English restored."
echo.
echo Volte to the navegador and press F5.
goto PAUSE_MAIN



:GUIDE_MAIN
@echo off
cls
call :Header
call :LogAction "Abrir Guia Completo do Komodo"
echo %CYAN%%BOLD%   COMPLETE GUIDE of the KOMODO - course of the ZERO to the USO real%RESET%
echo %GRAY%   for user leigo: siga always [P] next step.%RESET%
echo.
echo %YELLOW%mode more FACIL:%RESET%
echo   type 0 and after use [P] next step in each screen.
echo   you may fazer o course inteiro without decidir qual chapter open.
echo.
echo   %GREEN%[0]%RESET%  ZERO ABSOLUTO - server, Docker, port, volume, Git and deploy
echo   %GREEN%[1]%RESET% COMECANDO of the ZERO
echo   %GREEN%[2]%RESET% SERVERS and PERIPHERY
echo   %GREEN%[3]%RESET% DEPLOYMENTS - a container
echo   %GREEN%[4]%RESET% STACKS - DOCKER COMPOSE
echo   %GREEN%[5]%RESET% REPOS - GIT and SCRIPTS
echo   %GREEN%[6]%RESET% BUILDS - create images DOCKER
echo   %GREEN%[7]%RESET% BUILDERS - where O BUILD ACONTECE
echo   %GREEN%[8]%RESET% PROCEDURES - AUTOMACAO without CODIGO
echo   %GREEN%[9]%RESET% ACTIONS - AUTOMACAO with TYPESCRIPT
echo   %GREEN%[10]%RESET% RESOURCE SYNCS - configuration how CODIGO
echo   %GREEN%[11]%RESET% SWARM
echo   %GREEN%[12]%RESET% ALERTERS - alerts
echo   %GREEN%[13]%RESET% users, permissions and ADMIN
echo   %GREEN%[14]%RESET% logs, TERMINAL and diagnostics
echo   %GREEN%[15]%RESET% VARIAVEIS, SECRETS and credentials
echo   %GREEN%[16]%RESET% path recommended for learn
echo   %GREEN%[17]%RESET% GLOSSARIO YESPLES
echo   %GREEN%[18]%RESET% guide of the CONTROL CENTER BAT
echo   %GREEN%[19]%RESET% first project complete
echo   %GREEN%[20]%RESET% QUERO FAZER X - QUAL tool?
echo   %GREEN%[21]%RESET% solution of problems
echo   %GREEN%[22]%RESET% CHECKLIST before of production
echo   %GREEN%[23]%RESET% completion - how seguir using o Komodo in the pratica
echo.
echo   %GRAY%[B] Back to the menu main   [C] Cancel%RESET%
echo.
set "GOP="
set /p "GOP=%WHITE%%BOLD%choose o chapter: %RESET%"
call :LogAction "GUIA selecionado: %GOP%"
if /I "%GOP%"=="B" goto MAIN
if /I "%GOP%"=="C" goto MAIN
if "%GOP%"=="0" goto GUIDE_0
if "%GOP%"=="1" goto GUIDE_1
if "%GOP%"=="2" goto GUIDE_2
if "%GOP%"=="3" goto GUIDE_3
if "%GOP%"=="4" goto GUIDE_4
if "%GOP%"=="5" goto GUIDE_5
if "%GOP%"=="6" goto GUIDE_6
if "%GOP%"=="7" goto GUIDE_7
if "%GOP%"=="8" goto GUIDE_8
if "%GOP%"=="9" goto GUIDE_9
if "%GOP%"=="10" goto GUIDE_10
if "%GOP%"=="11" goto GUIDE_11
if "%GOP%"=="12" goto GUIDE_12
if "%GOP%"=="13" goto GUIDE_13
if "%GOP%"=="14" goto GUIDE_14
if "%GOP%"=="15" goto GUIDE_15
if "%GOP%"=="16" goto GUIDE_16
if "%GOP%"=="17" goto GUIDE_17
if "%GOP%"=="18" goto GUIDE_18
if "%GOP%"=="19" goto GUIDE_19
if "%GOP%"=="20" goto GUIDE_20
if "%GOP%"=="21" goto GUIDE_21
if "%GOP%"=="22" goto GUIDE_22
if "%GOP%"=="23" goto GUIDE_23
call :MsgError "chapter invalido: %GOP%"
goto GUIDE_MAIN

:GUIDE_0
@echo off
cls
call :Header
echo %CYAN%%BOLD%   chapter 0 - ZERO ABSOLUTO: before of TOCAR in the KOMODO%RESET%
echo.
echo %YELLOW%for QUEM and this chapter:%RESET%
echo   - never usou Komodo.
echo   - never administrou server.
echo   - Quase not conhece Docker.
echo   - Not sabe o that and container, port, volume, Git or deploy.
echo.
echo %YELLOW%goal:%RESET%
echo to the terminar this chapter, you NOT needs saber programar.
echo you needs only entender o that o Komodo controla.
echo.
echo %YELLOW%1. O that and a server?%RESET%
echo a server and yesplesmente a machine that runs programas.
echo may be:
echo   - your proprio PC;
echo   - outro computador of the network;
echo   - a VPS in the internet;
echo   - a machine virtual;
echo   - a machine of a empresa.
echo.
echo %YELLOW%2. O that and DOCKER?%RESET%
echo Docker runs programas inside of unidades chamadas containers.
echo Pense in a container how a caixa preparada for run a aplicativo.
echo.
echo %YELLOW%3. O that and a image DOCKER?%RESET%
echo A image and o "molde" usado for create a container.
echo example:
echo   image = nginx:latest
echo   container = nginx rodando a partir dessa image
echo.
echo %YELLOW%4. O that and a port?%RESET%
echo port and a numero usado for acessar a service.
echo example:
echo   localhost:9120
echo O 9120 and a port usada by the dashboard of the Komodo neste environment.
echo.
echo %YELLOW%5. O that and a volume?%RESET%
echo volume serve for manter data outside of the ciclo of vida of the container.
echo example:
echo   database of data inside of container
echo   data importantes inside of volume
echo if o container for recriado, o volume may preservar os data.
echo.
echo %YELLOW%6. O that and GIT?%RESET%
echo Git controla versoes of codigo and files.
echo GitHub and a service that hospeda repositorios Git.
echo.
echo %YELLOW%7. O that and DEPLOY?%RESET%
echo Deploy significa colocar a application for run in a environment.
echo in the Komodo, you may fazer deploy of containers and Stacks.
echo.
echo %YELLOW%8. where O KOMODO ENTRA NISSO?%RESET%
echo O Komodo junta these tarefas in a interface:
echo   Server -> machine
echo   Deployment -> a container
echo   Stack -> varios containers via Docker Compose
echo   Repo -> codigo Git
echo   Build -> create image Docker
echo   Procedure -> automatizar etapas
echo   logs -> entender errors
echo.
echo %GREEN%how SABER if you ENTENDEU:%RESET%
echo you must conseguir responder:
echo   - Server and a machine?
echo   - Deployment and a container?
echo   - Stack uses Docker Compose?
echo   - logs ajudam a descobrir errors?
echo.
echo if a resposta for YES, you is ready for o chapter 1.
echo.
echo %CYAN%next step:%RESET%
echo   chapter 1 - entender o Komodo how a all.
goto GUIDE_PAUSE_0

:GUIDE_1
@echo off
cls
call :Header
echo %CYAN%%BOLD%   chapter 1 - COMECANDO of the ZERO%RESET%
echo.
echo %YELLOW%O that and:%RESET%
echo   - Komodo and a central of controle for servidores, Docker, Git, deploys, automacoes, logs, users and alerts.
echo   - A ideia and avoid repetir tarefas manuais in varias tools diferentes.
echo.
echo %YELLOW%when / for that USAR:%RESET%
echo   - Centralizar operation of a or varios servidores.
echo   - Administrar containers and Docker Compose by interface.
echo   - Automatizar atualizacoes and deploys.
echo   - Acompanhar status, logs and failures in a unico lugar.
echo.
echo %YELLOW%step A step:%RESET%
echo   1. confirm that o Core is online.
echo   2. between in the dashboard with your user.
echo   3. Entenda only 4 conceitos first: Core, Periphery, Server and Resource.
echo   4. after learn Deployment or Stack.
echo   5. only after avance for Build and automacao.
echo.
echo %YELLOW%example PRATICO:%RESET%
echo   - a site with frontend, backend and database may virar a Stack.
echo   - O Server and a machine where ele roda.
echo   - a Procedure may update tudo after with a clique.
echo.
echo %RED%ERRORS common:%RESET%
echo   - Tentar learn all os resources of a vez.
echo   - Automatizar before of fazer o deploy manual funcionar.
echo   - change configurations without olhar logs/status.
echo.
echo %GREEN%how SABER if FUNCIONOU:%RESET%
echo   - dashboard abre.
echo   - Login works.
echo   - Core is online.
echo   - you entende a diferenca between Server, Deployment and Stack.
echo.
echo %CYAN%next step:%RESET%
echo   chapter 2: Server and Periphery.
goto GUIDE_PAUSE_1

:GUIDE_2
@echo off
cls
call :Header
echo %CYAN%%BOLD%   chapter 2 - SERVERS and PERIPHERY%RESET%
echo.
echo %YELLOW%O that and:%RESET%
echo   - Server representa a machine controlada by the Komodo.
echo   - Periphery and o agente that runs operations nessa machine.
echo.
echo %YELLOW%when / for that USAR:%RESET%
echo   - Conectar your PC, VPS, server Linux or machine of network.
echo   - view CPU, memoria, disco and status.
echo   - Hospedar Deployments, Stacks, Repos and Builders.
echo.
echo %YELLOW%step A step:%RESET%
echo   1. Abra Resources ^> Servers.
echo   2. Clique Create Server.
echo   3. choose a nome yesples, how server-main.
echo   4. Configure endereco/connection of the Periphery.
echo   5. Salve.
echo   6. wait o status Connected/Healthy.
echo   7. So after associe projects a esse Server.
echo.
echo %YELLOW%example PRATICO:%RESET%
echo   - your VPS of production may be o Server production-01.
echo   - your PC of testes may be o Server laboratorio.
echo.
echo %RED%ERRORS common:%RESET%
echo   - Periphery parado.
echo   - Endereco or port incorretos.
echo   - Firewall bloqueando.
echo   - Segredo/token divergente.
echo   - DNS not resolvendo.
echo.
echo %GREEN%how SABER if FUNCIONOU:%RESET%
echo   - Server aparece conectado.
echo   - Metricas can be visualizadas.
echo   - operations basicas respondem without error.
echo.
echo %CYAN%next step:%RESET%
echo   chapter 3 if usar a container; chapter 4 if usar Compose.
goto GUIDE_PAUSE_2

:GUIDE_3
@echo off
cls
call :Header
echo %CYAN%%BOLD%   chapter 3 - DEPLOYMENTS - a container%RESET%
echo.
echo %YELLOW%O that and:%RESET%
echo   - Deployment representa a container Docker individual.
echo.
echo %YELLOW%when / for that USAR:%RESET%
echo   - application with a unico container.
echo   - service independente.
echo   - when quer Start, Stop, Restart, Pull, Redeploy and logs of forma yesples.
echo.
echo %YELLOW%step A step:%RESET%
echo   1. Resources ^> Deployments.
echo   2. Create Deployment.
echo   3. choose o Server.
echo   4. Informe a image Docker and tag.
echo   5. Configure ports.
echo   6. Configure variaveis of environment.
echo   7. Configure volumes/network if need.
echo   8. Salve.
echo   9. Clique Deploy.
echo   10. Abra logs imediatamente.
echo.
echo %YELLOW%example PRATICO:%RESET%
echo   - image minhaempresa/api:latest publicada in the port 8080.
echo.
echo %RED%ERRORS common:%RESET%
echo   - image/tag inexistente.
echo   - Registry privado without credencial.
echo   - port already ocupada.
echo   - Variavel obrigatoria ausente.
echo   - volume with path errado.
echo   - container inicia and encerra.
echo.
echo %GREEN%how SABER if FUNCIONOU:%RESET%
echo   - Status Running.
echo   - logs without error critico.
echo   - A port responde how esperado.
echo.
echo %CYAN%next step:%RESET%
echo   chapter 14 for learn a diagnose; chapter 8 for automatizar.
goto GUIDE_PAUSE_3

:GUIDE_4
@echo off
cls
call :Header
echo %CYAN%%BOLD%   chapter 4 - STACKS - DOCKER COMPOSE%RESET%
echo.
echo %YELLOW%O that and:%RESET%
echo   - Stack administra a project Docker Compose inteiro.
echo.
echo %YELLOW%when / for that USAR:%RESET%
echo   - projects with varios containers.
echo   - applications that already possuem compose.yaml/docker-compose.yml.
echo   - Frontend, backend, database and outros services that sobem juntos.
echo.
echo %YELLOW%step A step:%RESET%
echo   1. Resources ^> Stacks.
echo   2. Create Stack.
echo   3. choose Server or Swarm.
echo   4. Defina a source of the Compose: texto or Git.
echo   5. Configure Environment.
echo   6. Revise volumes and ports.
echo   7. Salve.
echo   8. Clique Deploy.
echo   9. check services and logs.
echo.
echo %YELLOW%example PRATICO:%RESET%
echo   - frontend + backend + mongodb + redis in a unica Stack.
echo.
echo %RED%ERRORS common:%RESET%
echo   - YAML mal indentado.
echo   - Variavel ${VAR} faltando.
echo   - port ocupada.
echo   - volume errado.
echo   - image privada without login.
echo   - service depende of outro ainda not ready.
echo.
echo %GREEN%how SABER if FUNCIONOU:%RESET%
echo   - Services aparecem Running.
echo   - containers permanecem ativos.
echo   - application responde.
echo.
echo %CYAN%next step:%RESET%
echo   chapter 5 if quiser integrar Git; chapter 8 for automatizar.
goto GUIDE_PAUSE_4

:GUIDE_5
@echo off
cls
call :Header
echo %CYAN%%BOLD%   chapter 5 - REPOS - GIT and SCRIPTS%RESET%
echo.
echo %YELLOW%O that and:%RESET%
echo   - Repo conecta a repository Git to the Komodo.
echo.
echo %YELLOW%when / for that USAR:%RESET%
echo   - Clonar codigo.
echo   - update branch.
echo   - run scripts.
echo   - Preparar fontes for Build or outras automacoes.
echo.
echo %YELLOW%step A step:%RESET%
echo   1. Configure credencial Git if o repo for privado.
echo   2. Resources ^> Repos.
echo   3. Create Repo.
echo   4. Informe repository/URL.
echo   5. choose branch.
echo   6. choose Server or Builder.
echo   7. Salve.
echo   8. run Clone/Pull.
echo   9. check ultimo commit.
echo.
echo %YELLOW%example PRATICO:%RESET%
echo   - Repository of the backend clonado in the Server of build.
echo.
echo %RED%ERRORS common:%RESET%
echo   - token without permission.
echo   - Branch inexistente.
echo   - Repo privado without credencial.
echo   - URL incorreta.
echo.
echo %GREEN%how SABER if FUNCIONOU:%RESET%
echo   - Clone/Pull conclui.
echo   - Ultimo commit aparece.
echo   - Files ficam acessiveis for as tarefas configuradas.
echo.
echo %CYAN%next step:%RESET%
echo   chapter 6 if o goal for create a image Docker.
goto GUIDE_PAUSE_5

:GUIDE_6
@echo off
cls
call :Header
echo %CYAN%%BOLD%   chapter 6 - BUILDS - create images DOCKER%RESET%
echo.
echo %YELLOW%O that and:%RESET%
echo   - Build transforma codigo fonte in a image Docker.
echo.
echo %YELLOW%when / for that USAR:%RESET%
echo   - when you possui o Dockerfile of the your project.
echo   - when quer publish a new image apos changing o codigo.
echo.
echo %YELLOW%step A step:%RESET%
echo   1. Tenha a Repo with Dockerfile.
echo   2. Tenha a Builder.
echo   3. Resources ^> Builds ^> Create.
echo   4. choose repo and branch.
echo   5. Informe Dockerfile.
echo   6. Defina build context.
echo   7. Configure nome/tag of the image.
echo   8. Configure Registry.
echo   9. Salve.
echo   10. run Build.
echo   11. Abra Build logs.
echo.
echo %YELLOW%example PRATICO:%RESET%
echo   - Git -^> Dockerfile -^> Build -^> image minhaempresa/app:v2 -^> Registry.
echo.
echo %RED%ERRORS common:%RESET%
echo   - Dockerfile invalido.
echo   - Contexto errado.
echo   - Dependencia faltando.
echo   - Failure in the registry.
echo   - Disco cheio.
echo   - Memoria insuficiente.
echo.
echo %GREEN%how SABER if FUNCIONOU:%RESET%
echo   - Build termina with success.
echo   - image aparece in the registry or local esperado.
echo   - logs not mostram error final.
echo.
echo %CYAN%next step:%RESET%
echo   chapter 7 for entender Builder; after ligue o Build to the deploy.
goto GUIDE_PAUSE_6

:GUIDE_7
@echo off
cls
call :Header
echo %CYAN%%BOLD%   chapter 7 - BUILDERS - where O BUILD ACONTECE%RESET%
echo.
echo %YELLOW%O that and:%RESET%
echo   - Builder define where o trabalho of Build will be executed.
echo.
echo %YELLOW%when / for that USAR:%RESET%
echo   - choose a machine capaz of construir images.
echo   - Separar build of production when required.
echo.
echo %YELLOW%step A step:%RESET%
echo   1. Comece using a Server already conectado.
echo   2. confirm Docker funcionando.
echo   3. check CPU, RAM and disco.
echo   4. confirm acesso to the Git.
echo   5. confirm acesso to the Registry.
echo   6. Associe o Builder to the Build.
echo.
echo %YELLOW%example PRATICO:%RESET%
echo   - Server laboratorio may fazer os builds while production only runs containers.
echo.
echo %RED%ERRORS common:%RESET%
echo   - without espaco in disco.
echo   - Pouca memoria.
echo   - Docker indisponivel.
echo   - without acesso to the Git/Registry.
echo.
echo %GREEN%how SABER if FUNCIONOU:%RESET%
echo   - Build consegue start and concluir in the Builder.
echo.
echo %CYAN%next step:%RESET%
echo   Volte to the chapter 6 and run a Build real.
goto GUIDE_PAUSE_7

:GUIDE_8
@echo off
cls
call :Header
echo %CYAN%%BOLD%   chapter 8 - PROCEDURES - AUTOMACAO without CODIGO%RESET%
echo.
echo %YELLOW%O that and:%RESET%
echo   - Procedure junta varias operations in etapas chamadas stages.
echo.
echo %YELLOW%when / for that USAR:%RESET%
echo   - update varios resources with a clique.
echo   - run tarefas in sequencia.
echo   - Rodar algumas tarefas in paralelo inside of the same stage.
echo.
echo %YELLOW%step A step:%RESET%
echo   1. Resources ^> Procedures.
echo   2. Create Procedure.
echo   3. Crie Stage 1.
echo   4. Adicione a operation.
echo   5. Adicione outros stages.
echo   6. Salve.
echo   7. run manualmente first.
echo   8. check Execution History.
echo.
echo %YELLOW%example PRATICO:%RESET%
echo   - Stage 1: Build; Stage 2: Deploy backend + frontend; Stage 3: verificacao final.
echo.
echo %RED%ERRORS common:%RESET%
echo   - Automatizar algo that never funcionou manualmente.
echo   - Ordem of stages errada.
echo   - Ignorar resultado of etapa previous.
echo.
echo %GREEN%how SABER if FUNCIONOU:%RESET%
echo   - all os stages concluem.
echo   - Historico shows success.
echo   - Resultado final and o same of the process manual.
echo.
echo %CYAN%next step:%RESET%
echo   after considere webhook or Action.
goto GUIDE_PAUSE_8

:GUIDE_9
@echo off
cls
call :Header
echo %CYAN%%BOLD%   chapter 9 - ACTIONS - AUTOMACAO with TYPESCRIPT%RESET%
echo.
echo %YELLOW%O that and:%RESET%
echo   - Action permite escrever logica personalizada in TypeScript using a API of the Komodo.
echo.
echo %YELLOW%when / for that USAR:%RESET%
echo   - when not existe a operation ready.
echo   - Validacoes personalizadas.
echo   - Chamadas API and logica condicional.
echo.
echo %YELLOW%step A step:%RESET%
echo   1. first domine Server, Stack/Deployment, logs and Procedure.
echo   2. Crie a Action yesples.
echo   3. Comece only lendo data.
echo   4. after adicione alteracoes.
echo   5. Teste in environment safe.
echo   6. Registre and trate errors.
echo.
echo %YELLOW%example PRATICO:%RESET%
echo   - after of a deploy, chamar /health and falhar if a API not responder.
echo.
echo %RED%ERRORS common:%RESET%
echo   - Colocar secrets in the codigo.
echo   - run operations destructive without validation.
echo   - Not tratar retorno/error.
echo.
echo %GREEN%how SABER if FUNCIONOU:%RESET%
echo   - Action runs of forma previsivel and deixa resultado claro.
echo.
echo %CYAN%next step:%RESET%
echo   use only when Procedure not resolver your necessidade.
goto GUIDE_PAUSE_9

:GUIDE_10
@echo off
cls
call :Header
echo %CYAN%%BOLD%   chapter 10 - RESOURCE SYNCS - configuration how CODIGO%RESET%
echo.
echo %YELLOW%O that and:%RESET%
echo   - Resource Sync guarda configurations of resources in TOML versionado in the Git.
echo.
echo %YELLOW%when / for that USAR:%RESET%
echo   - Reproduzir ambientes.
echo   - Auditar mudancas.
echo   - Versionar configuration.
echo   - Gerenciar muitos resources.
echo.
echo %YELLOW%step A step:%RESET%
echo   1. learn first resources by the interface.
echo   2. Crie repository of configuration.
echo   3. Defina TOML of the resources.
echo   4. Crie Resource Sync.
echo   5. Compare mudancas sugeridas.
echo   6. Revise before of aplicar.
echo   7. use 'after' for ordem when required.
echo.
echo %YELLOW%example PRATICO:%RESET%
echo   - Configurar Stacks and Deployments of varios Servers a partir of files versionados.
echo.
echo %RED%ERRORS common:%RESET%
echo   - Aplicar mudancas without revisar.
echo   - Not entender dependencias.
echo   - Editar muitos resources of a vez in the inicio.
echo.
echo %GREEN%how SABER if FUNCIONOU:%RESET%
echo   - Diff and compreensivel.
echo   - Mudancas aplicadas correspondem to the TOML.
echo.
echo %CYAN%next step:%RESET%
echo   use after that o fluxo manual estiver dominado.
goto GUIDE_PAUSE_10

:GUIDE_11
@echo off
cls
call :Header
echo %CYAN%%BOLD%   chapter 11 - SWARM%RESET%
echo.
echo %YELLOW%O that and:%RESET%
echo   - Swarm integra Docker Swarm to the Komodo.
echo.
echo %YELLOW%when / for that USAR:%RESET%
echo   - Clusters Docker Swarm with managers, nodes, services and tasks.
echo.
echo %YELLOW%step A step:%RESET%
echo   1. use only if already entende Docker Swarm.
echo   2. Cadastre/configure o Swarm.
echo   3. confirm managers.
echo   4. check nodes.
echo   5. So after associe Stacks/Deployments.
echo.
echo %YELLOW%example PRATICO:%RESET%
echo   - Cluster with 3 nodes executando services distribuidos.
echo.
echo %RED%ERRORS common:%RESET%
echo   - Usar Swarm without necessidade.
echo   - Managers indisponiveis.
echo   - network/overlay mal configured.
echo.
echo %GREEN%how SABER if FUNCIONOU:%RESET%
echo   - Nodes aparecem saudaveis and services can be gerenciados.
echo.
echo %CYAN%next step:%RESET%
echo   if you tem only a PC/VPS, probably pule this chapter.
goto GUIDE_PAUSE_11

:GUIDE_12
@echo off
cls
call :Header
echo %CYAN%%BOLD%   chapter 12 - ALERTERS - alerts%RESET%
echo.
echo %YELLOW%O that and:%RESET%
echo   - Alerter encaminha alerts relevantes for destinos configurados.
echo.
echo %YELLOW%when / for that USAR:%RESET%
echo   - Server offline.
echo   - Failures.
echo   - Uso alto of resources.
echo   - Eventos importantes.
echo.
echo %YELLOW%step A step:%RESET%
echo   1. first estabilize o environment.
echo   2. Crie a Alerter.
echo   3. Configure destination.
echo   4. Ative poucos tipos of alert.
echo   5. Teste.
echo   6. after refine whitelist/blacklist.
echo.
echo %YELLOW%example PRATICO:%RESET%
echo   - Receber warning when Server of production stay offline.
echo.
echo %RED%ERRORS common:%RESET%
echo   - Ativar tudo and gerar excesso of notifications.
echo   - Not testar destination.
echo.
echo %GREEN%how SABER if FUNCIONOU:%RESET%
echo   - Evento of teste chega in the destination correto.
echo.
echo %CYAN%next step:%RESET%
echo   Aumente cobertura aos poucos.
goto GUIDE_PAUSE_12

:GUIDE_13
@echo off
cls
call :Header
echo %CYAN%%BOLD%   chapter 13 - users, permissions and ADMIN%RESET%
echo.
echo %YELLOW%O that and:%RESET%
echo   - Users controlam quem entra and o that may fazer.
echo.
echo %YELLOW%when / for that USAR:%RESET%
echo   - Separar member common of administrator.
echo   - Reduzir risco of alteracoes indevidas.
echo.
echo %YELLOW%step A step:%RESET%
echo   1. use [29] for view users real.
echo   2. use [27] for redefinir password of user existing.
echo   3. use [28] only when need tornar alguem Super Admin.
echo   4. use [30] for enable registration local.
echo   5. Revise privilegios periodicamente.
echo.
echo %YELLOW%example PRATICO:%RESET%
echo   - Operador common how member; responsavel by the plataforma how ADMIN.
echo.
echo %RED%ERRORS common:%RESET%
echo   - Confundir admin initial of the ENV with user already saved in the database.
echo   - Dar Super Admin for all.
echo.
echo %GREEN%how SABER if FUNCIONOU:%RESET%
echo   - each account possui privilegio adequado.
echo   - Login works.
echo   - Lista real shows classificacao correta.
echo.
echo %CYAN%next step:%RESET%
echo   chapter 15 for secrets and credentials.
goto GUIDE_PAUSE_13

:GUIDE_14
@echo off
cls
call :Header
echo %CYAN%%BOLD%   chapter 14 - logs, TERMINAL and diagnostics%RESET%
echo.
echo %YELLOW%O that and:%RESET%
echo   - logs mostram o that realmente aconteceu; diagnostics verifica a saude of the environment.
echo.
echo %YELLOW%when / for that USAR:%RESET%
echo   - Toda vez that algo not funcionar.
echo.
echo %YELLOW%step A step:%RESET%
echo   1. Rode [8] Diagnostics.
echo   2. Veja [5] Status.
echo   3. Abra [6] logs of the Core.
echo   4. use [7] logs gerais.
echo   5. Cheque [9] Health HTTP.
echo   6. Cheque [10] port/processes.
echo   7. use [33] for errors of the BAT.
echo.
echo %YELLOW%example PRATICO:%RESET%
echo   - container caiu: first veja logs, after corrija configuration.
echo.
echo %RED%ERRORS common:%RESET%
echo   - restart varias vezes without ler error.
echo   - change varias coisas to the same tempo.
echo   - Ignorar first error of the log.
echo.
echo %GREEN%how SABER if FUNCIONOU:%RESET%
echo   - you consegue apontar a causa, not only o sintoma.
echo.
echo %CYAN%next step:%RESET%
echo   chapter 21 possui roteiro of errors common.
goto GUIDE_PAUSE_14

:GUIDE_15
@echo off
cls
call :Header
echo %CYAN%%BOLD%   chapter 15 - VARIAVEIS, SECRETS and credentials%RESET%
echo.
echo %YELLOW%O that and:%RESET%
echo   - Variavel guarda value reutilizavel; Secret guarda dado sensivel; credencial permite acesso a services privados.
echo.
echo %YELLOW%when / for that USAR:%RESET%
echo   - Ports, URLs, passwords, tokens, Git privado, Registry privado.
echo.
echo %YELLOW%step A step:%RESET%
echo   1. Separe configuration of segredo.
echo   2. never coloque password in repo publico.
echo   3. use secrets/variaveis.
echo   4. Conceda menor permission possible.
echo   5. Rotacione tokens.
echo   6. Mantenha backup safe.
echo.
echo %YELLOW%example PRATICO:%RESET%
echo   - DATABASE_URL how configuration; token Git how segredo.
echo.
echo %RED%ERRORS common:%RESET%
echo   - token in print.
echo   - Password in the codigo.
echo   - same segredo in varios lugares.
echo   - token with permission excessiva.
echo.
echo %GREEN%how SABER if FUNCIONOU:%RESET%
echo   - Codigo may be compartilhado without revelar segredos.
echo.
echo %CYAN%next step:%RESET%
echo   chapter 22 for checklist of production.
goto GUIDE_PAUSE_15

:GUIDE_16
@echo off
cls
call :Header
echo %CYAN%%BOLD%   chapter 16 - path recommended for learn%RESET%
echo.
echo %YELLOW%O that and:%RESET%
echo   - a ordem yesples for not if perder.
echo.
echo %YELLOW%when / for that USAR:%RESET%
echo   - when estiver with muitas options and not souber by where comecar.
echo.
echo %YELLOW%step A step:%RESET%
echo   1. Nivel 1: dashboard, Status, logs, Server.
echo   2. Nivel 2: Deployment or Stack.
echo   3. Nivel 3: Repo and Build.
echo   4. Nivel 4: Procedure and alerts.
echo   5. Nivel 5: Action, Resource Sync, Swarm.
echo.
echo %YELLOW%example PRATICO:%RESET%
echo   - Not tente Action before of conseguir subir a Stack yesples.
echo.
echo %RED%ERRORS common:%RESET%
echo   - Pular etapas.
echo   - Automatizar process quebrado.
echo.
echo %GREEN%how SABER if FUNCIONOU:%RESET%
echo   - each new resource resolve a problem that you already entende.
echo.
echo %CYAN%next step:%RESET%
echo   chapter 19 for first project complete.
goto GUIDE_PAUSE_16

:GUIDE_17
@echo off
cls
call :Header
echo %CYAN%%BOLD%   chapter 17 - GLOSSARIO YESPLES%RESET%
echo.
echo %YELLOW%O that and:%RESET%
echo   - CORE = cerebro central.
echo   - PERIPHERY = agente of the machine.
echo   - SERVER = machine gerenciada.
echo   - RESOURCE = objeto administrado.
echo   - DEPLOYMENT = a container.
echo   - STACK = Docker Compose.
echo   - REPO = repository Git.
echo   - BUILD = cria image Docker.
echo   - BUILDER = environment that runs Build.
echo   - REGISTRY = repository of images.
echo   - PROCEDURE = automacao in etapas.
echo   - ACTION = automacao TypeScript/API.
echo   - RESOURCE SYNC = configuration TOML/Git.
echo   - SWARM = cluster Docker.
echo   - ALERTER = sistema of alerts.
echo   - SECRET = dado sensivel.
echo   - TAG = etiqueta for organizar.
echo.
echo %YELLOW%when / for that USAR:%RESET%
echo   - Volte here always that find a termo desconhecido.
echo.
echo %YELLOW%step A step:%RESET%
echo   1. Leia only o termo required.
echo   2. Volte to the chapter correspondente.
echo.
echo %YELLOW%example PRATICO:%RESET%
echo   - Stack not and Server: Stack roda in a Server.
echo.
echo %RED%ERRORS common:%RESET%
echo   - Confundir Build with Deployment: Build cria image; Deployment runs container.
echo.
echo %GREEN%how SABER if FUNCIONOU:%RESET%
echo   - you consegue explicar os termos with your proprias palavras.
echo.
echo %CYAN%next step:%RESET%
echo   chapter 20 ajuda a choose tool.
goto GUIDE_PAUSE_17

:GUIDE_18
@echo off
cls
call :Header
echo %CYAN%%BOLD%   chapter 18 - guide of the CONTROL CENTER BAT%RESET%
echo.
echo %YELLOW%O that and:%RESET%
echo   - O BAT and your camada of automacao Windows for administrar o Komodo local.
echo.
echo %YELLOW%when / for that USAR:%RESET%
echo   - avoid commands manuais.
echo   - Centralizar diagnostics, users, logs, translation and manutencao.
echo.
echo %YELLOW%step A step:%RESET%
echo   1. [1]-[4]: operation basica.
echo   2. [5]-[10]: monitoramento.
echo   3. [11]-[14]: manutencao.
echo   4. [15]-[23]: Windows/tools.
echo   5. [24]-[30]: users.
echo   6. [31]-[34]: extras, logs and guide.
echo   7. [90]-[91]: options destructive.
echo.
echo %YELLOW%example PRATICO:%RESET%
echo   - if dashboard not abre: 8 -^> 5 -^> 6 -^> 9 -^> 10.
echo.
echo %RED%ERRORS common:%RESET%
echo   - Usar [91] without backup.
echo   - close logs before of ler error.
echo.
echo %GREEN%how SABER if FUNCIONOU:%RESET%
echo   - you consegue administrar o environment without digitar commands Docker manualmente.
echo.
echo %CYAN%next step:%RESET%
echo   chapter 21 for diagnostics.
goto GUIDE_PAUSE_18

:GUIDE_19
@echo off
cls
call :Header
echo %CYAN%%BOLD%   chapter 19 - first project complete%RESET%
echo.
echo %YELLOW%O that and:%RESET%
echo   - example pratico of ponta a ponta for learn fazendo.
echo.
echo %YELLOW%when / for that USAR:%RESET%
echo   - your first deploy real.
echo.
echo %YELLOW%step A step:%RESET%
echo   1. [1] Inicie Komodo.
echo   2. [8] Rode diagnostics.
echo   3. [4] Abra dashboard.
echo   4. confirm user/admin.
echo   5. Conecte a Server and espere Healthy.
echo   6. choose Deployment for 1 container or Stack for Compose.
echo   7. Configure image/compose, ports, env and volumes.
echo   8. run Deploy.
echo   9. Abra logs.
echo   10. Teste a application in the navegador.
echo   11. only after configure Repo/Build/Procedure.
echo.
echo %YELLOW%example PRATICO:%RESET%
echo   - project with compose: crie Server -^> Stack -^> Deploy -^> logs -^> teste.
echo.
echo %RED%ERRORS common:%RESET%
echo   - Automatizar before of the first deploy manual.
echo   - changing varias configurations to the same tempo.
echo.
echo %GREEN%how SABER if FUNCIONOU:%RESET%
echo   - application responde and may be reiniciada by the Komodo.
echo.
echo %CYAN%next step:%RESET%
echo   chapter 20 for choose proximos resources.
goto GUIDE_PAUSE_19

:GUIDE_20
@echo off
cls
call :Header
echo %CYAN%%BOLD%   chapter 20 - QUERO FAZER X - QUAL tool?%RESET%
echo.
echo %YELLOW%O that and:%RESET%
echo   - Mapa of decisao quick.
echo.
echo %YELLOW%when / for that USAR:%RESET%
echo   - when souber o goal, mas not o nome of the tool.
echo.
echo %YELLOW%step A step:%RESET%
echo   1. Rodar 1 container -^> Deployment.
echo   2. Rodar docker-compose -^> Stack.
echo   3. Conectar outra machine -^> Server + Periphery.
echo   4. download codigo Git -^> Repo.
echo   5. create image Docker -^> Build + Builder.
echo   6. run varias tarefas with 1 clique -^> Procedure.
echo   7. Logica personalizada -^> Action.
echo   8. Guardar configuration in the Git -^> Resource Sync.
echo   9. Cluster Docker -^> Swarm.
echo   10. Receber alerts -^> Alerter.
echo   11. Investigar failure -^> Status + logs + Diagnostics.
echo.
echo %YELLOW%example PRATICO:%RESET%
echo   - Tenho compose.yaml: choose Stack, not Deployment.
echo.
echo %RED%ERRORS common:%RESET%
echo   - choose resource by the nome without relacionar to the goal.
echo.
echo %GREEN%how SABER if FUNCIONOU:%RESET%
echo   - you consegue choose a tool in less of 1 minuto.
echo.
echo %CYAN%next step:%RESET%
echo   Abra o chapter of the tool escolhida.
goto GUIDE_PAUSE_20

:GUIDE_21
@echo off
cls
call :Header
echo %CYAN%%BOLD%   chapter 21 - solution of problems%RESET%
echo.
echo %YELLOW%O that and:%RESET%
echo   - Roteiro for find causa of failures without tentativa aleatoria.
echo.
echo %YELLOW%when / for that USAR:%RESET%
echo   - dashboard, Server, Stack, Deployment, Build or Git falhando.
echo.
echo %YELLOW%step A step:%RESET%
echo   1. dashboard not abre -^> [5], [9], [10], [6].
echo   2. container reinicia -^> logs of the container + env + database + port.
echo   3. Stack not sobe -^> compose + env + images + volumes.
echo   4. image not baixa -^> nome/tag + registry + credencial.
echo   5. Git failure -^> token + permission + branch.
echo   6. Server offline -^> Periphery + firewall + endereco + port.
echo   7. Automacao failure -^> run each etapa manualmente.
echo.
echo %YELLOW%example PRATICO:%RESET%
echo   - Error 'port already in use': identifique process/service that ocupa a port before of changing qualquer outra coisa.
echo.
echo %RED%ERRORS common:%RESET%
echo   - restart without diagnostics.
echo   - delete data for tentar corrigir.
echo   - Ignorar mensagem of error exata.
echo.
echo %GREEN%how SABER if FUNCIONOU:%RESET%
echo   - you identifica a causa and consegue repetir a solution.
echo.
echo %CYAN%next step:%RESET%
echo   use [33] for consultar historico of errors of the BAT.
goto GUIDE_PAUSE_21

:GUIDE_22
@echo off
cls
call :Header
echo %CYAN%%BOLD%   chapter 22 - CHECKLIST before of production%RESET%
echo.
echo %YELLOW%O that and:%RESET%
echo   - Lista final for reduzir surpresas in uso real.
echo.
echo %YELLOW%when / for that USAR:%RESET%
echo   - before of considerar a environment ready.
echo.
echo %YELLOW%step A step:%RESET%
echo   1. Server Healthy.
echo   2. Disco/RAM suficientes.
echo   3. Firewall revisado.
echo   4. ports corretas.
echo   5. Env complete.
echo   6. volumes persistentes.
echo   7. logs without error critico.
echo   8. Secrets outside of the Git.
echo   9. Users with menor privilegio.
echo   10. database not exposto without necessidade.
echo   11. backup existing.
echo   12. alerts configurados.
echo   13. Rollback conhecido.
echo   14. update testada.
echo.
echo %YELLOW%example PRATICO:%RESET%
echo   - before of update production, saiba how back for a image/tag previous.
echo.
echo %RED%ERRORS common:%RESET%
echo   - without backup.
echo   - without rollback.
echo   - Password in file publico.
echo   - Tudo rodando how admin.
echo.
echo %GREEN%how SABER if FUNCIONOU:%RESET%
echo   - you consegue responder: how detect failure, how recuperar, how back versao and where are os data.
echo.
echo %CYAN%next step:%RESET%
echo   Environment muito more preparado for uso real.
goto GUIDE_PAUSE_22

:GUIDE_23
@echo off
cls
call :Header
echo %CYAN%%BOLD%   chapter 23 - completion: you already SABE OPERAR O KOMODO%RESET%
echo.
echo %YELLOW%O that you APRENDEU NESTE guide:%RESET%
echo   - o that and Server and Periphery;
echo   - a diferenca between Deployment and Stack;
echo   - how usar Git/Repo;
echo   - how Build and Builder trabalham juntos;
echo   - how automatizar with Procedures;
echo   - when usar Actions;
echo   - o that and Resource Sync;
echo   - when Swarm faz sentido;
echo   - how funcionam alerts;
echo   - users, permissions and Super Admin;
echo   - logs, diagnostics and investigacao;
echo   - variaveis, secrets and credentials;
echo   - checklist of production.
echo.
echo %YELLOW%O more important:%RESET%
echo you not needs decorar tudo.
echo O goal and saber:
echo   1. where procurar;
echo   2. qual resource usar;
echo   3. how verificar if funcionou;
echo   4. how investigar when falhar.
echo.
echo %GREEN%ROTINA recommended for USO real:%RESET%
echo   1. [1] Start Komodo.
echo   2. [8] Diagnostics if houver duvida.
echo   3. [4] open dashboard.
echo   4. Trabalhar in the resource desejado.
echo   5. Conferir logs after of change/deployar.
echo   6. Usar [33] if o proprio BAT registrar algum error.
echo.
echo %YELLOW%if you AINDA ESTIVER INSEGURO:%RESET%
echo Volte to the chapter 20:
echo   "Quero fazer X - qual tool?"
echo.
echo %YELLOW%if ALGO DER ERRADO:%RESET%
echo Volte to the chapter 21:
echo   "solution of problems".
echo.
echo %YELLOW%before of production:%RESET%
echo always use o chapter 22 how checklist.
echo.
echo %CYAN%next step:%RESET%
echo   Usar o Komodo in the pratica.
echo   O guide termina here, mas you may back a qualquer chapter when quiser.
goto GUIDE_PAUSE_23

:GUIDE_PAUSE_0
@echo off
echo.
echo %GRAY%--------------------------------------------------------------------------------%RESET%
echo %CYAN%next step recommended:%RESET% chapter 1
choice /C PBMC /N /M "[P] next step  [B] Back to the guide  [M] menu main  [C] Cancel: "
if errorlevel 4 goto MAIN
if errorlevel 3 goto MAIN
if errorlevel 2 goto GUIDE_MAIN
goto GUIDE_1

:GUIDE_PAUSE_1
@echo off
echo.
echo %GRAY%--------------------------------------------------------------------------------%RESET%
echo %CYAN%next step recommended:%RESET% chapter 2
choice /C PBMC /N /M "[P] next step  [B] Back to the guide  [M] menu main  [C] Cancel: "
if errorlevel 4 goto MAIN
if errorlevel 3 goto MAIN
if errorlevel 2 goto GUIDE_MAIN
goto GUIDE_2

:GUIDE_PAUSE_2
@echo off
echo.
echo %GRAY%--------------------------------------------------------------------------------%RESET%
echo %CYAN%next step recommended:%RESET% chapter 3
choice /C PBMC /N /M "[P] next step  [B] Back to the guide  [M] menu main  [C] Cancel: "
if errorlevel 4 goto MAIN
if errorlevel 3 goto MAIN
if errorlevel 2 goto GUIDE_MAIN
goto GUIDE_3

:GUIDE_PAUSE_3
@echo off
echo.
echo %GRAY%--------------------------------------------------------------------------------%RESET%
echo %CYAN%next step recommended:%RESET% chapter 4
choice /C PBMC /N /M "[P] next step  [B] Back to the guide  [M] menu main  [C] Cancel: "
if errorlevel 4 goto MAIN
if errorlevel 3 goto MAIN
if errorlevel 2 goto GUIDE_MAIN
goto GUIDE_4

:GUIDE_PAUSE_4
@echo off
echo.
echo %GRAY%--------------------------------------------------------------------------------%RESET%
echo %CYAN%next step recommended:%RESET% chapter 5
choice /C PBMC /N /M "[P] next step  [B] Back to the guide  [M] menu main  [C] Cancel: "
if errorlevel 4 goto MAIN
if errorlevel 3 goto MAIN
if errorlevel 2 goto GUIDE_MAIN
goto GUIDE_5

:GUIDE_PAUSE_5
@echo off
echo.
echo %GRAY%--------------------------------------------------------------------------------%RESET%
echo %CYAN%next step recommended:%RESET% chapter 6
choice /C PBMC /N /M "[P] next step  [B] Back to the guide  [M] menu main  [C] Cancel: "
if errorlevel 4 goto MAIN
if errorlevel 3 goto MAIN
if errorlevel 2 goto GUIDE_MAIN
goto GUIDE_6

:GUIDE_PAUSE_6
@echo off
echo.
echo %GRAY%--------------------------------------------------------------------------------%RESET%
echo %CYAN%next step recommended:%RESET% chapter 7
choice /C PBMC /N /M "[P] next step  [B] Back to the guide  [M] menu main  [C] Cancel: "
if errorlevel 4 goto MAIN
if errorlevel 3 goto MAIN
if errorlevel 2 goto GUIDE_MAIN
goto GUIDE_7

:GUIDE_PAUSE_7
@echo off
echo.
echo %GRAY%--------------------------------------------------------------------------------%RESET%
echo %CYAN%next step recommended:%RESET% chapter 8
choice /C PBMC /N /M "[P] next step  [B] Back to the guide  [M] menu main  [C] Cancel: "
if errorlevel 4 goto MAIN
if errorlevel 3 goto MAIN
if errorlevel 2 goto GUIDE_MAIN
goto GUIDE_8

:GUIDE_PAUSE_8
@echo off
echo.
echo %GRAY%--------------------------------------------------------------------------------%RESET%
echo %CYAN%next step recommended:%RESET% chapter 9
choice /C PBMC /N /M "[P] next step  [B] Back to the guide  [M] menu main  [C] Cancel: "
if errorlevel 4 goto MAIN
if errorlevel 3 goto MAIN
if errorlevel 2 goto GUIDE_MAIN
goto GUIDE_9

:GUIDE_PAUSE_9
@echo off
echo.
echo %GRAY%--------------------------------------------------------------------------------%RESET%
echo %CYAN%next step recommended:%RESET% chapter 10
choice /C PBMC /N /M "[P] next step  [B] Back to the guide  [M] menu main  [C] Cancel: "
if errorlevel 4 goto MAIN
if errorlevel 3 goto MAIN
if errorlevel 2 goto GUIDE_MAIN
goto GUIDE_10

:GUIDE_PAUSE_10
@echo off
echo.
echo %GRAY%--------------------------------------------------------------------------------%RESET%
echo %CYAN%next step recommended:%RESET% chapter 11
choice /C PBMC /N /M "[P] next step  [B] Back to the guide  [M] menu main  [C] Cancel: "
if errorlevel 4 goto MAIN
if errorlevel 3 goto MAIN
if errorlevel 2 goto GUIDE_MAIN
goto GUIDE_11

:GUIDE_PAUSE_11
@echo off
echo.
echo %GRAY%--------------------------------------------------------------------------------%RESET%
echo %CYAN%next step recommended:%RESET% chapter 12
choice /C PBMC /N /M "[P] next step  [B] Back to the guide  [M] menu main  [C] Cancel: "
if errorlevel 4 goto MAIN
if errorlevel 3 goto MAIN
if errorlevel 2 goto GUIDE_MAIN
goto GUIDE_12

:GUIDE_PAUSE_12
@echo off
echo.
echo %GRAY%--------------------------------------------------------------------------------%RESET%
echo %CYAN%next step recommended:%RESET% chapter 13
choice /C PBMC /N /M "[P] next step  [B] Back to the guide  [M] menu main  [C] Cancel: "
if errorlevel 4 goto MAIN
if errorlevel 3 goto MAIN
if errorlevel 2 goto GUIDE_MAIN
goto GUIDE_13

:GUIDE_PAUSE_13
@echo off
echo.
echo %GRAY%--------------------------------------------------------------------------------%RESET%
echo %CYAN%next step recommended:%RESET% chapter 14
choice /C PBMC /N /M "[P] next step  [B] Back to the guide  [M] menu main  [C] Cancel: "
if errorlevel 4 goto MAIN
if errorlevel 3 goto MAIN
if errorlevel 2 goto GUIDE_MAIN
goto GUIDE_14

:GUIDE_PAUSE_14
@echo off
echo.
echo %GRAY%--------------------------------------------------------------------------------%RESET%
echo %CYAN%next step recommended:%RESET% chapter 15
choice /C PBMC /N /M "[P] next step  [B] Back to the guide  [M] menu main  [C] Cancel: "
if errorlevel 4 goto MAIN
if errorlevel 3 goto MAIN
if errorlevel 2 goto GUIDE_MAIN
goto GUIDE_15

:GUIDE_PAUSE_15
@echo off
echo.
echo %GRAY%--------------------------------------------------------------------------------%RESET%
echo %CYAN%next step recommended:%RESET% chapter 16
choice /C PBMC /N /M "[P] next step  [B] Back to the guide  [M] menu main  [C] Cancel: "
if errorlevel 4 goto MAIN
if errorlevel 3 goto MAIN
if errorlevel 2 goto GUIDE_MAIN
goto GUIDE_16

:GUIDE_PAUSE_16
@echo off
echo.
echo %GRAY%--------------------------------------------------------------------------------%RESET%
echo %CYAN%next step recommended:%RESET% chapter 17
choice /C PBMC /N /M "[P] next step  [B] Back to the guide  [M] menu main  [C] Cancel: "
if errorlevel 4 goto MAIN
if errorlevel 3 goto MAIN
if errorlevel 2 goto GUIDE_MAIN
goto GUIDE_17

:GUIDE_PAUSE_17
@echo off
echo.
echo %GRAY%--------------------------------------------------------------------------------%RESET%
echo %CYAN%next step recommended:%RESET% chapter 18
choice /C PBMC /N /M "[P] next step  [B] Back to the guide  [M] menu main  [C] Cancel: "
if errorlevel 4 goto MAIN
if errorlevel 3 goto MAIN
if errorlevel 2 goto GUIDE_MAIN
goto GUIDE_18

:GUIDE_PAUSE_18
@echo off
echo.
echo %GRAY%--------------------------------------------------------------------------------%RESET%
echo %CYAN%next step recommended:%RESET% chapter 19
choice /C PBMC /N /M "[P] next step  [B] Back to the guide  [M] menu main  [C] Cancel: "
if errorlevel 4 goto MAIN
if errorlevel 3 goto MAIN
if errorlevel 2 goto GUIDE_MAIN
goto GUIDE_19

:GUIDE_PAUSE_19
@echo off
echo.
echo %GRAY%--------------------------------------------------------------------------------%RESET%
echo %CYAN%next step recommended:%RESET% chapter 20
choice /C PBMC /N /M "[P] next step  [B] Back to the guide  [M] menu main  [C] Cancel: "
if errorlevel 4 goto MAIN
if errorlevel 3 goto MAIN
if errorlevel 2 goto GUIDE_MAIN
goto GUIDE_20

:GUIDE_PAUSE_20
@echo off
echo.
echo %GRAY%--------------------------------------------------------------------------------%RESET%
echo %CYAN%next step recommended:%RESET% chapter 21
choice /C PBMC /N /M "[P] next step  [B] Back to the guide  [M] menu main  [C] Cancel: "
if errorlevel 4 goto MAIN
if errorlevel 3 goto MAIN
if errorlevel 2 goto GUIDE_MAIN
goto GUIDE_21

:GUIDE_PAUSE_21
@echo off
echo.
echo %GRAY%--------------------------------------------------------------------------------%RESET%
echo %CYAN%next step recommended:%RESET% chapter 22
choice /C PBMC /N /M "[P] next step  [B] Back to the guide  [M] menu main  [C] Cancel: "
if errorlevel 4 goto MAIN
if errorlevel 3 goto MAIN
if errorlevel 2 goto GUIDE_MAIN
goto GUIDE_22

:GUIDE_PAUSE_22
@echo off
echo.
echo %GRAY%--------------------------------------------------------------------------------%RESET%
echo %CYAN%next step recommended:%RESET% chapter 23
choice /C PBMC /N /M "[P] next step  [B] Back to the guide  [M] menu main  [C] Cancel: "
if errorlevel 4 goto MAIN
if errorlevel 3 goto MAIN
if errorlevel 2 goto GUIDE_MAIN
goto GUIDE_23

:GUIDE_PAUSE_23
@echo off
echo.
echo %GRAY%--------------------------------------------------------------------------------%RESET%
echo %CYAN%next step:%RESET% usar o Komodo in the pratica.
choice /C PBMC /N /M "[P] next step: menu main  [B] Back to the guide  [M] menu main  [C] Cancel: "
if errorlevel 4 goto MAIN
if errorlevel 3 goto MAIN
if errorlevel 2 goto GUIDE_MAIN
goto MAIN

:CONTROL_CENTER_LANGUAGE
@echo off
cls
call :Header
echo %CYAN%%BOLD%   CONTROL CENTER LANGUAGE%RESET%
echo.
echo V18 starts directly in English to guarantee immediate loading by double click.
echo Komodo WEB translations remain available in option 31.
echo.
echo Planned/compatible Control Center language packs:
echo   [1] English - Original / Restore
echo   [2] Portugues do Brasil - FULL SAFE PACK
echo   [3] Spanish - FULL SAFE PACK
echo   [4] German - FULL SAFE PACK
echo   [5] Japanese - FULL SAFE PACK
echo   [6] French - FULL SAFE PACK
echo.
echo The V17 startup extractor was removed because it could prevent the menu from loading.
echo In V18 this option never blocks startup.
echo.
choice /C BMC /N /M "[B] Back  [M] Main menu  [C] Cancel: "
goto MAIN

:DOCUMENTATION_CENTER
@echo off
cls
call :Header
echo %CYAN%%BOLD%   DOCUMENTATION / DOCUMENTACAO - INSIDE THE MENU%RESET%
echo.
echo   [1] English documentation - read inside this menu
echo   [2] Documentacao Portugues-BR - ler dentro deste menu
echo   [3] Export EN + PT-BR Markdown files
echo   [B] Back / Voltar
echo.
choice /C 123B /N /M "Choose / Escolha: "
if errorlevel 4 goto MAIN
if errorlevel 3 goto DOCUMENTATION_EXPORT
if errorlevel 2 goto DOC_PT_1
goto DOC_EN_1

:DOCUMENTATION_EXPORT
@echo off
call :WriteControlCenterDocs
if errorlevel 1 (
    call :MsgError "Documentation export failed."
    goto DOC_PAUSE
)
call :MsgOk "Documentation exported to .komodo-windows\docs"
start "" "%STATE_DIR%\docs"
goto DOC_PAUSE

:DOC_EN_1
@echo off
cls
call :Header
echo %CYAN%%BOLD%   DOCUMENTATION EN - 1/6 - WHAT THIS CONTROL CENTER IS%RESET%
echo.
echo Komodo Control Center is a Windows companion for Komodo.
echo It centralizes lifecycle, Docker, diagnostics, users, language,
echo documentation, backups, support and GitHub contribution workflows.
echo.
echo Main principle:
echo   common operations should be available without manually typing commands.
echo.
echo It does NOT replace Komodo. It helps operate the local Komodo environment.
goto DOC_EN_PAUSE_1

:DOC_EN_2
@echo off
cls
call :Header
echo %CYAN%%BOLD%   DOCUMENTATION EN - 2/6 - MAIN AREAS%RESET%
echo.
echo Operations      : Start, Stop, Restart, Dashboard.
echo Monitoring      : Status, Core logs, all logs, HTTP health, port 9120.
echo Maintenance     : Docker image updates, Git update, environment.
echo Windows         : Explorer, PowerShell, VS Code, shortcut, startup.
echo Users           : list users, password reset, Super Admin, signup.
echo Languages       : Control Center language + live Komodo UI languages.
echo Support         : logs, notifications, Support Bundle, backups.
echo GitHub          : fork + branch + Pull Request contribution workflow.
goto DOC_EN_PAUSE_2

:DOC_EN_3
@echo off
cls
call :Header
echo %CYAN%%BOLD%   DOCUMENTATION EN - 3/6 - SAFE OPERATION%RESET%
echo.
echo Normal Stop preserves volumes.
echo Full reset with volumes is separated and requires confirmation.
echo Password arguments are redacted from Control Center command logs.
echo Safe Support Bundle does not copy compose.env or secret files.
echo GitHub publication stages only the Control Center contribution files.
echo It never intentionally uses "git add ." for publication.
goto DOC_EN_PAUSE_3

:DOC_EN_4
@echo off
cls
call :Header
echo %CYAN%%BOLD%   DOCUMENTATION EN - 4/6 - GITHUB CONTRIBUTION%RESET%
echo.
echo Official upstream : moghtech/komodo
echo Contributor fork  : detected from the authenticated GitHub CLI account.
echo Branch            : contrib/windows-control-center
echo Target            : moghtech/komodo:main
echo.
echo The menu verifies/creates the authenticated user's fork, prepares an
echo isolated temporary workspace, pushes the branch, and creates or updates PR.
echo Maintainers decide whether the contribution is merged.
goto DOC_EN_PAUSE_4

:DOC_EN_5
@echo off
cls
call :Header
echo %CYAN%%BOLD%   DOCUMENTATION EN - 5/6 - LANGUAGES%RESET%
echo.
echo FULL SAFE PACK means:
echo   - built-in user-facing UI strings are translated;
echo   - dynamic UI is monitored and translated where supported;
echo   - resource names created by users are NOT modified;
echo   - commands, code, logs, paths, URLs, IDs, tokens and secrets stay intact.
echo.
echo This safety rule avoids changing technical values that could break Komodo.
goto DOC_EN_PAUSE_5

:DOC_EN_6
@echo off
cls
call :Header
echo %CYAN%%BOLD%   DOCUMENTATION EN - 6/6 - LEARNING AND TROUBLESHOOTING%RESET%
echo.
echo Beginner: open option 34, start at Chapter 0 and keep selecting Next Step.
echo.
echo Recommended troubleshooting order:
echo   1. Intelligent diagnostics
echo   2. Service status
echo   3. Core logs
echo   4. All-service logs
echo   5. HTTP health
echo   6. Port/process inspection
echo   7. Control Center error logs
echo.
echo Documentation complete.
goto DOC_END_PAUSE

:DOC_PT_1
@echo off
cls
call :Header
echo %CYAN%%BOLD%   DOCUMENTACAO PT-BR - 1/6 - O QUE E O CONTROL CENTER%RESET%
echo.
echo O Komodo Control Center e um companheiro Windows para o Komodo.
echo Centraliza operacao, Docker, diagnostico, usuarios, idiomas,
echo documentacao, backups, suporte e contribuicoes GitHub.
echo.
echo Principio principal:
echo   operacoes comuns devem funcionar sem digitar comandos manualmente.
echo.
echo Ele nao substitui o Komodo. Ele ajuda a operar o ambiente local.
goto DOC_PT_PAUSE_1

:DOC_PT_2
@echo off
cls
call :Header
echo %CYAN%%BOLD%   DOCUMENTACAO PT-BR - 2/6 - AREAS PRINCIPAIS%RESET%
echo.
echo Operacao        : Iniciar, Parar, Reiniciar, Dashboard.
echo Monitoramento   : Status, logs, health HTTP, porta 9120.
echo Manutencao      : imagens Docker, Git, ambiente.
echo Windows         : Explorer, PowerShell, VS Code, atalho, inicializacao.
echo Usuarios        : lista, reset senha, Super Admin, cadastro.
echo Idiomas         : Control Center + traducao ao vivo do Komodo.
echo Suporte         : logs, notificacoes, Support Bundle, backups.
echo GitHub          : fork + branch + Pull Request.
goto DOC_PT_PAUSE_2

:DOC_PT_3
@echo off
cls
call :Header
echo %CYAN%%BOLD%   DOCUMENTACAO PT-BR - 3/6 - OPERACAO SEGURA%RESET%
echo.
echo Parada normal preserva volumes.
echo Reset total com volumes fica separado e exige confirmacao.
echo Senhas sao ocultadas dos logs de comandos do Control Center.
echo Support Bundle seguro nao copia compose.env nem arquivos de secrets.
echo A publicacao GitHub adiciona somente arquivos da contribuicao.
echo Ela nao deve usar "git add ." para publicar.
goto DOC_PT_PAUSE_3

:DOC_PT_4
@echo off
cls
call :Header
echo %CYAN%%BOLD%   DOCUMENTACAO PT-BR - 4/6 - CONTRIBUICAO GITHUB%RESET%
echo.
echo Repositorio oficial : moghtech/komodo
echo Fork do contribuidor : detectado pela conta autenticada no GitHub CLI.
echo Branch               : contrib/windows-control-center
echo Destino              : moghtech/komodo:main
echo.
echo O menu verifica/cria o fork, usa workspace temporario isolado,
echo envia a branch e cria ou atualiza o Pull Request.
echo Os mantenedores decidem se a contribuicao sera aceita.
goto DOC_PT_PAUSE_4

:DOC_PT_5
@echo off
cls
call :Header
echo %CYAN%%BOLD%   DOCUMENTACAO PT-BR - 5/6 - FULL SAFE PACK%RESET%
echo.
echo FULL SAFE PACK significa:
echo   - textos internos da interface sao traduzidos;
echo   - UI dinamica e acompanhada onde suportado;
echo   - nomes criados pelo usuario NAO sao alterados;
echo   - comandos, codigo, logs, paths, URLs, IDs, tokens e secrets ficam intactos.
echo.
echo Essa regra evita alterar valores tecnicos que poderiam quebrar o Komodo.
goto DOC_PT_PAUSE_5

:DOC_PT_6
@echo off
cls
call :Header
echo %CYAN%%BOLD%   DOCUMENTACAO PT-BR - 6/6 - APRENDER E DIAGNOSTICAR%RESET%
echo.
echo Iniciante: abra a opcao 34, comece no Capitulo 0 e use Proximo Passo.
echo.
echo Ordem recomendada quando algo falhar:
echo   1. Diagnostico inteligente
echo   2. Status dos servicos
echo   3. Logs do Core
echo   4. Logs gerais
echo   5. Health HTTP
echo   6. Porta/processos
echo   7. Logs de erro do Control Center
echo.
echo Documentacao concluida.
goto DOC_END_PAUSE

:DOC_EN_PAUSE_1
choice /C NBMC /N /M "[N] Next [B] Documentation menu [M] Main menu [C] Cancel: "
if errorlevel 4 goto MAIN
if errorlevel 3 goto MAIN
if errorlevel 2 goto DOCUMENTATION_CENTER
goto DOC_EN_2
:DOC_EN_PAUSE_2
choice /C NPBMC /N /M "[N] Next [P] Previous [B] Docs [M] Main [C] Cancel: "
if errorlevel 5 goto MAIN
if errorlevel 4 goto MAIN
if errorlevel 3 goto DOCUMENTATION_CENTER
if errorlevel 2 goto DOC_EN_1
goto DOC_EN_3

:DOC_EN_PAUSE_3
choice /C NPBMC /N /M "[N] Next [P] Previous [B] Docs [M] Main [C] Cancel: "
if errorlevel 5 goto MAIN
if errorlevel 4 goto MAIN
if errorlevel 3 goto DOCUMENTATION_CENTER
if errorlevel 2 goto DOC_EN_2
goto DOC_EN_4

:DOC_EN_PAUSE_4
choice /C NPBMC /N /M "[N] Next [P] Previous [B] Docs [M] Main [C] Cancel: "
if errorlevel 5 goto MAIN
if errorlevel 4 goto MAIN
if errorlevel 3 goto DOCUMENTATION_CENTER
if errorlevel 2 goto DOC_EN_3
goto DOC_EN_5

:DOC_EN_PAUSE_5
choice /C NPBMC /N /M "[N] Next [P] Previous [B] Docs [M] Main [C] Cancel: "
if errorlevel 5 goto MAIN
if errorlevel 4 goto MAIN
if errorlevel 3 goto DOCUMENTATION_CENTER
if errorlevel 2 goto DOC_EN_4
goto DOC_EN_6

:DOC_PT_PAUSE_1
choice /C NBMC /N /M "[N] Proximo [B] Documentacao [M] Menu principal [C] Cancelar: "
if errorlevel 4 goto MAIN
if errorlevel 3 goto MAIN
if errorlevel 2 goto DOCUMENTATION_CENTER
goto DOC_PT_2
:DOC_PT_PAUSE_2
choice /C NPBMC /N /M "[N] Proximo [P] Anterior [B] Documentacao [M] Menu [C] Cancelar: "
if errorlevel 5 goto MAIN
if errorlevel 4 goto MAIN
if errorlevel 3 goto DOCUMENTATION_CENTER
if errorlevel 2 goto DOC_PT_1
goto DOC_PT_3

:DOC_PT_PAUSE_3
choice /C NPBMC /N /M "[N] Proximo [P] Anterior [B] Documentacao [M] Menu [C] Cancelar: "
if errorlevel 5 goto MAIN
if errorlevel 4 goto MAIN
if errorlevel 3 goto DOCUMENTATION_CENTER
if errorlevel 2 goto DOC_PT_2
goto DOC_PT_4

:DOC_PT_PAUSE_4
choice /C NPBMC /N /M "[N] Proximo [P] Anterior [B] Documentacao [M] Menu [C] Cancelar: "
if errorlevel 5 goto MAIN
if errorlevel 4 goto MAIN
if errorlevel 3 goto DOCUMENTATION_CENTER
if errorlevel 2 goto DOC_PT_3
goto DOC_PT_5

:DOC_PT_PAUSE_5
choice /C NPBMC /N /M "[N] Proximo [P] Anterior [B] Documentacao [M] Menu [C] Cancelar: "
if errorlevel 5 goto MAIN
if errorlevel 4 goto MAIN
if errorlevel 3 goto DOCUMENTATION_CENTER
if errorlevel 2 goto DOC_PT_4
goto DOC_PT_6

:DOC_END_PAUSE
echo.
choice /C BMC /N /M "[B] Documentation / Documentacao [M] Main menu [C] Cancel: "
if errorlevel 3 goto MAIN
if errorlevel 2 goto MAIN
goto DOCUMENTATION_CENTER

:DOC_PAUSE
echo.
choice /C BMC /N /M "[B] Documentation [M] Main [C] Cancel: "
if errorlevel 3 goto MAIN
if errorlevel 2 goto MAIN
goto DOCUMENTATION_CENTER

:SUPPORT_BUNDLE
@echo off
cls
call :Header
echo %CYAN%%BOLD%   SAFE SUPPORT BUNDLE%RESET%
echo.
echo Cria a pacote for diagnostics without copy compose.env, passwords or secrets.
echo.
choice /C SBC /N /M "create now? [S] Yes  [B] Back  [C] Cancel: "
if errorlevel 3 goto MAIN
if errorlevel 2 goto MAIN
set "SUPPORT_DIR=%STATE_DIR%\support-%RANDOM%-%RANDOM%"
mkdir "%SUPPORT_DIR%" >nul 2>&1
>"%SUPPORT_DIR%\summary.txt" echo Komodo Control Center Support Bundle
>>"%SUPPORT_DIR%\summary.txt" echo Generated: %DATE% %TIME%
>>"%SUPPORT_DIR%\summary.txt" echo Root: %KOMODO_ROOT%
>>"%SUPPORT_DIR%\summary.txt" echo Compose: %COMPOSE_FILE%
docker version >"%SUPPORT_DIR%\docker-version.txt" 2>&1
docker ps -a >"%SUPPORT_DIR%\docker-ps.txt" 2>&1
docker compose version >"%SUPPORT_DIR%\compose-version.txt" 2>&1
git --version >"%SUPPORT_DIR%\git-version.txt" 2>&1
curl.exe -fsS --max-time 5 "%DASHBOARD_URL%" >"%SUPPORT_DIR%\http-check.txt" 2>&1
if exist "%LOG_FILE%" copy /Y "%LOG_FILE%" "%SUPPORT_DIR%\control-center.log" >nul
if exist "%ERROR_LOG%" copy /Y "%ERROR_LOG%" "%SUPPORT_DIR%\errors.log" >nul
set "SUPPORT_ZIP=%STATE_DIR%\Komodo-Support-%RANDOM%.zip"
powershell.exe -NoProfile -Command "Compress-Archive -Path '%SUPPORT_DIR%\*' -DestinationPath '%SUPPORT_ZIP%' -Force" >nul 2>&1
if errorlevel 1 (
    call :MsgError "Could not create ZIP. A folder of suporte was mantida."
    start "" "%SUPPORT_DIR%"
    goto PAUSE_MAIN
)
rmdir /S /Q "%SUPPORT_DIR%" >nul 2>&1
call :MsgOk "Support Bundle created without files of secrets."
echo %SUPPORT_ZIP%
start "" "%STATE_DIR%"
goto PAUSE_MAIN


:SAFE_CONFIG_BACKUP
@echo off
cls
call :Header
echo %CYAN%%BOLD%   SAFE CONFIG backup%RESET%
echo.
echo Cria copy local of the files of configuration importantes.
echo O backup may conter credentials of the environment: mantenha-o privado.
echo.
choice /C SBC /N /M "create backup now? [S] Yes  [B] Back  [C] Cancel: "
if errorlevel 3 goto MAIN
if errorlevel 2 goto MAIN
set "BKP_DIR=%STATE_DIR%\backup-%RANDOM%-%RANDOM%"
mkdir "%BKP_DIR%" >nul 2>&1
if defined ENV_FILE if exist "%ENV_FILE%" copy /Y "%ENV_FILE%" "%BKP_DIR%\compose.env" >nul
if defined COMPOSE_FILE if exist "%COMPOSE_FILE%" copy /Y "%COMPOSE_FILE%" "%BKP_DIR%\compose.yaml" >nul
if exist "%KOMODO_ROOT%\config\" xcopy "%KOMODO_ROOT%\config" "%BKP_DIR%\config\" /E /I /Y >nul 2>&1
call :MsgOk "backup local created."
echo %BKP_DIR%
start "" "%BKP_DIR%"
goto PAUSE_MAIN


:QUICK_WIZARD
@echo off
cls
call :Header
echo %CYAN%%BOLD%   QUICK ASSISTANT - WHAT of the YOU WANT TO of the?%RESET%
echo.
echo   [1] open o Komodo                  -> dashboard
echo   [2] Komodo not abre                -> diagnostics
echo   [3] view error                       -> logs
echo   [4] Gerenciar users             -> users real
echo   [5] create/recuperar shortcut         -> Desktop
echo   [6] learn Komodo of the zero        -> guide complete
echo   [7] publish contribution GitHub    -> fork + PR
echo   [8] changing language deste Control Center
echo   [9] translate interface web of the Komodo
echo.
choice /C 123456789BC /N /M "choose [1-9] [B] Back [C] Cancel: "
if errorlevel 11 goto MAIN
if errorlevel 10 goto MAIN
if errorlevel 9 goto KOMODO_LANGUAGE_CENTER
if errorlevel 8 goto CONTROL_CENTER_LANGUAGE
if errorlevel 7 goto GITHUB_PUBLISH_CENTER
if errorlevel 6 goto GUIDE_MAIN
if errorlevel 5 goto SHORTCUT
if errorlevel 4 goto LIST_REAL_USERS
if errorlevel 3 goto LOG_CORE
if errorlevel 2 goto DIAG
goto OPEN_WEB


:KOMODO_LANGUAGE_CENTER
@echo off
cls
call :Header
echo %MAGENTA%%BOLD%   TRANSLATE KOMODO LANGUAGES - LIVE%RESET%
echo.
echo All supported translation packs use the FULL SAFE PACK profile:
echo interface text is translated while technical/user data remains untouched.
echo.
echo   [1] English - Original / Restore
echo   [2] Portugues do Brasil - FULL SAFE PACK
echo   [3] Spanish - FULL SAFE PACK
echo   [4] German - FULL SAFE PACK
echo   [5] Japanese - FULL SAFE PACK
echo   [6] French - FULL SAFE PACK
echo.
echo   [B] Back / Voltar   [C] Cancel
echo.
choice /C 123456BC /N /M "Choose language: "
if errorlevel 8 goto MAIN
if errorlevel 7 goto MAIN
if errorlevel 6 (
    set "GEN_LANG=fr"
    goto GENERIC_KOMODO_TRANSLATE
)
if errorlevel 5 (
    set "GEN_LANG=ja"
    goto GENERIC_KOMODO_TRANSLATE
)
if errorlevel 4 (
    set "GEN_LANG=de"
    goto GENERIC_KOMODO_TRANSLATE
)
if errorlevel 3 (
    set "GEN_LANG=es"
    goto GENERIC_KOMODO_TRANSLATE
)
if errorlevel 2 goto TRANSLATE_PTBR
goto RESTORE_ENGLISH

:GENERIC_KOMODO_TRANSLATE
@echo off
cls
call :Header
echo %MAGENTA%%BOLD%   LIVE LANGUAGE PACK: %GEN_LANG%%RESET%
echo.
echo this pack traduz os elementos common and dinamicos with MutationObserver.
echo Nomes of resources, commands, logs, paths and data tecnicos not are alterados.
echo.
call :ApplyGenericLang "%GEN_LANG%"
if errorlevel 1 (
    call :MsgError "Language pack could not be applied."
    goto PAUSE_MAIN
)
call :MsgOk "Language pack applied. Return to Komodo and press F5."
goto PAUSE_MAIN

:AUTONOMOUS_CENTER
@echo off
cls
call :Header
echo %GREEN%%BOLD%   KOMODO AUTONOMOUS CENTER%RESET%
echo.
echo Use Komodo resources without manually typing normal Komodo commands.
echo.
echo   [1] Servers - onboard / list / rename / delete / rotate keys / terminal
echo   [2] Swarms - list / API operations
echo   [3] Stacks - list / deploy / API operations
echo   [4] Deployments - list / deploy / API operations
echo   [5] Repos - list / API operations
echo   [6] Builds - list / run build / API operations
echo   [7] Builders - list / API operations
echo   [8] Procedures - list / run
echo   [9] Actions - list / run
echo   [10] Resource Syncs - list / execute sync
echo   [11] Alerters - list / API operations
echo   [12] Schedules - list
echo   [13] Terminals - list / connect
echo   [14] Variables - set/update
echo   [15] Users / authentication
echo   [16] Onboarding keys / API keys
echo   [17] Database backup / restore tools
echo   [18] Generic resource CREATE / UPDATE / DELETE
echo   [19] Complete resource overview
echo   [20] Universal Komodo API runner
echo.
echo   [B] Back   [C] Cancel
echo.
set "AOP="
set /p "AOP=Choose: "
if /I "%AOP%"=="B" goto MAIN
if /I "%AOP%"=="C" goto MAIN
if "%AOP%"=="1" goto AUTO_SERVERS
if "%AOP%"=="2" set "AUTO_RESOURCE=Swarms"&goto AUTO_RESOURCE_MENU
if "%AOP%"=="3" set "AUTO_RESOURCE=Stacks"&goto AUTO_STACKS
if "%AOP%"=="4" set "AUTO_RESOURCE=Deployments"&goto AUTO_DEPLOYMENTS
if "%AOP%"=="5" set "AUTO_RESOURCE=Repos"&goto AUTO_RESOURCE_MENU
if "%AOP%"=="6" goto AUTO_BUILDS
if "%AOP%"=="7" set "AUTO_RESOURCE=Builders"&goto AUTO_RESOURCE_MENU
if "%AOP%"=="8" goto AUTO_PROCEDURES
if "%AOP%"=="9" goto AUTO_ACTIONS
if "%AOP%"=="10" goto AUTO_SYNCS
if "%AOP%"=="11" set "AUTO_RESOURCE=Alerters"&goto AUTO_RESOURCE_MENU
if "%AOP%"=="12" goto AUTO_SCHEDULES
if "%AOP%"=="13" goto AUTO_TERMINALS
if "%AOP%"=="14" goto AUTO_VARIABLES
if "%AOP%"=="15" goto AUTO_USERS
if "%AOP%"=="16" goto AUTO_KEYS
if "%AOP%"=="17" goto AUTO_DATABASE
if "%AOP%"=="18" goto AUTO_GENERIC_CRUD
if "%AOP%"=="19" goto RESOURCE_OVERVIEW
if "%AOP%"=="20" goto UNIVERSAL_API
call :MsgError "Invalid autonomous option."
goto AUTONOMOUS_CENTER


:AUTO_SERVERS
@echo off
cls
call :Header
echo %GREEN%%BOLD%   AUTONOMOUS SERVERS%RESET%
echo.
echo   [1] List servers
echo   [2] Add Linux server automatically through SSH + Periphery onboarding
echo   [3] Create onboarding key
echo   [4] Rename server
echo   [5] Delete server
echo   [6] Rotate server keys
echo   [7] Open Komodo terminal on server
echo   [8] Read server JSON through API
echo   [B] Back
echo.
set "SOP="
set /p "SOP=Choose: "
if /I "%SOP%"=="B" goto AUTONOMOUS_CENTER
if "%SOP%"=="1" (
    call :KmApi ls servers -a
    goto AUTO_PAUSE_SERVERS
)
if "%SOP%"=="2" goto AUTO_SERVER_ONBOARD
if "%SOP%"=="3" goto AUTO_ONBOARDING_KEY
if "%SOP%"=="4" goto AUTO_SERVER_RENAME
if "%SOP%"=="5" goto AUTO_SERVER_DELETE
if "%SOP%"=="6" goto AUTO_SERVER_ROTATE
if "%SOP%"=="7" goto AUTO_SERVER_TERMINAL
if "%SOP%"=="8" (
    set "API_MODE=read"
    set "API_TYPE=ListServers"
    set "API_JSON={}"
    call :KomodoApiCall
    goto AUTO_PAUSE_SERVERS
)
goto AUTO_SERVERS

:AUTO_SERVER_ONBOARD
@echo off
cls
call :Header
echo %GREEN%%BOLD%   ADD SERVER AUTOMATICALLY%RESET%
echo.
echo Official Komodo onboarding flow:
echo   create onboarding key - install Periphery - server appears in Komodo.
echo.
set "SSH_TARGET="
set /p "SSH_TARGET=SSH target, example root@192.168.1.50 [B=back]: "
if /I "%SSH_TARGET%"=="B" goto AUTO_SERVERS
set "NEW_SERVER_NAME="
set /p "NEW_SERVER_NAME=Server name in Komodo, example production-01 [B=back]: "
if /I "%NEW_SERVER_NAME%"=="B" goto AUTO_SERVERS
set "PUBLIC_CORE="
set /p "PUBLIC_CORE=Core address reachable FROM that server, example https://komodo.example.com [B=back]: "
if /I "%PUBLIC_CORE%"=="B" goto AUTO_SERVERS
echo.
echo Creating a temporary privileged onboarding key...
set "ONBOARD_TMP=%STATE_DIR%\onboarding-%RANDOM%.txt"
call :Compose exec -T core km create onboarding-key "control-center-%RANDOM%" --privileged >"%ONBOARD_TMP%" 2>&1
type "%ONBOARD_TMP%"
set "ONBOARD_KEY="
for /F "tokens=* delims=" %%L in ('findstr /R /C:"O-[A-Za-z0-9_-]*" "%ONBOARD_TMP%"') do (
    for %%K in (%%L) do echo %%K|findstr /B /C:"O-" >nul && set "ONBOARD_KEY=%%K"
)
if not defined ONBOARD_KEY (
    echo.
    call :MsgError "Could not automatically extract onboarding key from km output."
    echo The output was kept at:
    echo %ONBOARD_TMP%
    echo.
    set /p "ONBOARD_KEY=Paste the O-... onboarding key here or B to cancel: "
    if /I "%ONBOARD_KEY%"=="B" goto AUTO_SERVERS
)
echo.
echo Connecting by SSH and installing Periphery...
echo The SSH client may ask for host confirmation or your SSH credential.
ssh "%SSH_TARGET%" "curl -sSL https://raw.githubusercontent.com/moghtech/komodo/main/scripts/setup-periphery.py | python3 - --core-address=\"%PUBLIC_CORE%\" --connect-as=\"%NEW_SERVER_NAME%\" --onboarding-key=\"%ONBOARD_KEY%\""
if errorlevel 1 (
    call :MsgError "Remote Periphery installation/onboarding failed."
    goto AUTO_PAUSE_SERVERS
)
call :MsgOk "Remote setup completed. Waiting for Komodo to receive the server..."
timeout /t 5 /nobreak >nul
call :KmApi ls servers -a
goto AUTO_PAUSE_SERVERS

:AUTO_ONBOARDING_KEY
@echo off
cls
call :Header
set "OKNAME="
set /p "OKNAME=Onboarding key name [B=back]: "
if /I "%OKNAME%"=="B" goto AUTO_SERVERS
echo.
choice /C YNBC /N /M "Privileged key? [Y] Yes [N] No [B] Back [C] Cancel: "
if errorlevel 4 goto AUTO_SERVERS
if errorlevel 3 goto AUTO_SERVERS
if errorlevel 2 (
    call :Compose exec -T core km create onboarding-key "%OKNAME%"
) else (
    call :Compose exec -T core km create onboarding-key "%OKNAME%" --privileged
)
goto AUTO_PAUSE_SERVERS

:AUTO_SERVER_RENAME
set "SRV_ID="
set /p "SRV_ID=Current server name/id [B=back]: "
if /I "%SRV_ID%"=="B" goto AUTO_SERVERS
set "SRV_NEW="
set /p "SRV_NEW=New name: "
set "API_MODE=write"
set "API_TYPE=RenameServer"
set "API_JSON={\"id\":\"%SRV_ID%\",\"name\":\"%SRV_NEW%\"}"
call :KomodoApiCall
goto AUTO_PAUSE_SERVERS

:AUTO_SERVER_DELETE
set "SRV_ID="
set /p "SRV_ID=Server name/id to DELETE [B=back]: "
if /I "%SRV_ID%"=="B" goto AUTO_SERVERS
choice /C YN /N /M "Really delete this Komodo server resource? [Y/N]: "
if errorlevel 2 goto AUTO_SERVERS
set "API_MODE=write"
set "API_TYPE=DeleteServer"
set "API_JSON={\"id\":\"%SRV_ID%\"}"
call :KomodoApiCall
goto AUTO_PAUSE_SERVERS

:AUTO_SERVER_ROTATE
set "SRV_ID="
set /p "SRV_ID=Server name/id [B=back]: "
if /I "%SRV_ID%"=="B" goto AUTO_SERVERS
set "API_MODE=write"
set "API_TYPE=RotateServerKeys"
set "API_JSON={\"server\":\"%SRV_ID%\"}"
call :KomodoApiCall
goto AUTO_PAUSE_SERVERS

:AUTO_SERVER_TERMINAL
set "SRV_ID="
set /p "SRV_ID=Server name [B=back]: "
if /I "%SRV_ID%"=="B" goto AUTO_SERVERS
call :KmApi ssh "%SRV_ID%"
goto AUTO_PAUSE_SERVERS

:AUTO_PAUSE_SERVERS
echo.
choice /C BM /N /M "[B] Servers  [M] Main menu: "
if errorlevel 2 goto MAIN
goto AUTO_SERVERS


:AUTO_RESOURCE_MENU
@echo off
cls
call :Header
echo %GREEN%%BOLD%   RESOURCE: %AUTO_RESOURCE%%RESET%
echo.
echo   [1] List
echo   [2] Generic CREATE / UPDATE / DELETE
echo   [3] Universal API for this or any resource
echo   [B] Back
echo.
choice /C 123B /N /M "Choose: "
if errorlevel 4 goto AUTONOMOUS_CENTER
if errorlevel 3 goto UNIVERSAL_API
if errorlevel 2 goto AUTO_GENERIC_CRUD
call :KmApi ls %AUTO_RESOURCE% -a
goto AUTO_RESOURCE_PAUSE

:AUTO_RESOURCE_PAUSE
echo.
choice /C BM /N /M "[B] Resource menu  [M] Main: "
if errorlevel 2 goto MAIN
goto AUTO_RESOURCE_MENU


:AUTO_STACKS
@echo off
cls
call :Header
echo %GREEN%%BOLD%   AUTONOMOUS STACKS%RESET%
echo.
echo   [1] List stacks
echo   [2] Deploy stack
echo   [3] Generic CREATE / UPDATE / DELETE
echo   [4] Universal API
echo   [B] Back
choice /C 1234B /N /M "Choose: "
if errorlevel 5 goto AUTONOMOUS_CENTER
if errorlevel 4 goto UNIVERSAL_API
if errorlevel 3 (
    set "AUTO_RESOURCE=Stack"
    goto AUTO_GENERIC_CRUD
)
if errorlevel 2 (
    set "RUN_NAME="
    set /p "RUN_NAME=Stack name [B=back]: "
    if /I "%RUN_NAME%"=="B" goto AUTO_STACKS
    call :KmApi deploy stack "%RUN_NAME%"
    goto AUTO_STACKS
)
call :KmApi ls stacks -a
goto AUTO_STACKS


:AUTO_DEPLOYMENTS
@echo off
cls
call :Header
echo %GREEN%%BOLD%   AUTONOMOUS DEPLOYMENTS%RESET%
echo.
echo   [1] List deployments
echo   [2] Deploy deployment
echo   [3] Generic CREATE / UPDATE / DELETE
echo   [4] Universal API
echo   [B] Back
choice /C 1234B /N /M "Choose: "
if errorlevel 5 goto AUTONOMOUS_CENTER
if errorlevel 4 goto UNIVERSAL_API
if errorlevel 3 (
    set "AUTO_RESOURCE=Deployment"
    goto AUTO_GENERIC_CRUD
)
if errorlevel 2 (
    set "RUN_NAME="
    set /p "RUN_NAME=Deployment name [B=back]: "
    if /I "%RUN_NAME%"=="B" goto AUTO_DEPLOYMENTS
    call :KmApi deploy deployment "%RUN_NAME%"
    goto AUTO_DEPLOYMENTS
)
call :KmApi ls deployments -a
goto AUTO_DEPLOYMENTS


:AUTO_BUILDS
@echo off
cls
call :Header
echo %GREEN%%BOLD%   AUTONOMOUS BUILDS%RESET%
echo.
echo   [1] List builds
echo   [2] Run build
echo   [3] Generic CREATE / UPDATE / DELETE
echo   [B] Back
choice /C 123B /N /M "Choose: "
if errorlevel 4 goto AUTONOMOUS_CENTER
if errorlevel 3 (
    set "AUTO_RESOURCE=Build"
    goto AUTO_GENERIC_CRUD
)
if errorlevel 2 (
    set "RUN_NAME="
    set /p "RUN_NAME=Build name [B=back]: "
    if /I "%RUN_NAME%"=="B" goto AUTO_BUILDS
    call :KmApi build "%RUN_NAME%" -y
    goto AUTO_BUILDS
)
call :KmApi ls builds -a
goto AUTO_BUILDS


:AUTO_PROCEDURES
@echo off
cls
call :Header
echo %GREEN%%BOLD%   AUTONOMOUS PROCEDURES%RESET%
echo.
echo   [1] List procedures
echo   [2] Run procedure
echo   [3] Generic CREATE / UPDATE / DELETE
echo   [B] Back
choice /C 123B /N /M "Choose: "
if errorlevel 4 goto AUTONOMOUS_CENTER
if errorlevel 3 (
    set "AUTO_RESOURCE=Procedure"
    goto AUTO_GENERIC_CRUD
)
if errorlevel 2 (
    set "RUN_NAME="
    set /p "RUN_NAME=Procedure name [B=back]: "
    if /I "%RUN_NAME%"=="B" goto AUTO_PROCEDURES
    call :KmApi run procedure "%RUN_NAME%" -y
    goto AUTO_PROCEDURES
)
call :KmApi ls procedures -a
goto AUTO_PROCEDURES


:AUTO_ACTIONS
@echo off
cls
call :Header
echo %GREEN%%BOLD%   AUTONOMOUS ACTIONS%RESET%
echo.
echo   [1] List actions
echo   [2] Run action
echo   [3] Generic CREATE / UPDATE / DELETE
echo   [B] Back
choice /C 123B /N /M "Choose: "
if errorlevel 4 goto AUTONOMOUS_CENTER
if errorlevel 3 (
    set "AUTO_RESOURCE=Action"
    goto AUTO_GENERIC_CRUD
)
if errorlevel 2 (
    set "RUN_NAME="
    set /p "RUN_NAME=Action name [B=back]: "
    if /I "%RUN_NAME%"=="B" goto AUTO_ACTIONS
    call :KmApi run action "%RUN_NAME%" -y
    goto AUTO_ACTIONS
)
call :KmApi ls actions -a
goto AUTO_ACTIONS


:AUTO_SYNCS
@echo off
cls
call :Header
echo %GREEN%%BOLD%   AUTONOMOUS RESOURCE SYNCS%RESET%
echo.
echo   [1] List syncs
echo   [2] Commit / execute sync
echo   [3] Generic CREATE / UPDATE / DELETE
echo   [B] Back
choice /C 123B /N /M "Choose: "
if errorlevel 4 goto AUTONOMOUS_CENTER
if errorlevel 3 (
    set "AUTO_RESOURCE=ResourceSync"
    goto AUTO_GENERIC_CRUD
)
if errorlevel 2 (
    set "RUN_NAME="
    set /p "RUN_NAME=Sync name [B=back]: "
    if /I "%RUN_NAME%"=="B" goto AUTO_SYNCS
    call :KmApi x commit "%RUN_NAME%"
    goto AUTO_SYNCS
)
call :KmApi ls syncs -a
goto AUTO_SYNCS


:AUTO_SCHEDULES
@echo off
cls
call :Header
call :KmApi ls schedules -a
goto AUTO_SIMPLE_PAUSE

:AUTO_TERMINALS
@echo off
cls
call :Header
echo   [1] List terminals
echo   [2] Connect to server terminal
echo   [B] Back
choice /C 12B /N /M "Choose: "
if errorlevel 3 goto AUTONOMOUS_CENTER
if errorlevel 2 goto AUTO_SERVER_TERMINAL
call :KmApi ls terminals -a
goto AUTO_SIMPLE_PAUSE

:AUTO_VARIABLES
@echo off
cls
call :Header
set "VAR_NAME="
set /p "VAR_NAME=Variable name [B=back]: "
if /I "%VAR_NAME%"=="B" goto AUTONOMOUS_CENTER
set "VAR_VALUE="
set /p "VAR_VALUE=Variable value: "
call :KmApi set var "%VAR_NAME%" "%VAR_VALUE%" -y
goto AUTO_SIMPLE_PAUSE

:AUTO_USERS
@echo off
cls
call :Header
echo   [1] List real users
echo   [2] Reset password
echo   [3] Promote Super Admin
echo   [4] Enable local signup
echo   [B] Back
choice /C 1234B /N /M "Choose: "
if errorlevel 5 goto AUTONOMOUS_CENTER
if errorlevel 4 goto ENABLE_LOCAL_SIGNUP
if errorlevel 3 goto MAKE_SUPER_ADMIN
if errorlevel 2 goto RESET_EXISTING_PASSWORD
goto LIST_REAL_USERS

:AUTO_KEYS
@echo off
cls
call :Header
echo   [1] Create onboarding key
echo   [2] Configure API key/secret used by autonomous API
echo   [3] Create API key using km direct database access
echo   [B] Back
choice /C 123B /N /M "Choose: "
if errorlevel 4 goto AUTONOMOUS_CENTER
if errorlevel 3 goto AUTO_CREATE_API_KEY
if errorlevel 2 goto API_CREDENTIALS
goto AUTO_ONBOARDING_KEY

:AUTO_CREATE_API_KEY
@echo off
set "API_USER="
set /p "API_USER=Existing Komodo username [B=back]: "
if /I "%API_USER%"=="B" goto AUTO_KEYS
echo.
echo The key and secret are shown once. Copy them into option 42 afterward.
call :Compose exec -T core km create api-key "control-center" --for "%API_USER%"
goto AUTO_SIMPLE_PAUSE

:AUTO_DATABASE
@echo off
cls
call :Header
echo   [1] Database backup
echo   [2] Show km database help
echo   [B] Back
choice /C 12B /N /M "Choose: "
if errorlevel 3 goto AUTONOMOUS_CENTER
if errorlevel 2 (
    call :Compose exec -T core km database --help
    goto AUTO_SIMPLE_PAUSE
)
call :Compose exec -T core km database backup -y
goto AUTO_SIMPLE_PAUSE

:AUTO_GENERIC_CRUD
@echo off
cls
call :Header
echo %GREEN%%BOLD%   GENERIC RESOURCE CREATE / UPDATE / DELETE%RESET%
echo.
echo Use singular API resource names such as:
echo Server, Stack, Deployment, Repo, Build, Builder, Procedure, Action,
echo ResourceSync, Alerter.
echo.
set "CRUD_RESOURCE="
set /p "CRUD_RESOURCE=Resource type [B=back]: "
if /I "%CRUD_RESOURCE%"=="B" goto AUTONOMOUS_CENTER
echo.
echo   [1] Create
echo   [2] Update
echo   [3] Delete
echo   [B] Back
choice /C 123B /N /M "Choose: "
if errorlevel 4 goto AUTONOMOUS_CENTER
if errorlevel 3 goto CRUD_DELETE
if errorlevel 2 goto CRUD_UPDATE
goto CRUD_CREATE

:CRUD_CREATE
set "CRUD_NAME="
set /p "CRUD_NAME=New resource name: "
echo Enter config JSON object. Example: {}
set "CRUD_CONFIG="
set /p "CRUD_CONFIG=Config JSON: "
if not defined CRUD_CONFIG set "CRUD_CONFIG={}"
set "API_MODE=write"
set "API_TYPE=Create%CRUD_RESOURCE%"
set "API_JSON={\"name\":\"%CRUD_NAME%\",\"config\":%CRUD_CONFIG%}"
call :KomodoApiCall
goto AUTO_SIMPLE_PAUSE

:CRUD_UPDATE
set "CRUD_ID="
set /p "CRUD_ID=Existing resource name/id: "
echo Enter PARTIAL config JSON object.
set "CRUD_CONFIG="
set /p "CRUD_CONFIG=Config JSON: "
if not defined CRUD_CONFIG set "CRUD_CONFIG={}"
set "API_MODE=write"
set "API_TYPE=Update%CRUD_RESOURCE%"
set "API_JSON={\"id\":\"%CRUD_ID%\",\"config\":%CRUD_CONFIG%}"
call :KomodoApiCall
goto AUTO_SIMPLE_PAUSE

:CRUD_DELETE
set "CRUD_ID="
set /p "CRUD_ID=Resource name/id to delete: "
choice /C YN /N /M "Confirm delete? [Y/N]: "
if errorlevel 2 goto AUTONOMOUS_CENTER
set "API_MODE=write"
set "API_TYPE=Delete%CRUD_RESOURCE%"
set "API_JSON={\"id\":\"%CRUD_ID%\"}"
call :KomodoApiCall
goto AUTO_SIMPLE_PAUSE

:AUTO_SIMPLE_PAUSE
echo.
choice /C BM /N /M "[B] Autonomous Center  [M] Main menu: "
if errorlevel 2 goto MAIN
goto AUTONOMOUS_CENTER


:API_CREDENTIALS
@echo off
cls
call :Header
echo %GREEN%%BOLD%   KOMODO API CREDENTIALS%RESET%
echo.
echo The autonomous API uses the official Komodo API:
echo   X-Api-Key + X-Api-Secret
echo.
echo Credentials are stored locally under .komodo-windows\secure.
echo The folder ACL is restricted to the current Windows user when possible.
echo.
echo   [1] Configure / replace API key and secret
echo   [2] Test saved API credentials
echo   [3] Remove saved API credentials
echo   [B] Back
choice /C 123B /N /M "Choose: "
if errorlevel 4 goto MAIN
if errorlevel 3 goto API_REMOVE
if errorlevel 2 goto API_TEST
set "API_KEY_INPUT="
set "API_SECRET_INPUT="
set /p "API_KEY_INPUT=API Key K-... [B=back]: "
if /I "%API_KEY_INPUT%"=="B" goto API_CREDENTIALS
set /p "API_SECRET_INPUT=API Secret S-...: "
if not exist "%STATE_DIR%\secure\" mkdir "%STATE_DIR%\secure" >nul 2>&1
>"%STATE_DIR%\secure\api-auth.cmd" echo set "KOMODO_CC_API_KEY=%API_KEY_INPUT%"
>>"%STATE_DIR%\secure\api-auth.cmd" echo set "KOMODO_CC_API_SECRET=%API_SECRET_INPUT%"
icacls "%STATE_DIR%\secure" /inheritance:r /grant:r "%USERNAME%:(OI)(CI)F" >nul 2>&1
set "API_KEY_INPUT="
set "API_SECRET_INPUT="
call :MsgOk "API credentials saved locally."
goto API_CREDENTIALS

:API_TEST
set "API_MODE=read"
set "API_TYPE=ListServers"
set "API_JSON={}"
call :KomodoApiCall
goto API_CREDENTIALS

:API_REMOVE
del /Q "%STATE_DIR%\secure\api-auth.cmd" >nul 2>&1
call :MsgOk "Saved autonomous API credentials removed."
goto API_CREDENTIALS


:UNIVERSAL_API
@echo off
cls
call :Header
echo %GREEN%%BOLD%   UNIVERSAL KOMODO API RUNNER%RESET%
echo.
echo This gives menu access to Komodo API requests not yet represented by a shortcut.
echo Official modules: read, write, execute.
echo.
set "API_MODE="
set /p "API_MODE=Module [read/write/execute] or B: "
if /I "%API_MODE%"=="B" goto AUTONOMOUS_CENTER
if /I not "%API_MODE%"=="read" if /I not "%API_MODE%"=="write" if /I not "%API_MODE%"=="execute" (
    call :MsgError "Invalid API module."
    goto UNIVERSAL_API
)
set "API_TYPE="
set /p "API_TYPE=Request type, example ListServers or UpdateBuild [B=back]: "
if /I "%API_TYPE%"=="B" goto AUTONOMOUS_CENTER
echo.
echo Enter ONE-LINE JSON parameters. Example:
echo   {}
echo   {\"server\":\"my-server\"}
echo   {\"id\":\"my-build\",\"config\":{\"version\":\"2.0.0\"}}
echo.
set "API_JSON="
set /p "API_JSON=JSON: "
if not defined API_JSON set "API_JSON={}"
call :KomodoApiCall
echo.
choice /C ABM /N /M "[A] Another API call  [B] Autonomous Center  [M] Main: "
if errorlevel 3 goto MAIN
if errorlevel 2 goto AUTONOMOUS_CENTER
goto UNIVERSAL_API


:RESOURCE_OVERVIEW
@echo off
cls
call :Header
echo %GREEN%%BOLD%   COMPLETE KOMODO RESOURCE OVERVIEW%RESET%
echo.
call :KmApi ls -a
if errorlevel 1 (
    echo.
    echo Falling back to API resource lists...
    for %%R in (Servers Swarms Stacks Deployments Builds Repos Procedures Actions ResourceSyncs Builders Alerters) do (
        echo.
        echo ===== %%R =====
        set "API_MODE=read"
        set "API_TYPE=List%%R"
        set "API_JSON={}"
        call :KomodoApiCall
    )
)
goto AUTO_SIMPLE_PAUSE


:LoadApiCredentials
@echo off
set "KOMODO_CC_API_KEY="
set "KOMODO_CC_API_SECRET="
if exist "%STATE_DIR%\secure\api-auth.cmd" call "%STATE_DIR%\secure\api-auth.cmd"
if not defined KOMODO_CC_API_KEY exit /b 1
if not defined KOMODO_CC_API_SECRET exit /b 1
exit /b 0

:KomodoApiCall
@echo off
call :LoadApiCredentials
if errorlevel 1 (
    call :MsgError "Komodo API credentials are not configured. Use main option 42."
    exit /b 1
)
set "API_TMP=%STATE_DIR%\api-payload-%RANDOM%.json"
>"%API_TMP%" echo %API_JSON%
echo.
echo POST %DASHBOARD_URL%/%API_MODE%/%API_TYPE%
curl.exe -sS --fail-with-body ^
  -H "Content-Type: application/json" ^
  -H "X-Api-Key: %KOMODO_CC_API_KEY%" ^
  -H "X-Api-Secret: %KOMODO_CC_API_SECRET%" ^
  --data-binary "@%API_TMP%" ^
  "%DASHBOARD_URL%/%API_MODE%/%API_TYPE%"
set "API_RC=%ERRORLEVEL%"
echo.
del /Q "%API_TMP%" >nul 2>&1
set "KOMODO_CC_API_KEY="
set "KOMODO_CC_API_SECRET="
if not "%API_RC%"=="0" (
    call :MsgError "Komodo API request failed."
    exit /b %API_RC%
)
exit /b 0

:KmApi
@echo off
call :LoadApiCredentials
if errorlevel 1 (
    call :MsgError "API credentials not configured. Use option 42 first."
    exit /b 1
)
call :Compose exec -T ^
  -e "KOMODO_CLI_HOST=%DASHBOARD_URL%" ^
  -e "KOMODO_CLI_KEY=%KOMODO_CC_API_KEY%" ^
  -e "KOMODO_CLI_SECRET=%KOMODO_CC_API_SECRET%" ^
  core km %*
set "KM_RC=%ERRORLEVEL%"
set "KOMODO_CC_API_KEY="
set "KOMODO_CC_API_SECRET="
exit /b %KM_RC%

:GITHUB_PUBLISH_CENTER
@echo off
cls
call :Header
call :LogAction "Abrir Central de Publicacao GitHub"
echo %MAGENTA%%BOLD%   CENTRAL of publication GITHUB - KOMODO CONTROL CENTER%RESET%
echo.
echo %GRAY%Repository official:%RESET% https://github.with/moghtech/komodo
echo %GRAY%your fork:%RESET%            detected automatically by the account GitHub authenticated
echo %GRAY%Branch of contribution:%RESET% contrib/windows-control-center
echo.
echo %YELLOW%how works:%RESET%
echo   1. Verifica Git and GitHub CLI.
echo   2. confirms authentication.
echo   3. Detecta your account and cria/verifica your fork automatically.
echo   4. Prepara tudo in folder temporaria isolada.
echo   5. copy this BAT current for scripts/windows/.
echo   6. Gera README of the contribution.
echo   7. Faz commit only desses files.
echo   8. Envia for your branch in the fork.
echo   9. Cria PR in the repository official or atualiza o PR already aberto.
echo.
echo %GREEN%[1]%RESET% Publish / Update contribution now
echo %GREEN%[2]%RESET% View Pull Request status
echo %GREEN%[3]%RESET% Test GitHub connection / authentication
echo %GREEN%[4]%RESET% Open your fork in browser
echo %GREEN%[5]%RESET% Open official Komodo Pull Requests
echo %GREEN%[6]%RESET% Preview what will be sent
echo.
echo %GRAY%[B] Back   [C] Cancel%RESET%
echo.
set "GHOP="
set /p "GHOP=%WHITE%%BOLD%choose: %RESET%"
call :LogAction "GITHUB publicar menu: %GHOP%"

if /I "%GHOP%"=="B" goto MAIN
if /I "%GHOP%"=="C" goto MAIN
if "%GHOP%"=="1" goto GITHUB_PUBLISH_RUN
if "%GHOP%"=="2" goto GITHUB_PUBLISH_STATUS
if "%GHOP%"=="3" goto GITHUB_PUBLISH_TEST
if "%GHOP%"=="4" (
    call :EnsureGitHubCli
    if errorlevel 1 goto GITHUB_PUBLISH_PAUSE
    call :CheckGitHubAuth
    if errorlevel 1 goto GITHUB_PUBLISH_PAUSE
    call :DetectGitHubLogin
    if errorlevel 1 goto GITHUB_PUBLISH_PAUSE
    gh repo view "%GH_LOGIN%/komodo" >nul 2>&1
    if errorlevel 1 (
        call :MsgError "your fork ainda not existe. use [1] for cria-lo/publish."
        goto GITHUB_PUBLISH_PAUSE
    )
    gh repo view "%GH_LOGIN%/komodo" --web
    goto GITHUB_PUBLISH_CENTER
)
if "%GHOP%"=="5" (
    start "" "https://github.com/moghtech/komodo/pulls"
    goto GITHUB_PUBLISH_CENTER
)
if "%GHOP%"=="6" goto GITHUB_PUBLISH_PREVIEW

call :MsgError "Invalid option in the Central GitHub: %GHOP%"
goto GITHUB_PUBLISH_PAUSE


:GITHUB_PUBLISH_PREVIEW
@echo off
cls
call :Header
echo %MAGENTA%%BOLD%   PREVIEW of the contribution%RESET%
echo.
echo Files that will be incluidos/atualizados in the PR:
echo.
echo   scripts/windows/KOMODO_CONTROL_CENTER.bat
echo   scripts/windows/README.md
echo.
echo O BAT enviado will be exatamente is versao:
echo   %~f0
echo.
echo NOT will be usado "git add ."
echo NOT will be enviados outros files locais.
echo NOT will be feito push for moghtech/komodo:main.
echo.
echo O fluxo uses:
echo   your-account/komodo
echo   branch contrib/windows-control-center
echo   Pull Request for moghtech/komodo:main
goto GITHUB_PUBLISH_PAUSE


:GITHUB_PUBLISH_TEST
@echo off
cls
call :Header
echo %MAGENTA%%BOLD%   TESTE GITHUB%RESET%
echo.
call :EnsureGitHubCli
if errorlevel 1 goto GITHUB_PUBLISH_PAUSE

call :CheckGitHubAuth
if errorlevel 1 goto GITHUB_PUBLISH_PAUSE

echo.
call :MsgOk "GitHub CLI authenticated and ready."
echo.
call :DetectGitHubLogin
if errorlevel 1 goto GITHUB_PUBLISH_PAUSE
echo account detected: %GH_LOGIN%
echo.
goto GITHUB_PUBLISH_PAUSE


:GITHUB_PUBLISH_STATUS
@echo off
cls
call :Header
echo %MAGENTA%%BOLD%   STATUS of the contribution%RESET%
echo.
call :EnsureGitHubCli
if errorlevel 1 goto GITHUB_PUBLISH_PAUSE
call :CheckGitHubAuth
if errorlevel 1 goto GITHUB_PUBLISH_PAUSE
call :DetectGitHubLogin
if errorlevel 1 goto GITHUB_PUBLISH_PAUSE

echo %GRAY%fork:%RESET%
gh repo view "%GH_LOGIN%/komodo" --json nameWithOwner,url,defaultBranchRef --jq ".nameWithOwner + \" | \" + .url + \" | default=\" + .defaultBranchRef.name" 2>nul
if errorlevel 1 echo   Ainda nao encontrado ou sem acesso.
echo.

echo %GRAY%Pull Requests relacionados:%RESET%
gh pr list --repo moghtech/komodo --state all --search "head:%GH_LOGIN%:contrib/windows-control-center" --limit 10 2>nul
if errorlevel 1 (
    call :LogWarn "Nao foi possivel consultar PRs."
)
echo.
goto GITHUB_PUBLISH_PAUSE


:GITHUB_PUBLISH_RUN
@echo off
cls
call :Header
echo %MAGENTA%%BOLD%   publish / update contribution%RESET%
echo.
echo %YELLOW%important:%RESET%
echo O repository official pertence a moghtech.
echo is option envia a proposta by fork + Pull Request.
echo Os maintainers decide if a contribution will be accepted.
echo.
choice /C SBC /N /M "Continue? [S] Yes  [B] Back  [C] Cancel: "
if errorlevel 3 goto GITHUB_PUBLISH_CENTER
if errorlevel 2 goto GITHUB_PUBLISH_CENTER

call :EnsureGitHubCli
if errorlevel 1 goto GITHUB_PUBLISH_PAUSE

call :CheckGitHubAuth
if errorlevel 1 goto GITHUB_PUBLISH_PAUSE

call :EnsureGit
if errorlevel 1 goto GITHUB_PUBLISH_PAUSE

call :DetectGitHubLogin
if errorlevel 1 goto GITHUB_PUBLISH_PAUSE
call :LogAction "Garantir fork %GH_LOGIN%/komodo"
gh repo view "%GH_LOGIN%/komodo" >nul 2>&1
if errorlevel 1 (
    echo.
    echo %CYAN%Creating fork lowdrus/komodo...%RESET%
    gh repo fork moghtech/komodo --clone=false
    if errorlevel 1 (
        call :MsgError "Failed to create o fork %GH_LOGIN%/komodo."
        goto GITHUB_PUBLISH_PAUSE
    )
    call :LogSuccess "Fork %GH_LOGIN%/komodo criado."
) else (
    call :LogInfo "Fork %GH_LOGIN%/komodo ja existe."
)

set "GH_WORK=%TEMP%\komodo-control-center-pr-%RANDOM%-%RANDOM%"
if exist "%GH_WORK%\" rmdir /S /Q "%GH_WORK%" >nul 2>&1
mkdir "%GH_WORK%" >nul 2>&1

echo.
echo %CYAN%Preparing isolated temporary workspace...%RESET%
call :LogAction "Clonar upstream em workspace temporario: %GH_WORK%"

git clone --depth 1 https://github.com/moghtech/komodo.git "%GH_WORK%\repo"
if errorlevel 1 (
    call :MsgError "Failed to clonar moghtech/komodo."
    goto GITHUB_PUBLISH_CLEANUP
)

pushd "%GH_WORK%\repo"

git remote remove fork >nul 2>&1
git remote add fork https://github.com/%GH_LOGIN%/komodo.git
if errorlevel 1 (
    popd
    call :MsgError "Failed to configurar remote of the fork."
    goto GITHUB_PUBLISH_CLEANUP
)

echo.
echo %CYAN%Verificando branch of contribution existing...%RESET%
git ls-remote --exit-code --heads fork contrib/windows-control-center >nul 2>&1
if not errorlevel 1 (
    git fetch fork contrib/windows-control-center:refs/remotes/fork/contrib/windows-control-center >nul 2>&1
    git checkout -B contrib/windows-control-center fork/contrib/windows-control-center
    if errorlevel 1 (
        popd
        call :MsgError "Failed to open branch existing of the fork."
        goto GITHUB_PUBLISH_CLEANUP
    )
) else (
    git checkout -B contrib/windows-control-center
    if errorlevel 1 (
        popd
        call :MsgError "Failed to create branch of contribution."
        goto GITHUB_PUBLISH_CLEANUP
    )
)

if not exist "scripts\windows\" mkdir "scripts\windows" >nul 2>&1

set "PUBLISH_SOURCE=%~f0"
if defined KCC_MASTER if exist "%KCC_MASTER%" set "PUBLISH_SOURCE=%KCC_MASTER%"
copy /Y "%PUBLISH_SOURCE%" "scripts\windows\KOMODO_CONTROL_CENTER.bat" >nul
if errorlevel 1 (
    popd
    call :MsgError "Failed to copy o BAT for o workspace."
    goto GITHUB_PUBLISH_CLEANUP
)

call :WriteContributionReadme "%GH_WORK%\repo\scripts\windows\README.md"
if errorlevel 1 (
    popd
    call :MsgError "Failed to gerar README of the contribution."
    goto GITHUB_PUBLISH_CLEANUP
)

git config user.name >nul 2>&1
if errorlevel 1 git config user.name "lowdrus"
git config user.email >nul 2>&1
if errorlevel 1 git config user.email "lowdrus@users.noreply.github.com"

git add -- "scripts/windows/KOMODO_CONTROL_CENTER.bat" "scripts/windows/README.md"

git diff --cached --quiet
if not errorlevel 1 (
    echo.
    echo %YELLOW%in the mudanca new detected in the files of the contribution.%RESET%
    call :LogInfo "Nenhuma mudanca nova para commit."
) else (
    echo.
    echo %CYAN%Creating commit...%RESET%
    git commit -m "feat(windows): add/update Komodo Control Center"
    if errorlevel 1 (
        popd
        call :MsgError "Failed to create commit."
        goto GITHUB_PUBLISH_CLEANUP
    )
    call :LogSuccess "Commit da contribuicao criado."
)

echo.
echo %CYAN%Pushing branch to %GH_LOGIN%/komodo...%RESET%
git push -u fork contrib/windows-control-center
if errorlevel 1 (
    popd
    call :MsgError "Failure in the push for %GH_LOGIN%/komodo."
    goto GITHUB_PUBLISH_CLEANUP
)

popd

echo.
echo %CYAN%Checking existing Pull Request...%RESET%
set "EXISTING_PR="
for /F "usebackq delims=" %%P in (`gh pr list --repo moghtech/komodo --state open --head %GH_LOGIN%:contrib/windows-control-center --json number --jq ".[0].number" 2^>nul`) do set "EXISTING_PR=%%P"

if defined EXISTING_PR (
    call :LogSuccess "PR existente atualizado: #%EXISTING_PR%"
    echo.
    echo %GREEN%%BOLD%PR existing updated automatically: #%EXISTING_PR%%RESET%
    gh pr view "%EXISTING_PR%" --repo moghtech/komodo --web >nul 2>&1
) else (
    call :WritePrBody "%GH_WORK%\pr-body.md"
    echo.
    echo %CYAN%creating new Pull Request in the repository official...%RESET%
    gh pr create --repo moghtech/komodo ^
        --base main ^
        --head %GH_LOGIN%:contrib/windows-control-center ^
        --title "feat(windows): add Komodo Control Center" ^
        --body-file "%GH_WORK%\pr-body.md"
    if errorlevel 1 (
        call :MsgError "Push was enviado, mas o Pull Request not may be created automatically."
        echo %YELLOW%A branch became saved in the fork. you may tentar novamente by the option 35.%RESET%
        goto GITHUB_PUBLISH_CLEANUP
    )
    call :LogSuccess "Novo Pull Request criado em moghtech/komodo."
)

echo.
call :MsgOk "Publication completed via fork + Pull Request."
echo %GRAY%As alteracoes so entram in the repository official if os maintainers fizerem merge.%RESET%

:GITHUB_PUBLISH_CLEANUP
@echo off
if defined GH_WORK (
    if exist "%GH_WORK%\" rmdir /S /Q "%GH_WORK%" >nul 2>&1
)
set "GH_WORK="
goto GITHUB_PUBLISH_PAUSE


:GITHUB_PUBLISH_PAUSE
@echo off
echo.
echo %GRAY%--------------------------------------------------------------------------------%RESET%
choice /C BC /N /M "[B] Back for Central GitHub  [C] menu main: "
if errorlevel 2 goto MAIN
goto GITHUB_PUBLISH_CENTER




:WriteControlCenterDocs
@echo off
call :WriteDocsEN
if errorlevel 1 exit /b 1
call :WriteDocsPTBR
if errorlevel 1 exit /b 1
exit /b 0

:WriteDocsEN
@echo off
if not exist "%STATE_DIR%\docs\" mkdir "%STATE_DIR%\docs" >nul 2>&1
set "DOC_EN=%STATE_DIR%\docs\KOMODO_CONTROL_CENTER_EN.md"
>"%DOC_EN%" echo # Komodo Control Center for Windows
>>"%DOC_EN%" echo.
>>"%DOC_EN%" echo ## Purpose
>>"%DOC_EN%" echo Komodo Control Center is a Windows batch-based companion for Komodo. It centralizes common lifecycle, diagnostics, account, translation, documentation, support and contribution workflows in a single menu.
>>"%DOC_EN%" echo.
>>"%DOC_EN%" echo ## Main capabilities
>>"%DOC_EN%" echo - Start, stop and restart the local Komodo stack.
>>"%DOC_EN%" echo - Open the local dashboard.
>>"%DOC_EN%" echo - Inspect service status, HTTP health, port 9120 and Docker logs.
>>"%DOC_EN%" echo - Run a guided diagnostic.
>>"%DOC_EN%" echo - Pull Docker images and update a Git checkout.
>>"%DOC_EN%" echo - Open Explorer, PowerShell and VS Code in the project.
>>"%DOC_EN%" echo - Create or repair a Windows Desktop shortcut and startup entry.
>>"%DOC_EN%" echo - Inspect real Komodo users, reset an existing user password and promote a user to Super Admin.
>>"%DOC_EN%" echo - Enable local registration.
>>"%DOC_EN%" echo - Apply live UI language packs to the running Komodo Core.
>>"%DOC_EN%" echo - Keep Control Center action/error logs and optional Windows error notifications.
>>"%DOC_EN%" echo - Export a safe support bundle without environment/secret files.
>>"%DOC_EN%" echo - Create a private local configuration backup.
>>"%DOC_EN%" echo - Publish the Control Center contribution through the authenticated user's fork and a Pull Request.
>>"%DOC_EN%" echo - Provide a complete beginner course from zero knowledge to real Komodo usage.
>>"%DOC_EN%" echo.
>>"%DOC_EN%" echo ## GitHub contribution model
>>"%DOC_EN%" echo The official repository is `moghtech/komodo`. The Control Center never assumes a specific contributor username. It detects the user authenticated in GitHub CLI, verifies or creates `^<user^>/komodo`, pushes `contrib/windows-control-center`, and opens or updates a Pull Request to `moghtech/komodo:main`.
>>"%DOC_EN%" echo.
>>"%DOC_EN%" echo The publication workflow stages only:
>>"%DOC_EN%" echo - `scripts/windows/KOMODO_CONTROL_CENTER.bat`
>>"%DOC_EN%" echo - `scripts/windows/README.md`
>>"%DOC_EN%" echo.
>>"%DOC_EN%" echo It does not use `git add .`.
>>"%DOC_EN%" echo.
>>"%DOC_EN%" echo ## Safety notes
>>"%DOC_EN%" echo Destructive operations require confirmation. Reset-with-volumes is intentionally separated from normal operations. Password values should never be written to Control Center logs. Support bundles intentionally exclude compose environment files and secrets.
>>"%DOC_EN%" echo.
>>"%DOC_EN%" echo ## Language
>>"%DOC_EN%" echo English is the default Control Center language for upstream collaboration. Portuguese-BR can be selected from the Language menu without closing the Command Prompt window.
>>"%DOC_EN%" echo.
>>"%DOC_EN%" echo ## Beginner learning path
>>"%DOC_EN%" echo Open the Complete Guide and start at Chapter 0. Use `[P] Next step` after each chapter until Chapter 23.
>>"%DOC_EN%" echo.
>>"%DOC_EN%" echo ## Typical troubleshooting order
>>"%DOC_EN%" echo 1. Intelligent diagnostics
>>"%DOC_EN%" echo 2. Service status
>>"%DOC_EN%" echo 3. Core logs
>>"%DOC_EN%" echo 4. All-service logs
>>"%DOC_EN%" echo 5. HTTP health
>>"%DOC_EN%" echo 6. Port/process inspection
>>"%DOC_EN%" echo 7. Control Center error log
exit /b 0

:WriteDocsPTBR
@echo off
if not exist "%STATE_DIR%\docs\" mkdir "%STATE_DIR%\docs" >nul 2>&1
set "DOC_PT=%STATE_DIR%\docs\KOMODO_CONTROL_CENTER_PTBR.md"
>"%DOC_PT%" echo # Komodo Control Center para Windows
>>"%DOC_PT%" echo.
>>"%DOC_PT%" echo ## Objetivo
>>"%DOC_PT%" echo O Komodo Control Center e um companheiro em BAT para Windows que centraliza operacao, diagnostico, usuarios, traducao, documentacao, suporte e contribuicoes do Komodo em um unico menu.
>>"%DOC_PT%" echo.
>>"%DOC_PT%" echo ## Principais recursos
>>"%DOC_PT%" echo - Iniciar, parar e reiniciar o stack local do Komodo.
>>"%DOC_PT%" echo - Abrir o painel local.
>>"%DOC_PT%" echo - Ver status, health HTTP, porta 9120 e logs Docker.
>>"%DOC_PT%" echo - Executar diagnostico guiado.
>>"%DOC_PT%" echo - Atualizar imagens Docker e repositorio Git.
>>"%DOC_PT%" echo - Abrir Explorer, PowerShell e VS Code na pasta do projeto.
>>"%DOC_PT%" echo - Criar/reparar atalho do Desktop e inicializacao com o Windows.
>>"%DOC_PT%" echo - Listar usuarios reais, resetar senha de usuario existente e promover Super Admin.
>>"%DOC_PT%" echo - Habilitar cadastro local.
>>"%DOC_PT%" echo - Aplicar idiomas ao vivo na interface do Core.
>>"%DOC_PT%" echo - Manter logs de acoes/erros e notificacoes opcionais.
>>"%DOC_PT%" echo - Exportar Support Bundle seguro sem arquivos de ambiente/secrets.
>>"%DOC_PT%" echo - Criar backup local privado de configuracao.
>>"%DOC_PT%" echo - Publicar a contribuicao via fork do usuario autenticado e Pull Request.
>>"%DOC_PT%" echo - Ensinar Komodo do zero ao uso real com guia sequencial.
>>"%DOC_PT%" echo.
>>"%DOC_PT%" echo ## Modelo de contribuicao GitHub
>>"%DOC_PT%" echo O repositorio oficial e `moghtech/komodo`. O Control Center nao depende de um usuario especifico. Ele detecta a conta autenticada no GitHub CLI, verifica/cria `^<usuario^>/komodo`, envia a branch `contrib/windows-control-center` e cria/atualiza um Pull Request para `moghtech/komodo:main`.
>>"%DOC_PT%" echo.
>>"%DOC_PT%" echo A publicacao adiciona apenas:
>>"%DOC_PT%" echo - `scripts/windows/KOMODO_CONTROL_CENTER.bat`
>>"%DOC_PT%" echo - `scripts/windows/README.md`
>>"%DOC_PT%" echo.
>>"%DOC_PT%" echo Nao utiliza `git add .`.
>>"%DOC_PT%" echo.
>>"%DOC_PT%" echo ## Seguranca
>>"%DOC_PT%" echo Operacoes destrutivas exigem confirmacao. Reset com volumes fica separado das operacoes normais. Senhas nao devem ser gravadas no log do Control Center. O Support Bundle nao copia compose.env nem secrets.
>>"%DOC_PT%" echo.
>>"%DOC_PT%" echo ## Idioma
>>"%DOC_PT%" echo O padrao da V16 e English para facilitar contribuicao ao projeto upstream. Portugues-BR pode ser selecionado no menu Idioma sem fechar a janela do CMD.
>>"%DOC_PT%" echo.
>>"%DOC_PT%" echo ## Aprendizado
>>"%DOC_PT%" echo Abra o Guia Completo, comece no Capitulo 0 e utilize `[P] Proximo passo` ate o Capitulo 23.
>>"%DOC_PT%" echo.
>>"%DOC_PT%" echo ## Ordem recomendada para diagnostico
>>"%DOC_PT%" echo 1. Diagnostico inteligente
>>"%DOC_PT%" echo 2. Status dos servicos
>>"%DOC_PT%" echo 3. Logs do Core
>>"%DOC_PT%" echo 4. Logs gerais
>>"%DOC_PT%" echo 5. Health HTTP
>>"%DOC_PT%" echo 6. Porta/processos
>>"%DOC_PT%" echo 7. Log de erros do Control Center
exit /b 0


:RemoveGenericLanguageSilent
@echo off
call :FindCoreContainer >nul 2>&1
if errorlevel 1 exit /b 0
set "RG_DIR=%STATE_DIR%\generic-lang"
if not exist "%RG_DIR%\" mkdir "%RG_DIR%" >nul 2>&1
set "RG_INDEX=%RG_DIR%\remove-index.html"
docker cp "%CORE_ID%:/app/ui/index.html" "%RG_INDEX%" >nul 2>&1
if errorlevel 1 exit /b 0
powershell.exe -NoProfile -Command "$p='%RG_INDEX%';$s=[IO.File]::ReadAllText($p);$s=[regex]::Replace($s,'<script id=\"komodo-generic-lang-loader\".*?</script>','','Singleline');[IO.File]::WriteAllText($p,$s,(New-Object Text.UTF8Encoding($false)))" >nul 2>&1
docker cp "%RG_INDEX%" "%CORE_ID%:/app/ui/index.html" >nul 2>&1
docker exec "%CORE_ID%" sh -lc "rm -f /app/ui/komodo-lang.js" >nul 2>&1
del /Q "%STATE_DIR%\generic-lang.enabled" >nul 2>&1
exit /b 0

:ApplyGenericLang
@echo off
set "GEN_CODE=%~1"
call :Preflight
if errorlevel 1 exit /b 1
call :EnsureDocker
if errorlevel 1 exit /b 1
call :FindCoreContainer
if errorlevel 1 exit /b 1
set "GEN_DIR=%STATE_DIR%\generic-lang"
if not exist "%GEN_DIR%\" mkdir "%GEN_DIR%" >nul 2>&1
set "GEN_JS=%GEN_DIR%\komodo-lang.js"
set "GEN_B64=%GEN_DIR%\lang.b64"
if /I "%GEN_CODE%"=="es" >"%GEN_B64%" echo KCgpPT57Y29uc3QgRD17IkRhc2hib2FyZCI6IlBhbmVsIiwiU2VydmVycyI6IlNlcnZpZG9yZXMiLCJEZXBsb3ltZW50cyI6IkRlc3BsaWVndWVzIiwiU3RhY2tzIjoiUGlsYXMiLCJCdWlsZHMiOiJDb21waWxhY2lvbmVzIiwiQnVpbGRlcnMiOiJDb25zdHJ1Y3RvcmVzIiwiUHJvY2VkdXJlcyI6IlByb2NlZGltaWVudG9zIiwiQWN0aW9ucyI6IkFjY2lvbmVzIiwiU2V0dGluZ3MiOiJDb25maWd1cmFjaW9uIiwiVXNlcnMiOiJVc3VhcmlvcyIsIkxvZ3MiOiJSZWdpc3Ryb3MiLCJTdGFydCI6IkluaWNpYXIiLCJTdG9wIjoiRGV0ZW5lciIsIlJlc3RhcnQiOiJSZWluaWNpYXIiLCJEZXBsb3kiOiJEZXNwbGVnYXIiLCJVcGRhdGUiOiJBY3R1YWxpemFyIiwiQ3JlYXRlIjoiQ3JlYXIiLCJEZWxldGUiOiJFbGltaW5hciIsIlNhdmUiOiJHdWFyZGFyIiwiQ2FuY2VsIjoiQ2FuY2VsYXIiLCJTZWFyY2giOiJCdXNjYXIiLCJTdGF0dXMiOiJFc3RhZG8iLCJOYW1lIjoiTm9tYnJlIiwiRGVzY3JpcHRpb24iOiJEZXNjcmlwY2lvbiIsIkVuYWJsZWQiOiJIYWJpbGl0YWRvIiwiRGlzYWJsZWQiOiJEZXNoYWJpbGl0YWRvIiwiQWRtaW4iOiJBZG1pbmlzdHJhZG9yIiwiTm90aWZpY2F0aW9ucyI6Ik5vdGlmaWNhY2lvbmVzIn07Y29uc3Qgc2tpcD1uZXcgU2V0KFsnU0NSSVBUJywnU1RZTEUnLCdDT0RFJywnUFJFJywnVEVYVEFSRUEnLCdOT1NDUklQVCcsJ1NWRycsJ1BBVEgnXSk7ZnVuY3Rpb24gdHIocyl7Y29uc3QgdD1zLnRyaW0oKTtyZXR1cm4gRFt0XXx8czt9ZnVuY3Rpb24gd2FsayhyKXtjb25zdCB3PWRvY3VtZW50LmNyZWF0ZVRyZWVXYWxrZXIocixOb2RlRmlsdGVyLlNIT1dfVEVYVCk7bGV0IG47d2hpbGUobj13Lm5leHROb2RlKCkpe2lmKG4ucGFyZW50RWxlbWVudCYmIXNraXAuaGFzKG4ucGFyZW50RWxlbWVudC50YWdOYW1lKSl7Y29uc3QgeD10cihuLm5vZGVWYWx1ZSk7aWYoeCE9PW4ubm9kZVZhbHVlKW4ubm9kZVZhbHVlPXg7fX1yLnF1ZXJ5U2VsZWN0b3JBbGwmJnIucXVlcnlTZWxlY3RvckFsbCgnW3BsYWNlaG9sZGVyXSxbdGl0bGVdLFthcmlhLWxhYmVsXScpLmZvckVhY2goZT0+WydwbGFjZWhvbGRlcicsJ3RpdGxlJywnYXJpYS1sYWJlbCddLmZvckVhY2goYT0+e2lmKGUuaGFzQXR0cmlidXRlKGEpKXtjb25zdCB2PWUuZ2V0QXR0cmlidXRlKGEpLHg9dHIodik7aWYoeCE9PXYpZS5zZXRBdHRyaWJ1dGUoYSx4KTt9fSkpO313YWxrKGRvY3VtZW50LmJvZHkpO25ldyBNdXRhdGlvbk9ic2VydmVyKG1zPT5tcy5mb3JFYWNoKG09Pm0uYWRkZWROb2Rlcy5mb3JFYWNoKG49Pm4ubm9kZVR5cGU9PT0xJiZ3YWxrKG4pKSkpLm9ic2VydmUoZG9jdW1lbnQuYm9keSx7Y2hpbGRMaXN0OnRydWUsc3VidHJlZTp0cnVlfSk7ZG9jdW1lbnQuZG9jdW1lbnRFbGVtZW50Lmxhbmc9J2VzJzt9KSgpOw==
if /I "%GEN_CODE%"=="de" >"%GEN_B64%" echo KCgpPT57Y29uc3QgRD17IkRhc2hib2FyZCI6IlViZXJzaWNodCIsIlNlcnZlcnMiOiJTZXJ2ZXIiLCJEZXBsb3ltZW50cyI6IkJlcmVpdHN0ZWxsdW5nZW4iLCJTdGFja3MiOiJTdGFja3MiLCJCdWlsZHMiOiJCdWlsZHMiLCJCdWlsZGVycyI6IkJ1aWxkZXIiLCJQcm9jZWR1cmVzIjoiUHJvemVkdXJlbiIsIkFjdGlvbnMiOiJBa3Rpb25lbiIsIlNldHRpbmdzIjoiRWluc3RlbGx1bmdlbiIsIlVzZXJzIjoiQmVudXR6ZXIiLCJMb2dzIjoiUHJvdG9rb2xsZSIsIlN0YXJ0IjoiU3RhcnRlbiIsIlN0b3AiOiJTdG9wcGVuIiwiUmVzdGFydCI6Ik5ldSBzdGFydGVuIiwiRGVwbG95IjoiQmVyZWl0c3RlbGxlbiIsIlVwZGF0ZSI6IkFrdHVhbGlzaWVyZW4iLCJDcmVhdGUiOiJFcnN0ZWxsZW4iLCJEZWxldGUiOiJMb3NjaGVuIiwiU2F2ZSI6IlNwZWljaGVybiIsIkNhbmNlbCI6IkFiYnJlY2hlbiIsIlNlYXJjaCI6IlN1Y2hlbiIsIlN0YXR1cyI6IlN0YXR1cyIsIk5hbWUiOiJOYW1lIiwiRGVzY3JpcHRpb24iOiJCZXNjaHJlaWJ1bmciLCJFbmFibGVkIjoiQWt0aXZpZXJ0IiwiRGlzYWJsZWQiOiJEZWFrdGl2aWVydCIsIkFkbWluIjoiQWRtaW5pc3RyYXRvciIsIk5vdGlmaWNhdGlvbnMiOiJCZW5hY2hyaWNodGlndW5nZW4ifTtjb25zdCBza2lwPW5ldyBTZXQoWydTQ1JJUFQnLCdTVFlMRScsJ0NPREUnLCdQUkUnLCdURVhUQVJFQScsJ05PU0NSSVBUJywnU1ZHJywnUEFUSCddKTtmdW5jdGlvbiB0cihzKXtjb25zdCB0PXMudHJpbSgpO3JldHVybiBEW3RdfHxzO31mdW5jdGlvbiB3YWxrKHIpe2NvbnN0IHc9ZG9jdW1lbnQuY3JlYXRlVHJlZVdhbGtlcihyLE5vZGVGaWx0ZXIuU0hPV19URVhUKTtsZXQgbjt3aGlsZShuPXcubmV4dE5vZGUoKSl7aWYobi5wYXJlbnRFbGVtZW50JiYhc2tpcC5oYXMobi5wYXJlbnRFbGVtZW50LnRhZ05hbWUpKXtjb25zdCB4PXRyKG4ubm9kZVZhbHVlKTtpZih4IT09bi5ub2RlVmFsdWUpbi5ub2RlVmFsdWU9eDt9fXIucXVlcnlTZWxlY3RvckFsbCYmci5xdWVyeVNlbGVjdG9yQWxsKCdbcGxhY2Vob2xkZXJdLFt0aXRsZV0sW2FyaWEtbGFiZWxdJykuZm9yRWFjaChlPT5bJ3BsYWNlaG9sZGVyJywndGl0bGUnLCdhcmlhLWxhYmVsJ10uZm9yRWFjaChhPT57aWYoZS5oYXNBdHRyaWJ1dGUoYSkpe2NvbnN0IHY9ZS5nZXRBdHRyaWJ1dGUoYSkseD10cih2KTtpZih4IT09dillLnNldEF0dHJpYnV0ZShhLHgpO319KSk7fXdhbGsoZG9jdW1lbnQuYm9keSk7bmV3IE11dGF0aW9uT2JzZXJ2ZXIobXM9Pm1zLmZvckVhY2gobT0+bS5hZGRlZE5vZGVzLmZvckVhY2gobj0+bi5ub2RlVHlwZT09PTEmJndhbGsobikpKSkub2JzZXJ2ZShkb2N1bWVudC5ib2R5LHtjaGlsZExpc3Q6dHJ1ZSxzdWJ0cmVlOnRydWV9KTtkb2N1bWVudC5kb2N1bWVudEVsZW1lbnQubGFuZz0nZGUnO30pKCk7
if /I "%GEN_CODE%"=="fr" >"%GEN_B64%" echo KCgpPT57Y29uc3QgRD17IkRhc2hib2FyZCI6IlRhYmxlYXUgZGUgYm9yZCIsIlNlcnZlcnMiOiJTZXJ2ZXVycyIsIkRlcGxveW1lbnRzIjoiRGVwbG9pZW1lbnRzIiwiU3RhY2tzIjoiU3RhY2tzIiwiQnVpbGRzIjoiQnVpbGRzIiwiQnVpbGRlcnMiOiJDb25zdHJ1Y3RldXJzIiwiUHJvY2VkdXJlcyI6IlByb2NlZHVyZXMiLCJBY3Rpb25zIjoiQWN0aW9ucyIsIlNldHRpbmdzIjoiUGFyYW1ldHJlcyIsIlVzZXJzIjoiVXRpbGlzYXRldXJzIiwiTG9ncyI6IkpvdXJuYXV4IiwiU3RhcnQiOiJEZW1hcnJlciIsIlN0b3AiOiJBcnJldGVyIiwiUmVzdGFydCI6IlJlZGVtYXJyZXIiLCJEZXBsb3kiOiJEZXBsb3llciIsIlVwZGF0ZSI6Ik1ldHRyZSBhIGpvdXIiLCJDcmVhdGUiOiJDcmVlciIsIkRlbGV0ZSI6IlN1cHByaW1lciIsIlNhdmUiOiJFbnJlZ2lzdHJlciIsIkNhbmNlbCI6IkFubnVsZXIiLCJTZWFyY2giOiJSZWNoZXJjaGVyIiwiU3RhdHVzIjoiU3RhdHV0IiwiTmFtZSI6Ik5vbSIsIkRlc2NyaXB0aW9uIjoiRGVzY3JpcHRpb24iLCJFbmFibGVkIjoiQWN0aXZlIiwiRGlzYWJsZWQiOiJEZXNhY3RpdmUiLCJBZG1pbiI6IkFkbWluaXN0cmF0ZXVyIiwiTm90aWZpY2F0aW9ucyI6Ik5vdGlmaWNhdGlvbnMifTtjb25zdCBza2lwPW5ldyBTZXQoWydTQ1JJUFQnLCdTVFlMRScsJ0NPREUnLCdQUkUnLCdURVhUQVJFQScsJ05PU0NSSVBUJywnU1ZHJywnUEFUSCddKTtmdW5jdGlvbiB0cihzKXtjb25zdCB0PXMudHJpbSgpO3JldHVybiBEW3RdfHxzO31mdW5jdGlvbiB3YWxrKHIpe2NvbnN0IHc9ZG9jdW1lbnQuY3JlYXRlVHJlZVdhbGtlcihyLE5vZGVGaWx0ZXIuU0hPV19URVhUKTtsZXQgbjt3aGlsZShuPXcubmV4dE5vZGUoKSl7aWYobi5wYXJlbnRFbGVtZW50JiYhc2tpcC5oYXMobi5wYXJlbnRFbGVtZW50LnRhZ05hbWUpKXtjb25zdCB4PXRyKG4ubm9kZVZhbHVlKTtpZih4IT09bi5ub2RlVmFsdWUpbi5ub2RlVmFsdWU9eDt9fXIucXVlcnlTZWxlY3RvckFsbCYmci5xdWVyeVNlbGVjdG9yQWxsKCdbcGxhY2Vob2xkZXJdLFt0aXRsZV0sW2FyaWEtbGFiZWxdJykuZm9yRWFjaChlPT5bJ3BsYWNlaG9sZGVyJywndGl0bGUnLCdhcmlhLWxhYmVsJ10uZm9yRWFjaChhPT57aWYoZS5oYXNBdHRyaWJ1dGUoYSkpe2NvbnN0IHY9ZS5nZXRBdHRyaWJ1dGUoYSkseD10cih2KTtpZih4IT09dillLnNldEF0dHJpYnV0ZShhLHgpO319KSk7fXdhbGsoZG9jdW1lbnQuYm9keSk7bmV3IE11dGF0aW9uT2JzZXJ2ZXIobXM9Pm1zLmZvckVhY2gobT0+bS5hZGRlZE5vZGVzLmZvckVhY2gobj0+bi5ub2RlVHlwZT09PTEmJndhbGsobikpKSkub2JzZXJ2ZShkb2N1bWVudC5ib2R5LHtjaGlsZExpc3Q6dHJ1ZSxzdWJ0cmVlOnRydWV9KTtkb2N1bWVudC5kb2N1bWVudEVsZW1lbnQubGFuZz0nZnInO30pKCk7
if /I "%GEN_CODE%"=="ja" >"%GEN_B64%" echo KCgpPT57Y29uc3QgRD17IkRhc2hib2FyZCI6IlxcdTMwYzBcXHUzMGMzXFx1MzBiN1xcdTMwZTVcXHUzMGRjXFx1MzBmY1xcdTMwYzkiLCJTZXJ2ZXJzIjoiXFx1MzBiNVxcdTMwZmNcXHUzMGQwXFx1MzBmYyIsIkRlcGxveW1lbnRzIjoiXFx1MzBjN1xcdTMwZDdcXHUzMGVkXFx1MzBhNCIsIlN0YWNrcyI6IlxcdTMwYjlcXHUzMGJmXFx1MzBjM1xcdTMwYWYiLCJCdWlsZHMiOiJcXHUzMGQzXFx1MzBlYlxcdTMwYzkiLCJCdWlsZGVycyI6IlxcdTMwZDNcXHUzMGViXFx1MzBjMFxcdTMwZmMiLCJQcm9jZWR1cmVzIjoiXFx1MzBkN1xcdTMwZWRcXHUzMGI3XFx1MzBmY1xcdTMwYjhcXHUzMGUzIiwiQWN0aW9ucyI6IlxcdTMwYTJcXHUzMGFmXFx1MzBiN1xcdTMwZTdcXHUzMGYzIiwiU2V0dGluZ3MiOiJcXHU4YTJkXFx1NWI5YSIsIlVzZXJzIjoiXFx1MzBlNlxcdTMwZmNcXHUzMGI2XFx1MzBmYyIsIkxvZ3MiOiJcXHUzMGVkXFx1MzBiMCIsIlN0YXJ0IjoiXFx1OTU4YlxcdTU5Y2IiLCJTdG9wIjoiXFx1NTA1Y1xcdTZiNjIiLCJSZXN0YXJ0IjoiXFx1NTE4ZFxcdThkNzdcXHU1MmQ1IiwiRGVwbG95IjoiXFx1MzBjN1xcdTMwZDdcXHUzMGVkXFx1MzBhNCIsIlVwZGF0ZSI6IlxcdTY2ZjRcXHU2NWIwIiwiQ3JlYXRlIjoiXFx1NGY1Y1xcdTYyMTAiLCJEZWxldGUiOiJcXHU1MjRhXFx1OTY2NCIsIlNhdmUiOiJcXHU0ZmRkXFx1NWI1OCIsIkNhbmNlbCI6IlxcdTMwYWRcXHUzMGUzXFx1MzBmM1xcdTMwYmJcXHUzMGViIiwiU2VhcmNoIjoiXFx1NjkxY1xcdTdkMjIiLCJTdGF0dXMiOiJcXHU3MmI2XFx1NjE0YiIsIk5hbWUiOiJcXHU1NDBkXFx1NTI0ZCIsIkRlc2NyaXB0aW9uIjoiXFx1OGFhY1xcdTY2MGUiLCJFbmFibGVkIjoiXFx1NjcwOVxcdTUyYjkiLCJEaXNhYmxlZCI6IlxcdTcxMjFcXHU1MmI5IiwiQWRtaW4iOiJcXHU3YmExXFx1NzQwNlxcdTgwMDUiLCJOb3RpZmljYXRpb25zIjoiXFx1OTAxYVxcdTc3ZTUifTtjb25zdCBza2lwPW5ldyBTZXQoWydTQ1JJUFQnLCdTVFlMRScsJ0NPREUnLCdQUkUnLCdURVhUQVJFQScsJ05PU0NSSVBUJywnU1ZHJywnUEFUSCddKTtmdW5jdGlvbiB0cihzKXtjb25zdCB0PXMudHJpbSgpO3JldHVybiBEW3RdfHxzO31mdW5jdGlvbiB3YWxrKHIpe2NvbnN0IHc9ZG9jdW1lbnQuY3JlYXRlVHJlZVdhbGtlcihyLE5vZGVGaWx0ZXIuU0hPV19URVhUKTtsZXQgbjt3aGlsZShuPXcubmV4dE5vZGUoKSl7aWYobi5wYXJlbnRFbGVtZW50JiYhc2tpcC5oYXMobi5wYXJlbnRFbGVtZW50LnRhZ05hbWUpKXtjb25zdCB4PXRyKG4ubm9kZVZhbHVlKTtpZih4IT09bi5ub2RlVmFsdWUpbi5ub2RlVmFsdWU9eDt9fXIucXVlcnlTZWxlY3RvckFsbCYmci5xdWVyeVNlbGVjdG9yQWxsKCdbcGxhY2Vob2xkZXJdLFt0aXRsZV0sW2FyaWEtbGFiZWxdJykuZm9yRWFjaChlPT5bJ3BsYWNlaG9sZGVyJywndGl0bGUnLCdhcmlhLWxhYmVsJ10uZm9yRWFjaChhPT57aWYoZS5oYXNBdHRyaWJ1dGUoYSkpe2NvbnN0IHY9ZS5nZXRBdHRyaWJ1dGUoYSkseD10cih2KTtpZih4IT09dillLnNldEF0dHJpYnV0ZShhLHgpO319KSk7fXdhbGsoZG9jdW1lbnQuYm9keSk7bmV3IE11dGF0aW9uT2JzZXJ2ZXIobXM9Pm1zLmZvckVhY2gobT0+bS5hZGRlZE5vZGVzLmZvckVhY2gobj0+bi5ub2RlVHlwZT09PTEmJndhbGsobikpKSkub2JzZXJ2ZShkb2N1bWVudC5ib2R5LHtjaGlsZExpc3Q6dHJ1ZSxzdWJ0cmVlOnRydWV9KTtkb2N1bWVudC5kb2N1bWVudEVsZW1lbnQubGFuZz0namEnO30pKCk7
certutil -f -decode "%GEN_B64%" "%GEN_JS%" >nul 2>&1
if errorlevel 1 exit /b 1
set "GEN_INDEX=%GEN_DIR%\index.html"
docker cp "%CORE_ID%:/app/ui/index.html" "%GEN_INDEX%" >nul 2>&1
if errorlevel 1 exit /b 1
powershell.exe -NoProfile -Command "$p='%GEN_INDEX%';$s=[IO.File]::ReadAllText($p);$s=[regex]::Replace($s,'<script id="komodo-generic-lang-loader".*?</script>','','Singleline');$s=$s.Replace('</body>','<script id="komodo-generic-lang-loader" src="/komodo-lang.js?v=%RANDOM%" defer></script></body>');[IO.File]::WriteAllText($p,$s,(New-Object Text.UTF8Encoding($false)))" >nul 2>&1
if errorlevel 1 exit /b 1
docker cp "%GEN_JS%" "%CORE_ID%:/app/ui/komodo-lang.js" >nul 2>&1
if errorlevel 1 exit /b 1
docker cp "%GEN_INDEX%" "%CORE_ID%:/app/ui/index.html" >nul 2>&1
if errorlevel 1 exit /b 1
>"%STATE_DIR%\generic-lang.enabled" echo(%GEN_CODE%
del /Q "%STATE_DIR%\ptbr.enabled" >nul 2>&1
exit /b 0


:DetectGitHubLogin
@echo off
set "GH_LOGIN="
for /F "usebackq delims=" %%U in (`gh api user --jq ".login" 2^>nul`) do set "GH_LOGIN=%%U"
if not defined GH_LOGIN (
    call :MsgError "Could not detect a account GitHub authenticated."
    exit /b 1
)
call :LogInfo "GitHub account detected: %GH_LOGIN%"
exit /b 0

:EnsureGitHubCli
@echo off
where gh.exe >nul 2>&1
if not errorlevel 1 exit /b 0

if exist "%ProgramFiles%\GitHub CLI\gh.exe" (
    set "PATH=%ProgramFiles%\GitHub CLI;%PATH%"
    exit /b 0
)

echo %YELLOW%GitHub CLI (gh) not found.%RESET%
echo.
echo O Control Center may tentar instalar automatically using winget.
choice /C SBC /N /M "Instalar GitHub CLI now? [S] Yes  [B] Back  [C] Cancel: "
if errorlevel 3 exit /b 1
if errorlevel 2 exit /b 1

where winget.exe >nul 2>&1
if errorlevel 1 (
    call :MsgError "winget not is disponivel. GitHub CLI not may be installed automatically."
    exit /b 1
)

call :LogAction "Instalar GitHub CLI via winget"
winget install --id GitHub.cli -e --source winget --accept-package-agreements --accept-source-agreements
if errorlevel 1 (
    call :MsgError "Failed to instalar GitHub CLI."
    exit /b 1
)

if exist "%ProgramFiles%\GitHub CLI\gh.exe" set "PATH=%ProgramFiles%\GitHub CLI;%PATH%"
where gh.exe >nul 2>&1
if errorlevel 1 (
    call :MsgError "GitHub CLI was installed, mas ainda not was localizado nesta sessao."
    exit /b 1
)

call :MsgOk "GitHub CLI installed."
exit /b 0


:CheckGitHubAuth
@echo off
gh auth status --hostname github.com >nul 2>&1
if not errorlevel 1 exit /b 0

echo %YELLOW%GitHub CLI ainda not is authenticated.%RESET%
echo O navegador will be usado by the GitHub for autorizar your account.
echo in the password and armazenada by the BAT.
echo.
choice /C SBC /N /M "start login official of the GitHub now? [S] Yes  [B] Back  [C] Cancel: "
if errorlevel 3 exit /b 1
if errorlevel 2 exit /b 1

call :LogAction "Iniciar autenticacao GitHub CLI"
gh auth login --hostname github.com --git-protocol https --web
if errorlevel 1 (
    call :MsgError "authentication GitHub not was completed."
    exit /b 1
)

gh auth setup-git >nul 2>&1
gh auth status --hostname github.com >nul 2>&1
if errorlevel 1 (
    call :MsgError "GitHub CLI continues without authentication valida."
    exit /b 1
)

call :MsgOk "GitHub authenticated."
exit /b 0


:EnsureGit
@echo off
where git.exe >nul 2>&1
if not errorlevel 1 exit /b 0
call :MsgError "Git not was found in the Windows."
exit /b 1


:WriteContributionReadme
@echo off
set "README_TARGET=%~1"
>"%README_TARGET%" echo # Komodo Control Center for Windows
>>"%README_TARGET%" echo.
>>"%README_TARGET%" echo This contribution adds a Windows batch control center for local Komodo management.
>>"%README_TARGET%" echo.
>>"%README_TARGET%" echo ## Goals
>>"%README_TARGET%" echo.
>>"%README_TARGET%" echo - One-click start, stop, restart and dashboard access
>>"%README_TARGET%" echo - Diagnostics and health checks
>>"%README_TARGET%" echo - Docker and Core logs
>>"%README_TARGET%" echo - Windows shortcut and startup integration
>>"%README_TARGET%" echo - Local user/admin maintenance helpers
>>"%README_TARGET%" echo - Optional live PT-BR UI translation helper
>>"%README_TARGET%" echo - Built-in action/error logging with optional Windows notifications
>>"%README_TARGET%" echo - Built-in beginner guide covering Komodo resources and workflows
>>"%README_TARGET%" echo - Safe GitHub contribution flow through fork and Pull Request
>>"%README_TARGET%" echo.
>>"%README_TARGET%" echo ## Safety
>>"%README_TARGET%" echo.
>>"%README_TARGET%" echo The script avoids committing unrelated local repository files when publishing updates.
>>"%README_TARGET%" echo Destructive operations require explicit confirmation.
>>"%README_TARGET%" echo.
>>"%README_TARGET%" echo ## Usage
>>"%README_TARGET%" echo.
>>"%README_TARGET%" echo Run `KOMODO_CONTROL_CENTER.bat` on Windows and use the numbered menu.
>>"%README_TARGET%" echo The default local dashboard expected by the script is `http://localhost:9120`.
exit /b 0


:WritePrBody
@echo off
set "PR_BODY=%~1"
>"%PR_BODY%" echo ## Summary
>>"%PR_BODY%" echo.
>>"%PR_BODY%" echo Adds/updates a Windows-oriented Komodo Control Center batch script.
>>"%PR_BODY%" echo.
>>"%PR_BODY%" echo ## Included
>>"%PR_BODY%" echo.
>>"%PR_BODY%" echo - one-click lifecycle controls
>>"%PR_BODY%" echo - diagnostics, health checks and logs
>>"%PR_BODY%" echo - Windows shortcut/startup helpers
>>"%PR_BODY%" echo - local account/admin maintenance helpers
>>"%PR_BODY%" echo - optional PT-BR runtime translation helper
>>"%PR_BODY%" echo - Control Center audit/error logs and optional notifications
>>"%PR_BODY%" echo - beginner-oriented Komodo guide
>>"%PR_BODY%" echo.
>>"%PR_BODY%" echo ## Notes
>>"%PR_BODY%" echo.
>>"%PR_BODY%" echo This PR is intentionally limited to `scripts/windows/`.
>>"%PR_BODY%" echo It does not include unrelated local changes from the contributor's working tree.
exit /b 0

:LOG_CENTER
@echo off
cls
call :Header
call :GetNotificationState
echo %BLUE%%BOLD%   [=] CENTRAL of logs and notifications%RESET%
echo.
echo %GRAY%File main:%RESET%
echo   %LOG_FILE%
echo %GRAY%Errors separados:%RESET%
echo   %ERRORR_LOG%
echo.
call :LogSummary
echo.
echo   %CYAN%[1]%RESET% view ultimas 80 linhas of the log
echo   %CYAN%[2]%RESET% view report complete
echo   %RED%[3]%RESET% view only ERRORS
echo   %YELLOW%[4]%RESET% view only warnings
echo   %CYAN%[5]%RESET% open folder of the logs
echo   %GREEN%[6]%RESET% Ativar notifications of error
echo   %YELLOW%[7]%RESET% Desativar notifications of error
echo   %CYAN%[8]%RESET% Testar notification
echo   %RED%[9]%RESET% Limpar logs
echo.
echo   %GRAY%[B] Back   [C] Cancel%RESET%
echo.
set "LOGOP="
set /p "LOGOP=%WHITE%%BOLD%choose: %RESET%"
call :LogAction "CENTRAL DE LOGS selecionado: %LOGOP%"

if /I "%LOGOP%"=="B" goto MAIN
if /I "%LOGOP%"=="C" goto MAIN
if "%LOGOP%"=="1" goto LOG_TAIL
if "%LOGOP%"=="2" goto LOG_FULL
if "%LOGOP%"=="3" goto LOG_ERRORS
if "%LOGOP%"=="4" goto LOG_WARNINGS
if "%LOGOP%"=="5" goto LOG_FOLDER
if "%LOGOP%"=="6" goto NOTIFY_ON
if "%LOGOP%"=="7" goto NOTIFY_OFF
if "%LOGOP%"=="8" goto NOTIFY_TEST
if "%LOGOP%"=="9" goto LOG_CLEAR
call :MsgError "Invalid option in the Central of logs: %LOGOP%"
timeout /t 2 /nobreak >nul
goto LOG_CENTER

:LOG_TAIL
cls
call :Header
echo %BLUE%%BOLD%   ULTIMAS 80 LINHAS of the log%RESET%
echo.
powershell.exe -NoProfile -ExecutionPolicy Bypass -Command "if(Test-Path -LiteralPath $env:LOG_FILE){Get-Content -LiteralPath $env:LOG_FILE -Tail 80}else{'Log ainda nao existe.'}"
goto LOG_CENTER_PAUSE

:LOG_FULL
cls
call :Header
echo %BLUE%%BOLD%   report complete of the CONTROL CENTER%RESET%
echo.
if exist "%LOG_FILE%" (
    type "%LOG_FILE%"
) else (
    echo in the log registrado.
)
goto LOG_CENTER_PAUSE

:LOG_ERRORS
cls
call :Header
echo %RED%%BOLD%   report of ERRORS%RESET%
echo.
if exist "%ERROR_LOG%" (
    type "%ERROR_LOG%"
) else (
    echo in the error registrado.
)
goto LOG_CENTER_PAUSE

:LOG_WARNINGS
cls
call :Header
echo %YELLOW%%BOLD%   report of warnings%RESET%
echo.
if exist "%LOG_FILE%" (
    findstr /I /C:"[WARN]" "%LOG_FILE%"
    if errorlevel 1 echo Nenhum aviso registrado.
) else (
    echo in the log registrado.
)
goto LOG_CENTER_PAUSE

:LOG_FOLDER
call :LogAction "Abrir pasta de logs"
start "" explorer.exe "%LOG_DIR%"
goto LOG_CENTER

:NOTIFY_ON
>"%NOTIFY_FLAG%" echo(ON
call :LogInfo "Notificacoes de erro ATIVADAS"
call :MsgOk "notifications of error ativadas."
goto LOG_CENTER_PAUSE

:NOTIFY_OFF
>"%NOTIFY_FLAG%" echo(OFF
call :LogInfo "Notificacoes de erro DESATIVADAS"
echo %YELLOW%[OK] notifications of error desativadas.%RESET%
goto LOG_CENTER_PAUSE

:NOTIFY_TEST
call :LogAction "Teste de notificacao solicitado"
call :NotifyError "Teste: notificacoes do Komodo Control Center estao funcionando."
call :MsgOk "Teste enviado. if as notifications estiverem ON, o Windows exibira o warning."
goto LOG_CENTER_PAUSE

:LOG_CLEAR
echo.
choice /C SBC /N /M "Limpar all os logs? [S] Yes  [B] Back  [C] Cancel: "
if errorlevel 3 goto LOG_CENTER
if errorlevel 2 goto LOG_CENTER
del /Q "%LOG_FILE%" >nul 2>&1
del /Q "%ERROR_LOG%" >nul 2>&1
call :BeginSession
call :LogInfo "Logs limpos pelo usuario"
call :MsgOk "logs limpos. a new sessao of log was iniciada."
goto LOG_CENTER_PAUSE

:LOG_CENTER_PAUSE
echo.
echo %GRAY%--------------------------------------------------------------------------------%RESET%
choice /C BC /N /M "[B] Back for Central of logs  [C] menu main: "
if errorlevel 2 goto MAIN
goto LOG_CENTER


:REMOVE_CONTAINERS
cls
call :Header
echo %RED%%BOLD%   [!] removes containers%RESET%
echo.
echo is acao runs docker compose down.
echo %GREEN%volumes and data will be preservados.%RESET%
echo.
choice /C SBC /N /M "Continue? [S] Yes  [B] Back  [C] Cancel: "
if errorlevel 3 goto MAIN
if errorlevel 2 goto MAIN
call :Preflight
if errorlevel 1 goto PAUSE_MAIN
call :EnsureDocker
if errorlevel 1 goto PAUSE_MAIN
call :Compose down --remove-orphans
if errorlevel 1 (
    call :MsgError "Failed to removes containers."
) else (
    call :Log "Containers removed without volumes"
    call :MsgOk "containers removidos. volumes preservados."
)
goto PAUSE_MAIN

:FACTORY_RESET
cls
call :Header
echo %RED%%BOLD%   [!!] RESET TOTAL%RESET%
echo.
echo %RED%%BOLD%ATENCAO: is ACAO may delete O database and data in volumes DOCKER.%RESET%
echo.
set "CONFIRM="
set /p "CONFIRM=type delete-TUDO for continue, or B/C for cancel: "
if /I "%CONFIRM%"=="B" goto MAIN
if /I "%CONFIRM%"=="C" goto MAIN
if /I not "%CONFIRM%"=="APAGAR-TUDO" (
    echo.
    call :MsgOk "cancelled. Nada was apagado."
    goto PAUSE_MAIN
)
call :Preflight
if errorlevel 1 goto PAUSE_MAIN
call :EnsureDocker
if errorlevel 1 goto PAUSE_MAIN
call :Compose down -v --remove-orphans
if errorlevel 1 (
    call :MsgError "Failure durante o reset."
) else (
    call :Log "FULL RESET executed"
    call :MsgOk "Reset total completed."
)
goto PAUSE_MAIN

:EXIT_MENU
call :LogAction "Sair do Control Center"
call :LogInfo "Control Center encerrado"
cls
echo.
echo %CYAN%%BOLD%   KOMODO CONTROL CENTER encerrado.%RESET%
echo.
timeout /t 1 /nobreak >nul
exit /b 0

:ROOT_ERROR
if not exist "%LOG_DIR%\" mkdir "%LOG_DIR%" >nul 2>&1
call :LogError "Pasta do repositorio nao encontrada: %KOMODO_ROOT%"
call :NotifyError "Pasta do repositorio Komodo nao encontrada."
cls
echo.
echo ============================================================
echo   KOMODO CONTROL CENTER - ERROR
echo ============================================================
echo.
echo A folder configured not was found:
echo.
echo   %KOMODO_ROOT%
echo.
echo check if a unidade H: is conectada and if a folder existe.
echo.
pause
exit /b 2

:PAUSE_MAIN
echo.
echo %GRAY%   ------------------------------------------------------------------------------------------------%RESET%
echo %GRAY%   [B] Back to the menu main   [C] Cancel / back%RESET%
choice /C BC /N /M "choose: "
goto MAIN

rem ============================================================================
rem  FUNCTIONS
rem ============================================================================

:Header
echo %CYAN%%BOLD%================================================================================================%RESET%
echo %CYAN%%BOLD%       K O M O D O   C O N T R O L   C and N T and R    //    W I N D O W S                       %RESET%
echo %CYAN%%BOLD%================================================================================================%RESET%
echo.
echo %GRAY%   ROOT   :%RESET% %KOMODO_ROOT%
if defined COMPOSE_FILE (
    echo %GRAY%   STACK  :%RESET% %COMPOSE_FILE%
) else (
    echo %GRAY%   STACK  :%RESET% %RED%NOT detected%RESET%
)
echo %GRAY%   DOCKER :%RESET% !QS_DOCKER!   %GRAY%KOMODO:%RESET% !QS_KOMODO!   %GRAY%HTTP:%RESET% !QS_HTTP!
set "HEADER_NOTIFY=OFF"
if exist "%NOTIFY_FLAG%" (
    findstr /I /X "ON" "%NOTIFY_FLAG%" >nul 2>&1
    if not errorlevel 1 set "HEADER_NOTIFY=ON"
)
echo %GRAY%   log    :%RESET% %LOG_FILE%   %GRAY%notifications:%RESET% !HEADER_NOTIFY!
echo %GRAY%   ------------------------------------------------------------------------------------------------%RESET%
echo %GRAY%   Navegacao: [B] Back to the menu previous   [C] Cancel OPERATION%RESET%
echo.
exit /b 0

:QuickState
set "QS_DOCKER=%RED%OFFLINE%RESET%"
set "QS_KOMODO=%GRAY%DESCONHECIDO%RESET%"
set "QS_HTTP=%RED%OFFLINE%RESET%"

where docker >nul 2>&1
if not errorlevel 1 (
    docker info >nul 2>&1
    if not errorlevel 1 (
        set "QS_DOCKER=%GREEN%ONLINE%RESET%"
        docker ps --format "{{.Names}}" 2>nul | findstr /I "komodo" >nul 2>&1
        if not errorlevel 1 set "QS_KOMODO=%GREEN%ATIVO%RESET%"
    )
)

where curl >nul 2>&1
if not errorlevel 1 (
    curl.exe -fsS --max-time 2 "%DASHBOARD_URL%" >nul 2>&1
    if not errorlevel 1 set "QS_HTTP=%GREEN%ONLINE%RESET%"
)
exit /b 0

:DetectCompose
set "COMPOSE_FILE="
set "ENV_FILE="

for %%F in (
    "%KOMODO_ROOT%\compose\mongo.compose.yaml"
    "%KOMODO_ROOT%\compose\sqlite.compose.yaml"
    "%KOMODO_ROOT%\compose\compose.yaml"
    "%KOMODO_ROOT%\compose\docker-compose.yml"
    "%KOMODO_ROOT%\compose\docker-compose.yaml"
    "%KOMODO_ROOT%\compose.yaml"
    "%KOMODO_ROOT%\docker-compose.yml"
    "%KOMODO_ROOT%\docker-compose.yaml"
    "%KOMODO_ROOT%\dev.compose.yaml"
) do (
    if not defined COMPOSE_FILE if exist "%%~fF" set "COMPOSE_FILE=%%~fF"
)

for %%F in (
    "%KOMODO_ROOT%\compose\compose.env"
    "%KOMODO_ROOT%\compose\.env"
    "%KOMODO_ROOT%\.env"
) do (
    if not defined ENV_FILE if exist "%%~fF" set "ENV_FILE=%%~fF"
)
exit /b 0

:Compose
if not defined COMPOSE_FILE (
    call :MsgError "in the file compose was detected."
    exit /b 1
)
call :LogAction "docker compose command executed (arguments redacted)"
if defined ENV_FILE (
    docker compose -p "%PROJECT_NAME%" --env-file "%ENV_FILE%" -f "%COMPOSE_FILE%" %*
) else (
    docker compose -p "%PROJECT_NAME%" -f "%COMPOSE_FILE%" %*
)
set "COMPOSE_RC=%errorlevel%"
if "%COMPOSE_RC%"=="0" (
    call :LogSuccess "docker compose %* - exit code 0"
) else (
    call :LogError "docker compose %* - exit code %COMPOSE_RC%"
    call :NotifyError "Docker Compose falhou. Comando: %* - codigo %COMPOSE_RC%"
)
exit /b %COMPOSE_RC%

:Preflight
if not exist "%KOMODO_ROOT%\" (
    call :MsgError "folder of the repository not found."
    exit /b 1
)
call :DetectCompose
if not defined COMPOSE_FILE (
    call :MsgError "in the file compose suportado was found."
    exit /b 1
)
where docker >nul 2>&1
if errorlevel 1 (
    call :MsgError "Docker CLI not found. Instale o Docker Desktop."
    exit /b 1
)
docker compose version >nul 2>&1
if errorlevel 1 (
    call :MsgError "Docker Compose v2 not found."
    exit /b 1
)
exit /b 0

:EnsureDocker
docker info >nul 2>&1
if not errorlevel 1 exit /b 0

echo %YELLOW%Docker Engine offline. trying start Docker Desktop...%RESET%
call :StartDockerDesktop

for /L %%I in (1,1,45) do (
    docker info >nul 2>&1
    if not errorlevel 1 (
        echo.
        call :MsgOk "Docker Engine is ready."
        exit /b 0
    )
    <nul set /p "=."
    timeout /t 2 /nobreak >nul
)

echo.
call :MsgError "Docker not respondeu. Abra o Docker Desktop manualmente."
exit /b 1

:StartDockerDesktop
set "DOCKER_EXE="
if exist "%ProgramFiles%\Docker\Docker\Docker Desktop.exe" set "DOCKER_EXE=%ProgramFiles%\Docker\Docker\Docker Desktop.exe"
if exist "%LOCALAPPDATA%\Docker\Docker Desktop.exe" set "DOCKER_EXE=%LOCALAPPDATA%\Docker\Docker Desktop.exe"

if defined DOCKER_EXE (
    start "" "%DOCKER_EXE%"
) else (
    start "" "docker-desktop:" >nul 2>&1
)
exit /b 0

:WaitHttp
set "MAXWAIT=%~1"
if "%MAXWAIT%"=="" set "MAXWAIT=20"
echo %GRAY%Aguardando o dashboard responder...%RESET%
for /L %%I in (1,1,%MAXWAIT%) do (
    curl.exe -fsS --max-time 2 "%DASHBOARD_URL%" >nul 2>&1
    if not errorlevel 1 (
        echo %GREEN%dashboard online.%RESET%
        exit /b 0
    )
    <nul set /p "=."
    timeout /t 1 /nobreak >nul
)
echo.
echo %YELLOW%O dashboard ainda not respondeu. Consulte os logs.%RESET%
exit /b 1


:CreateShortcut
call :LogAction "Criar/Reparar atalho Desktop"
if /I not "%~f0"=="%INSTALL_BAT%" copy /Y "%~f0" "%INSTALL_BAT%" >nul 2>&1
call :EnsureControlIcon
if errorlevel 1 (
    call :MsgError "Could not preparar o file of icone."
    exit /b 1
)
set "VBS=%TEMP%\komodo_shortcut_%RANDOM%.vbs"
> "%VBS%" echo Set oWS = CreateObject("WScript.Shell")
>>"%VBS%" echo sDesktop = oWS.SpecialFolders("Desktop")
>>"%VBS%" echo linkPath = sDesktop ^& "\Komodo Control Center.lnk"
>>"%VBS%" echo Set oLink = oWS.CreateShortcut(linkPath)
>>"%VBS%" echo oLink.TargetPath = "%INSTALL_BAT%"
>>"%VBS%" echo oLink.WorkingDirectory = "%KOMODO_ROOT%"
>>"%VBS%" echo oLink.Description = "Komodo Control Center - Windows"
>>"%VBS%" echo oLink.IconLocation = "%CONTROL_ICON%,0"
>>"%VBS%" echo oLink.WindowStyle = 1
>>"%VBS%" echo oLink.Save
cscript //nologo "%VBS%" >nul 2>&1
set "RC=%errorlevel%"
del /Q "%VBS%" >nul 2>&1
if not "%RC%"=="0" (
    call :MsgError "Could not create o shortcut."
    exit /b 1
)
if not exist "%USERPROFILE%\Desktop\Komodo Control Center.lnk" (
    rem Desktop can be redirected by OneDrive; VBS SpecialFolders handles that path.
    call :LogInfo "Atalho criado via SpecialFolders Desktop."
)
ie4uinit.exe -show >nul 2>&1
call :LogSuccess "Atalho Desktop criado/reparado com ICO real: %CONTROL_ICON%"
call :MsgOk "shortcut created/reparado with ICONE."
echo %GRAY%if o Explorer estava aberto, o cache of icones was solicitado for update.%RESET%
exit /b 0

:InstallAutostart
call :LogAction "Habilitar inicio automatico com Windows"
if /I not "%~f0"=="%INSTALL_BAT%" copy /Y "%~f0" "%INSTALL_BAT%" >nul 2>&1
call :EnsureControlIcon
if errorlevel 1 (
    call :MsgError "Could not preparar o file of icone."
    exit /b 1
)
set "VBS=%TEMP%\komodo_startup_%RANDOM%.vbs"
> "%VBS%" echo Set oWS = CreateObject("WScript.Shell")
>>"%VBS%" echo sStartup = oWS.SpecialFolders("Startup")
>>"%VBS%" echo Set oLink = oWS.CreateShortcut(sStartup ^& "\Komodo Control Center.lnk")
>>"%VBS%" echo oLink.TargetPath = "%INSTALL_BAT%"
>>"%VBS%" echo oLink.WorkingDirectory = "%KOMODO_ROOT%"
>>"%VBS%" echo oLink.Description = "Komodo Control Center"
>>"%VBS%" echo oLink.IconLocation = "%CONTROL_ICON%,0"
>>"%VBS%" echo oLink.Save
cscript //nologo "%VBS%" >nul 2>&1
set "RC=%errorlevel%"
del /Q "%VBS%" >nul 2>&1
if not "%RC%"=="0" (
    call :MsgError "Failed to configurar startup."
) else (
    call :Log "Windows autostart enabled"
    call :MsgOk "shortcut adicionado a startup of the Windows."
)
exit /b 0

:RemoveAutostart
set "STARTUP_LNK=%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup\Komodo Control Center.lnk"
if exist "%STARTUP_LNK%" del /Q "%STARTUP_LNK%" >nul 2>&1
if exist "%STARTUP_LNK%" (
    call :MsgError "Failed to removes startup."
) else (
    call :Log "Windows autostart disabled"
    call :MsgOk "startup automatic removida."
)
exit /b 0

:SecurityCheck
findstr /I /C:"%~1" "%ENV_FILE%" >nul 2>&1
if not errorlevel 1 (
    set /A SECWARN+=1
    echo %YELLOW%   [!] %~2%RESET%
) else (
    echo %GREEN%   [OK] %~2 - not detected%RESET%
)
exit /b 0

:DiagOk
echo %GREEN%   [OK]%RESET% %~1
call :LogSuccess "DIAGNOSTICO: %~1"
exit /b 0

:DiagWarn
set /A WARNINGS+=1
echo %YELLOW%   [warning]%RESET% %~1
call :LogWarn "DIAGNOSTICO: %~1"
exit /b 0

:DiagFail
set /A ERRORS+=1
echo %RED%   [ERROR]%RESET% %~1
call :LogError "DIAGNOSTICO: %~1"
call :NotifyError "%~1"
exit /b 0

:MsgOk
echo %GREEN%%BOLD%[OK] %~1%RESET%
call :LogSuccess "%~1"
exit /b 0

:MsgError
echo %RED%%BOLD%[ERROR] %~1%RESET%
call :LogError "%~1"
call :NotifyError "%~1"
exit /b 0

:Log
call :LogInfo "%~1"
exit /b 0

:LogInfo
>>"%LOG_FILE%" echo [%date% %time%] [INFO] %~1
exit /b 0

:LogAction
>>"%LOG_FILE%" echo [%date% %time%] [ACTION] %~1
exit /b 0

:LogSuccess
>>"%LOG_FILE%" echo [%date% %time%] [SUCCESS] %~1
exit /b 0

:LogWarn
>>"%LOG_FILE%" echo [%date% %time%] [WARN] %~1
exit /b 0

:LogError
>>"%LOG_FILE%" echo [%date% %time%] [ERROR] %~1
>>"%ERROR_LOG%" echo [%date% %time%] [ERROR] %~1
exit /b 0

:BeginSession
>>"%LOG_FILE%" echo.
>>"%LOG_FILE%" echo ================================================================================================
>>"%LOG_FILE%" echo [%date% %time%] [SESSION] Komodo Control Center iniciado - PID aproximado %RANDOM%
>>"%LOG_FILE%" echo ================================================================================================
exit /b 0

:GetNotificationState
set "NOTIFY_STATE=%RED%OFF%RESET%"
if exist "%NOTIFY_FLAG%" (
    findstr /I /X "ON" "%NOTIFY_FLAG%" >nul 2>&1
    if not errorlevel 1 set "NOTIFY_STATE=%GREEN%ON%RESET%"
)
echo %GRAY%notifications of error:%RESET% !NOTIFY_STATE!
exit /b 0

:NotifyError
if not exist "%NOTIFY_FLAG%" exit /b 0
findstr /I /X "ON" "%NOTIFY_FLAG%" >nul 2>&1
if errorlevel 1 exit /b 0
if not exist "%NOTIFY_PS1%" call :EnsureNotifyScript
if not exist "%NOTIFY_PS1%" exit /b 0
set "KOMODO_NOTIFY_MSG=%~1"
start "" /B powershell.exe -NoProfile -ExecutionPolicy Bypass -WindowStyle Hidden -File "%NOTIFY_PS1%" >nul 2>&1
exit /b 0

:LogSummary
set "LOG_TOTAL=0"
set "LOG_ERRORS=0"
set "LOG_WARNS=0"
set "LOG_ACTIONS=0"
if exist "%LOG_FILE%" (
    for /F %%N in ('find /V /C "" ^< "%LOG_FILE%"') do set "LOG_TOTAL=%%N"
    for /F %%N in ('findstr /I /C:"[ERROR]" "%LOG_FILE%" ^| find /C /V ""') do set "LOG_ERRORS=%%N"
    for /F %%N in ('findstr /I /C:"[WARN]" "%LOG_FILE%" ^| find /C /V ""') do set "LOG_WARNS=%%N"
    for /F %%N in ('findstr /I /C:"[ACTION]" "%LOG_FILE%" ^| find /C /V ""') do set "LOG_ACTIONS=%%N"
)
echo %WHITE%%BOLD%RESUMO:%RESET%  Linhas: !LOG_TOTAL!   Acoes: !LOG_ACTIONS!   %YELLOW%warnings: !LOG_WARNS!%RESET%   %RED%Errors: !LOG_ERRORRS!%RESET%
exit /b 0

:EnsureSupportFiles
call :EnsureControlIcon
call :EnsureNotifyScript
exit /b 0

:EnsureControlIcon
if exist "%CONTROL_ICON%" exit /b 0
set "ICON_B64=%TEMP%\komodo_icon_%RANDOM%.b64"
> "!ICON_B64!" echo AAABAAUAEBAAAAAAIADqAgAAVgAAABgYAAAAACAA4wQAAEADAAAgIAAAAAAgAJEGAAAjCAAAMDAA
>>"!ICON_B64!" echo AAAAIABUCgAAtA4AAEBAAAAAACAAgAIAAAgZAACJUE5HDQoaCgAAAA1JSERSAAAAEAAAABAIBgAA
>>"!ICON_B64!" echo AB/z/2EAAAKxSURBVHicjZNNaFRnFIaf77vXO7lz7yQzI9FoFrFtSlBoVeLCiARTVOyiNFoMInQ1
>>"!ICON_B64!" echo iBCELptduij+bFT8/9sIcSFKCaVIQFEqLU3pQtpa8SeLijUTmhFnJnOdmeu933ExJlHMwgNncRbn
>>"!ICON_B64!" echo OYf3vEdl2roErZFyGRIO1tJliDEsFEprzPT/SK2OSnlgBButkaCC07cZNzeIlUwCEBuDmutUjRSI
>>"!ICON_B64!" echo CwXqI2ep//Y7OpnAlnIZp28z/sGjBMNDlH69DY6D57qICErrxkbVgDiy6TrzFUtGt3Knv0zll3to
>>"!ICON_B64!" echo HAc3N0gwPERw9RI9n6ykp3MF4VQeKT4nnvwPKUzTtH0AgJd/jGPyk7hf5yCRgOyqDbJi7E9p/Xid
>>"!ICON_B64!" echo WG6b3H84IfcfPBKrpUOyiz+Q7Efd0jE6Lp15kdYtu6QluVxa1/ZKx9hdya7qEZtZwRwHjCEIXiBK
>>"!ICON_B64!" echo Q6mAvaEP/9QFQFEc+JLo3t9o3yOq1FBEYAQ9J7EIAJZt44U1nC8GSF8ZJX76lOL2bUR/3UE1uWAM
>>"!ICON_B64!" echo ylLwWuJ5AKC0RRgE/NDehb//CNWRi5RzuzEzJZSfYm5bme95CwCgjSFIepgmFzMxgZSKKMte0Bfv
>>"!ICON_B64!" echo AMQYbN9n5793qZ8+gn9oP6lj52GRg8zMgP0uaB6gFCBEUUxsDNXD31PasxendxPpKz9hf7oGeTbd
>>"!ICON_B64!" echo 0ErrNwCzRRgiWpPJpMlkM9C8mPD6NYo7PscUpklf/hF3z77Gter1xkCtsc1UHqnVsFZ34zz6h+Mn
>>"!ICON_B64!" echo zkEckxADvk/85DGl3f14Q9+R2LGL6rmTLFq/EanVMFN5VNpvl1krvxgeovDzTXAStKT811a2kPgl
>>"!ICON_B64!" echo RBHKS2F92Il/4DCVb78hvHUDlVm+UiSo4PR+hpsbxPY8AKI4fvuZAIkiiCKqF04S3r6J8nzU+76z
>>"!ICON_B64!" echo ArAs4qlJqIeo5mYwhlfnwS2rwnWmAQAAAABJRU5ErkJggolQTkcNChoKAAAADUlIRFIAAAAYAAAA
>>"!ICON_B64!" echo GAgGAAAA4Hc9+AAABKpJREFUeJy1lktsVVUUhr+9z6P3/egrxGpEKJTSYEBAGJhQXwwYgAYNojzE
>>"!ICON_B64!" echo KBiCJEYwQYEERYxCo00NMoDEiANJrCQmJA4cqhFKLG2tJbwcKDRQ+7i9t72vc85ycPpS6QUHrtFJ
>>"!ICON_B64!" echo 9sn69/rXv9a/VXJGnTAehgFOERkdRQoFlFLcTYgIyrZRoRCYFrjuxJk58aU1kh5GhSPYjz6J+eAi
>>"!ICON_B64!" echo KBTgTiAiYFk43R0U284hwylUNAqeNwVAaySXw1q0lPC+9zDr6/GGhkCbgEyfXCkQUOLi2RG43E7m
>>"!ICON_B64!" echo w/cpXOhEBy3wPEwMA0kPYy5aQvzU1zg9l0i9sBb57QratvFcD9dzUai/gynt314r3OEslVtWMndP
>>"!ICON_B64!" echo PYXPV9G9OUu2rRsdi2BK0UGFo0T2H8Lp6SG1ZT382ceoMsjnspTZNuFwGE+8SRBtIIU8ShuoMgs3
>>"!ICON_B64!" echo NULVnAFs+imzc8x4az3XNjWjikNosiNYS5Zh1NUz8s5eVH8fhXCUxsZH2L79ZRofW0HecTEsG0wT
>>"!ICON_B64!" echo AkFwHJRlEzncQtm6TWjy9H5zg9z1LEO/ZBmtWUng4cVINgeJQJXcs7dZZnUPSvl9DVJVUy/Y1dJ6
>>"!ICON_B64!" echo +oyIiLSePiPY1VJ1b4MkaxokXlYllUuekJnfdUttr8gD53ulom6ZxGIPSOX986Si9iGZ2TUsNXs/
>>"!ICON_B64!" echo lkSgWkxQvlq04UsslwWtSWcyOI5DOp0BrcEwkIEBAk89S3DHG5i1syh2dDLa0oQ30I8ZC+Jk8uiI
>>"!ICON_B64!" echo g7ZACsUpKhqXokw2UWuNaZpoywQRvL6bBF58lfCb+0ApimfPkd69A+/676hEEnE9lKFA+8oazzk5
>>"!ICON_B64!" echo B7eTOKDzObAsIgcPY65aA1qTPXGUbEsTaAOVrADXmZDsP1WtS2lcAdlwjMgHzQQ3P4c4DtljzYw2
>>"!ICON_B64!" echo HQLLAtvyk5eIaSuImgaXPTgydzlVC8Pku66SObCH4tkfULG4T+fYtJaKaStwgYQCDBMBvOt/4HS2
>>"!ICON_B64!" echo g13mN11KTPjdAORcjyoFa/qukbnRS6BxBfHWb7EWL0X6bvmLUU3P8B0BvLHyF1/tYvD1V3AudGDO
>>"!ICON_B64!" echo mU206VOCO3cjQ4PgFMEoqZMSTR6vJBLD/bmN4W0bybW2osJhQq/tIvrRMVSyHBns9xteEmCczymr
>>"!ICON_B64!" echo 2fM8XNdFikUIRcFzyezazsjhg8hAP2WrVxNtOY614nG8WzfHGq7+NVMaBGwbPNcveayBoVAQwzAI
>>"!ICON_B64!" echo hfzdg9KoikpyJ08wvHUDTtevWA0LiB45SmjbTlQkAoj/r+f6ORE0lo3T0Y6KJzBq6/DSaaxQiK7u
>>"!ICON_B64!" echo i3z/4zm6ui9i2jYyJkuVLMe5eonUpmfIffUlOhkjuHUHqrwSLzWEMWceKp7A6WgHy0YlqueKMgzi
>>"!ICON_B64!" echo X7SChtSW55G+WxQsi2K+gGWZBAIBH2CCWO1Tl8sS2PgS5AuMfvYJ5uwGYsdPggepDWsR10Ula+bL
>>"!ICON_B64!" echo uOEkTp3G6bnEyIE9eNeuoCwL8TxEbjNQY4Yj2SwA5vwFhPcfxKyvY2jd0zjt51HRGCo5o04mLHPB
>>"!ICON_B64!" echo wimWmfI3bCnLRKG0RlwHHY/h9PQw8u7bFLsuoAKBMUrHXxVaI5k0KhzBWrr8v5m+beN0tlNs+wkZ
>>"!ICON_B64!" echo yaAik6av/u9ny19v1h6Od7oLvQAAAABJRU5ErkJggolQTkcNChoKAAAADUlIRFIAAAAgAAAAIAgG
>>"!ICON_B64!" echo AAAAc3p69AAABlhJREFUeJy9l11sHGcVhp/zzczO/q93nQSSICQk1OSiUpCtJo2bEjtNHITaBIki
>>"!ICON_B64!" echo rkBtI1pKCCLCxSAaCmoDSmtSFGGKkUjSy0gVKGlL0xJTAsjBrswFqoRFhUQLBcWu1971/szuzHwf
>>"!ICON_B64!" echo F7P+SW0rdks50kijmU/nfee8Z86P5D+8zbCS2TYEQXSJrHjkpmZM5Gfe10owy55YFoQhZmoSyeaQ
>>"!ICON_B64!" echo XBvoEFgvCQPKwpRLmJkiki8s+F6dgGVjSjNIro3EkYexd+7G6dyF8eogap34Gokn8P88RvDan6j/
>>"!ICON_B64!" echo 8jkoFaMPWkJCFiSwbEyljNO5i/gDXyZ+z13oaR89eR0cOwrnekwEggApbEKyMfSrl5kbOos/PobK
>>"!ICON_B64!" echo pBZIRBGwLExpFmfn7WQGz6EKabxLV/DODRG8MYHEXNB67cAiIGCqVWJdd7Lt5B1ku/9DuetzTNwX
>>"!ICON_B64!" echo 0hgbReVyEIbYiECokVyOxANHUYU0jUsvUOk/hvF9rHQG1UogrTWh1qtng1IQBphGA5VJ40/X2fqR
>>"!ICON_B64!" echo tygUtuBNBRQKf2Hr0V38/a9vQlgGsVBYFmZmGvfwvbiHemi+ciUCtx2stjyVWo2p6SJT00UqXgPl
>>"!ICON_B64!" echo OFEyzWf3/OW6mGYDybYR29MNYYjYNpLOIukMuDGM7xPr6cX97OcxM0WwLGzCEMnmcHZ2oafL1M8N
>>"!ICON_B64!" echo YfwAO52hMlviwP697Nm9E4A/joxxZfgqqXQKvVQSy8JUq6hUmtRjP8Tp3MnswTuwEx5vn3+d1LY8
>>"!ICON_B64!" echo 7d1bmBypMrVtM/HbOwgu5FsS+D6Sa8Pp3IWenCP82wSSTKKMwavW6N23l2NHjwCQHPwFL1x6iUw2
>>"!ICON_B64!" echo ExEQAaUw0+8QO/BpUt8/hbhxJJkg9fhTVPq/hm6GTPRdxcnaBL5D7kKAs6MDyeYwpVkUIqA1pl4D
>>"!ICON_B64!" echo W4G7JOGUUKlWCYKAIAioVKugZFFvYzBzZWKfuof0k2cQ10UshS4Wab78YpRbMQuxBH+6jihBHIWp
>>"!ICON_B64!" echo 1yMMkSV1oOXw3dmulMK27YV7ALFtqNVAh2R+9FNiPb2YagVpy9Mcvkz1sX50uYwkEhBGQOK06ohp
>>"!ICON_B64!" echo YbVseSW8mVk2plwCA+nTzxDb14uZnUFSaZqvvEjl28fBGCSdvrHqrVJG1kzAANq2wZvB/uRnSBz5
>>"!ICON_B64!" echo Ck7nbejZGSQWY67vKM3fvowkU63fMbypz3URECBZKqK69pMbOo/vh6A1kkhQOf4wzeHLSPuGSMK1
>>"!ICON_B64!" echo Fq21EJiPnB9qRjv2svHgFwm9BrgJ/NeuUR86gz92DdmwcdWO974IzNuX/lHmpY9/gk3aJ0BoXn6e
>>"!ICON_B64!" echo 6ve+hfE8pC3/nsABbtri5iOwL+OQ8RsEQYCI0Pz1RfTkdaRV09+rrbnH7kjYpDCYeAJ8n/TJ06T6
>>"!ICON_B64!" echo vxt9eaMRlecPgsD8gR0Zly9cucD1S78itjkPIiS//g3STwwgqRSmVot6wjoHlzVHwAB2s0Ht0T6q
>>"!ICON_B64!" echo p58GY9DFErGuveQuDhO7sxv977cjfLX24WXNJwUwTgwwVAd+QOVEH6bRiN65LunHB0j1fSc66Xlr
>>"!ICON_B64!" echo lmR9c5YxYMDaspXmH35H6fBdNEeuogo5AJLH+0g/8dS6JFkkoPWq4TPGYBZGMoMJAiSZxFSrVB59
>>"!ICON_B64!" echo hNrTA5GLGyTpiSTR4eJUrVSEsaRQKYwBpZBEEgIdZfQSEkopRAQRWWhGQPTrxeNEkpxcQZInSX3z
>>"!ICON_B64!" echo BFJoB9+PJGk0INARVqv5KRwHUy7hj4+iNmWwbtkehc+yEBE8z2OuUmGuUsHzPGTpjqB1NH2vJIll
>>"!ICON_B64!" echo k3jwGNZHP9Zq93WsW7ajNmXwx0ejhuY4rZGsXMIfG0G1Z0nc/xDi2Pj1OrmNGxgcOs+tHd3c2tHN
>>"!ICON_B64!" echo 4M+fJVfIE4ThDcMnYbige+XEI9QGTiGpFLVnfoz/+2GIxxFLkbj/IVR7Fn9sJCJgWUh+83aDASxF
>>"!ICON_B64!" echo 5tRPiB3oWRxKgwDjxtGt7Ui9W4ZlGaXAaPQ7U8R6DhL+6030P99C4nHSp87gHrqb5m9eZa7/q605
>>"!ICON_B64!" echo AWyMaY3lJepnB7E7b8M9dDfE43jnhgjfmMB2XdAGg1m1ry+YgJXLE0y8Dsbg7N5D/L4HcXv3o4sV
>>"!ICON_B64!" echo 6mcHMaXSQgn/wBYTsSxMs4na+CFUu4P3/DDe2Z/hj48i6SyEUfOSG5bTJatZ/PC9/5vVbHyUYOwa
>>"!ICON_B64!" echo 3sXnMKVZJJdfAF9OABaX05litJxmc+9/OS2XVl1OlxOYt//Tev5fftj5cQ2o8b0AAAAASUVORK5C
>>"!ICON_B64!" echo YIKJUE5HDQoaCgAAAA1JSERSAAAAMAAAADAIBgAAAFcC+YcAAAobSURBVHic1ZprjBXlGcd/7zsz
>>"!ICON_B64!" echo 55w5tz17hVVKFQUVRFpst2qiX5rG1MQatYmlUpASI8EaSyq4XoCNYKFiolbbAEURBYmJ2Ehijf1Q
>>"!ICON_B64!" echo E2qlxSveQFBBvHDZZffsnvuZmffth/fsjWVhYQ+pfZLJJruzM///+9yfZ0Tt2As0/8dij+guKUEI
>>"!ICON_B64!" echo 8H3QZ5ivEGDb5j1KnfT2kxOwLHQmAypAJFMIxzlzJIRAex66Ow3SQiQSEAQn/JfhCQgBlo3uTuO0
>>"!ICON_B64!" echo XIYuFokuuAf7wsnoQh6ErC54rRBuFH/3x+QfWYGIRPB2/BtRU2NIDHNo4rg+ICV4Hqr9MO4ttxFb
>>"!ICON_B64!" echo tsqYT7mEVgoQQLW1UHmmlMhIBG1ZZO9fRPHptcjGBnBCxzWpoQSkRHd2YrdcTvTORdgTLwAhEK6L
>>"!ICON_B64!" echo zmTQgW+0cybwK40IOxCKQrmIlAH+nt1kHn2M4M1/Imrrh5AYakKeh9NyOfFHVyPrGtDZDCiFt/Nd
>>"!ICON_B64!" echo Co+vwv/4I0TUBVVdBloKKBURZ59H8g8rOW/C64T9w8irknx9/nz23R6B3dvBCjHw9AYTsG3UoW+I
>>"!ICON_B64!" echo rn7GgD/ajmhoJNfWSmH9GmRjk9FQPl8NyP1+pDUI0IHG6vic76YfIeqeS5BRBEc7+M7ZO1B3XcXn
>>"!ICON_B64!" echo 172CM24s2u937H5PtCx0dxp3zm1YEyehsz2IhiayS+6msOlp5PhzwHbMS2UVLifUH2FsG2FZqKIi
>>"!ICON_B64!" echo NnUMtddcjNcdgGUhnAgq3UPDFJ/w7Pnoni6wrGMISInOZHBariD2wCoDUilyba0UN29AjmmGchm0
>>"!ICON_B64!" echo RgiBbduDLiHEqR2+ZaGzWWRDIzgOlIrGr9BGG/7gqKO1QgtBfNkynB9eYcK6lMdoQAXoYgF8HxFx
>>"!ICON_B64!" echo CfbuobB+NXJsM3hlAIQQeJ5He3vHoMvzvJGREMLE+nwe4bok/vQUsdalfWCla5Pf3UX6PwcJNUaM
>>"!ICON_B64!" echo qUuwkgnShQuRGnSxCOpYEwoCZDJFdME96HIJlc2Q/+MqZOMYc/KAlJJcLs+0qVPYvu1lXn9tK6+/
>>"!ICON_B64!" echo tpXt215m2tQp5HJ5pDxBbhDCRBDPQ6ZqSa7dhHX+JEI//ikimUJ19yDDDuX2PO/PeoWed46glcnG
>>"!ICON_B64!" echo e5a9Q3t6KqJcwF1wLzKZ6jM/u6IjcBzsCycbdr6Pv+ujPjX1nr7v+6RSNVzWcukgbKlUDb7vD68F
>>"!ICON_B64!" echo IQCB6ukmsfxhwtdeD36A7uyEUIjEE+vI3vs7gq+/xEpEUEWPnbNeQYZtUAFByaJ2Vhnthw1GxwHf
>>"!ICON_B64!" echo AyEGmJDWlehi1Cyi0SHZr5dEEASUSmVKpTJBEJwcfBCgu7uIL1tF+KaZ6GIJ7XnGSW0ba8L5WM1n
>>"!ICON_B64!" echo ge+jFYiQBQqCvEeQ95ERuy9i6Xx+EK7BYXSgCQxTSAkhsCwLXXmIZVnDg5eyzwTjbSuJzPgV6ki7
>>"!ICON_B64!" echo iSKVS/WkyfzmVvz33kI0NELgVw6RynPF4IM8xkxHVo2eqlRMUh05TOTmOcQWLQZAdRythECNTCbJ
>>"!ICON_B64!" echo Ll5EcesLYDsGvF/J8qcgVa7IqMR4B33kMO6M2cTbVvY5rykQLURNrQG/eQMi4iJs2zjlqYZjqqoB
>>"!ICON_B64!" echo barXXA7d3YU789fEH1iJSnfRZwahEADZ3vzSNMac+iikOgS0BstBZ7qxzh6Pc+NNxO59AJXuBowT
>>"!ICON_B64!" echo y6ZGips2kHtoGYBJYqMED1UgoAEdCqGzndg/upLY2meQqTpUZ6e5QSlEUyOFzc+Sa2tFxBPGD0bQ
>>"!ICON_B64!" echo bY1ERkVAY6pgu+so1uRpJNdtQlk2qqMdhEREwoiaBMVNz5JdvBCZqqOvXKiSnDYBpcGRkp5Ac/SS
>>"!ICON_B64!" echo H1D/89kQCps4LSQikSQ4sA9vyzZyv1+KTNVWHTyMgoCUgh4EM/Zl+PT2Vmq9In4+D2hkXR3eBzvp
>>"!ICON_B64!" echo mTsDlU4j6+oM8DPQS592GA3QzNyf4bWMR30hi+ebMCjCEbw3t5OZNwtdLpseQqkzNgg4bQIWguVn
>>"!ICON_B64!" echo Rbki7pBDYAlhynLfI/fwCtSRQ8iaGiiXqol3iIwqkTU7koQU+EIgKnMcYdkkV6/Hnt5CsH8fSGtI
>>"!ICON_B64!" echo +q+mnPaTldbYCM5ypCnMwmFQCl0uI6JxEo+uIfnkc4hQCIpF8/MMyGkTEECdUPx5fIypD95N59tv
>>"!ICON_B64!" echo Ydc3mMozn0fEE4SvvobEE+sQrkvwzVdgVb/0GhUBr+KYdj5L9/yZ+B+8hxzXZGoaz0MdPowzbTqJ
>>"!ICON_B64!" echo x9fhzp0PhULlrdUzqVE9qa/0irjofIHMnbfSM/cWdLkErotGoDo7sadMI/7gcmKtS9CZHnQuO6gx
>>"!ICON_B64!" echo H41UR6e+jwjH0Nks5Vf/hmpvJ7nmGazmJtThDnQhj/6ih8jM2WilKb7wHMGnexGx6ElnnyeT6uhS
>>"!ICON_B64!" echo CNOKWhZyzFi8nW+TuWMuhSfXghsxp207qPajRG78BamX/k7kZzegO9r7Gv3Tlep6ldbglZB19Xgf
>>"!ICON_B64!" echo 7qS87R/4n31KfOkK0y5mM+h8FvJZ4g+uAgHFrVtMQxMKfUsamkoPLCJR5PhzKW58ityy+wj27jIm
>>"!ICON_B64!" echo AyAkqruHaGsbta/+C/u8Seh0lwnFpyiDCQwscUcbKVQA5RKysYnils2kr/sJxa0vIuobzN+DYMCI
>>"!ICON_B64!" echo ZQP21O+hvjxw/JLjBL36wLmJmURUKkZTVQ5Vp9Yaz/MGXfpEdY5SiFjcjCnvu4vS8xtNme04JvEV
>>"!ICON_B64!" echo 8gg3SuKxtSTWPousbxzs2JVBmCGmDcYBuGTvTXge/u6PTeq3beyLpgxhq7UmFArhOA6u6+K6Lo7j
>>"!ICON_B64!" echo EAqFTkwiCEArZE0NuRVtdF19Jf5nexCpWggUOl9AxOK4N11L5JezzYSw99SVMlhsG6RlMPb21/Tu
>>"!ICON_B64!" echo B6RE93RjT5tOzca/msHW+++SvvFqM9Qtl5FCUCyVmHDuOdw84wZUhZyUkk2bX+TzffuJRCJ9vz+u
>>"!ICON_B64!" echo VF6qi0VExCX5l404LdNRBzvAjVJ6YTO5FUtN11bpodWB/aS2vIp9yffBtumeeT3+zncQyRpDrl8X
>>"!ICON_B64!" echo FiLigm2jsxmsiZNw58yjsGk9ckwzyisTDofZt/8LWhe19WcxDfHaGsLh8InBGxUaHtEoOpcjc+et
>>"!ICON_B64!" echo 2JOnErt/ObKxgdKLz5sBlhAm7B46iDtnnpmWFwuIVK3BKPuToN2rJpFI4O14g9yShUQXtIKUxNpW
>>"!ICON_B64!" echo opUyE4SxzehymVAoRNO45v4dgwDf809sQsdKECBiJvGVXn4JtMKaMBH/k11mJyYl6tBBIjNmGwyZ
>>"!ICON_B64!" echo bjMtX7IQb8cb5vQrfjJ4xWTbqK8OGJVd2jLsgqNq66XeHqKQN6SiMVABqv0I7pzbDPiOdkR9I/7b
>>"!ICON_B64!" echo O4xJjxs/aJoxdEcWBNgXXTxkxeTv/eTMrZhkhUg2iz35Ytw7FprdnJSIeALV2UH2t/Pwd304pIb6
>>"!ICON_B64!" echo di35tEZYNiKRQBcKoDX+3k/IP/YQ/o7tiLq6IZHxW7dmFVJCKAy2TW7xQgpPrzF7ikreGPJfw34r
>>"!ICON_B64!" echo 8a1ZdKfMxPqUFt0D5X/xqUFPugqfGvRKECDi8b6PPbTvVQntMCKEOXWtR9QrjKyc7rW9UdbuI5ZT
>>"!ICON_B64!" echo aHL+CzYl58VtwisZAAAAAElFTkSuQmCCiVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHe
>>"!ICON_B64!" echo AAACR0lEQVR4nO2bPVLDMBCFXxg6KlNBnYIUXMD0OQSXyGG4BIdIjy+Qwk1qOlylhkpDcPSziqXd
>>"!ICON_B64!" echo 55A3k8aJ7X2fdteyYi+ah6dv/GPdWAdgrSsA6wCsdTtl52bblYpjkoZ1e/a+i9wmyGI6pFwYWSXA
>>"!ICON_B64!" echo bh7Ij1GUAXMw7pMkG5I9IGZ+Su2VlItxdff2Z3u/7ZIxRjMgZJ7F+LHaj1fv9v6wicabfRmck3ng
>>"!ICON_B64!" echo NCvGCgLwjT6jeYliZewFcEnmnUIQRCXAYL7WlWgWU2FnPgShe3kP7tsfNtFjnwAYn0Qy+l+fvfdT
>>"!ICON_B64!" echo QtKR90EYb/Mda9K9QE2FjDeRa3ssE0KiLIHUqJfsB3QAJOZKNmUqANrmAZIeYGHcyTwDLM0DxgCs
>>"!ICON_B64!" echo zQOGABjMA0Y9IGVec+qtCoBl1I+lVgKM5gElAKzmAQUAzOaBij1guRuoml1IVTJguRuSv2EwD1QA
>>"!ICON_B64!" echo MCfzQAUA++cm+j2TeaBSCcQgsP3LZNIDmCCoZ4BTs+0oQFSbB+yfG1G9W0OoPhFih6AyFZZCsACh
>>"!ICON_B64!" echo djM0rFvKbFBfEGGDYLIixFQSZktiLCVhvipsDcEcAGBbEhQAALuSoAHgpA2BDgCgWxKUAAC9kjgB
>>"!ICON_B64!" echo MD6p9c1KCkLse8nTLkUWRe8fVyUOE9Swbqs9uSYqAessAOQl4SSN2QvAdyIGCMBvbDmpH/t99rPC
>>"!ICON_B64!" echo bIuaY+XGnH0VYMkEn86JLfm+wJwel/dp0uPykhMwSzJAWe8MzQVETmZm9QCWlI8pN8bst8aOxZIR
>>"!ICON_B64!" echo qq/NXZpob4a0dAVgHYC1fgAmLwmWdPEI/AAAAABJRU5ErkJggg==
certutil -f -decode "!ICON_B64!" "%CONTROL_ICON%" >nul 2>&1
set "ICON_RC=!errorlevel!"
del /Q "!ICON_B64!" >nul 2>&1
if not "!ICON_RC!"=="0" exit /b 1
exit /b 0

:EnsureNotifyScript
if exist "%NOTIFY_PS1%" exit /b 0
set "NOTIFY_B64=%TEMP%\komodo_notify_%RANDOM%.b64"
> "!NOTIFY_B64!" echo JEVycm9yQWN0aW9uUHJlZmVyZW5jZSA9ICJTaWxlbnRseUNvbnRpbnVlIgpBZGQtVHlwZSAtQXNz
>>"!NOTIFY_B64!" echo ZW1ibHlOYW1lIFN5c3RlbS5XaW5kb3dzLkZvcm1zCkFkZC1UeXBlIC1Bc3NlbWJseU5hbWUgU3lz
>>"!NOTIFY_B64!" echo dGVtLkRyYXdpbmcKJG1zZyA9ICRlbnY6S09NT0RPX05PVElGWV9NU0cKaWYgKFtzdHJpbmddOjpJ
>>"!NOTIFY_B64!" echo c051bGxPcldoaXRlU3BhY2UoJG1zZykpIHsgJG1zZyA9ICJPIENvbnRyb2wgQ2VudGVyIGVuY29u
>>"!NOTIFY_B64!" echo dHJvdSB1bSBlcnJvLiIgfQokbiA9IE5ldy1PYmplY3QgU3lzdGVtLldpbmRvd3MuRm9ybXMuTm90
>>"!NOTIFY_B64!" echo aWZ5SWNvbgokbi5JY29uID0gW1N5c3RlbS5EcmF3aW5nLlN5c3RlbUljb25zXTo6RXJyb3IKJG4u
>>"!NOTIFY_B64!" echo QmFsbG9vblRpcFRpdGxlID0gIktvbW9kbyBDb250cm9sIENlbnRlciAtIEVSUk8iCiRuLkJhbGxv
>>"!NOTIFY_B64!" echo b25UaXBUZXh0ID0gJG1zZwokbi5CYWxsb29uVGlwSWNvbiA9IFtTeXN0ZW0uV2luZG93cy5Gb3Jt
>>"!NOTIFY_B64!" echo cy5Ub29sVGlwSWNvbl06OkVycm9yCiRuLlZpc2libGUgPSAkdHJ1ZQokbi5TaG93QmFsbG9vblRp
>>"!NOTIFY_B64!" echo cCg2NTAwKQpTdGFydC1TbGVlcCAtU2Vjb25kcyA3CiRuLkRpc3Bvc2UoKQo=
certutil -f -decode "!NOTIFY_B64!" "%NOTIFY_PS1%" >nul 2>&1
set "NOTIFY_RC=!errorlevel!"
del /Q "!NOTIFY_B64!" >nul 2>&1
if not "!NOTIFY_RC!"=="0" exit /b 1
exit /b 0