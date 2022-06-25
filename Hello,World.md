按照传统，我们用新语言编写的第一个程序都是 Hello，world。

我们首先创建一个文件夹用来存放这系列教程的相关代码，我把它存放在用户根目录下，文件名为`learn-rust-with-tests`

我们使用`cargo`来创建和管理我们的项目，进入到存放代码的文件夹下，在终端输入以下命令来创建一个新项目

```shell
cargo new hello		
```

它会帮我们自动生成rust的初始化代码，生成的代码结构如下入所示，如果你的上级目录没有git，它还会帮你自动初始化一个git，关于每个文件的含义我们后续再详细介绍，我们目前先实现命令行打印`Hello, world!`

![image-20220624190300972](https://repo-1256831547.cos.ap-shanghai.myqcloud.com/2022/image-20220624190300972.png)

自动生成的代码就是打印`Hello,world!`的程序

![image-20220624190546434](https://repo-1256831547.cos.ap-shanghai.myqcloud.com/2022/image-20220624190546434.png)

我们无需任何修改，直接在该目录下运行`cargo run `

![image-20220624190730770](https://repo-1256831547.cos.ap-shanghai.myqcloud.com/2022/image-20220624190730770.png)



## 它是如何运行的

用rust编写程序，你需要定义一个main函数，

函数（function）使用 `fn` 关键字来声明。函数的参数需要标注类型，就和变量一样，如果函数返回一个值，返回类型必须在箭头 `->` 之后指定。

函数最后的表达式将作为返回值。也可以在函数内使用 `return` 语句来提前返一个值

打印是由std::fmt里面所定义的一系列`宏`来处理的，`println！`是将文本输出到控制台，并在输出结果追加一个换行符，如果是`print!`则输出结果没有换行。



## 如何测试

大多数单元测试都会被放到一个叫 `tests` 的、带有 `#[cfg(test)]` 属性 的**模块**中，测试函数要加上 `#[test]` 属性。

![image-20220624193248768](https://repo-1256831547.cos.ap-shanghai.myqcloud.com/2022/image-20220624193248768.png)

我们终端运行`cargo test`进行测试

![image-20220624193437105](https://repo-1256831547.cos.ap-shanghai.myqcloud.com/2022/image-20220624193437105.png)

### 声明变量

在Rust中变量默认都是`不可变`的，如果要使其可变要在声明的时候加上关键字`mut`

### 常量

常量类似于变量，不过它永远都是不可变的，而且声明常量是使用`const`关键字，名称全部大写，用下划线进行分割增加可读性，同时它**必须要注明值的类型**

```rust
const ENGLISH_HELLO_PREFIX: String = "Hello, "
```

重构代码：

`hello.rs`

```rust
const ENGLISH_HELLO_PREFIX: &str = "Hello, ";

pub fn hello(name: &String) -> String {
    return format!("{}{}", ENGLISH_HELLO_PREFIX, name);
}
```

`main.rs`

```rust
pub mod hello;

fn main() {
    let name: String = "Chris".to_string();
    println!("{}", hello::hello(&name));
}
```

`lib.rs`

```rust
pub mod hello;

#[cfg(test)]
mod tests {
    use crate::hello;
    #[test]
    fn test_hello() {
        let name: String = "Chris".to_string();
        let got = hello::hello(&name);
        let want: String = String::from("Hello, Chris");
        assert_eq!(got, want);
    }
}
```

## 再次回到Hello,World

下一个需求是当我们的函数用空字符串调用时，它默认为打印 "Hello, World" 而不是 "Hello, "

首先编写一个新的失败测试

![image-20220625113957174](https://repo-1256831547.cos.ap-shanghai.myqcloud.com/2022/image-20220625113957174.png)

`lib.rs`

+ 因为要判断传进来的字符串是否是空字符串，是的话我们要改变其值，所以变量要改为可变的

~~~rust
pub mod hello;

#[cfg(test)]
mod tests {
    use crate::hello;
    #[test]
    fn test_hello() {
        let mut name: String = "Chris".to_string();
        let got = hello::hello(&mut name);
        let want: String = String::from("Hello, Chris");
        assert_eq!(got, want);
    }

    #[test]
    fn test_hello_use_empty_string() {
        let mut name: String = "".to_string();
        let got = hello::hello(&mut name);
        let want: String = String::from("Hello, World");
        assert_eq!(got, want);
    }
}

~~~

`hello.rs`

~~~rust
const ENGLISH_HELLO_PREFIX: &str = "Hello, ";

pub fn hello(name: &mut String) -> String {
    if *name == "".to_string() {
        *name = "World".to_string();
    }
    return format!("{}{}", ENGLISH_HELLO_PREFIX, name);
}

~~~



两个测试都通过了

~~~shell
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/hello-d9b0de51daf7d8a1)

running 2 tests
test tests::test_hello ... ok
test tests::test_hello_use_empty_string ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
~~~



## 规律

让我们再次回顾一下这个周期

- 编写一个测试
- 让编译通过
- 运行测试，查看失败原因并检查错误消息是很有意义的
- 编写足够的代码以使测试通过
- 重构

从表面上看可能很乏味，但坚持这种反馈循环非常重要。

它不仅确保你有 *相关的测试*，还可以确保你通过重构测试的安全性来 *设计优秀的软件*。

查看测试失败是一个重要的检查手段，因为它还可以让你看到错误信息。作为一名开发人员，如果测试失败时不能清楚地说明问题所在，那么使用这个代码库可能会非常困难。

通过确保你的测试的 *快速*，并设置你的工具，可以使运行测试足够简单，你在编写代码时就可以进入流畅的状态。

如果不写测试，你提交的时候通过运行软件来手动检查你的代码，这会打破你的流畅状态，而且你任何时候都无法将自己从这种状态中拯救出来，尤其是从长远来看。



## 更多的需求

为西班牙语的用户编写测试，将其添加到现有的测试用例中，我们再接收一个参数，指定语言，如果不能识别其语言，则默认为英语。

`lib.rs`

~~~rust
pub mod hello;

#[cfg(test)]
mod tests {
    const ENGLISH: &str = "English";
    const SPANISH: &str = "Spanish";
    const FRANCH: &str = "France";
    use crate::hello;
    #[test]
    fn test_hello() {
        let mut name: String = "Chris".to_string();
        let got = hello::hello(&mut name, ENGLISH);
        let want: String = String::from("Hello, Chris");
        assert_eq!(got, want);
    }

    #[test]
    fn test_hello_use_empty_string() {
        let mut name: String = "".to_string();
        let got = hello::hello(&mut name, ENGLISH);
        let want: String = String::from("Hello, World");
        assert_eq!(got, want);
    }

    #[test]
    fn test_hello_in_spanish() {
        let mut name: String = "Elodie".to_string();
        let got = hello::hello(&mut name, SPANISH);
        let want: String = String::from("Hola, Elodie");
        assert_eq!(got, want);
    }
    #[test]
    fn test_hello_in_franch() {
        let mut name: String = "Elodie".to_string();
        let got = hello::hello(&mut name, FRANCH);
        let want: String = String::from("Bonjour, Elodie");
        assert_eq!(got, want);
    }
}

~~~



`hello.rs`

~~~rust
const ENGLISH_HELLO_PREFIX: &str = "Hello, ";
const SPANISH_HELLO_PREFIX: &str = "Hola, ";
const FRANCH_HELLO_PREFIX: &str = "Bonjour, ";
pub fn hello(name: &mut String, language: &str) -> String {
    if *name == "".to_string() {
        *name = "World".to_string();
    }

    if language == "Spanish" {
        return format!("{}{}", SPANISH_HELLO_PREFIX, name);
    }

    if language == "France" {
        return format!("{}{}", FRANCH_HELLO_PREFIX, name);
    }
    return format!("{}{}", ENGLISH_HELLO_PREFIX, name);
}

~~~



## 最后一次重构

你可能会抱怨说也许我们的函数正在变得很臃肿。对此最简单的重构是将一些功能提取到另一个函数中。在Rust中使用`match`模式匹配类似于其他语言的`switch`

`hello.rs`

~~~rust
const ENGLISH_HELLO_PREFIX: &str = "Hello, ";
const SPANISH_HELLO_PREFIX: &str = "Hola, ";
const FRANCH_HELLO_PREFIX: &str = "Bonjour, ";

pub fn hello(name: &mut String, language: &str) -> String {
    if *name == "".to_string() {
        *name = "World".to_string();
    }
    return format!("{}{}", greeting_prefix(language), name);
}

fn greeting_prefix(language: &str) -> &str {
    match language {
        "Spanish" => SPANISH_HELLO_PREFIX,
        "Franch" => FRANCH_HELLO_PREFIX,
        _ => ENGLISH_HELLO_PREFIX,
    }
}

~~~

+ `match`分支必须覆盖所有可能的值，这里我们只处理西班牙语和法语，其他的语言我们用`_`表示，我们默认都是用英语打招呼
+ `match`匹配到后用`=>`来进行后续处理
+ 函数名都是`全部小写`，如果多个单词可以用下划线分割增加可读性



# 总结

谁会知道你可以从 `Hello, world` 中学到这么多东西呢？

现在你应该对以下内容有了一定的理解：

## Rust 的一些语法

- 编写测试
- 用参数和返回类型声明函数
- `if`，`const`，`match`
- 声明变量和常量

## TDD 过程以及步骤的重要性

- *编写一个失败的测试，并查看失败信息*，我们知道现在有一个为需求编写的 *相关* 的测试，并且看到它产生了 *易于理解的失败描述*
- 编写最少量的代码使其通过，以获得可以运行的程序
- *然后* 重构，基于我们测试的安全性，以确保我们拥有易于使用的精心编写的代码

在我们的例子中，我们通过小巧易懂的步骤从 `Hello()` 到 `Hello("name")`，到 `Hello("name", "french")`。

与「现实世界」的软件相比，这当然是微不足道的，但原则依然通用。TDD 是一门需要通过开发去实践的技能，通过将问题分解成更小的可测试的组件，你编写软件将会更加轻松。

