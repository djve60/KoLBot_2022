@echo off
pushd %~dp0
cd ..
cd ..
C:\<path to binary> install
if errorlevel 5 echo You need to run this file as an administrator. Right click and click Run As Administrator.
popd
pause
