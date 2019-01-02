
cargo build --release
rd /s /q "D:\git\rust-algorithm-problems\codejam\log"
cat .\src\y2017qual\D-small-practice.in | .\target\release\codejam.exe > .\src\y2017qual\D-small-practice.out 
cat .\src\y2017qual\D-large-practice.in | .\target\release\codejam.exe   > .\src\y2017qual\D-large-practice.out 