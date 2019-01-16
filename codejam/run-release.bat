@echo off
echo "Reset" > "D:\git\rust-algorithm-problems\codejam\log\output.log"
rem rd /s /q "D:\git\rust-algorithm-problems\codejam\log"
rem cat .\src\y2017qual\D-small-practice.in | .\target\release\codejam.exe > .\src\y2017qual\D-small-practice.out
rem cat .\src\y2017qual\D-large-practice.in | .\target\release\codejam.exe   > .\src\y2017qual\D-large-practice.out
rem cargo build --release && cat .\src\y2017round1C\B-large-practice.in | .\target\release\codejam.exe > .\src\y2017round1C\B-large-practice.out

rem cargo build --release && cat .\src\y2017round2\C-small-practice.in | .\target\release\codejam.exe > .\src\y2017round2\C-small-practice.out
rem cargo build --release && cat .\src\y2017round2\C-large-practice.in | .\target\release\codejam.exe > .\src\y2017round2\C-large-practice.out

rem cargo build --release && cat .\src\y2017round2\D-small-practice.in | .\target\release\codejam.exe > .\src\y2017round2\D-small-practice.out
rem cargo build --release && cat .\src\y2017round2\D-large-practice.in | .\target\release\codejam.exe > .\src\y2017round2\D-large-practice.out

cargo build --release && cat .\src\y2017round3\C-small-practice.in | .\target\release\rust-algorithm-problems.exe > .\src\y2017round3\C-small-practice.out
cargo build --release && cat .\src\y2017round3\C-large-practice.in | .\target\release\rust-algorithm-problems.exe > .\src\y2017round3\C-large-practice.out