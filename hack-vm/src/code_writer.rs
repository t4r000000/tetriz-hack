use std::fs::File;
struct CodeWriter {
    f: File,
}

trait TCodeWriter {
    fn new(f: File) -> Self;
    fn set_file_name(file_name: &str);
    fn write_arithmetic(command: &str);
    fn write_push_pop(command: &str, segment: &str, index: u32);
    fn close();
}
