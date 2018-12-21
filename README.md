# sh-os
## Object Dump
```
objdump -t -D  ./kernel/target/i686-unknown-linux-gnu/debug/deps/libshos-3935fdfd4fa79821.a
```

## nm
```$xslt
nm values ./kernel/target/i686-unknown-linux-gnu/debug/deps/libshos-3935fdfd4fa79821.a
```

## readelf
### ELF Headerの確認
```
readelf -h ./kernel/target/i686-unknown-linux-gnu/debug/deps/libshos-3935fdfd4fa79821.a
```
### ELB Section Headerの確認
```
readelf -S ./kernel/target/i686-unknown-linux-gnu/debug/deps/libshos-3935fdfd4fa79821.a
```

### odコマンド
```$xslt
od -t x1z -A x ./kernel/target/i686-unknown-linux-gnu/debug/deps/libshos-3935fdfd4fa79821.a
```

