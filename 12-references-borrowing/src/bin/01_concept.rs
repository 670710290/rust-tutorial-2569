/* 

// 01_concept.rs
// จุดประสงค์: เพื่อแสดงให้เห็นว่า References ช่วยให้เราใช้งานข้อมูลได้โดยไม่ต้องแย่งความเป็นเจ้าของ (Ownership)

fn main() {
    // 1. สร้างตัวแปร String โดย my_string จะเป็น "เจ้าของ" (Owner) ของข้อมูลนี้
    let my_string = String::from("Silpakorn");

    // 2. ส่ง REFERENCE ของ my_string ไปให้ฟังก์ชันโดยใช้เครื่องหมาย &
    // สิ่งนี้เรียกว่า "การยืม" (Borrowing) คือการให้ฟังก์ชันอื่นอ่านข้อมูลได้ แต่ไม่ได้ให้ความเป็นเจ้าของไป
    let length = calculate_length(&my_string);

    // 3. เนื่องจากเราแค่ "ให้ยืม" my_string จึงยังมีชีวิตอยู่และสามารถใช้งานต่อได้ในบรรทัดนี้!
    // (ถ้าเราไม่ใช้ & โค้ดบรรทัดนี้จะเกิด Error ตอน Compile ทันที)
    println!("ความยาวของคำว่า '{}' คือ {} ตัวอักษร", my_string, length);
}

// ฟังก์ชันนี้รับค่าเป็น Reference ของ String (&String) แทนที่จะยึดความเป็นเจ้าของมา
fn calculate_length(s: &String) -> usize {
    s.len() // คำนวณความยาวและส่งค่ากลับ
}

*/

// -----------------------------------English----------------------------------------
// 01_concept.rs
// Purpose: Demonstrating how References allow us to use data without taking ownership.

fn main() {
    // 1. We create a String. `my_string` is the "owner" of this data.
    let my_string = String::from("Silpakorn");

    // 2. We pass a REFERENCE to my_string using the `&` symbol.
    // This is called "Borrowing". We are letting the function look at the data, 
    // but we are NOT giving away ownership.
    let length = calculate_length(&my_string);

    // 3. Because we only "borrowed" it, my_string is still valid and we can still use it here!
    // (If we didn't use `&`, this line would cause a compile error).
    println!("The length of '{}' is {}.", my_string, length);
}

// This function takes a reference to a String (&String) instead of taking ownership.
fn calculate_length(s: &String) -> usize {
    s.len() // It calculates the length and returns it
}