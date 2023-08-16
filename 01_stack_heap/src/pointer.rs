static MAX: u32 = 0;

fn foo() {}

fn main() {
    let hello = "hello world".to_string();
    let data = Box::new(1);

    // string literals 指向 RODATA 地址
    println!("RODATA: {:p}", "hello world!");
    // static 变量在 DATA section
    println!("DATA (static var): {:p}", &MAX);
    // function 在 TEXT
    println!("TEXT (function): {:p}", foo as *const ());
    // String 结构体分配在栈上，所以其引用指向一个栈地址
    println!("STACK (&hello): {:p}", &hello);
    // 需要通过解引用获取其堆上数据，然后取其引用
    println!("HEAP (&*hello): {:p}", &*hello);
    // Box 实现了 Pointer trait 无需额外解引用
    println!("HEAP (box impl Pointer) {:p} {:p}", data, &*data);

    // 代码区（TEXT 或 Code Segment）：
    // 这是存储可执行程序代码的区域。它包含编译后的指令，如函数代码、循环和条件语句等。代码区通常是只读的，以防止程序在运行时意外地修改代码。
    //
    // 数据区（DATA 和 BSS Segment）：
    // 数据区分为初始化数据区（Data Segment）和未初始化数据区（BSS - Block Started by Symbol Segment）。初始化数据区用于存储静态和全局变量的初始值，而未初始化数据区用于存储静态和全局变量的初始值为零或空的变量。
    //
    // 堆区（Heap）：
    // 堆区是用于动态分配内存的区域。在堆上分配的内存需要手动进行管理，包括分配和释放。动态分配的数据（如通过 malloc 或 new 分配的数据）位于堆区。
    //
    // 栈区（Stack）：
    // 栈区用于存储函数调用期间的局部变量、函数参数以及函数调用的返回地址。栈是一种后进先出（LIFO）结构，意味着最后分配的局部变量首先被释放。栈的管理是由编译器自动完成的。
    //
    // 只读数据区（RODATA）：
    // 只读数据区用于存储只读的数据，如字符串字面量和常量。这些数据通常在程序中被声明，并且在运行时不能被修改。
    //
    // 内存映射区（Memory-Mapped Segment）：
    // 内存映射区将文件映射到内存中，使得文件的内容可以像访问内存一样被访问。这种技术通常用于处理大文件和一些特殊的 I/O 操作。
    //
    // 环境变量区（Environment Variable Segment）：
    // 环境变量区存储程序运行时使用的环境变量。
}
