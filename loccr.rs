
type line_info = {mut code: uint, mut blank: uint};

fn line_info() -> line_info {
    ret {mut code: 0u, mut blank: 0u};
}

fn add_lines(a: line_info, b:line_info)
{
    a.code += b.code;
    a.blank += b.blank;
}

// read_line does not appear in the reader interface for some reason
fn read_line(file: io::reader) -> str
{
    let mut buf: [u8] = [];
    while true {
        let ch = file.read_byte();
        if ch == -1 || ch == 10 { break; }
        buf += [ch as u8];
    }
    ret str::from_bytes(buf);
}

// count lines on one file
fn count_lines(file_path: str) -> line_info {
    
    let file = result::get(io::file_reader(file_path));
    let count = line_info();
    
    while true {
        let line = read_line(file);
        if file.eof() {
            break
        }
        if str::is_empty(str::trim(line)) {
            count.blank += 1u;
        } else {
            count.code += 1u;
        }
    }
    ret count;
}

type dir_info = (line_info, uint);

// recursively count in each file and sub-directory in this dir
fn count_dir(dir_path: str, extensions: [str]) -> dir_info {
    
    let total = line_info();
    let mut num_tasks = 0u;
    let port = comm::port::<dir_info>();
    let chan = comm::chan::<dir_info>(port);
    let mut num_files = 0u;
    
    for path in os::list_dir(dir_path) {
        if os::path_is_dir(path) {
            num_tasks += 1u;
            task::spawn {||
                comm::send(chan, count_dir(path, extensions));
            }
        } else {
            let (_, ext) = path::splitext(path);
            if vec::contains(extensions, ext) {
                num_files += 1u;
                add_lines(total, count_lines(path));
            }
        }
    }

    let mut i = 0u;
    while i < num_tasks {
        let (count, nfiles) = comm::recv(port);
        add_lines(total, count);
        num_files += nfiles;
        i += 1u;
    }

    ret (total, num_files);
}

fn main(args: [str]) {
    if vec::len(args) == 3u {

        let path = args[1];
        let ext  = args[2];

        let exts = vec::map(str::split_char(ext, ',')) {|e|
            "." + e
        };
    
        let (res, num_files) = count_dir(path, exts);
        let total = res.code + res.blank;
        io::println(#fmt("Line count: %u code %u blank (%u total) in %u files",
                         res.code, res.blank, total, num_files));

    } else {
        io::println("Usage: loccr [path] [extensions]");
        io::println("  Example: ./loccr src rs,cpp,h");
    }
}
