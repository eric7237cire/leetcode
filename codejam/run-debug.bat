@echo off 
rem cargo build && cat .\src\y2017qual\A-small-practice.in | .\target\debug\codejam.exe > .\src\y2017qual\A-small-practice.out 
rem cargo build && cat .\src\y2017qual\A-large-practice.in | .\target\debug\codejam.exe > .\src\y2017qual\A-large-practice.out 
set RUST_BACKTRACE=0
rem cargo build && cat .\src\y2017qual\B-small-practice.in | .\target\debug\codejam.exe > .\src\y2017qual\B-small-practice.out 
rem cargo build && cat .\src\y2017qual\B-large-practice.in | .\target\debug\codejam.exe > .\src\y2017qual\B-large-practice.out 

cargo build && cat .\src\y2017qual\C-small-practice-1.in | .\target\debug\codejam.exe > .\src\y2017qual\C-small-practice-1.out 
cargo build && cat .\src\y2017qual\C-small-practice-2.in | .\target\debug\codejam.exe > .\src\y2017qual\C-small-practice-2.out 
cargo build && cat .\src\y2017qual\C-large-practice.in | .\target\debug\codejam.exe > .\src\y2017qual\C-large-practice.out 