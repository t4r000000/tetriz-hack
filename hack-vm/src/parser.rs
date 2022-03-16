struct Parser {}

trait TParser {
    fn new() -> Self;
    fn has_more_commands() -> bool;
    fn advance() {}
    fn command_type() -> String;
    fn arg1() -> String;
    fn arg2() -> i32;
}
