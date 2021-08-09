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

## linker
- linkerについてのman的なもの
```sh
info ld scripts
```

## qemu
[コマンド参照](https://qemu.weilnetz.de/doc/qemu-doc.html#pcsys_005fkeys)
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

### Show infos for each CPU.
```
info cpus
```

### Show the interrupts statistics (if available).
```
info irq
```

### Show PIC state.
```
info pic
```

### Show virtual to physical memory mappings.
```
info tlb
```

### Show memory tree
```
info mtree
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

## GDB
- qemuを立ち上げてGDBでデバッグする
[参考](http://yuyubu.hatenablog.com/entry/2018/07/17/QEMUにGDBを繋げてhariboteOSをデバッグする方法)
```
# Makefileのコメントアウトを外す
DEBUG := -S -gdb tcp::9000
```
- gdbを立ち上げる
```sh
> gdb
(gdb) target remote localhost:9000
# break pointを貼りたい場合
(gdb) b *0x7c00
(gdb) continueなど

# break pointの削除
## 設定した全てのブレークポイントを削除する。
delete 
## 番号breakp(info break で表示される番号)のブレークポイントを削除する。 番号はスペースで区切って、複数を指定することが可能。
delete breakp

```

- intel記法で表示させたい([ref](http://bttb.s1.valueserver.jp/wordpress/blog/2017/10/07/gdbで初期設定をatt記法からintel記法に変更する方法/))
```
set disassembly-flavor intel

show disassembly-flavor

# defaultを変更したい場合
echo set disassembly-flavor intel > ~/.gdbinit
```


