use crate::util::input::{Input, InputReader};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

pub fn run_cases<F>(file_base_names: &[&str], package_name: &str, solver: F)
where
    F: Fn(&mut InputReader, &mut File),
{
    let now = Instant::now();

    let sol_path = Path::new(r"D:\git\rust-algorithm-problems\codejam\src");
    let round_path = sol_path.join(package_name);

    for file_base_name in file_base_names {
        let mut reader = InputReader {
            s: String::new(),
            i: Input::file(
                round_path
                    .join(format!("{}.in", file_base_name))
                    .to_str()
                    .unwrap(),
            )
            .unwrap(),
        };

        let mut buffer = File::create(
            round_path
                .join(format!("{}.out", file_base_name))
                .to_str()
                .unwrap(),
        )
        .unwrap();

        solver(&mut reader, &mut buffer);

        let duration = now.elapsed();
        let secs =
            f64::from(duration.as_secs() as u32) + f64::from(duration.subsec_nanos()) / 1e9f64;
        let _ = writeln!(
            ::std::io::stderr(),
            "Elapsed time {:.2} second(s) for {} in {}",
            secs,
            file_base_name,
            package_name
        );
    }
}
