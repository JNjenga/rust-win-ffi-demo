@ECHO off
SET LIBS=-l kernel32 -l user32
SET SRC_FILES=..\src\app.rs
pushd .
REM mkdir build
cd build

rustc -g %LIBS% %SRC_FILES%

if NOT ERRORLEVEL 1 call app.exe

popd
