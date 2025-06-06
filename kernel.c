void kmain() {
    // VGA buffer address
    volatile unsigned char *video = (volatile unsigned char*)0xB8000;
    const char *message = "Hello, World!";
    while (*message) {
        *video++ = *message++;
        *video = 0x07; // light grey on black
    }
    for (;;) {
    }
}
