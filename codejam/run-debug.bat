@echo off 
rd /s /q "D:\git\rust-algorithm-problems\codejam\log"
rem cargo build && cat .\src\y2017qual\A-small-practice.in | .\target\debug\codejam.exe > .\src\y2017qual\A-small-practice.out 
rem cargo build && cat .\src\y2017qual\A-large-practice.in | .\target\debug\codejam.exe > .\src\y2017qual\A-large-practice.out 
rem cargo build && cat .\src\y2017qual\B-small-practice.in | .\target\debug\codejam.exe > .\src\y2017qual\B-small-practice.out
rem cargo build && cat .\src\y2017qual\B-large-practice.in | .\target\debug\codejam.exe > .\src\y2017qual\B-large-practice.out 

rem cargo build && cat .\src\y2017qual\C-small-practice-1.in | .\target\debug\codejam.exe > .\src\y2017qual\C-small-practice-1.out 
rem cargo build && cat .\src\y2017qual\C-small-practice-2.in | .\target\debug\codejam.exe > .\src\y2017qual\C-small-practice-2.out 
rem cargo build && cat .\src\y2017qual\C-large-practice.in | .\target\debug\codejam.exe > .\src\y2017qual\C-large-practice.out 

rem cargo build && cat .\src\y2017qual\D-small-practice.in | .\target\debug\codejam.exe > .\src\y2017qual\D-small-practice.out 
rem cargo build && cat .\src\y2017qual\D-large-practice.in | .\target\debug\codejam.exe > .\src\y2017qual\D-large-practice.out 

rem cargo build && cat .\src\y2017round1A\B-small-practice.in | .\target\debug\codejam.exe > .\src\y2017round1A\B-small-practice.out 
rem cargo build && cat .\src\y2017round1A\B-large-practice.in | .\target\debug\codejam.exe > .\src\y2017round1A\B-large-practice.out 

rem cargo build && cat .\src\y2017round1A\C-small-practice.in | .\target\debug\codejam.exe > .\src\y2017round1A\C-small-practice.out 
rem cargo build && cat .\src\y2017round1A\C-large-practice.in | .\target\debug\codejam.exe > .\src\y2017round1A\C-large-practice.out 

rem cargo build && cat .\src\y2017round1B\A-test.in | .\target\debug\codejam.exe 
rem cargo build && cat .\src\y2017round1B\A-small-practice.in | .\target\debug\codejam.exe > .\src\y2017round1B\A-small-practice.out 
rem cargo build && cat .\src\y2017round1B\A-large-practice.in | .\target\debug\codejam.exe > .\src\y2017round1B\A-large-practice.out 

rem cargo build && cat .\src\y2017round1B\B-test.in | .\target\debug\codejam.exe
set RUST_BACKTRACE=1
rem cargo build && cat .\src\y2017round1B\B-small-practice.in | .\target\debug\codejam.exe > .\src\y2017round1B\B-small-practice.out
rem cargo build && cat .\src\y2017round1B\B-large-practice.in | .\target\debug\codejam.exe > .\src\y2017round1B\B-large-practice.out

rem cargo build && cat .\src\y2017round1B\C-test.in | .\target\debug\codejam.exe > .\src\y2017round1B\C-small-practice.out
cargo build && cat .\src\y2017round1B\C-small-practice.in | .\target\debug\codejam.exe > .\src\y2017round1B\C-small-practice.out
