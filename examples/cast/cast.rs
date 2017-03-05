// Убрать все предупреждения
// которые вызываются переполнением при преобразование типов.
#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // Ошибка ! Нет неявного преобразование
    let integer: u8 = decimal;
    // ИСПРАВЬТЕ ^ Закомментируйте данную строку

    // Явное преобразование
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // Когда преобразовывается любое значение в беззнаковый тип T
    // std::T::MAX + 1 добавляется или вычитается до тех пор, пока значение
    // не будет помещаться в новый тип.

    // 1000 поместится в u16
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // Подробнее. Первые 8 младших битов (LSB) сохраняются,
    // а старшие биты (MSB) будут усечены.
    println!("1000 as a u8 is : {}", 1000 as u8);
    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    // Для положительных чисел результатом будет остаток от деления
    println!("1000 mod 256 is : {}", 1000 % 256);

    // Когда значение преобразовывается в знаковый тип,
    // побитовый результат будет таким же, как и
    // первое преобразование к соответствующему типу без знака. Если старший бит этого значения
    // равен 1, то это значение отрицательное.

    // За исключением случая, когда значение умещается в тип.
    println!(" 128 as a i16 is: {}", 128 as i16);
    // 128 as u8 -> 128, дополнительный код которого в 8 битах:
    println!(" 128 as a i8 is : {}", 128 as i8);

    // повторяем примеры
    // 1000 as u8 -> 232
    println!("1000 as a i8 is : {}", 1000 as i8);
    // и дополнительный код 232 - это -24
    println!(" 232 as a i8 is : {}", 232 as i8);
}
