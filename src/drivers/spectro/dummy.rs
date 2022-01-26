#[cfg(test)]
fn test_kb() {
    // test keys
    let kb_driver = spectro::KBDriver;
    // normally: wait for hardware to make interrupts
    // maybe we'll just test the handle_interrupt function directly for now, integration test the hardware or blackbox it somehow

    // press one key "A"
    let key_pressed = 0; // A = key 0
    let res = kb_driver.handle_interrupt(vec!(0));
    println!("key press recorded = {}", res);

    // press ctrl + A
    let keys_pressed = vec!(0, 10);
    let res = kb_driver.handle_interrupt(keys_pressed);
    println!("key presses recorded = {}", res);
}