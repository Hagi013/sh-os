# sh-os
## Object Dump
```
objdump -t -D  ./kernel/target/i686-unknown-linux-gnu/debug/deps/libshos-3935fdfd4fa79821.a

# intelで対応するソースコード
gobjdump -d -S -M intel ./kernel/target/i686-unknown-linux-gnu/debug/deps/libshos-fa391831c105a91e.a > ./sh-os.obj
```

### 実行形式の確認
```
gobjdump -h -p ./kernel/target/i686-unknown-linux-gnu/debug/deps/libshos-fa391831c105a91e.a > ./sh-os.obj
```

## nm
```
nm ./kernel/target/i686-unknown-linux-gnu/debug/deps/libshos-3935fdfd4fa79821.a
```

## readelf
### ELF Headerの確認
```
readelf -h ./kernel/target/i686-unknown-linux-gnu/debug/deps/libshos-3935fdfd4fa79821.a
```
### ELF Section Headerの確認
```
readelf -S ./kernel/target/i686-unknown-linux-gnu/debug/deps/libshos-3935fdfd4fa79821.a
```

### ELF Program Headerの確認
```
readelf -l ./kernel/target/i686-unknown-linux-gnu/debug/deps/libshos-3935fdfd4fa79821.a
```

### ELFのSymbolテーブルの確認
```
readelf -s ./kernel/target/i686-unknown-linux-gnu/debug/deps/libshos-3935fdfd4fa79821.a
```

### ELFのリロケーションテーブルの確認
```
readelf -r ./kernel/target/i686-unknown-linux-gnu/debug/deps/libshos-3935fdfd4fa79821.a
```

### 4番目のsection headerを確認する
```
readelf -x 4 ./kernel/target/i686-unknown-linux-gnu/debug/libshos.a
```

### odコマンド
```
od -t x1z -A x ./kernel/target/i686-unknown-linux-gnu/debug/deps/libshos-3935fdfd4fa79821.a
```

## qemu
### block deviceの確認
```
info block
```

### registerの内容の確認
```
info registers
```

### memory mappingの確認
```
info mem
```

### メモリの中身の確認
```
# 100byte分
xp /100xb 0x7c00

# アセンブリで表示
xp /100i 0x7c00

```

### 現在実行中のコードを確認
```
xp /16i $eip

# 実行中のコードの物理アドレス
print /x $cs * 16 + $eip

# 10進数
print /d $cs * 16 + $eip
```





