/* 入力オブジェクトファイル */

/* 出力オブジェクトファイル */

ENTRY(init_os)

/* 出力フォーマット */
OUTPUT_FORMAT("elf32-i386", "elf32-i386", "elf32-i386")
OUTPUT_ARCH(i386)

/* セクション */
SECTIONS
{
  . = SIZEOF_HEADERS;
  .text : {
    *(.text.init_os)
    *(.text)
  }

  "file offset of data" = .;

  . = 0;
  .stack : AT("file offset of data") {
    . = 0;
  }

  .data : {
    *(.rodata*)
    *(.data)
  }

  .bss  : {
    *(.bss)
  }

    . = ALIGN(4096); /* align to page size */
    __bss_end = .;

  .malloc : {
    . = __bss_end + 1024*1024;
  }
}
