use std::fs::File;
use std::io::Write;
use std::io::{self, BufRead};
use std::path::Path;

const PCM_BYTES_PER_SAMPLE: usize = 2;

pub fn generate_pcm_file(in_file: &str, out_file: &[&str]) {
    let mut file_vec: Vec<File> = Vec::new();
    for (_, file) in out_file.iter().enumerate() {
        if Path::new(file).exists() {
            std::fs::remove_file(file).unwrap();
            file_vec.push(File::create(file).unwrap());
        } else {
            file_vec.push(File::create(file).unwrap());
        }
    }

    // let mut file_far = File::create(out_file[0]).unwrap();
    // let mut file_near = File::create(out_file[1]).unwrap();
    // let mut totoal_lines = 0;

    if let Ok(mut lines) = read_lines(in_file) {
        lines.next();
        // totoal_lines = lines.count();
    }

    let mut count = 0;
    let mut file_flag = 0;
    // let (main_tx, main_rx) = mpsc::channel();

    // thread::spawn(|move| {
    //     println!("{}", totoal_lines);
    //     // while count != totoal_lines {
    //     //    println!(".");
    //     //    thread::sleep(Duration::from_millis(10));
    //     // }
    // });

    if let Ok(mut lines) = read_lines(in_file) {
        if let Some(Ok(line)) = lines.next() {
            dbg!(check_file_format(&line));
        }

        for line in lines {
            if let Ok(line) = line {
                let data: String = line.split(',').filter(|w| w.contains("0x")).collect();
                let data = data.trim_start_matches("0x");
                let data = u8::from_str_radix(data, 16).unwrap();

                if (count != 0) && (count % PCM_BYTES_PER_SAMPLE == 0) {
                    file_flag = (file_flag + 1) % out_file.len();
                }

                file_vec[file_flag].write(&[data]).unwrap();
                // match file_flag {
                //     FAR_FILE_NUMBER => file_far.write(&[data]).unwrap(),
                //     NEAR_FILE_NUMBER => file_near.write(&[data]).unwrap(),
                //     _ => 0,
                // };
            }
            count += 1;
        }
    }
}

pub fn get_library_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

const KINGST_FILE_FORMAT: &'static str = "Time [s],Packet ID,MOSI,MISO";

fn check_file_format(str: &str) -> bool {
    str == KINGST_FILE_FORMAT
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
