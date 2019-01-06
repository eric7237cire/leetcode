

rem rd /s /q "D:\git\rust-algorithm-problems\codejam\log"
rem cat .\src\y2017qual\D-small-practice.in | .\target\release\codejam.exe > .\src\y2017qual\D-small-practice.out
rem cat .\src\y2017qual\D-large-practice.in | .\target\release\codejam.exe   > .\src\y2017qual\D-large-practice.out
cargo build --release && cat .\src\y2017round1C\B-large-practice.in | .\target\release\codejam.exe > .\src\y2017round1C\B-large-practice.out