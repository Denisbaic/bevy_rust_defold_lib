# bevy_rust_defold_lib

## Описание

Это мой пет-проект, в котором я пытаюсь соединить Defold (игровой движок на Lua и C) и Rust с помощью FFI-интерфейса.
Библиотека написана с поддержкой no_std и компилируется в статическую библиотеку. Build-скрипт дополнительно генерирует заголовочный файл (.h) с описанием обвязок.

Игровая часть включает модули для двунаправленного графа, игровой доски и описания сторон света.
Ключевые игровые модули покрыты юнит-тестами.
