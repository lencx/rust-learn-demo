# 数据类型(Data Types)

## 标量(scalar) & 复合(compound)

### 标量类型(Scalar Types)

> 整型(integers), 浮点型(floating-point number), 布尔类型(booleans), 字符类型(characters)

- Integer Types

|长度(Length)|有符号(Signed)|无符号(Unsigned)|
|---|---|---|
|8-bit|i8|u8|
|16-bit|i16|u16|
|32-bit|i32|u32|
|64-bit|i64|u64|
|arch|isize|usize|

>> i8: 有符号范围(Signed Range) -(2<sup>7</sup>) ~ (2<sup>7</sup>-1) => -128 ~ 127; 无符号范围(Unsigned Range) 2<sup>8</sup>-1 => 0 ~ 255

### 复合类型(Compound Types)

> 元组(tuple) & 数组(array)