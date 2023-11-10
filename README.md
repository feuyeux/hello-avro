# Hello Avro

<https://github.com/apache/avro>

Serialization

```xml
<dependency>
    <groupId>org.apache.avro</groupId>
    <artifactId>avro</artifactId>
    <version>${avro.version}</version>
</dependency>
```

- Static serialization frameworks(with IDL)
  - [Protocol Buffers](https://github.com/protocolbuffers/protobuf)
  - [Apache Thrift](https://github.com/apache/thrift)
  - ASN.1(Abstract Syntax Notation One)
- Dynamic serialization frameworks(without IDL)
  - [Fury](https://github.com/alipay/fury)
  - [Kryo](https://github.com/EsotericSoftware/kryo)
  - [Hessian](http://hessian.caucho.com/)

IPC

```xml
<dependency>
    <groupId>org.apache.avro</groupId>
    <artifactId>avro-ipc</artifactId>
    <version>${avro.version}</version>
</dependency>
```

IDL:Interface description language
IPC:Inter-Process Communication
