> tool是一个集合了各类常用工具的命令行工具

```bash
➜  ~ toolx -h
toolx 0.0.1
Levi Xia <xiawenyang@bonbonbwork.com>
A collection of common tools for the command line tools

USAGE:
    toolx [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    e2u     url decode
    help    Print this message or the help of the given subcommand(s)
    j2f     json format
    s2s     Replace text content with specified characters
    t2u     transfer timestamp to unix time
    u2e     url encode
    u2t     transfer unix time to timestamp
```

# 字符串替换: 
s2s
    -F 指定分隔符
    -r 指定替换符
    -s 前后拼入字符
    -t 要处理的内容
    -e 使用vim编辑器输入待处理内容

eg:
直接使用文本
```bash
toolx s2s -F "\n" -r "," -s '"' -t "123
4555"

"123","4555"
```
使用vim输入
```bash
toolx s2s -F "\n" -r "," -s '"' -e
```


# json格式化输出
j2f 
    -t 要处理的内容
    -e 使用vim编辑器输入待处理内容
eg: 
```bash
➜  ~ toolx j2f -t '{"name":"levi", "age": 20}'
{
  "age": 20,
  "name": "levi"
}
```

# unix转时间戳
u2t
```bash
➜  ~ toolx t2u -t "2022-01-01 12:00:00"
1641009600
```

# 时间戳转unix
t2u

```bash
➜  ~ toolx u2t -t 1641009600
2022-01-01 12:00:00
```

# url编码
u2e
```bash
➜  ~ toolx u2e -t "http://www.baidu.com"
http%3A%2F%2Fwww%2Ebaidu%2Ecom
```

# url解码
e2u
```bash
➜  ~ toolx e2u -t "http%3A%2F%2Fwww%2Ebaidu%2Ecom"
http://www.baidu.com
```