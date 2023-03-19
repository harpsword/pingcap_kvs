

一个log structure storage的设计

从外到内

1. 对外的 get, set, rm 接口
2. storage存储




1. 对外的 get, set, rm 接口
2. encode, decode， 可以用serde json
3. 文件操作层

TODO
1. 性能优化尝试: BtreeMap换成HashMap