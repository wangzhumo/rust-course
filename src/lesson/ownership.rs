pub fn main_ownership() {
    //! 好家伙，测试一下所有权
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
                                    // ... 所以到这里不再有效
    

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动函数里，
                                    // 但 i32 是 Copy 的，所以在后面可继续使用 x

    println!("makes_copy x {}", x);                                

} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作


/// 获取所有权，然后释放
/// 不会将所有权放出去
pub fn takes_ownership(some_string: String) { 
    // some_string 进入作用域
    println!("{}", some_string);
    // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放
} 

/**
 * ## 获取所有权
 * > 基本类型
 * 
 * 移除作用域，不做其他操作。
 */
pub fn makes_copy(some_integer: i32) { 
    // some_integer 进入作用域
    println!("{}", some_integer);
    // 这里，some_integer 移出作用域。不会有特殊操作
} 