@echo off
REM Сборка Stella's Crack Food mod

set HORIZON_PATH=F:\Horizon Launcher\horizon
set MIXINS_PATH=%HORIZON_PATH%\..

if not exist "%MIXINS_PATH\.git" (
    echo [ERROR] horizon-mixins not found! Run setup.bat first.
    pause
    exit /b 1
)

echo ====================================
echo  Building Stella's Crack Food Mod
echo ====================================
echo.

cd F:\stellascrack-food

python3 tools\horizon.py build --horizon "%HORIZON_PATH%"

if %ERRORLEVEL% EQU 0 (
    echo.
    echo ====================================
    echo  BUILD SUCCESSFUL!
    echo ====================================
    echo.
    
    REM Найти .hnmod файл
    for %%f in (*.hnmod) do (
        if exist "%%~ff" (
            echo [OK] Created: %%f
            pause
            exit /b 0
        )
    ) else (
        echo [WARN] No .hnmod file found, checking build logs...
        type target\horizon-build.log >nul 2>&1 || (echo [INFO] Log not generated)
        pause
    )
) else (
    echo.
    echo ====================================
    echo  BUILD FAILED!
    echo ====================================
    echo Check the error messages above
    pause
)

endlocal
