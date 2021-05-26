## mut a: &T 和 a:&mut T 的区别

![image-20210522131914852](X:\notes\markdown\image\rust_base\image-20210522131914852.png)

##  宏里面的 token 类型

- item ：一个项（item），像一个函数，结构体，模块等。

- block：一个块 （block）（即一个语句块或一个表达式，由花括号所包围）

- stmt ： 一个语句（statement）

- pat：一个模式（pattern）

- expr ： 一个表达式（expression）
- ty ：一个类型（type）
- ident： 一个标识符（indentfier）
- path ： 一个路径（path）（例如，foo，::std::mem::replace，transmute::<_, int>，...）
- meta ： 一个元数据项；位于#[...]和#![...]属性
- tt：一个词法树
- vis：一个可能为空的Visibility限定词