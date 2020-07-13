// instead of classes we have lightweight structs, enums and implementations

enum ErrorCode {
    FileNotFound
    UnexpectChar {expected: Vec<String>, found: char},

}