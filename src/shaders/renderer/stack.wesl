struct Stack {
    data: array<u32, 64>, // Fixed size required for most address spaces
    top: i32,
};

fn new_stack() -> Stack {
    return Stack(
        array<u32, 64>(),
        -1i,
    );
}

fn push(s: ptr<function, Stack>, value: u32) {
    if (*s).top < 63 { // Boundary check
        (*s).top += 1;
        (*s).data[(*s).top] = value;
    }
}

fn pop(s: ptr<function, Stack>) -> u32 {
    let val = (*s).data[(*s).top];
    (*s).top -= 1;
    return val;
}

fn is_empty(s: ptr<function, Stack>) -> bool {
    return (*s).top < 0i;
}
